use adw::subclass::prelude::*;
use ashpd::flatpak::HostCommandFlags;
use async_std::stream::StreamExt;
use gio::prelude::*;
use gio::SettingsBindFlags;
use glib::translate::FromGlib;
use glib::translate::IntoGlib;
use glib::ObjectExt;
use gtk::glib;
use gtk::graphene;
use gtk::CompositeTemplate;
use gtk::SystemSetting;
use tracing::*;
use vte::BoxExt;
use vte::CursorBlinkMode;
use vte::CursorShape;
use vte::EventControllerExt;
use vte::StyleContextExt;

use std::time::Duration;
use std::{cell::RefCell, collections::HashMap, env};

use glib::{clone, subclass::Signal, JoinHandle, Pid, SignalHandlerId, SpawnFlags, StaticType, Type, Value};
use once_cell::sync::Lazy;
use vte::{PopoverExt, PtyFlags, TerminalExt, TerminalExtManual, WidgetExt};

use crate::components::search_toolbar::SearchToolbar;
use crate::config::APP_NAME;
use crate::pcre2::PCRE2Flags;
use crate::services::settings::ScrollbackMode;
use crate::services::settings::Settings;
use crate::services::theme_provider::Theme;
use crate::services::theme_provider::ThemeProvider;

use super::constants::URL_REGEX_STRINGS;
use super::spawn::get_spawner;
use super::spawn::FlatpakSpawner;
use super::spawn::NativeSpawner;
use super::spawn::Spawner;
use super::*;

static TERMS_ENV: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut env = HashMap::from([
        ("TERM".to_string(), "xterm-256color".to_string()),
        ("COLORTERM".to_string(), "truecolor".to_string()),
        ("TERM_PROGRAM".to_string(), APP_NAME.to_string()),
        (
            "VTE_VERSION".to_string(),
            format!(
                "{}",
                vte::ffi::VTE_MAJOR_VERSION * 10000 + vte::ffi::VTE_MINOR_VERSION * 100 + vte::ffi::VTE_MICRO_VERSION
            ),
        ),
    ]);

    if let Some(themes_dir) = ThemeProvider::user_themes_dir() {
        env.insert("TERMS_THEMES_DIR".to_string(), themes_dir.to_string_lossy().into());
    }
    env
});

static TERMS_DEFAULT_SHELL: &str = "/bin/sh";

#[derive(Debug, Clone)]
pub struct SpawnArgs {
    pub cwd_path: PathBuf,
    pub argv: Vec<PathBuf>,
    pub envv: HashMap<String, String>,
}

#[derive(Debug)]
pub enum SpawnCtx {
    Native,
    Flatpak,
}

#[derive(Debug, Default)]
struct TerminalContext {
    // pub pid: Option<Pid>,
    pub exit_handler: Option<SignalHandlerId>,
    pub spawn_handle: Option<JoinHandle<()>>,
    pub spawn_ctx: Option<SpawnCtx>,
    pub drop_handler_id: Option<SignalHandlerId>,

    pub padding_provider: Option<gtk::CssProvider>,
}

#[derive(Debug, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal.ui")]
pub struct Terminal {
    pub init_args: RefCell<TerminalInitArgs>,
    pub spawner: Box<dyn Spawner>,

    pub settings: Settings,
    ctx: RefCell<TerminalContext>,

    #[template_child]
    term: TemplateChild<vte::Terminal>,

    #[template_child]
    search_toolbar: TemplateChild<SearchToolbar>,

    #[template_child]
    popover_menu: TemplateChild<gtk::PopoverMenu>,

    #[template_child]
    scrolled: TemplateChild<gtk::ScrolledWindow>,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            init_args: Default::default(),
            spawner: get_spawner(),
            settings: Default::default(),
            ctx: Default::default(),
            term: Default::default(),
            search_toolbar: Default::default(),
            popover_menu: Default::default(),
            scrolled: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Terminal {
    const NAME: &'static str = "TermsTerminal";
    type Type = super::Terminal;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Terminal {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("exit").param_types([i32::static_type()]).build()]);
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| vec![]);

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, _value: &Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            _ => unimplemented!(),
        }
    }

    fn dispose(&self) {
        if let Some(spawn_handle) = self.ctx.borrow_mut().spawn_handle.take() {
            if spawn_handle.as_raw_source_id().is_some() {
                spawn_handle.abort()
            }
        }

        // if let Some(drop_handler_id) = self.ctx.borrow_mut().drop_handler_id.take() {
        //     drop_handler_id.
        //     // if drop_handler_id.
        // }
    }
}

impl WidgetImpl for Terminal {}
impl BoxImpl for Terminal {}

#[gtk::template_callbacks]
impl Terminal {
    pub fn set_init_args(&self, init_args: TerminalInitArgs) {
        let mut args = self.init_args.borrow_mut();
        *args = init_args;
    }

    fn setup_widgets(&self) {
        ThemeProvider::default().connect_notify_local(
            Some("current-theme"),
            clone!(@weak self as this => move |_, _| {
               this.on_theme_changed();
            }),
        );

        self.settings.connect_system_font_changed(clone!(@weak self as this => move |_| {
            this.on_font_changed();
        }));
        self.settings.connect_custom_font_changed(clone!(@weak self as this => move |_| {
            this.on_font_changed();
        }));

        self.settings.connect_terminal_padding_changed(clone!(@weak self as this => move |_| {
            this.on_padding_changed();
        }));

        self.settings.connect_opacity_changed(clone!(@weak self as this => move |_| {
            this.on_theme_changed();
        }));

        self.setup_drag_drop();
        self.setup_regexes();
        self.connect_signals();
        self.bind_data();
        self.on_theme_changed();
        self.on_font_changed();
        self.on_padding_changed();

        self.spawn();
    }

    fn setup_drag_drop(&self) {
        let drop_target = gtk::DropTarget::new(gio::File::static_type(), gdk::DragAction::COPY | gdk::DragAction::MOVE);

        drop_target.set_types(&[gio::File::static_type(), String::static_type(), gdk::FileList::static_type()]);

        let drop_handler_id = drop_target.connect_drop(clone!(@weak self as this => @default-return true, move |_, value, _, _| {
            this.on_drop(value).into()
        }));

        self.ctx.borrow_mut().drop_handler_id = Some(drop_handler_id);

        self.term.add_controller(drop_target);
    }

    async fn spawn_async(&self) {
        let init_args = self.init_args.borrow().clone();

        let env = self.spawner.env().await;
        if let Err(err) = &env {
            error!("Could not get spawn env: {}", err);
            return;
        }

        // I don't think we need the extra env here
        // let mut env = env.unwrap();
        // env.extend(TERMS_ENV.clone());

        let mut env = TERMS_ENV.clone();

        let mut cmd = match self.settings.shell_command().map(|cmd| glib::shell_parse_argv(&cmd)) {
            Some(Ok(shell_command)) => shell_command.iter().map(PathBuf::from).collect(),
            _ => {
                info!("Getting shell from spawner");

                let shell = self.spawner.shell().await.unwrap_or(TERMS_DEFAULT_SHELL.to_string());
                info!("Got shell: {:?}", shell);

                let mut shell_command = vec![shell.into()];
                if self.settings.command_as_login_shell() && init_args.command.is_none() {
                    shell_command.push("--login".into())
                }
                shell_command
            },
        };

        if let Some(command) = &init_args.command {
            cmd.push("-c".into());
            cmd.push(command.into());
        }

        let working_dir = match init_args.working_dir {
            Some(working_dir) => working_dir,
            None => self.spawner.working_dir().await.unwrap_or(PathBuf::from("/")),
        };

        let flags = vte::PtyFlags::DEFAULT;
        info!("Spawning pty");

        let exit_fut = match self.spawner.spawn(&*self.term, flags, working_dir, cmd, env, Duration::from_secs(1)).await {
            Ok(handle) => {
                info!("Spawned pty with id: {:?}", handle.pid);
                handle.exit_fut
            },
            Err(err) => {
                error!("Could not spawn pty: {:?}", err);
                return;
            },
        };

        // Wait for child to exit
        self.on_child_exited(exit_fut.await);
    }

    fn spawn(&self) {
        info!("Spawn");

        let spawn_handle = glib::spawn_future_local(clone!(@weak self as this => async move {
            this.spawn_async().await
        }));

        self.ctx.borrow_mut().spawn_handle = Some(spawn_handle);
    }

    fn connect_signals(&self) {
        let handler = self.term.connect_child_exited(clone!(@weak self as this => move |_, status| {
                this.on_child_exited(status)
        }));
        self.ctx.borrow_mut().exit_handler = Some(handler);

        let keypress_controller = gtk::EventControllerKey::builder().build();
        keypress_controller.connect_key_pressed(
            clone!(@weak self as this =>  @default-return glib::Propagation::Stop, move |_, key, keycode, modifier| {
                this.on_key_pressed(key, keycode, modifier)
            }),
        );
        self.term.add_controller(keypress_controller);

        let primary_click = gtk::GestureClick::builder().button(gdk::BUTTON_PRIMARY).build();
        primary_click.connect_pressed(clone!(@weak self as this => move |gesture: &gtk::GestureClick, _: i32, x: f64, y: f64| {
            if let Some(event) = gesture.current_event() {
                if let (Some(match_str), _tag) = this.term.check_match_at(x, y) {
                    if event.modifier_state().contains(gdk::ModifierType::CONTROL_MASK) {

                        // TODO: get active window
                        glib::spawn_future_local(gtk::UriLauncher::new(&match_str).launch_future(None::<&gtk::Window>));
                    }
                }
            }
        }));

        self.term.add_controller(primary_click);

        let secondary_click = gtk::GestureClick::builder().button(gdk::BUTTON_SECONDARY).build();
        secondary_click.connect_pressed(clone!(@weak self as this => move |_: &gtk::GestureClick, _: i32, x: f64, y: f64| {
                                this.show_menu(x, y);
        }));

        self.term.add_controller(secondary_click);
    }

    fn setup_regexes(&self) {
        for reg_str in URL_REGEX_STRINGS {
            if let Ok(reg) = vte::Regex::for_match(reg_str, PCRE2Flags::MULTILINE.bits()) {
                let id = self.term.match_add_regex(&reg, 0);
                self.term.match_set_cursor_name(id, "pointer")
            }
        }
    }

    fn bind_data(&self) {
        self.settings.bind_pixel_scrolling(&self.term.clone(), "scroll-unit-is-pixels").build();
        self.settings.bind_use_sixel(&self.term.clone(), "enable-sixel").build();
        self.settings.bind_theme_bold_is_bright(&self.term.clone(), "bold-is-bright").build();
        self.settings.bind_terminal_bell(&self.term.clone(), "audible-bell").build();
        self.settings.bind_terminal_cell_width(&self.term.clone(), "cell-width-scale").build();
        self.settings.bind_terminal_cell_height(&self.term.clone(), "cell-height-scale").build();
        self.settings
            .bind_cursor_shape(&self.term.clone(), "cursor-shape")
            .mapping(|variant, _ty| variant.get::<i32>().map(|shape| unsafe { CursorShape::from_glib(shape) }.to_value()))
            .set_mapping(|value, _ty| value.get::<CursorShape>().ok().map(|shape| shape.into_glib().into()))
            .build();
        self.settings
            .bind_cursor_blink_mode(&self.term.clone(), "cursor-blink-mode")
            .mapping(|variant, _ty| variant.get::<i32>().map(|mode| unsafe { CursorBlinkMode::from_glib(mode) }.to_value()))
            .set_mapping(|value, _ty| value.get::<CursorBlinkMode>().ok().map(|mode| mode.into_glib().into()))
            .build();

        self.settings.bind_use_overlay_scrolling(&*self.scrolled, "overlay-scrolling").build();

        self.settings
            .bind_show_scrollbars(&*self.scrolled, "vscrollbar-policy")
            .get_only()
            .mapping(|variant, _ty| {
                variant
                    .get::<bool>()
                    .map(|show| if show { gtk::PolicyType::Automatic } else { gtk::PolicyType::Never }.to_value())
            })
            .build();

        self.settings.bind_scroll_on_keystroke(&*self.term, "scroll-on-keystroke").build();
        self.settings.bind_scroll_on_output(&*self.term, "scroll-on-output").build();

        self.settings.connect_scrollback_mode_changed(clone!(@weak self as this => move |_| {
            this.on_scrollback_changed();
        }));
        self.settings.connect_scrollback_lines_changed(clone!(@weak self as this => move |_| {
            this.on_scrollback_changed();
        }));
        self.on_scrollback_changed();
    }

    fn on_drop(&self, value: &Value) -> glib::Propagation {
        dbg!("Dropped value {:?}", value);

        if let Ok(file_list) = value.get::<gdk::FileList>() {
            for file in file_list.files() {
                self.feed_child_file(&file);
            }
            return glib::Propagation::Proceed;
        }

        if let Ok(file) = value.get::<gio::File>() {
            self.feed_child_file(&file);
            return glib::Propagation::Proceed;
        }

        if let Ok(text) = value.get::<String>() {
            self.term.feed_child(glib::shell_quote(&text).as_encoded_bytes());
            self.term.feed_child(" ".as_bytes());
            return glib::Propagation::Proceed;
        }

        warn!("You dropped something Terms can't handle yet :(");
        glib::Propagation::Stop
    }

    fn on_key_pressed(&self, key: gdk::Key, _keycode: u32, modifier: gdk::ModifierType) -> glib::Propagation {
        if !modifier.contains(gdk::ModifierType::CONTROL_MASK) {
            return glib::Propagation::Proceed;
        }

        // TODO: get shortcuts from system settings?
        match key.name().as_ref().map(glib::GString::as_str) {
            Some("c") => {
                if self.term.has_selection() && self.settings.easy_copy_paste() {
                    // TODO: allow html?
                    self.term.copy_clipboard_format(vte::Format::Text);
                    self.term.unselect_all();
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            },
            Some("v") => {
                if self.settings.easy_copy_paste() {
                    self.term.paste_clipboard();
                    glib::Propagation::Stop
                } else {
                    glib::Propagation::Proceed
                }
            },
            _ => glib::Propagation::Proceed,
        }
    }

    fn on_font_changed(&self) {
        // TODO: system font
        let font = self.settings.custom_font();
        self.term.set_font_desc(Some(&pango::FontDescription::from_string(&font)))
    }

    fn on_padding_changed(&self) {
        // TODO: move to themeprovider
        if let Some(padding_provider) = self.ctx.borrow_mut().padding_provider.take() {
            self.term.style_context().remove_provider(&padding_provider);
        }

        let (top, right, bottom, left) = self.settings.terminal_padding();

        let provider = gtk::CssProvider::new();
        provider.load_from_data(&format!("vte-terminal {{ padding: {}px {}px {}px {}px; }}", top, right, bottom, left));

        self.term.style_context().add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        self.ctx.borrow_mut().padding_provider = Some(provider);
    }

    fn background_color(&self, theme: &Theme) -> Option<gdk::RGBA> {
        theme.background.as_ref().cloned().map(|mut color| {
            color.set_alpha(self.settings.opacity() as f32 * 0.01);
            color
        })
    }

    fn scrollback_lines(&self) -> i64 {
        match self.settings.scrollback_mode() {
            ScrollbackMode::FixedSize => self.settings.scrollback_lines() as i64,
            ScrollbackMode::Unlimited => -1i64,
            ScrollbackMode::Disabled => 0i64,
        }
    }

    fn on_scrollback_changed(&self) {
        self.term.set_scrollback_lines(self.scrollback_lines());
    }

    fn on_theme_changed(&self) {
        if let Some(theme) = ThemeProvider::default().current_theme() {
            self.term.set_color_cursor_foreground(theme.cursor.as_ref());

            let bg = self.background_color(&theme);

            if let Some(palette) = theme.palette {
                self.term
                    .set_colors(theme.foreground.as_ref(), bg.as_ref(), palette.iter().collect::<Vec<&gdk::RGBA>>().as_slice());
            } else {
                if let Some(color) = bg.as_ref() {
                    self.term.set_color_background(color)
                }
                if let Some(color) = theme.foreground.as_ref() {
                    self.term.set_color_foreground(color)
                }
            }
        } else {
            warn!("No theme set")
        }
    }

    fn show_menu(&self, x: f64, y: f64) {
        let (match_str, tag) = self.term.check_match_at(x, y);

        // TODO: customize menu based on match_str
        dbg!("match {:?}, {:?}", match_str, tag);

        if let Some(point) = self.term.compute_point(self.obj().as_ref(), &graphene::Point::new(x as f32, y as f32)) {
            let r = gdk::Rectangle::new(point.x() as i32, point.y() as i32, 0, 0);

            self.popover_menu.set_has_arrow(true);
            self.popover_menu.set_halign(gtk::Align::Center);
            self.popover_menu.set_pointing_to(Some(&r));
            self.popover_menu.set_visible(true);
        }
    }

    fn feed_child_file(&self, file: &gio::File) {
        if let Some(path) = file.path() {
            self.term.feed_child(glib::shell_quote(&path).as_encoded_bytes());
            self.term.feed_child(" ".as_bytes());
        }
    }

    fn on_child_exited(&self, status: i32) {
        dbg!("Terminal child exited with status {}", status);
        // self.ctx.borrow_mut().pid = None;

        let handler = self.ctx.borrow_mut().exit_handler.take();
        match handler {
            None => error!("missing exit signal handler"),
            Some(handler) => self.term.disconnect(handler),
        };

        // TODO: terminal exit behaviour
        // let handler: SignalHandlerId = self.term.connect_child_exited(move |vte, _| {
        //     let mut cxb = ctx.borrow_mut();
        //     let behavior: TerminalExitBehavior =
        //         cxb.ctx.borrow().cfg.behavior.terminal_exit_behavior;
        //     match behavior {
        //         // TerminalExitBehavior::DropToDefaultShell => {
        //         //     todo!("DropToDefaultShell");
        //         // }
        //         // TerminalExitBehavior::RestartCommand => {
        //         //     todo!("RestartCommand");
        //         // }
        //         TerminalExitBehavior::ExitTerminal => {
        //             let handler = cxb.exit_handler.take();
        //             match handler {
        //                 None => error!("missing exit signal handler"),
        //                 Some(handler) => vte.disconnect(handler),
        //             };
        //             remove_page_by_hbox(&cxb.ctx, &cxb.hbox);
        //         }
        //     };
        // });

        self.obj().emit_by_name::<()>("exit", &[&status]);
    }

    // async fn spawn_native(&self, spawn_args: SpawnArgs) -> Result<Pid, glib::Error> {
    //     let spawner = NativeSpawner {};
    //     spawner.spawn(&*self.term, spawn_args.clone()).await;

    //     let SpawnArgs { cwd_path, argv, envv } = spawn_args;

    //     let args: Vec<&str> = argv.iter().map(|path| path.to_str().unwrap_or_default()).collect();

    //     self.term
    //         .spawn_future(
    //             PtyFlags::DEFAULT,
    //             Some(&cwd_path.to_string_lossy()),
    //             &args,
    //             &[],                 // TODO
    //             SpawnFlags::DEFAULT, // TODO
    //             || {},               // TODO
    //             10,                  // TODO
    //         )
    //         .await
    // }

    // async fn spawn_sandboxed(&self, spawn_args: SpawnArgs) -> Result<Pid, glib::Error> {
    //     let SpawnArgs { cwd_path, argv, envv } = spawn_args;

    //     let proxy = ashpd::flatpak::Development::new().await.unwrap();
    //     let fds = HashMap::new();
    //     let envs: HashMap<&str, &str> = envv.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
    //     let flags = HostCommandFlags::WatchBus;

    //     let pid = proxy.host_command(cwd_path, &argv, fds, envs, flags.into()).await.unwrap();
    //     let mut spawn_exit = proxy.receive_spawn_exited().await.unwrap();

    //     // proxy.host_command_signal(pid, signal, to_process_group)
    //     let spawn_handle = glib::spawn_future_local(clone!(@weak self as this => async move {

    //         loop {
    //             if let Some((pid, exit_status)) = spawn_exit.next().await {
    //                 this.on_child_exited(exit_status);
    //             }
    //         }

    //     }));

    //     todo!("spawn sandboxed")
    // }
}

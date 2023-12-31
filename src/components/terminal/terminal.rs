use adw::subclass::prelude::*;
use gio::prelude::*;
use gio::SettingsBindFlags;
use glib::ffi::G_REGEX_MULTILINE;
use glib::translate::FromGlib;
use glib::translate::IntoGlib;
use glib::ObjectExt;
use glib::ParamSpec;
use glib::Variant;
use gtk::glib;
use gtk::CompositeTemplate;
use std::borrow::Borrow;
use std::path::PathBuf;
use tracing::*;
use vte::CursorBlinkMode;
use vte::CursorShape;
use vte::StyleContextExt;

use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    env,
};

use glib::{clone, subclass::Signal, JoinHandle, Pid, Properties, SignalHandlerId, SpawnFlags, StaticType, Type, Value};
use once_cell::sync::Lazy;
use shell_quote::Sh;
use vte::{PopoverExt, PtyFlags, TerminalExt, TerminalExtManual, WidgetExt};

use crate::components::search_toolbar::SearchToolbar;
use crate::services::settings;
use crate::services::settings::Settings;
use crate::{services::sandbox, utils::constants::URL_REGEX_STRINGS};

use super::*;

struct SpawnArgs {
    pub cwd_path: PathBuf,
    pub argv: Vec<PathBuf>,
    pub envv: HashMap<String, String>,
}

#[derive(Debug, Default)]
struct TerminalCtx {
    pub pid: Option<Pid>,
    pub exit_handler: Option<SignalHandlerId>,
    pub spawn_handle: Option<JoinHandle<()>>,
    pub drop_handler_id: Option<SignalHandlerId>,

    pub original_scrollback_lines: Option<i64>,

    pub padding_provider: Option<gtk::CssProvider>,
}

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal.ui")]
pub struct Terminal {
    pub init_args: RefCell<TerminalInitArgs>,

    ctx: RefCell<TerminalCtx>,

    #[template_child]
    term: TemplateChild<vte::Terminal>,

    #[template_child]
    search_toolbar: TemplateChild<SearchToolbar>,

    #[template_child]
    popover_menu: TemplateChild<gtk::PopoverMenu>,

    pub settings: Settings,
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
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("exit").param_types([i64::static_type()]).build()]);
        SIGNALS.as_ref()
    }

    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| vec![glib::ParamSpecInt64::builder("user-scrollback-lines").readwrite().build()]);

        PROPERTIES.as_ref()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "user-scrollback-lines" => {
                if let Ok(lines) = value.get::<i64>() {
                    self.term.set_scrollback_lines(lines)
                }
            },
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "user-scrollback-lines" => self.term.scrollback_lines().to_value(),
            _ => unimplemented!(),
        }
    }

    fn dispose(&self) {
        if let Some(spawn_handle) = self.ctx.borrow_mut().spawn_handle.take() {
            if spawn_handle.as_raw_source_id().is_some() {
                spawn_handle.abort()
            }
        }

        if let Some(drop_handler_id) = self.ctx.borrow_mut().drop_handler_id.take() {
            // if drop_handler_id.
        }
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
        self.ctx.borrow_mut().original_scrollback_lines = Some(self.term.scrollback_lines());

        // ThemeProvider.get_default ().notify ["current-theme"].connect (this.on_theme_changed);

        self.settings.connect_font_changed(clone!(@weak self as this => move |_| {
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
                                this.on_drop(value)
        }));

        self.ctx.borrow_mut().drop_handler_id = Some(drop_handler_id);

        self.term.add_controller(drop_target);
    }

    fn spawn(&self) {
        let spawn_handle = glib::spawn_future_local(clone!(@weak self as this => async move {
                                let init_args = this.init_args.borrow().clone();

                                let spawn_args = SpawnArgs {
                                        // TODO: remove fallback when args are passed through
                                        cwd_path: init_args.working_dir.or(env::current_dir().ok()).unwrap_or(PathBuf::new()),
                                        argv: vec!["zsh".into()],
                                        envv: HashMap::new(),
                                };

                                let spawn_result = if sandbox::is_sandboxed().await {
                                        this.spawn_sandboxed(spawn_args).await
                                } else {
                                        this.spawn_native(spawn_args).await
                                } ;

                                match spawn_result {
                                        Ok(pid) => this.ctx.borrow_mut().pid = Some(pid),
                                        Err(err) => eprintln!("Could now spawn vte subprocess {:?}", err)
                                };
        }));

        self.ctx.borrow_mut().spawn_handle = Some(spawn_handle);
    }

    fn connect_signals(&self) {
        let handler = self.term.connect_child_exited(clone!(@weak self as this => move |_, status| {
                                this.on_child_exited(status)
        }));
        self.ctx.borrow_mut().exit_handler = Some(handler);

        let click = gtk::GestureClick::builder().button(gdk::BUTTON_SECONDARY).build();
        click.connect_pressed(clone!(@weak self as this => move |_: &gtk::GestureClick, _: i32, x: f64, y: f64| {
                                this.show_menu(x, y);
        }));

        self.term.add_controller(click);
    }

    fn setup_regexes(&self) {
        for reg_str in URL_REGEX_STRINGS {
            if let Ok(reg) = vte::Regex::for_match(reg_str, G_REGEX_MULTILINE) {
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

        self.obj()
            .bind_property("user-scrollback-lines", &self.term.clone(), "scrollback-lines")
            .sync_create()
            .build();

        // Fallback scrolling makes it so that VTE handles scrolling on its own. We
        // want VTE to let GtkScrolledWindow take care of scrolling if the user
        // enabled "show scrollbars". Thus we set
        // `enable-fallback-scrolling = !show-scrollbars`
        //
        // See:
        // - https://gitlab.gnome.org/raggesilver/blackbox/-/issues/179
        // - https://gitlab.gnome.org/GNOME/vte/-/issues/336
        self.settings
            .bind_show_scrollbars(&self.term.clone(), "enable-fallback-scrolling")
            .flags(SettingsBindFlags::INVERT_BOOLEAN)
            .build();
    }

    fn on_drop(&self, value: &Value) -> bool {
        dbg!("Dropped value {:?}", value);

        if let Ok(file_list) = value.get::<gdk::FileList>() {
            for file in file_list.files() {
                self.feed_child_file(&file);
            }
            return true;
        }

        if let Ok(file) = value.get::<gio::File>() {
            self.feed_child_file(&file);
            return true;
        }

        if let Ok(text) = value.get::<String>() {
            self.term.feed_child(&Sh::quote(&text));
            self.term.feed_child(" ".as_bytes());
            return true;
        }

        warn!("You dropped something Terms can't handle yet :(");
        false
    }

    fn on_font_changed(&self) {
        let font = self.settings.font();
        self.term.set_font_desc(Some(&pango::FontDescription::from_string(&font)))
    }

    fn on_padding_changed(&self) {
        if let Some(padding_provider) = self.ctx.borrow_mut().padding_provider.take() {
            self.term.style_context().remove_provider(&padding_provider);
        }

        let (top, right, bottom, left) = self.settings.terminal_padding();

        let provider = gtk::CssProvider::new();
        provider.load_from_data(&format!("vte-terminal {{ padding: {}px {}px {}px {}px; }}", top, right, bottom, left));

        self.term.style_context().add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        self.ctx.borrow_mut().padding_provider = Some(provider);
    }

    fn on_theme_changed(&self) {}

    fn show_menu(&self, x: f64, y: f64) {
        let (match_str, tag) = self.term.check_match_at(x, y);

        // TODO: customize menu based on match_str
        dbg!("match {:?}, {:?}", match_str, tag);

        let coords = self.term.translate_coordinates(self.obj().as_ref(), x, y).unwrap();

        let r = gdk::Rectangle::new(coords.0 as i32, coords.1 as i32, 0, 0);

        self.popover_menu.set_has_arrow(true);
        self.popover_menu.set_halign(gtk::Align::Center);
        self.popover_menu.set_pointing_to(Some(&r));
        self.popover_menu.show();
    }

    fn feed_child_file(&self, file: &gio::File) {
        if let Some(path) = file.path() {
            self.term.feed_child(&Sh::quote(&path.to_string_lossy().to_string()));
            self.term.feed_child(" ".as_bytes());
        }
    }

    fn on_child_exited(&self, status: i32) {
        dbg!("Terminal child exited with status {}", status);
        self.ctx.borrow_mut().pid = None;
        let handler = self.ctx.borrow_mut().exit_handler.take();
        match handler {
            None => eprintln!("missing exit signal handler"),
            Some(handler) => self.term.disconnect(handler),
        };

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
        //                 None => eprintln!("missing exit signal handler"),
        //                 Some(handler) => vte.disconnect(handler),
        //             };
        //             remove_page_by_hbox(&cxb.ctx, &cxb.hbox);
        //         }
        //     };
        // });

        self.obj().emit_by_name::<()>("exit", &[&status]);
    }

    async fn spawn_native(&self, spawn_args: SpawnArgs) -> Result<Pid, glib::Error> {
        let SpawnArgs { cwd_path, argv, envv } = spawn_args;

        let args: Vec<&str> = argv.iter().map(|path| path.to_str().unwrap_or_default()).collect();

        self.term
            .spawn_future(
                PtyFlags::DEFAULT,
                Some(&cwd_path.to_string_lossy()),
                &args,
                &[],                 // TODO
                SpawnFlags::DEFAULT, // TODO
                || {},               // TODO
                10,                  // TODO
            )
            .await
    }

    async fn spawn_sandboxed(&self, _spawn_args: SpawnArgs) -> Result<Pid, glib::Error> {
        todo!("spawn sandboxed")
    }
}
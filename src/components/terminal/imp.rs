use std::time::Duration;
use std::{cell::RefCell, collections::HashMap};

use adw::subclass::prelude::*;
use gio::prelude::*;
use glib::translate::FromGlib;
use glib::translate::IntoGlib;
use glib::Properties;
use gtk::glib;
use gtk::graphene;
use gtk::CompositeTemplate;

use tracing::*;
use vte::prelude::*;
use vte::TerminalExtManual;
use vte::{CursorBlinkMode, CursorShape};

use glib::{clone, subclass::Signal, JoinHandle, Value};
use once_cell::sync::Lazy;

use crate::components::search_toolbar::SearchToolbar;
use crate::config::APP_NAME;
use crate::pcre2::PCRE2Flags;
use crate::settings::ScrollbackMode;
use crate::settings::Settings;
use crate::theme_provider::Theme;
use crate::theme_provider::ThemeProvider;
use crate::util::EnvMap;

use super::constants::URL_REGEX_STRINGS;
use super::spawn::get_spawner;
use super::spawn::Spawner;
use super::*;

static TERMS_DEFAULT_SHELL: &str = "/bin/sh";
static TERMS_ENV: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut env = HashMap::from([
        ("TERM".to_string(), "xterm-256color".to_string()),
        ("COLORTERM".to_string(), "truecolor".to_string()),
        ("TERM_PROGRAM".to_string(), APP_NAME.to_string()),
        (
            "VTE_VERSION".to_string(),
            format!("{}", unsafe {
                vte::ffi::vte_get_major_version() * 10000 + vte::ffi::vte_get_minor_version() * 100 + vte::ffi::vte_get_micro_version()
            }),
        ),
    ]);

    env.insert("TERMS_THEMES_DIR".to_string(), ThemeProvider::user_themes_dir().to_string_lossy().into());

    env
});

#[derive(Debug, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/terminal.ui")]
#[properties(wrapper_type=super::Terminal)]
pub struct Terminal {
    pub spawner: Box<dyn Spawner>,

    pub settings: Settings,

    spawn_handle: RefCell<Option<JoinHandle<()>>>,

    padding_provider: RefCell<Option<gtk::CssProvider>>,

    update_source: Option<glib::SourceId>,

    #[template_child]
    term: TemplateChild<vte::Terminal>,

    #[template_child]
    terminal_menu: TemplateChild<gio::MenuModel>,

    #[template_child]
    search_toolbar: TemplateChild<SearchToolbar>,

    #[template_child]
    popover_menu: TemplateChild<gtk::PopoverMenu>,

    #[template_child]
    scrolled: TemplateChild<gtk::ScrolledWindow>,

    #[template_child]
    banner: TemplateChild<adw::Banner>,

    #[property(get, set, construct, nullable)]
    directory: RefCell<Option<PathBuf>>,

    #[property(get, set, construct, nullable)]
    command: RefCell<Option<String>>,

    #[property(get, set, construct, nullable)]
    env: RefCell<Option<EnvMap>>,

    #[property(get, set, construct, nullable)]
    title: RefCell<Option<String>>,

    #[property(get, set, construct, nullable)]
    icon: RefCell<Option<String>>,

    #[property(get)]
    uuid: String,

    // The last matched string under the cursor
    last_match: RefCell<Option<String>>,
}

impl Default for Terminal {
    fn default() -> Self {
        Self {
            settings: Default::default(),
            term: Default::default(),
            terminal_menu: Default::default(),
            spawn_handle: Default::default(),
            padding_provider: Default::default(),
            search_toolbar: Default::default(),
            popover_menu: Default::default(),
            scrolled: Default::default(),
            banner: Default::default(),

            directory: Default::default(),
            command: Default::default(),
            env: Default::default(),
            title: Default::default(),
            icon: Default::default(),

            uuid: glib::uuid_string_random().to_string(),
            spawner: get_spawner(),
            last_match: Default::default(),

            update_source: Default::default(),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Terminal {
    const NAME: &'static str = "TermsTerminal";
    type Type = super::Terminal;
    type ParentType = gtk::Widget;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();

        klass.install_action("clipboard.copy", None, |obj: &super::Terminal, _, _| {
            obj.imp().on_clipboard_copy();
        });
        klass.install_action("clipboard.copy-html", None, |obj: &super::Terminal, _, _| {
            obj.imp().on_clipboard_copy_html();
        });
        klass.install_action("clipboard.paste", None, |obj: &super::Terminal, _, _| {
            obj.imp().on_clipboard_paste();
        });
        klass.install_action("clipboard.copy-link", None, |obj: &super::Terminal, _, _| {
            obj.imp().on_clipboard_copy_link();
        });
        klass.install_action("win.open-link", None, |obj: &super::Terminal, _, _| {
            // TODO
            obj.imp().on_clipboard_copy_link();
        });
        // klass.install_action("clipboard.paste", None, |obj: &super::Terminal, _, payload| {
        //     if let Some(contents) = payload.and_then(|v| v.get::<String>()) {
        //         obj.imp().on_clipboard_paste(&contents);
        //     }
        // });
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for Terminal {
    fn constructed(&self) {
        self.parent_constructed();

        self.setup_widgets();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| vec![Signal::builder("exit").param_types([i32::static_type()]).build()]);
        SIGNALS.as_ref()
    }

    fn dispose(&self) {
        if let Some(spawn_handle) = self.spawn_handle.borrow_mut().take() {
            if spawn_handle.as_raw_source_id().is_some() {
                spawn_handle.abort()
            }
        }

        self.term.unparent();
        self.search_toolbar.unparent();
        self.popover_menu.unparent();
        self.scrolled.unparent();
        self.banner.unparent();
    }
}

impl WidgetImpl for Terminal {
    fn grab_focus(&self) -> bool {
        self.term.grab_focus()
    }
}

#[gtk::template_callbacks]
impl Terminal {
    fn setup_widgets(&self) {
        // self.process_manager.set_terminal(&*self.term);
        ThemeProvider::default().connect_notify_local(
            Some("current-theme"),
            clone!(@weak self as this => move |_, _| {
               this.on_theme_changed();
            }),
        );

        self.settings.connect_use_system_font_changed(clone!(@weak self as this => move |_| {
            this.on_font_changed();
        }));
        self.settings.connect_custom_font_changed(clone!(@weak self as this => move |_| {
            this.on_font_changed();
        }));

        self.settings
            .system_settings()
            .connect_gtk_font_name_notify(clone!(@weak self as this => move |_| {
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

        drop_target.connect_drop(clone!(@weak self as this => @default-return true, move |_, value, _, _| {
            this.on_drop(value).into()
        }));

        self.term.add_controller(drop_target);
    }

    async fn spawn_async(&self) {
        let env = self.spawner.env().await;
        if let Err(err) = &env {
            error!("Could not get spawn env: {}", err);
            return;
        }

        // I don't think we need the extra env here
        // let mut env = env.unwrap();
        // env.extend(TERMS_ENV.clone());
        let env = TERMS_ENV.clone();

        let mut cmd = match self.settings.shell_command().map(glib::shell_parse_argv) {
            Some(Ok(shell_command)) => shell_command.iter().map(PathBuf::from).collect(),
            _ => {
                info!("Getting shell from spawner");

                let shell = self.spawner.shell().await.unwrap_or(TERMS_DEFAULT_SHELL.to_string());
                info!("Got shell: {:?}", shell);

                let mut shell_command = vec![shell.into()];
                if self.settings.command_as_login_shell() && self.command.borrow().is_none() {
                    shell_command.push("--login".into())
                }
                shell_command
            },
        };

        if let Some(command) = self.command.borrow().as_ref() {
            cmd.push("-c".into());
            cmd.push(command.into());
        }

        info!("Working directory {:?}", self.directory.borrow());

        let working_dir = match self.directory.borrow().as_ref() {
            Some(working_dir) => working_dir.clone(),
            None => self.spawner.working_dir().await.unwrap_or(PathBuf::from("/")),
        };

        info!("Spawning pty");
        let flags = vte::PtyFlags::DEFAULT;
        let child_exit = match self.spawner.spawn(&self.term, flags, working_dir, cmd, env, Duration::from_secs(1)).await {
            Ok(handle) => {
                info!("Spawned pty with id: {:?}", handle.pid);
                handle.child_exit
            },
            Err(err) => {
                error!("Could not spawn pty: {:?}", err);
                return;
            },
        };

        // Wait for child to exit
        self.on_child_exited(child_exit.await);
    }

    fn spawn(&self) {
        info!("Spawn");
        let spawn_handle = glib::spawn_future_local(clone!(@weak self as this => async move {
            // TODO: show something if this errors
            this.spawn_async().await
        }));

        self.spawn_handle.borrow_mut().replace(spawn_handle);
    }

    fn connect_signals(&self) {
        let keypress_controller = gtk::EventControllerKey::builder().propagation_phase(gtk::PropagationPhase::Capture).build();
        keypress_controller.connect_key_pressed(
            clone!(@weak self as this =>  @default-return glib::Propagation::Stop, move |_, key, keycode, modifier| {
                this.on_key_pressed(key, keycode, modifier)
            }),
        );
        self.term.add_controller(keypress_controller);

        let primary_click_controller = gtk::GestureClick::builder().button(gdk::BUTTON_PRIMARY).build();
        primary_click_controller.connect_pressed(clone!(@weak self as this => move |gesture: &gtk::GestureClick, _: i32, x: f64, y: f64| {
            if let Some(event) = gesture.current_event() {
                if let (Some(match_str), _tag) = this.term.check_match_at(x, y) {
                    if event.modifier_state().contains(gdk::ModifierType::CONTROL_MASK) {


                        // TODO: get active window
                        // TODO: launch through xdg portal on flatpak
                        glib::spawn_future_local(gtk::UriLauncher::new(&match_str).launch_future(None::<&gtk::Window>));
                    }
                }
            }
        }));
        self.term.add_controller(primary_click_controller);

        let secondary_click_controller = gtk::GestureClick::builder().button(gdk::BUTTON_SECONDARY).build();
        secondary_click_controller.connect_pressed(clone!(@weak self as this => move |_: &gtk::GestureClick, _: i32, x: f64, y: f64| {
                                this.show_menu(x, y);
        }));

        self.term.add_controller(secondary_click_controller);
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
            self.term.feed_child(glib::shell_quote(text).as_encoded_bytes());
            self.term.feed_child(" ".as_bytes());
            return glib::Propagation::Proceed;
        }

        warn!("You dropped something Terms can't handle yet :(");
        glib::Propagation::Stop
    }

    fn on_key_pressed(&self, key: gdk::Key, _keycode: u32, modifier: gdk::ModifierType) -> glib::Propagation {
        if self.update_source.is_none() && !modifier.contains(gdk::ModifierType::CONTROL_MASK) {
            return glib::Propagation::Proceed;
        }

        if let Some(_update_source) = self.update_source.as_ref() {
            glib::Propagation::Proceed
        } else {
            // Handle easy copy/paste
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
    }

    fn on_font_changed(&self) {
        let font = if self.settings.use_system_font() {
            self.settings.system_settings().gtk_font_name().map(|f| f.to_string())
        } else {
            Some(self.settings.custom_font())
        };
        self.term.set_font_desc(font.map(|f| pango::FontDescription::from_string(&f)).as_ref())
    }

    fn on_padding_changed(&self) {
        // TODO: move to themeprovider
        if let Some(padding_provider) = self.padding_provider.borrow_mut().take() {
            gtk::style_context_remove_provider_for_display(&self.obj().display(), &padding_provider);
        }

        let (top, right, bottom, left) = self.settings.terminal_padding();

        let provider = gtk::CssProvider::new();
        provider.load_from_string(&format!("vte-terminal {{ padding: {}px {}px {}px {}px; }}", top, right, bottom, left));

        gtk::style_context_add_provider_for_display(&self.obj().display(), &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        self.padding_provider.borrow_mut().replace(provider);
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
        let match_str = match_str.map(|t| t.to_string());

        info!("match: {:?} {:?}", match_str, tag);

        if let Some(point) = self.term.compute_point(self.obj().as_ref(), &graphene::Point::new(x as f32, y as f32)) {
            let r = gdk::Rectangle::new(point.x() as i32, point.y() as i32, 0, 0);

            // TODO: what is `tag`?
            self.setup_context_menu(match_str, tag);

            self.popover_menu.set_has_arrow(true);
            self.popover_menu.set_halign(gtk::Align::Center);
            self.popover_menu.set_pointing_to(Some(&r));
            self.popover_menu.set_visible(true);
        }
    }

    fn setup_context_menu(&self, match_str: Option<String>, _tag: i32) {
        let has_match = match_str.is_some();
        *self.last_match.borrow_mut() = match_str;

        self.obj().action_set_enabled("clipboard.copy-link", has_match);
        self.obj().action_set_enabled("win.open-link", has_match);

        let clipboard = self.term.clipboard();
        let can_paste = clipboard.formats().contains_type(String::static_type());
        let has_selection = self.term.has_selection();

        self.obj().action_set_enabled("clipboard.copy", has_selection);
        self.obj().action_set_enabled("clipboard.copy-html", has_selection);

        self.obj().action_set_enabled("clipboard.paste", can_paste);
    }

    fn feed_child_file(&self, file: &gio::File) {
        if let Some(path) = file.path() {
            self.term.feed_child(glib::shell_quote(path).as_encoded_bytes());
            self.term.feed_child(" ".as_bytes());
        }
    }

    fn on_child_exited(&self, status: i32) {
        dbg!("Terminal child exited with status {}", status);
        // self.ctx.borrow_mut().pid = None;

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

    #[template_callback]
    fn on_contents_changed(&self) {
        self.enqueue_update();
    }

    #[template_callback]
    fn on_current_directory_uri_changed(&self) {
        let current_dir = self
            .term
            .current_directory_uri()
            .and_then(|uri_string| glib::Uri::parse(&uri_string, glib::UriFlags::NONE).ok())
            .map(|uri| PathBuf::from(uri.path()));
        *(self.directory.borrow_mut()) = current_dir;
        self.obj().notify_directory();

        self.update_title();
    }

    #[template_callback]
    fn update_title(&self) {
        let title = self.term.window_title().map(|t| t.to_string());

        // TODO: substitutions
        // ${title} 	The title of the terminal as reported by the terminal
        // ${iconTitle} 	The icon title of the terminal
        // ??? ${id} 	The numeric terminal ID (i.e. 1,2,3,4)
        // ${directory} 	The current working directory in the terminal
        // ${columns} 	The number of columns in the terminal
        // ${rows} 	the number of rows in the terminal
        // ${hostname} 	the hostname of the current session, availability dependent on the VTE script being configured on remote systems or triggers
        // ${username} 	the current username, requires trigger support and an appropriate trigger be configured

        *self.title.borrow_mut() = title;
        self.obj().notify_title();

        // TODO: icon title. vte icon title is deprecated
        // let icon = self.term.icon_title().map(|t| t.to_string());
        // *self.icon.borrow_mut() = icon;
        // self.obj().notify_icon();
    }

    fn on_clipboard_copy(&self) {
        let clipboard = self.term.clipboard();

        if let Some(text) = self.term.text_selected(vte::Format::Text) {
            clipboard.set_text(text.as_str());
            // TODO: show toast
        }
    }

    fn on_clipboard_copy_html(&self) {
        let clipboard = self.term.clipboard();

        if let Some(text) = self.term.text_selected(vte::Format::Html) {
            clipboard.set_text(text.as_str());
            // TODO: show toast
        }
    }

    fn on_clipboard_copy_link(&self) {
        // TODO: get url from match
    }

    fn on_clipboard_paste(&self) {
        self.term.paste_clipboard();
    }

    #[template_callback]
    fn on_has_focus(&self) {
        if self.term.has_focus() {
            debug!("term has focus");
        }
    }

    fn enqueue_update(&self) {
        // glib::timeout_add(interval, func)

        //   if G_UNLIKELY (self->update_source == NULL)
        //     {
        //       self->update_source = g_source_new ((GSourceFuncs *)&source_funcs, sizeof (GSource));
        //       g_source_set_callback (self->update_source,
        //                              prompt_tab_monitor_update_source_func,
        //                              self, NULL);
        //       g_source_set_static_name (self->update_source, "[prompt-tab-monitor]");
        //       g_source_set_priority (self->update_source, G_PRIORITY_LOW);
        //       prompt_tab_monitor_reset_delay (self);
        //       g_source_attach (self->update_source, NULL);
        //       return;
        //     }

        //   if G_UNLIKELY (self->current_delay_msec > DELAY_MIN_MSEC)
        //     {
        //       prompt_tab_monitor_reset_delay (self);
        //       return;
        //     }
    }

    // static gboolean
    // prompt_tab_monitor_update_source_func (gpointer user_data)
    // {
    //   PromptTabMonitor *self = user_data;
    //   g_autoptr(PromptTab) tab = NULL;
    //   PromptIpcProcess *process;

    //   g_assert (PROMPT_IS_TAB_MONITOR (self));

    //   if ((tab = g_weak_ref_get (&self->tab_wr)) &&
    //       (process = prompt_tab_get_process (tab)))
    //     {
    //       if (prompt_tab_poll_agent (tab))
    //         prompt_tab_monitor_reset_delay (self);
    //       else
    //         prompt_tab_monitor_backoff_delay (self);

    //       return G_SOURCE_CONTINUE;
    //     }

    //   g_clear_pointer (&self->update_source, g_source_unref);

    //   return G_SOURC
}

use std::cell::Cell;
use std::cell::OnceCell;
use std::cell::RefCell;
use std::marker::PhantomData;

use adw::subclass::prelude::*;
use gdk::prelude::*;
use gdk::KeyEvent;
use gdk::ModifierType;
use gettextrs::gettext;
use gio::prelude::*;
use glib::subclass::Signal;
use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use tracing::*;

use crate::config::PROFILE;
use crate::i18n::gettext_f;
use crate::settings::ShortcutSettings;

#[derive(Default, CompositeTemplate, Properties)]
#[template(resource = "/io/github/vhdirk/Terms/gtk/shortcut_dialog.ui")]
#[properties(wrapper_type=super::ShortcutDialog)]
pub struct ShortcutDialog {
    pub settings: ShortcutSettings,

    #[property(get, construct_only)]
    pub shortcut: OnceCell<String>,

    #[template_child]
    pub heading: TemplateChild<gtk::Label>,

    #[template_child]
    pub event_controller: TemplateChild<gtk::EventControllerKey>,

    #[template_child]
    pub shortcut_label: TemplateChild<gtk::ShortcutLabel>,

    #[template_child]
    pub accept_button: TemplateChild<gtk::Button>,

    #[property(get, set)]
    pub is_in_use: Cell<bool>,

    #[property(get, set, nullable)]
    pub accel: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for ShortcutDialog {
    const NAME: &'static str = "TermsShortcutDialog";
    type Type = super::ShortcutDialog;
    type ParentType = adw::Window;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[glib::derived_properties]
impl ObjectImpl for ShortcutDialog {
    fn constructed(&self) {
        self.parent_constructed();

        if PROFILE.should_use_devel_class() {
            let obj = self.obj();
            obj.add_css_class("devel");
        }

        self.setup();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![Signal::builder("response")
                .param_types([gtk::ResponseType::static_type(), Option::<String>::static_type()])
                .build()]
        });
        SIGNALS.as_ref()
    }
}

impl WidgetImpl for ShortcutDialog {}
impl WindowImpl for ShortcutDialog {}
impl AdwWindowImpl for ShortcutDialog {}

#[gtk::template_callbacks]
impl ShortcutDialog {
    fn setup(&self) {
        self.set_heading_text();

        self.obj().connect_response(move |obj, _, _| {
            obj.close();
        });

        self.obj().connect_accel_notify(move |obj| {
            obj.imp().accept_button.set_sensitive(obj.accel().is_some());
        });
    }

    fn set_heading_text(&self) {
        info!("Heading text");
        let text = self
            .shortcut
            .get()
            .map(|s| gettext_f("Enter new shortcut for \"{shortcut}\"", &[("shortcut", s)]))
            .unwrap_or(gettext("Enter new shortcut"));

        self.heading.set_text(&text);
    }

    #[template_callback]
    fn key_pressed(&self) -> bool {
        let event = match self.event_controller.current_event() {
            Some(event) => event,
            None => return false,
        };

        info!("key pressed event {:?}", event);

        let key_event = match event.downcast_ref::<KeyEvent>() {
            Some(key_event) => key_event,
            None => return false,
        };

        info!("key pressed key event {:?}", key_event);

        let valid_modifiers = gdk::ModifierType::CONTROL_MASK | gdk::ModifierType::SHIFT_MASK | gdk::ModifierType::ALT_MASK;

        let real_modifiers = valid_modifiers & key_event.modifier_state();

        let keyval = key_event.keyval();
        info!("key pressed keyval {:?}", keyval);

        let kt = gtk::KeyvalTrigger::new(keyval, real_modifiers);
        info!("key pressed keyval trigger {:?}", kt);

        if key_event.modifier_state().is_empty() {
            match keyval {
                gdk::Key::Escape => {
                    self.cancel();
                    return false;
                },
                gdk::Key::BackSpace => {
                    self.obj().set_accel(None::<String>);
                    self.apply();
                    return true;
                },
                _ => (),
            }
        }

        let accel = kt.to_str().to_string();

        info!("key pressed accel {:?}", accel);

        let gtk_accel = gtk::accelerator_parse(&accel);

        let consumed = key_event.consumed_modifiers();

        let mut mods = gtk_accel.as_ref().map(|(_, mods)| mods.clone()).unwrap_or(gdk::ModifierType::empty());
        mods &= !consumed;

        let is_valid = gtk_accel.is_some() &&
        // This is a very stupid way to check if the keyval is not Control_L,
        // Shift_L, or Alt_L. We don't want these keys to be valid.
        !keyval.name().map(|kv| kv.ends_with("_L")).unwrap_or(false) &&
        !keyval.name().map(|kv| kv.ends_with("_R")).unwrap_or(false) &&
        // Unless keyval is one of the Function keys, shortcuts need to have a
        // modifier.
        (
          (keyval >= gdk::Key::F1 && keyval <= gdk::Key::F35) || (mods.bits() > 0)
        );

        self.shortcut_label.set_accelerator(&accel);

        // TODO: since we can get the name of the action that is currently using
        // this shortcut we should show it to the user
        //
        // E.g.: This shortcut is currently assigned to "Reset Zoom"
        let in_use = self.settings.accel_in_use(&accel).is_some();
        self.obj().set_is_in_use(in_use);

        let is_shortcut_set = is_valid && !in_use;

        self.obj().set_accel(if is_shortcut_set { Some(accel) } else { None });

        true
    }

    #[template_callback]
    fn cancel(&self) {
        self.obj().emit_by_name::<()>("response", &[&gtk::ResponseType::Cancel, &None::<String>]);
    }

    #[template_callback]
    fn apply(&self) {
        info!("Apply accel");
        self.obj().emit_by_name::<()>("response", &[&gtk::ResponseType::Apply, &self.obj().accel()]);
    }
}

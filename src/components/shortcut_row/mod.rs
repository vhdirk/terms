mod shortcut_row;
use shortcut_row as imp;

pub const ACTION_ADD_SHORTCUT: &str = "shortcuts.add";
pub const ACTION_REMOVE_SHORTCUT: &str = "shortcuts.remove";
pub const ACTION_RESET_SHORTCUTS: &str = "shortcuts.reset";
pub const ACTION_RESET_ALL_SHORTCUTS: &str = "shortcuts.reset-all";

glib::wrapper! {
        pub struct ShortcutRow(ObjectSubclass<imp::ShortcutRow>)
                @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
                @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ShortcutRow {
    pub fn new(action: &str) -> Self {
        glib::Object::builder().property("action", action).build()
    }
}

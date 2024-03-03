use gtk::prelude::*;
use tracing::*;

const TWL_PANED_CSS_CLASS: &str = "twl-paned";
pub trait PanedExt {
    fn new(orientation: gtk::Orientation) -> Self;
    fn is_twl_paned(&self) -> bool;
    fn replace(&self, child: Option<&impl IsA<gtk::Widget>>, new_child: Option<&impl IsA<gtk::Widget>>);
    fn sibling(&self, child: Option<&impl IsA<gtk::Widget>>) -> Option<gtk::Widget>;
}

impl PanedExt for gtk::Paned {
    fn new(orientation: gtk::Orientation) -> Self {
        let paned = gtk::Paned::new(orientation);
        paned.add_css_class(TWL_PANED_CSS_CLASS);
        paned
    }

    fn is_twl_paned(&self) -> bool {
        self.has_css_class(TWL_PANED_CSS_CLASS)
    }

    fn replace(&self, child: Option<&impl IsA<gtk::Widget>>, new_child: Option<&impl IsA<gtk::Widget>>) {
        debug!("Panel::replace {:?} with {:?}", child, new_child);
        debug!("Panel::start_child {:?}", self.start_child());
        debug!("Panel::end_child {:?}", self.end_child());

        let child = child.map(AsRef::as_ref);
        let new_child = new_child.map(AsRef::as_ref);

        if self.start_child().as_ref() == child {
            self.set_start_child(new_child);
        } else if self.end_child().as_ref() == child {
            self.set_end_child(new_child);
        } else {
            warn!("Not a parent of child {:?}", child);
        }
    }

    fn sibling(&self, child: Option<&impl IsA<gtk::Widget>>) -> Option<gtk::Widget> {
        debug!("Panel::sibling {:?}", child);
        let child = child.map(AsRef::as_ref);

        if self.start_child().as_ref() == child {
            self.end_child()
        } else if self.end_child().as_ref() == child {
            self.start_child()
        } else {
            warn!("Not a parent of child {:?}", child);
            None
        }
    }
}

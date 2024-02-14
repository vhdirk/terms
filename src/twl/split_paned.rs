use gtk::prelude::*;
use tracing::*;

pub trait SplitPaned {
    fn replace(&self, child: Option<&impl IsA<gtk::Widget>>, new_child: Option<&impl IsA<gtk::Widget>>);
    fn sibling(&self, child: Option<&impl IsA<gtk::Widget>>) -> Option<gtk::Widget>;
}

impl SplitPaned for gtk::Paned {
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

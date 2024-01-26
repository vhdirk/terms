use glib::subclass::prelude::*;
use std::{cell::RefCell, collections::HashMap};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct EnvMap {
        pub inner: RefCell<HashMap<String, String>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EnvMap {
        const NAME: &'static str = "TermsEnvMap";
        type Type = super::EnvMap;
        type ParentType = glib::Object;
    }

    impl ObjectImpl for EnvMap {}
}

glib::wrapper! {
        pub struct EnvMap(ObjectSubclass<imp::EnvMap>);
}

impl Default for EnvMap {
    fn default() -> Self {
        Self::new()
    }
}

impl From<HashMap<String, String>> for EnvMap {
    fn from(value: HashMap<String, String>) -> Self {
        let this = Self::new();
        this.imp().inner.borrow_mut().extend(value);
        this
    }
}

impl Into<HashMap<String, String>> for EnvMap {
    fn into(self) -> HashMap<String, String> {
        self.imp().inner.take()
    }
}

impl EnvMap {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn clear(&self) {
        self.imp().inner.borrow_mut().clear()
    }
}

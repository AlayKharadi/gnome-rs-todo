use std::cell::{OnceCell, RefCell};

use adw::prelude::*;
use adw::subclass::prelude::*;
use adw::glib;
use glib::Properties;
use gtk::gio::ListStore;

#[derive(Properties, Default)]
#[properties(wrapper_type=super::CollectionObject)]
pub struct CollectionObject {
    #[property(get, set)]
    pub title: RefCell<String>,
    #[property(get, set)]
    pub tasks: OnceCell<ListStore>
}


#[glib::object_subclass]
impl ObjectSubclass for CollectionObject {
    const NAME: &'static str = "TodoCollectionObject";
    type Type = super::CollectionObject;
}

#[glib::derived_properties]
impl ObjectImpl for CollectionObject {}
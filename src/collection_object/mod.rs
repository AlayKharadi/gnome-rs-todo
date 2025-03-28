mod imp;

use adw::glib;
use adw::{gio::ListStore, glib::Object};
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use gtk::prelude::ListModelExtManual;
use serde::{Deserialize, Serialize};

use crate::task_object::{TaskData, TaskObject};

glib::wrapper!{
    pub struct CollectionObject(ObjectSubclass<imp::CollectionObject>);
}


impl CollectionObject {
    pub fn new(title: &str, tasks: ListStore) -> Self {
        Object::builder()
            .property("title", title)
            .property("tasks", tasks)
            .build()
    }

    pub fn to_collection_data(&self) -> CollectionData {
        let title = self.imp().title.borrow().clone();
        let tasks_data = self
            .tasks()
            .iter::<TaskObject>()
            .filter_map(Result::ok)
            .map(|task_object| task_object.task_data())
            .collect();
        CollectionData { title, tasks_data }
    }

    pub fn from_collection_data(collection_data: CollectionData) -> Self {
        let title = collection_data.title;
        let tasks_to_extend: Vec<TaskObject> = collection_data
            .tasks_data
            .into_iter()
            .map(TaskObject::from_task_data)
            .collect();

        let tasks = ListStore::new::<TaskObject>();
        tasks.extend_from_slice(&tasks_to_extend);

        Self::new(&title, tasks)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CollectionData {
    pub title: String,
    pub tasks_data: Vec<TaskData>
}
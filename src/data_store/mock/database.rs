use std::{any::Any, collections::HashMap};

use crate::data_store::Table;

pub struct Database {
    tables: HashMap<&'static str, Box<dyn Any>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table<T: Table + 'static>(&mut self, table: T) {
        self.tables.insert(table.name(), Box::new(table));
    }

    pub fn get_table<T: Table + 'static>(&self, name: &'static str) -> Option<&T> {
        self.tables
            .get(name)
            .and_then(|table| table.downcast_ref::<T>())
    }

    pub fn get_table_mut<T: Table + 'static>(&mut self, name: &'static str) -> Option<&mut T> {
        self.tables
            .get_mut(name)
            .and_then(|table| table.downcast_mut::<T>())
    }
}

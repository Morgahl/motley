pub mod data_store;
pub mod http;

use std::{fmt::Display, hash::Hash};

pub trait Identifier: Copy + Display + Eq + Hash + Ord {}

impl<T> Identifier for T where T: Copy + Display + Eq + Hash + Ord {}

pub trait Identifiable {
    type Id: Identifier;
    fn id(&self) -> Self::Id;
    fn set_id(&mut self, id: Self::Id);
}

impl<Id> PartialEq for dyn Identifiable<Id = Id>
where
    Id: Identifier,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T> Eq for dyn Identifiable<Id = T> where T: Identifier {}

impl<T> PartialOrd for dyn Identifiable<Id = T>
where
    T: Identifier + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl<T> Ord for dyn Identifiable<Id = T>
where
    T: Identifier + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

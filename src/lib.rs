pub mod data_source;
pub mod transport;

use std::{fmt::Display, hash::Hash};

pub trait Identifiable {
    type Id: Default + Display + Eq + Hash + Copy;
    fn id(&self) -> Self::Id;
    fn set_id(&mut self, id: Self::Id);
}

impl<T> PartialEq for dyn Identifiable<Id = T>
where
    T: Default + Display + Eq + Hash + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl<T> Eq for dyn Identifiable<Id = T> where T: Default + Display + Eq + Hash + Copy {}

impl<T> PartialOrd for dyn Identifiable<Id = T>
where
    T: Default + Display + Eq + Hash + Copy + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id().partial_cmp(&other.id())
    }
}

impl<T> Ord for dyn Identifiable<Id = T>
where
    T: Default + Display + Eq + Hash + Copy + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

pub trait IdentifiableGenerator<Id: Identifiable> {
    fn generate_id(&mut self) -> Option<Id::Id>;
}

impl<T, Id: Identifiable<Id = T>> IdentifiableGenerator<Id> for Id
where
    Id: Iterator<Item = Id::Id>,
{
    fn generate_id(&mut self) -> Option<Id::Id> {
        self.next()
    }
}

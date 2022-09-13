use std::{fmt::Display, hash::Hash, str::FromStr};

pub trait Identifier: Copy + Display + Eq + FromStr + Hash + Send + Sync + 'static {}

impl<T> Identifier for T where T: Copy + Display + Eq + FromStr + Hash + Send + Sync + 'static {}

pub trait Data: Clone + Display + Send + Sync {}

pub trait Entity<Data>
where
    Self: Display + Sized,
    Self::Id: Identifier,
    Data: super::Data,
{
    type Id;

    fn new(id: Self::Id, init: Self) -> Self;

    fn id(&self) -> Self::Id;
    fn data(&self) -> &Data;
}

// Foo is a basic entity validation implementation but can be used to effectviely act as a uniquelly
// identifiable entity within the framework... Oh god I used the word framework... I'm so sorry...
#[derive()]
pub struct Foo<Id, Data>
where
    Id: Identifier,
    Data: super::Data,
{
    pub id: Id,
    pub data: Data,
}

impl<Id, Data> Entity<Data> for Foo<Id, Data>
where
    Id: Identifier,
    Data: super::Data,
{
    type Id = Id;

    fn new(id: Self::Id, init: Self) -> Self {
        Self {
            id,
            data: init.data,
        }
    }

    fn id(&self) -> Self::Id {
        self.id
    }

    fn data(&self) -> &Data {
        &self.data
    }
}

impl<Id, Data> Display for Foo<Id, Data>
where
    Id: Identifier,
    Data: super::Data,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo {{ id: {}, data: {} }}", self.id, self.data)
    }
}

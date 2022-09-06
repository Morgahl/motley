use std::hash::Hash;

pub trait Identifiable {
    type Id: Eq + Hash;
    fn id(&self) -> Self::Id;
}

pub mod http;

use std::hash::Hash;

pub trait Identifiable {
    type Id: Eq + Hash + Default;
    fn id(&self) -> Self::Id;
}

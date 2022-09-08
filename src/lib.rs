pub mod http;

use std::{fmt::Display, hash::Hash};

pub trait Identifiable {
    type Id: Eq + Hash + Default + Display;
    fn id(&self) -> Self::Id;
}

use crate::resource::Resource;

pub trait Relation<W: Resource>: Resource {}

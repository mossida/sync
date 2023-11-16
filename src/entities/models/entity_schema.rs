pub struct EntitySchema {}

pub trait EntitySchemaManager {
    fn register();

    fn is_registered() -> bool;
}

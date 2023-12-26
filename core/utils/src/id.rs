use surrealdb::opt::RecordId;

pub trait NewId: From<String> {
    const TABLE: &'static str;

    fn id(&self) -> String;

    fn record(&self) -> RecordId;

    fn new() -> Self;
}

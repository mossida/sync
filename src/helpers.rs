use surrealdb::Error;

pub struct Helper {}

impl Helper {
    pub fn reject_db(_: Error) -> warp::reject::Rejection {
        warp::reject()
    }
}

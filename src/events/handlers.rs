use crate::api::rejections::{Rejection, RejectionCode};
use crate::db;
use crate::events::models::Event;
use crate::helpers::Helper;

pub async fn fetch_events() -> Result<impl warp::Reply, warp::Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM event")
        .await
        .map_err(Helper::reject_db)?;

    let list = response.take::<Vec<Event>>(0).map_err(|e| Rejection {
        reason: RejectionCode::DATABASE,
        message: e.to_string(),
    })?;

    Ok(warp::reply::json(&list))
}

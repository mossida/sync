use crate::db;
use crate::errors::Error;
use crate::events::models::Event;

pub async fn fetch_events() -> Result<impl warp::Reply, warp::Rejection> {
    let mut response = db::get()
        .query("SELECT * FROM event")
        .await
        .map_err(Error::from)?;
    let list = response.take::<Vec<Event>>(0).map_err(Error::from)?;

    Ok(warp::reply::json(&list))
}

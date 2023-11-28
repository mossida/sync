use crate::devices::api::list_all;

pub async fn fetch_devices() -> Result<impl warp::Reply, warp::Rejection> {
    let list = list_all().await.map_err(warp::reject::custom)?;
    Ok(warp::reply::json(&list))
}

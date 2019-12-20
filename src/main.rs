use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

#[tokio::main]
async fn main() {
    let db = Arc::new(Mutex::new(0));
    let db = warp::any().map(move || Arc::clone(&db));
    let routes = warp::path("counter").and(db.clone()).and_then(counter);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn counter(db: Arc<Mutex<u8>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut counter = db.lock().await;
    *counter += 1;
    Ok(counter.to_string())
}

use warp::Filter;

#[tokio::main]
async fn main() {
    let server = warp::serve(main_filter());

    server.run(([127, 0, 0, 1], 8080)).await;
}

fn main_filter() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("api").and(
        warp::get()
            .and(warp::path!("todos"))
            .map(|| warp::reply::json(&Vec::<()>::new())),
    )
}

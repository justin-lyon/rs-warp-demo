use warp::Filter;

async fn say_hi(param: String) -> Result<String, warp::Rejection> {
    Ok(format!("Hello {}", param))
}

async fn say_bye(param: String) -> Result<String, warp::Rejection> {
    Ok(format!("Goodbye {}", param))
}

pub fn greetings_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    let greet = warp::get()
        .and(warp::path("hi"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(say_hi);

    let farewell = warp::get()
        .and(warp::path("bye"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and_then(say_bye);

    greet.or(farewell)
}

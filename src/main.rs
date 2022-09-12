mod routes;

use crate::routes::greetings::greetings_filter;

#[tokio::main]
async fn main() {
    let routes = greetings_filter();

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

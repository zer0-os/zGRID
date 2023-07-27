mod routes;

use std::sync::Arc;
use crate::repository::Repository;

pub async fn start_server(arc_repository: Arc<Repository>) {
    let routes = routes::routes(arc_repository);

    let addr = ([192, 168, 1, 203], 3690); //let addr = ([127, 0, 0, 1], 3690);
    let ip = format!("{}.{}.{}.{}", addr.0[0], addr.0[1], addr.0[2], addr.0[3]);

    println!(" ");
    println!("<***/***>");
    println!("You are connected to THE GRID.");
    println!("ZERO Node Live :: http://{}:{}", ip, addr.1);
    println!("<***/***>");

    warp::serve(routes).run(addr).await;
}

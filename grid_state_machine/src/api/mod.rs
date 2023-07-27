mod routes;

use std::sync::Arc;
use std::error::Error;
use crate::repository::Repository;

pub async fn start_server(
    arc_repository: Arc<Repository>
) -> Result<(), Box<dyn Error>> {
    let routes = routes::routes(arc_repository);

    let addr = ([192, 168, 1, 203], 3690); //let addr = ([127, 0, 0, 1], 3690);
    let ip = format!("{}.{}.{}.{}", addr.0[0], addr.0[1], addr.0[2], addr.0[3]);

    println!();
    println!("<***/***>");
    println!("You are connected to THE GRID.");
    println!("ZERO Node Live :: http://{}:{}", ip, addr.1);
    println!("<***/***>");

    warp::serve(routes).run(addr).await;

    Err("Server failed to start.".into())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::repository::Repository;
    use crate::db::DatabaseState;
    use crate::api::{start_server};
    use std::future::Future;

    fn init_repository() -> Arc<Repository> {
        let db_path: String = "./test_db_api".to_string();
        let db_state: DatabaseState = DatabaseState::init(db_path);
        let arc_repository = Arc::new(Repository::new(db_state));
        arc_repository
    }

    #[tokio::test]
    async fn test_start_server() {
        let repository = init_repository();
        let server_fut = start_server(repository);
        // ToDo: Add assertion logic here
    }
}

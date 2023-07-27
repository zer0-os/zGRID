mod db;
mod api;
mod repository;

use db::DatabaseState;
use api::{start_server};
use repository::Repository;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_path: String = "./grid_db".to_string();
    let db_state: DatabaseState = DatabaseState::init(db_path);
    let arc_repository = Arc::new(Repository::new(db_state));
    start_server(arc_repository).await;
}

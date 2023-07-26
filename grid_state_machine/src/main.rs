mod db;
mod api;
mod repository;

use db::{DatabaseState};
use api::{start_server};
use repository::Repository;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db_state: DatabaseState = DatabaseState::init();
    let arc_repository = Arc::new(Repository::new(db_state));
    start_server(arc_repository).await;
}

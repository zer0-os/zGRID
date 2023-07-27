use warp::{
    http::StatusCode,
    Filter, Reply, Rejection,
};
use crate::repository::Repository;
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use serde_json::Error;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    data: String,
}

#[derive(Debug)]
struct CustomRejection {
    message: String,
    status_code: StatusCode,
}

impl warp::reject::Reject for CustomRejection {}

impl CustomRejection {
    fn message(&self) -> String {
        format!("Status Code: {}: {}", self.status_code, self.message)
    }
}


pub fn routes(
    arc_repository: Arc<Repository>
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let route_get_transaction = warp::path("transaction")
        .and(warp::path("get"))
        .and(warp::path::param::<i32>())
        .and(handle_repository_injection(Arc::clone(&arc_repository)))
        .and_then(handle_get_transaction);

    let route_post_transaction = warp::path("transaction")
        .and(warp::path("post"))
        .and(warp::post())
        .and(handle_repository_injection(Arc::clone(&arc_repository)))
        .and(warp::body::json())
        .and_then(handle_post_transaction);

    let routes = 
        route_get_transaction
        .or(route_post_transaction);

    routes
}


pub async fn handle_get_transaction(
    key: i32, 
    arc_repository: Arc<Repository>
) -> Result<impl Reply, Rejection> {
    let return_value = arc_repository.get_transaction(&key);
    
    match return_value {
        Ok(transaction_bytes) => {
            let transaction_json = bytes_to_json(transaction_bytes).unwrap();

            println!("Success: Request received and fulfilled");
            println!("JSON: {}", transaction_json);

            Ok(warp::reply::with_status(transaction_json, StatusCode::OK))
        },
        Err(e) => {      
            let rejection = handle_custom_rejection
                (e, "Object not inserted", StatusCode::NOT_FOUND);
            let _custom_rejection_message = rejection.message();
            
            Err(warp::reject::custom(rejection))
        }
    }
}


pub async fn handle_post_transaction(
    arc_repository: Arc<Repository>, 
    transaction: Transaction
) -> Result<impl Reply, Rejection> {    
    let key = generate_random_index(1, 100000000);

    println!("API: Key used: {}", key);

    let transaction_string = serde_json::to_string(&transaction).unwrap();
    let transaction_bytes = transaction_string.as_bytes().to_vec();

    let return_value = arc_repository.add_transaction(&key, transaction_bytes);

    match return_value {
        Ok(()) => Ok(warp::reply::with_status("Succes", StatusCode::OK)),
        Err(e) => {
            let rejection = handle_custom_rejection
                (e, "Object not found", StatusCode::NO_CONTENT);
            let _custom_rejection_message = rejection.message();
 
            Err(warp::reject::custom(rejection))
        }
    }
}


fn handle_repository_injection(
    arc_repository: Arc<Repository>
) -> impl Filter<Extract = (
        Arc<Repository>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&arc_repository))
}


fn handle_custom_rejection(error_msg: String, message: &str, status_code: StatusCode) -> CustomRejection {
    eprintln!("Error: {}", error_msg);
    CustomRejection {
        message: message.to_string(),
        status_code
    }
}


fn bytes_to_json(bytes: Vec<u8>) -> Result<String, Error> {
    let input_bytes: Transaction = serde_json::from_slice(&bytes)
        .expect("Failed to deserialize Vec<u8> into data structure");
    let output_json = serde_json::to_string_pretty(&input_bytes)
        .expect("Failed to serialize to human-readable JSON");
    Ok(output_json)
}

fn generate_random_index(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::error::Error;
    use warp::http::StatusCode;
    use warp::Rejection;
    use crate::Repository;
    use crate::db::{DatabaseState};
    use crate::api::routes::routes;

    fn init_repository() -> Arc<Repository> {
        let db_path: String = "./test_db_routing".to_string();
        let db_state: DatabaseState = <DatabaseState>::init(db_path);
        let arc_repository = Arc::new(Repository::new(db_state));
        arc_repository
    }

    #[test]
    fn test_get_transaction() {
        let arc_repository = init_repository();
        
        let route = routes(Arc::clone(&arc_repository));
        
        let request = warp::test::request()
            .method("GET")
            .path("/transaction/get/123");
        
        //ToDo: Implement rest of test
            //let response = request.reply(&route);     
            //println!("res {}", response);
            //assert_eq!(response.status(), 200);
    }
}

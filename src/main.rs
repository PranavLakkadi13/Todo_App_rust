use axum::{
    routing::{get, patch},
    Router,
};

use sqlx::postgres::PgPoolOptions;

use tokio::net::TcpListener;

mod task_ops;
use task_ops::{create_tasks, delete_tasks, get_tasks, update_tasks};

#[tokio::main]
async fn main() {
    // expose env variables
    dotenvy::dotenv().expect("The env file is not accessed");

    // set variables from the env variables
    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE URL is not found!!!");

    // create a db pool
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Can't connect to the db....");

    // create a TCP listner
    let listner = TcpListener::bind(&server_addr)
        .await
        .expect("TCP listener failed to connect..");

    println!("listening on {}", listner.local_addr().ok().unwrap());

    // compose the routes
    let app = Router::new()
        .route("/", get(|| async { "hello world " }))
        .route("/tasks", get(get_tasks).post(create_tasks))
        .route("/tasks/{id}", patch(update_tasks).delete(delete_tasks))
        .with_state(db_pool);

    // .. start the application
    axum::serve(listner, app)
        .await
        .expect("Error Starting the Server...");
}

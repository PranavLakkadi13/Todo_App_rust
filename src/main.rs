use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};

use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};

use tokio::net::TcpListener;


#[tokio::main]
async fn main() {
    
    // expose evn variables 

    // set variables from the env variables 

    // create a db pool

    // create a TCP listner 

    // ompose the routes 

    // .. start the application

}

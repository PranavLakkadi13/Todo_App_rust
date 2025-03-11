use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

// this is type aliasing
pub type Db = Pool<Postgres>;

const PG_POOL_HOST: &str = "localhost";
const PG_POOL_DB: &str = "postgres";
const PG_POOL_ROOT_USER: &str = "postgres";
const PG_POOL_PWD: &str = "postgres";

// APP_DB
const PG_APP_DB: &str = "app_db";
const PG_APP_USER: &str = "app_user";
const PG_APP_PWD: &str = "app_pwd_to_change";
const PG_APP_MAX_CONN: u32 = 5;

// SQL routes
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00_recreate_db.sql";

pub async fn init_db() -> Result<Db, sqlx::Error> {
    // CREATE a root db on with the DEV
    {
        let root_db = new_db(PG_POOL_HOST, PG_POOL_DB, PG_POOL_ROOT_USER, PG_POOL_PWD, 1).await?;
        pg_exec(&root_db, SQL_RECREATE).await?;
    }

    // we need to create the app db
    let app_db = new_db(
        PG_POOL_HOST,
        PG_APP_DB,
        PG_APP_USER,
        PG_APP_PWD,
        PG_APP_MAX_CONN,
    )
    .await?;
    // here we will go through the directory and get the paths
    let mut path_buf: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    path_buf.sort();
    // after we got the paths we will execute the files
    for path in path_buf {
        if let Some(path) = path.to_str() {
            if path.ends_with(".sql") && path != SQL_RECREATE {
                pg_exec(&app_db, &path).await?;
            }
        }
    }

    // returning the app db
    new_db(
        PG_POOL_HOST,
        PG_APP_DB,
        PG_APP_USER,
        PG_APP_PWD,
        PG_APP_MAX_CONN,
    )
    .await
}

async fn pg_exec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
    // the 00 - > file will be executed as root
    // next 2 will be executed as pg admin
    // read the file
    let content = fs::read_to_string(file).map_err(|ex| {
        println!("ERROR Reading: {} , {}", file, ex);
        ex
    })?;

    // here we are reading the file and collecting each like of executable code as a statement in the Vec
    // then we will parse it through the array and execute it on the db
    let sql_queries: Vec<&str> = content.split(';').collect();
    for sql in sql_queries {
        match sqlx::query(sql).execute(db).await {
            Ok(_) => (),
            Err(err) => println!(
                "The error is model_db in {} file and the reason {}",
                file, err
            ),
        }
    }

    Ok(())
}

// creating a new db.... and connecting to it
async fn new_db(
    host: &str,
    db: &str,
    user: &str,
    pwd: &str,
    max_conn: u32,
) -> Result<Db, sqlx::Error> {
    // here the postgres port opens when run allows you to choose your db and password along with the port to listen to
    let conn_str = format!("postgres://{}:{}@{}:{}/{}", user, pwd, host, 5433, db);
    PgPoolOptions::new()
        .max_connections(max_conn)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&conn_str)
        .await
}

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;

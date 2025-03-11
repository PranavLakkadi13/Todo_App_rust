use super::init_db; // this allows us to access the fn declared above since its public...

#[tokio::test] // using this because it's an async test
               // also the name of the test fn is folder_name-filename-fn_name
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    // Action of the test
    let db = init_db().await?;

    // Check
    let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;
    assert_eq!(8, result.len(), "number of seed todo...");

    Ok(())
}

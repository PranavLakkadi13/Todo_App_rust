use super::init_db; // this allows us to access the fn declared above since its public...

#[tokio::test] // using this because it's an async test
// also the name of the test fn is folder_name-filename-fn_name
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    Ok(())
}
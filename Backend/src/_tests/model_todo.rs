use super::{Todo, TodoMAC, TodoPatch, TodoStatus};
use crate::model::db::init_db;
use crate::security::{utx_from_token, UserCtx};
use crate::model;

#[tokio::test]
async fn test_todo_list() -> Result<(), Box<dyn std::error::Error>> {
    // Initialise
    let new_db = init_db().await?;
    let utx = utx_from_token("123").await?;

    // get the db values
    let values = TodoMAC::list(&new_db, &utx).await?;

    // Check
    assert_eq!(values.len(), 2);
    println!("The db values are... {:#?}", values);

    // Testing for the db values....
    assert_eq!(values[0].cid, 123);
    assert_eq!(values[1].cid, 123);
    assert_eq!(values[0].id, 101);
    assert_eq!(values[1].id, 100);

    Ok(())
}

#[tokio::test]
async fn test_todo_create() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let db_patch = TodoPatch {
        title: Some("New title data...".to_string()),
        status: Some(TodoStatus::Open),
        ..Default::default()
    };

    let db_patch1 = TodoPatch {
        title: Some("New title data with no default data".to_string()),
        ..Default::default()
    };

    let utx = utx_from_token("123").await?;

    let todo_create = TodoMAC::create(&db, &utx, db_patch).await?;

    println!(
        "line is run after the todo_create call in the test, {:#?}",
        todo_create
    );

    assert_eq!(todo_create.id, 1000);
    assert_eq!(todo_create.cid, 123);
    assert_eq!(todo_create.status, TodoStatus::Open);

    Ok(())
}

#[tokio::test]
async fn test_todo_get_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let todo = TodoMAC::get(&db, &utx, 100).await?;
    println!("The row data is {:?}", todo);

    assert_eq!(todo.id, 100);
    assert_eq!(todo.title, "todo 100".to_string());

    Ok(())
}

#[tokio::test]
async fn test_todo_get_not_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let todo = TodoMAC::get(&db, &utx, 93).await;
    println!("The row data is {:?}", todo);

    // this is to handle the error case when it fails
    match todo {
        Ok(_) => assert!(false, "should not succeed"), // an assertion saying that it should fail and not be ok (this will still fail the testcase)
        Err(model::Error::EntityNotFound(typ,id)) => { // if this is matched it means that the error is matched to expected error
            assert_eq!(typ, "todo"); // doing the assertions
            assert_eq!(id, 93.to_string());
        }
        other_err => assert!(false, "wrong error {:#?}", other_err) // if any other error than to handle it
    }

    Ok(())
}

#[tokio::test]
async fn test_todo_update_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;
    let utx = utx_from_token("123").await?;
    let todo_patch = TodoPatch {
        status: Some(TodoStatus::Close),
        title: Some("Updated Status".to_string()),
    };
    let todo = TodoMAC::create(&db, &utx, todo_patch).await?;
    println!("The row data is {:#?}", todo);

    assert_eq!(todo.status, TodoStatus::Close);

    let todo_patch2 = TodoPatch {
        status: Some(TodoStatus::Open),
        title: Some("Updated Status using the update...".to_string()),
    };

    let todo2 = TodoMAC::update(&db, &utx, todo_patch2.clone(), 1000).await?;
    println!("the updated id {:#?} is {:#?}", todo.id, todo2);

    assert_eq!(todo.id, 1000);
    assert_eq!(todo.title, "Updated Status".to_string());

    Ok(())
}


#[tokio::test]
async fn test_todo_delete_not_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let todo = TodoMAC::delete(&db, &utx, 99).await;
    println!("The row data is {:?}", todo);

    // this is to handle the error case when it fails
    match todo {
        Ok(_) => assert!(false, "should not succeed"), // an assertion saying that it should fail and not be ok (this will still fail the testcase)
        Err(model::Error::EntityNotFound(typ,id)) => { // if this is matched it means that the error is matched to expected error
            assert_eq!(typ, "todo"); // doing the assertions
            assert_eq!(id, 99.to_string());
        }
        other_err => assert!(false, "wrong error {:#?}", other_err) // if any other error than to handle it
    }
    Ok(())
}

#[tokio::test]
async fn test_todo_delete_ok() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db().await?;

    let utx = utx_from_token("123").await?;

    let todo = TodoMAC::delete(&db, &utx, 100).await?;
    println!("The row data is {:?}", todo);

    assert_eq!(todo.id, 100);
    assert_eq!(todo.status, TodoStatus::Close);

    let sql: Vec<Todo> = sqlb::select().table("todo").fetch_all(&db).await?;
    assert_eq!(sql.len(),1);

    Ok(())
}



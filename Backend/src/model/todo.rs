use crate::model;
use crate::model::db::Db;
use crate::security::UserCtx;
use sqlb::{HasFields, Raw, SqlBuilder, Whereable};

// TODO types
// this will allow us to get the db data just like serde
// #[derive(sqlx::FromRow, Debug, Clone)]
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64,
    pub title: String,
    pub status: TodoStatus,
}

#[derive(Default, Debug, Clone, sqlb::Fields)] // "sqlb::Fields" this values will tell the sql builder that only those struct value should be retrieved
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

// the PartialEq and the Eq macro allow it to be used in testing for assertions
#[derive(Debug, Clone, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")] // this is telling rust to map it to the sql enum that we had defined
#[sqlx(rename_all = "lowercase")] // doing this because we have defined everything in lowercase in the enum in the sql code
pub enum TodoStatus {
    Open,
    Close,
}

// this is just to make sure that the code is convertable to sql types since this is a custom type
sqlb::bindable!(TodoStatus);

// TODO MAC -> Model Access Controller
pub struct TodoMAC;

// currently if u see the below example the table name and the column names in the below sql builder are being manually written, we will make a constant variable for it
impl TodoMAC {
    const TABLE: &'static str = "todo";
    const COLUMNS: &'static [&'static str] = &["id", "cid", "title", "status"];
}

impl TodoMAC {
    // this is the crud to create as in insert into the db...
    // also the reason we are writing the new struct because we don't want to allow the user to update the id
    pub async fn create(db: &Db, utx: &UserCtx, todo: TodoPatch) -> Result<Todo, model::Error> {
        // this is commented because its to verbose to write the code manually always, we will try to use a ORM or a sql builder
        // let sql = "INSERT INTO Todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";
        //
        // let query = sqlx::query_as::<_, Todo>(&sql)
        //     .bind(todo.cid.unwrap_or_else(|| 1) as i64) // this should actually come from the User Side we will have a look at how to get it a bit later
        //     .bind(todo.title.unwrap_or_else(|| "Untitled".to_string()));

        // previously we had cid in the field but since user won't pass the value we have removed it and since it's a not null field we will have put it manually
        let mut fields = todo.all_fields();
        fields.push(("cid", utx.user_id).into()); // with this we can do the admin data to pass and not worry about the user arguments
        let sb = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::COLUMNS);

        // here if u observe the code was able to insert 1 row into the table even without execute because of the command
        // in the above sql code we have explicitly mentioned that the table must return values post insert
        // that's y only 1 row was inserted and was able
        // let todo = query.fetch_one(db).await.expect("Error during db ops");

        // using the sql builder
        let todos = sb.fetch_one(db).await?;

        Ok(todos)
    }

    // here if you see we are returning a custom error, it automatically maps the error to sqlx::Error....
    // also we will use the utx as access control making it more secure...
    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        // this is the sql query to be used to list the db
        // let sql = "SELECT id,cid,title,status from todo ORDER BY id DESC"; // just like above we will be using the sqlb
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .order_by("!id"); // here the ! means Descending

        // the sqlx query builder
        // here when we query we would want the query to be returned in the Todo type and not the whole table
        // that's why when u did the query as just like serde it was able to typecast it to Todo type and return it
        // NOTE: Because we explicitly mentioned the return type, even if we didn't keep the ::<_,Todo> it would have worked
        // let sql_query = sqlx::query_as::<_, Todo>(&sql);

        // execute the query
        // the values from the db will be loaded from top to the bottom -> because sorted based on id, higher ids will be on top
        // let list = sql_query
        //     .fetch_all(db)
        //     .await
        //     .expect("Error in the query to fetch all db data");

        let list = sb.fetch_all(db).await?;

        Ok(list)
    }

    // gets the data of a particular id from the db
    pub async fn get(db: &Db, _utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sb = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::COLUMNS)
            .and_where_eq("id", id);

        let todo = sb
            .fetch_one(db)
            .await;

        // here we are basically handling the error using our custom methods see the below function impl
        let result = handle_fetch_one_result(todo,Self::TABLE,id.to_string())?;

        Ok(result)
    }

    pub async fn update(
        db: &Db,
        utx: &UserCtx,
        todo: TodoPatch,
        id: i64,
    ) -> Result<Todo, model::Error> {
        let mut fields = todo.all_fields();
        fields.push(("mid" , utx.user_id ).into());
        fields.push(("ctime", Raw("now()")).into());
        let sb = sqlb::update()
            .table(Self::TABLE)
            .data(fields)
            .and_where_eq("id", id)
            .returning(Self::COLUMNS);

        let todo = sb.fetch_one(db).await;

        handle_fetch_one_result(todo,Self::TABLE,id.to_string())
    }

    pub async fn delete(db: &Db, utx: &UserCtx, id: i64) -> Result<Todo, model::Error> {
        let sb = sqlb::delete().table(Self::TABLE).returning(Self::COLUMNS).and_where_eq("id",id);

        let todo = sb.fetch_one(db).await;

        handle_fetch_one_result(todo,Self::TABLE,id.to_string())
    }
}

fn handle_fetch_one_result(result: Result<Todo, sqlx::Error>, typ: &'static str, id: String) -> Result<Todo, model::Error>{
    result .map_err(|sqlx_error| match sqlx_error {
        sqlx::Error::RowNotFound => {
            model::Error::EntityNotFound(typ, id)
        }
        other => model::Error::SqlxError(other),
    })
}


#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;

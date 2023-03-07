use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::sgba::db::con::{ConnectionPool, DatabaseConnection, internal_error};
use crate::sgba::db::users::query::get_users;
use crate::sgba::db::users::user::User;

pub async fn users(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
                    // Result<Json<Vec<User>>, (StatusCode, String)> {
    let users = get_users(&conn).await.unwrap();

    Ok(Json(users))
}

pub async fn using_connection_extractor(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<String, (StatusCode, String)> {


    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}

pub async fn using_connection_pool_extractor(
    State(pool): State<ConnectionPool>,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let row = conn
        .query_one("select 1 + 1", &[])
        .await
        .map_err(internal_error)?;
    let two: i32 = row.try_get(0).map_err(internal_error)?;

    Ok(two.to_string())
}
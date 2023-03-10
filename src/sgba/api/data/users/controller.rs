
use axum::http::StatusCode;
use axum::Json;

use crate::sgba::api::data::users::crypt::generate_from_password;
use crate::sgba::db::con::{DatabaseConnection};
use crate::sgba::db::*;
use crate::sgba::db::users::user::User;
use crate::sgba::api::data::users::dto::{SingUp, SingIn};

pub async fn get_users(
    DatabaseConnection(conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {

    let users = users::query::get(
        &conn).await.unwrap();
    Ok(Json(users))
}

pub async fn sing_up(
    DatabaseConnection(conn): DatabaseConnection,
    Json(input): Json<SingUp>,
) -> Result<Json<bool>, (StatusCode, String)> {

    let new_input = SingUp {
        name: input.name,
        email: input.email,
        passwd: generate_from_password(&input.passwd).unwrap(),
    };

    let created = users::query::create(
        &conn,
        new_input).await.unwrap();

    Ok(Json(created))
}

pub async fn sing_in(
    DatabaseConnection(conn): DatabaseConnection,
    Json(input): Json<SingIn>,
) -> Result<Json<bool>, (StatusCode, String)> {

    let new_input = SingIn {
        email: input.email,
        passwd: generate_from_password(&input.passwd).unwrap(),
    };

    let check = users::query::check(
        &conn, new_input).await.unwrap();
    Ok(Json(check))
}


// // example
// pub async fn using_connection_pool_extractor(
//     State(pool): State<ConnectionPool>,
// ) -> Result<String, (StatusCode, String)> {
//     let conn = pool.get().await.map_err(internal_error)?;
//
//     let row = conn
//         .query_one("select 1 + 1", &[])
//         .await
//         .map_err(internal_error)?;
//     let two: i32 = row.try_get(0).map_err(internal_error)?;
//
//     Ok(two.to_string())
// }
use axum::http::StatusCode;
use axum::Json;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Header};
use crate::sgba::api::auth::auth::{AuthError, Claims, KEYS};

use crate::sgba::api::data::users::crypt::generate_from_password;
use crate::sgba::db::con::{DatabaseConnection};
use crate::sgba::db::*;
use crate::sgba::db::users::user::User;
use crate::sgba::api::data::users::dto::{SingUp, SingIn};
use crate::sgba::api::validate_payload;

pub async fn get_users(_claims: Claims,
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

    validate_payload(&input).unwrap();

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
) -> Result<Json<String>, (StatusCode, String)> {

    let new_input = SingIn {
        email: input.email,
        passwd: generate_from_password(&input.passwd).unwrap(),
    };

    let id = users::query::check(
        &conn, new_input).await.unwrap();

    let mut token = "".to_owned();
    if id > 0 {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);
        let claims = Claims {
            sub: id,
            company: "sgba".to_owned(),
            iat: iat.timestamp(),
            exp: exp.timestamp(),

        };
        token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation).unwrap();
    }

    Ok(Json(token))
}



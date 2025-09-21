use diesel::prelude::*;
use crate::db::schema::users;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User{
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}
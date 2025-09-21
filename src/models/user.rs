use diesel::prelude::*;
use crate::schema::users;
use serde::{Debug, Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct User{
    id: i32,
    first_name: String,
    last_name: String,
    username: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = "users")]
pub struct NewUser{
    first_name: String,
    last_name: String,
    username: String,
}
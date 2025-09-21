// use serde_json::json;
use actix_web::{ web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::{models::user::{User, NewUser}, db::schema::users};

type DbPool = web::Data<diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>>;

pub async fn users(
    pool: DbPool,
    new_user: web::Json<NewUser>
) -> impl Responder{
    // create user
    let conn = &mut pool.get().expect("Failed to connect to the database");

    let user = diesel::insert_into(users::table)
        .values(&*new_user)
        .execute(conn)
        .map(|_| {
            users::table
                .order(users::id.desc())
                .first::<User>(conn)
        });
    match user{
        Ok(Ok(user)) => HttpResponse::Created()
            .json(user),
        Ok(Err(_)) => HttpResponse::InternalServerError().into(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
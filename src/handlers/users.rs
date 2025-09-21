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

pub async fn get_users(
    pool: DbPool
) -> impl Responder{
    let conn = &mut pool.get().expect("Failed to connect to the database");

    let users = users::table
        .load::<User>(conn);
    match users{
        Ok(users) => HttpResponse::Ok()
            .json(users),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

// 
pub async fn get_user_by_id(
    pool: DbPool,
    user_id: web::Path<i32>
) -> impl Responder{
    let conn = &mut pool.get().expect("Failed to connect to the database");

    let user = users::table
        .filter(users::id.eq(*user_id))
        .first::<User>(conn);
    match user{
        Ok(user) => HttpResponse::Ok()
            .json(user),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete_user(
    pool: DbPool,
    user_id: web::Path<i32>
) -> impl Responder{
    let conn = &mut pool.get().expect("Failed to connect to the database");

    let count = diesel::delete(users::table.filter(users::id.eq(*user_id)))
        .execute(conn);
    match count{
        Ok(0) => HttpResponse::NotFound().finish(),
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn update_user(
    pool: DbPool,
    user_id: web::Path<i32>,
    updated_user: web::Json<NewUser>
) -> impl Responder{
    let conn = &mut pool.get().expect("Failed to connect to the database");

    let user = diesel::update(users::table.filter(users::id.eq(*user_id)))
        .set((
            users::first_name.eq(&updated_user.first_name),
            users::last_name.eq(&updated_user.last_name),
            users::username.eq(&updated_user.username),
            users::updated_at.eq(diesel::dsl::now),
        ))
        .get_result::<User>(conn);
    match user{
        Ok(user) => HttpResponse::Ok()
            .json(user),
        Err(diesel::result::Error::NotFound) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}
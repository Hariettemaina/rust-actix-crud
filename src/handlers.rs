use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use diesel::PgConnection;
use super::DbPool;
use crate::models::{NewTweet, Tweet, TweetPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn add_a_tweet(_message: &str, conn: &PgConnection) -> Result<Tweet, DbError> {
  use crate::schema::tweets::dsl::*;

  let new_tweet = NewTweet {
    message: _message,
    created_at: chrono::Local::now().naive_local(),
  };

  let res = diesel::insert_into(tweets)
    .values(&new_tweet)
    .get_result(conn)?;
  Ok(res)
}


#[get("/tweets")]
async fn index() -> impl Responder {
  HttpResponse::Ok().body("Tweet#index")
}

#[post("/tweets")]
async fn create(pool: web::Data<DbPool>,payload: web::Json<TweetPayload>) -> Result<HttpResponse, Error> {
  let tweet = web::block(move || {
    let conn = pool.get()?;
    add_a_tweet(&payload.message, &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(tweet))
}




#[get("/tweets/{id}")]
async fn show(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#show {}", id))
}

#[put("/tweets/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#edit {}", id))
}

#[delete("/tweets/{id}")]
async fn destroy(id: web::Path<String>) -> impl Responder {
  HttpResponse::Ok().body(format!("Tweet#delete {}", id))
}

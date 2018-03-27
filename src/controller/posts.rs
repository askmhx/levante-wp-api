extern crate rocket;

use rocket_contrib::{Json,Value};
use orm::database::DbConn;


#[get("/wp-json/wp/v2/posts")]
pub fn list(conn: DbConn) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[post("/wp-json/wp/v2/posts", format = "application/json")]
pub fn create(conn: DbConn) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn retrieve(conn: DbConn,id: String) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn update(conn: DbConn,id: String) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn delete(conn: DbConn,id: String) -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
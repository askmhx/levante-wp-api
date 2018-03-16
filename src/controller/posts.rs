extern crate rocket;

use rocket::response::content;
use orm::database::DbConn;


#[get("/wp-json/wp/v2/posts")]
pub fn list(conn: DbConn) -> content::Json<String> {

}

#[post("/wp-json/wp/v2/posts")]
pub fn create() -> content::Json<String> {}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn retrieve(id: &str) -> content::Json<String> {}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn update(id: &str) -> content::Json<String> {}

#[get("/wp-json/wp/v2/posts/<id>")]
pub fn delete(id: &str) -> content::Json<String> {}
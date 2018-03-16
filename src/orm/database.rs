extern crate diesel;

use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::http::Status;

use diesel::mysql::MysqlConnection;
use diesel::r2d2;

use util::config::DbConfig;
use std::ops::Deref;

type Pool = r2d2::Pool<r2d2::ConnectionManager<MysqlConnection>>;

pub fn init_pool(config: DbConfig) -> Pool {
    let manager = r2d2::ConnectionManager::<MysqlConnection>::new(config.to_string());
    Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<r2d2::ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = MysqlConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
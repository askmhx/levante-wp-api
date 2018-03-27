#![feature(plugin,custom_attribute,custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate diesel;

pub mod controller;
pub mod orm;
pub mod model;
pub mod util;

use std::env;
use controller::{errors,posts};
use orm::database;
use util::config::Config;

fn main() {

    let path = env::args().nth(1).expect("Missing argument");

    let config= Config::from_file(path).unwrap();

    let db_pool = database::init_pool(config.database);

    rocket::ignite()
        .manage(db_pool)
        .mount("/", routes![posts::list,posts::create,posts::retrieve,posts::update,posts::delete])
        .catch(errors![errors::e404])
        .catch(errors![errors::e500])
        .launch();
}
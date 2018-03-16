#![feature(plugin,custom_attribute,custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
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

    let config = match env::args()[1] {
        Some(cfg_path) => Config::from_file(cfg_path).unwrap(),
        None => (),
    };

    let db_pool = database::init_pool(config.db);

    rocket::ignite()
        .manage(db_pool)
        .mount("/", routes![posts::list,posts::create])
        .mount("/", routes![posts::list,posts::create])
        .mount("/", routes![posts::list,posts::create])
        .mount("/", routes![posts::list,posts::create])
        .mount("/", routes![posts::list,posts::create])
        .mount("/", routes![posts::list,posts::create])
        .catch(errors![errors::e404])
        .catch(errors![errors::e500])
        .launch();
}
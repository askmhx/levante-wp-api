#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/boot")]
fn boot() -> &'static str {
    "Hello, boot!"
}


fn main() {
    rocket::ignite().mount("/", routes![index,boot]).launch();
}
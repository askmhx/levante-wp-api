use rocket_contrib::Json;

pub trait WpCRUD {

    fn list() -> Json<String>;

    fn create() -> Json<String>;

    fn retrieve() -> Json<String>;

    fn update() -> Json<String>;

    fn delete() -> Json<String>;

}

pub trait WpCRUD {

    fn list() -> content::Json<String>;

    fn create() -> content::Json<String>;

    fn retrieve() -> content::Json<String>;

    fn update() -> content::Json<String>;

    fn delete() -> content::Json<String>;

}
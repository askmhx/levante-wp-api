
extern crate rocket;

use rocket::response::content;

#[error(404)]
pub fn e404(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, but '{}' is not a valid path!</p>
            <p>Try visiting /hello/&lt;name&gt;/&lt;age&gt; instead.</p>",
                          req.uri()))
}


#[error(500)]
pub fn e500(req: &rocket::Request) -> content::Html<String> {
    content::Html(format!("<p>Sorry, '{}' Some thing wrong with the server!</p>", req.uri()))
}
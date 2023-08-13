#![feature(decl_macro)]
#[macro_use] extern crate rocket;

pub use rocket::routes;

use rocket::response::{Responder, NamedFile};
use rocket::response::content::Html;

#[get("/")]
pub fn index() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("./pages/index.html").map_err(|e| e.into())
}

#[get("/<proj>")]
pub fn show_proj_info(proj: String) -> Html<String> {
    let project_name = match &proj[..] {
        "people_vs_undeads" => "People vs Undeads",
        "escape_from_bobrovo" => "Escape from Bobrovo",
        "mematrica" => "Mematrica",
        "txt_process" => "Text processing lib",
        "database" => "Database app",
        &_ => "Error 404",
    };

    Html(
        format!("<head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Document</title>
    </head>
    <body>
        <link rel=\"stylesheet\" type=\"text/css\" href=\"../static/styles/style.css\"></link>   
        <div class=\"proj_name_header\">{}</div>
    </body>", project_name)
    )
}
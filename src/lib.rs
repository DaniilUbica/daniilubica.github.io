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
        "txt_process" => "Txt processor",
        "database" => "Database app",
        &_ => "Error 404",
    };

    let project_description = match &proj[..] {
        "people_vs_undeads" => "Strategy game, made with my own game engine, C++ and SFML",
        "escape_from_bobrovo" => "Little top-down shooter with bosses, buffs and ultimates, made with my own game engine on C++/SFML.",
        "mematrica" => "An easy to use library for working with matrices.",
        "txt_process" => "A little library for text processing.",
        "database" => "An application that will help you manage your databases. It works with a tcp-server made on rust",
        &_ => " ",
    };

    let project_repo_link = match &proj[..] {
        "people_vs_undeads" => "https://github.com/DaniilUbica/People-vs-Undeads",
        "escape_from_bobrovo" => "https://github.com/DaniilUbica/escape-from-bobrovo",
        "mematrica" => "https://github.com/DaniilUbica/mematrica",
        "txt_process" => "https://github.com/DaniilUbica/txt_processor",
        "database" => "https://github.com/DaniilUbica/databases-app",
        &_ => " ",
    };

    let project_link = match &proj[..] {
        "people_vs_undeads" => " https://drive.google.com/file/d/16OEKe-G-OlR5mi3BHdaPnWDOAsv9QW4a/view?usp=sharing",
        "escape_from_bobrovo" => "https://drive.google.com/file/d/1GVT7yEVmIJxBMDYBiPXPLe0PjBa0tAfL/view?usp=sharing",
        "mematrica" => "https://crates.io/crates/mematrica",
        "txt_process" => "https://crates.io/crates/txt_processor",
        "database" => "",
        &_ => " ",
    };

    let project_full_description = match &proj[..] {
        "people_vs_undeads" => "
        A game inspired by the old age of war flash game. Implemented in the C++ programming language using OOP, the SFML graphics library and my game engine, which was developed in one of my projects. The essence of the game is to prevent the enemy from destroying your town hall and at the same time demolishing his town hall. The game has 3 types of units, the ability to build towers that will attack the enemy, the ability to improve wars.",
        "escape_from_bobrovo" => "https://github.com/DaniilUbica/escape-from-bobrovo",
        "mematrica" => "https://github.com/DaniilUbica/mematrica",
        "txt_process" => "https://github.com/DaniilUbica/txt_processor",
        "database" => "https://github.com/DaniilUbica/databases-app",
        &_ => " ",
    };

    Html(
        format!("<head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Document</title>
    </head>
    <body>
        <link rel=\"stylesheet\" type=\"text/css\" href=\"../static/styles/style.css\"></link>   
        <style> .header {{
            width: 400px;
            left: 0;
            right: 0;
            margin: 0 auto;
          }}</style>
        <div class=\"header\">{}</div>
        <style>.description {{
            left: 0;
            right: 0;
            margin: 0 auto;
            top: 30%;
          }}</style>
        <h3 class=\"description\">{}</h3>
        <style>.full_description {{
            width: 550px;
            left: 0;
            right: 0;
            margin: 0 auto;
            top: 40%;
          }}</style>
        <h3 class=\"full_description\">{}</h3>
        <style>.proj_block{{
            width: 150px;
            height: 40px;
            top: 60%;
            line-height: 40px;
            left: calc(46% - 90px);
            cursor: pointer;
        }}
        </style>
        <div class=\"proj_block\" onclick=\"window.location.href='{}';\">Try it!</div>
        <style>.proj_block{{
            width: 150px;
            height: 40px;
            top: 60%;
            line-height: 40px;
            cursor: pointer;
        }}
        </style>
        <div class=\"proj_block\" onclick=\"window.location.href='{}';\">Repository</div>
    </body>", project_name, project_description, project_full_description, project_link, project_repo_link)
    )
}
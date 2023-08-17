#![feature(decl_macro)]
#[macro_use] extern crate rocket;

pub use rocket::routes;

use rocket::response::{Responder, NamedFile};
use rocket::response::content::Html;

use std::fs::File;
use std::io::Write;


fn remove_slashes(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek() == Some(&'"') {
            chars.next();
            result.push('"');
        } else {
            result.push(c);
        }
    }

    result
}

fn create_html(name: String, code: String) {
    let mut file = File::create(format!("{name}.html")).expect("Error in creating html file!");
    file.write_all(code.as_bytes()).expect("Error in writing to file!");
}

#[get("/")]
pub fn index() -> Result<impl Responder<'static>, failure::Error> {
    NamedFile::open("./index.html").map_err(|e| e.into())
}

// Need pages of all projects to send site to github-pages, so, i need to do that:
pub fn init_file(proj: String) {
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
        "escape_from_bobrovo" => "A small top-down shooter and roguelike created on my own game engine with help of C++, SFML. There are many features in the project, such as random level generation, consumable items, bosses, buffs, ultimates. The goal was to make a game about me and my friends, so there are a lot of local memes in the game that not everyone will understand :). At the moment, the textures of my friends and enemies are needed for the release of the game.
        ",
        "mematrica" => "Matrix library written in Rust. The goal was to create a library that would be convenient for me specifically and put it on crates.io. One of the few projects that I have completed, I am extremely pleased with it, at the moment more than 300 people have installed the library! (I hope it was useful to at least someone :))",
        "txt_process" => "A library for working with text data written in Rust. At the time of the start of development, I wanted to release another crate, which could also be useful to someone, especially since I did not find similar analogs to my idea (maybe I was looking badly). The essence of the library is that the user creates an object of the TxtProcessor structure and uses it to process text or search for the necessary data. Pretty simple and helpful.",
        "database" => "Application developed in Qt, C++. The application is an analogue of the SQlite manager, which also has a server written in Rust to provide the user with his database.",
        &_ => " ",
    };

    let project_img_name = match &proj[..] {
        "people_vs_undeads" => "pu",
        "escape_from_bobrovo" => "eb",
        "mematrica" => "mm",
        "txt_process" => "txt",
        "database" => "db",
        &_ => " ",
    };

    create_html(proj, remove_slashes(&format!("
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <script src=\"./static/js/project_page.js\"></script>
        <title>Document</title>
        <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM\" crossorigin=\"anonymous\">
        <script defer src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz\" crossorigin=\"anonymous\"></script>
        <link rel=\"shortcut icon\" href=\"./static/images/favicon.ico\">
    </head>
    <body onload=\"init_elements()\">
        <link rel=\"stylesheet\" type=\"text/css\" href=\"./static/styles/style.css\"></link>   
        <div id=\"main\" class=\"container-fluid\">
            <div class=\"ml-auto row-auto\">
                <div class=\"col align-self-center\">
                    <div id=\"proj_header\" class=\"header\">{}</div>
                    <div class=\"p-3\"></div>
                    <div id=\"description\" class=\"h3\">{}</div>
                    <div class=\"p-3\"></div>
                    <div id=\"full_description\" class=\"h3\">{}</div>
                    <div class=\"p-3\"></div>
                    <div class=\"p-5\"></div>
                    <div class=\"p-5\"></div>
                    <div class=\"p-5\"></div>
                    <div class=\"p-2\"></div>
                    <div class=\"image-parent\">
                        <img src=\"./static/images/{}1.png\" class=\"img-fluid\">
                    </div> 
                </div>
                </div>
            </div>
            <div class=\"container\">
                <div class=\"image-parent\">
                    <img src=\"./static/images/{}2.png\" class=\"img-fluid\">
                </div>  
            </div>
            <div id=\"btns\" class=\"container\">
                <div class=\"row justify-content-center\">
                    <div class=\"col align-self-center\">
                        <div id=\"try\" class=\"proj_block\" onclick=\"window.location.href='{}';\">Try it!</div>
                        <div class=\"col align-self-center\"></div>
                        <div id=\"repo\" class=\"proj_block\" onclick=\"window.location.href='{}';\">Repository</div>
                    </div>
                </div>
            </div>
    </body>", project_name, project_description, project_full_description, project_img_name, project_img_name, project_link, project_repo_link)
    ));
}

// if i will host site not on github...
// #[get("/<proj>")]
// pub fn show_proj_info(proj: String) -> Html<String> {
//     let project_name = match &proj[..] {
//         "people_vs_undeads" => "People vs Undeads",
//         "escape_from_bobrovo" => "Escape from Bobrovo",
//         "mematrica" => "Mematrica",
//         "txt_process" => "Txt processor",
//         "database" => "Database app",
//         &_ => "Error 404",
//     };

//     let project_description = match &proj[..] {
//         "people_vs_undeads" => "Strategy game, made with my own game engine, C++ and SFML",
//         "escape_from_bobrovo" => "Little top-down shooter with bosses, buffs and ultimates, made with my own game engine on C++/SFML.",
//         "mematrica" => "An easy to use library for working with matrices.",
//         "txt_process" => "A little library for text processing.",
//         "database" => "An application that will help you manage your databases. It works with a tcp-server made on rust",
//         &_ => " ",
//     };

//     let project_repo_link = match &proj[..] {
//         "people_vs_undeads" => "https://github.com/DaniilUbica/People-vs-Undeads",
//         "escape_from_bobrovo" => "https://github.com/DaniilUbica/escape-from-bobrovo",
//         "mematrica" => "https://github.com/DaniilUbica/mematrica",
//         "txt_process" => "https://github.com/DaniilUbica/txt_processor",
//         "database" => "https://github.com/DaniilUbica/databases-app",
//         &_ => " ",
//     };

//     let project_link = match &proj[..] {
//         "people_vs_undeads" => " https://drive.google.com/file/d/16OEKe-G-OlR5mi3BHdaPnWDOAsv9QW4a/view?usp=sharing",
//         "escape_from_bobrovo" => "https://drive.google.com/file/d/1GVT7yEVmIJxBMDYBiPXPLe0PjBa0tAfL/view?usp=sharing",
//         "mematrica" => "https://crates.io/crates/mematrica",
//         "txt_process" => "https://crates.io/crates/txt_processor",
//         "database" => "",
//         &_ => " ",
//     };

//     let project_full_description = match &proj[..] {
//         "people_vs_undeads" => "
//         A game inspired by the old age of war flash game. Implemented in the C++ programming language using OOP, the SFML graphics library and my game engine, which was developed in one of my projects. The essence of the game is to prevent the enemy from destroying your town hall and at the same time demolishing his town hall. The game has 3 types of units, the ability to build towers that will attack the enemy, the ability to improve wars.",
//         "escape_from_bobrovo" => "A small top-down shooter and roguelike created on my own game engine with help of C++, SFML. There are many features in the project, such as random level generation, consumable items, bosses, buffs, ultimates. The goal was to make a game about me and my friends, so there are a lot of local memes in the game that not everyone will understand :). At the moment, the textures of my friends and enemies are needed for the release of the game.
//         ",
//         "mematrica" => "Matrix library written in Rust. The goal was to create a library that would be convenient for me specifically and put it on crates.io. One of the few projects that I have completed, I am extremely pleased with it, at the moment more than 300 people have installed the library! (I hope it was useful to at least someone :))",
//         "txt_process" => "A library for working with text data written in Rust. At the time of the start of development, I wanted to release another crate, which could also be useful to someone, especially since I did not find similar analogs to my idea (maybe I was looking badly). The essence of the library is that the user creates an object of the TxtProcessor structure and uses it to process text or search for the necessary data. Pretty simple and helpful.",
//         "database" => "Application developed in Qt, C++. The application is an analogue of the SQlite manager, which also has a server written in Rust to provide the user with his database.",
//         &_ => " ",
//     };

//     let project_img_name = match &proj[..] {
//         "people_vs_undeads" => "pu",
//         "escape_from_bobrovo" => "eb",
//         "mematrica" => "mm",
//         "txt_process" => "txt",
//         "database" => "db",
//         &_ => " ",
//     };

//     Html(
    //     format!("
    // <head>
    //     <meta charset=\"UTF-8\">
    //     <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    //     <script src=\"../static/js/project_page.js\"></script>
    //     <title>Document</title>
    //     <link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css\" rel=\"stylesheet\" integrity=\"sha384-9ndCyUaIbzAi2FUVXJi0CjmCapSmO7SnpJef0486qhLnuZ2cdeRhO02iuK6FUUVM\" crossorigin=\"anonymous\">
    //     <script defer src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js\" integrity=\"sha384-geWF76RCwLtnZ8qwWowPQNguL3RmwHVBC9FhGdlKrxdiJJigb/j/68SIy3Te4Bkz\" crossorigin=\"anonymous\"></script>
    // </head>
    // <body onload=\"init_elements()\">
    //     <link rel=\"stylesheet\" type=\"text/css\" href=\"../static/styles/style.css\"></link>   
    //     <div id=\"main\" class=\"container-fluid\">
    //         <div class=\"ml-auto row-auto\">
    //             <div class=\"col align-self-center\">
    //                 <div id=\"proj_header\" class=\"header\">{}</div>
    //                 <div class=\"p-3\"></div>
    //                 <div id=\"description\" class=\"h3\">{}</div>
    //                 <div class=\"p-3\"></div>
    //                 <div id=\"full_description\" class=\"h3\">{}</div>
    //                 <div class=\"p-3\"></div>
    //                 <div class=\"p-5\"></div>
    //                 <div class=\"p-5\"></div>
    //                 <div class=\"p-5\"></div>
    //                 <div class=\"p-2\"></div>
    //                 <div class=\"image-parent\">
    //                     <img src=\"../static/images/{}1.png\" class=\"img-fluid\">
    //                 </div> 
    //             </div>
    //             </div>
    //         </div>
    //         <div class=\"container\">
    //             <div class=\"image-parent\">
    //                 <img src=\"../static/images/{}2.png\" class=\"img-fluid\">
    //             </div>  
    //         </div>
    //         <div id=\"btns\" class=\"container\">
    //             <div class=\"row justify-content-center\">
    //                 <div class=\"col align-self-center\">
    //                     <div id=\"try\" class=\"proj_block\" onclick=\"window.location.href='{}';\">Try it!</div>
    //                     <div class=\"col align-self-center\"></div>
    //                     <div id=\"repo\" class=\"proj_block\" onclick=\"window.location.href='{}';\">Repository</div>
    //                 </div>
    //             </div>
    //         </div>
    // </body>", project_name, project_description, project_full_description, project_img_name, project_img_name, project_link, project_repo_link)
    // )
// }
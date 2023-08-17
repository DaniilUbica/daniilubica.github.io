use my_site::*;

use rocket_contrib::serve::StaticFiles;

fn main() {
    // rocket::ignite()
    // .mount("/", routes![index])
    // .mount("/", routes![show_proj_info])
    // .mount("/static", StaticFiles::from("./static"))
    // .launch();

    init_file(String::from("people_vs_undeads"));
    init_file(String::from("mematrica"));
    init_file(String::from("escape_from_bobrovo"));
    init_file(String::from("txt_process"));
    init_file(String::from("database"));
}
use my_site::*;

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .mount("/", routes![show_proj_info])
    .mount("/static", StaticFiles::from("./static"))
    .launch();
}
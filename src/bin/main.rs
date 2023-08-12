use proj_with_sereja::*;

use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite().mount("/", routes![hello])
    .mount("/", routes![index])
    .mount("/static", StaticFiles::from("./static"))
    .launch();
}

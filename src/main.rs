#[macro_use]
extern crate rocket;
mod config_app;
mod database;
mod routes;
use routes::{index, update};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index::index, index::find])
        .mount("/update", routes![update::index, update::update])
}

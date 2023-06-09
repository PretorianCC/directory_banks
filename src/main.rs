#[macro_use]
extern crate rocket;
mod config_app;
mod database;
mod routes;
use routes::{index, update};

#[launch]
fn rocket() -> _ {
    let config = config_app::parse().unwrap();

    let figment = rocket::Config::figment()
        .merge(("address", &config.host))
        .merge(("port", config.port));

    rocket::custom(figment)
        .manage(config)
        .mount("/", routes![index::index, index::find])
        .mount("/update", routes![update::index, update::update])
}

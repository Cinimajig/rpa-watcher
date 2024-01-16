mod api;

use rocket::fs::FileServer;

#[macro_use] extern crate rocket;


#[launch]
fn rocket() -> _ {
    let cfg = load_config();

    rocket::build()
    .configure(&cfg)
    .mount("/api", routes![api::check_in])
    .mount("/", FileServer::from("wwwroot/"))
}

fn load_config() -> rocket::Config {
    let mut config = {
        #[cfg(debug_assertions)]
        return rocket::Config::debug_default();
        #[cfg(not(debug_assertions))]
        return rocket::Config::release_default();
    };

    todo!()
}
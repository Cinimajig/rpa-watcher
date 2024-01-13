#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let cfg = load_config();

    rocket::build()
    .configure(&cfg)
    .mount("/", routes![index])
}

fn load_config() -> rocket::Config {
    let mut config = {
        #[cfg(debug_assertions)]
        return rocket::Config::debug_default();
        #[cfg(not(debug_assertions))]
        return rocket::Config::release_default();
    };

    todo!()


    config
}
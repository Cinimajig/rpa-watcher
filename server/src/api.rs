use rocket::serde::json::Json;
use sharedef::rpa::*;

#[post("/api/checkin", format = "json", data = "<req>")]
pub fn check_in(req: Json<RpaData>) -> &'static str {
    




    "Ok"
}

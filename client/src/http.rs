use std::time;


pub fn post(url: &str, token: &str, data: &str) -> Result<(), ureq::Error> {
    let res = ureq::post(url)
    .set("Api-Token", token)
    .send_string(data)?;

    if res.status() > 200 && res.status() < 300 {
        Ok(())
    } else {
        Err(ureq::Error::from(res))
    }
}
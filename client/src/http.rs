use std::time;

const TIME_SECS: time::Duration = time::Duration::from_secs(2);

pub fn post(url: &str, token: &str, data: &str) -> Result<(), ureq::Error> {
    let res = ureq::post(url)
    .timeout(TIME_SECS)
    .set("Api-Token", token)
    .send_string(data)?;
    Ok(())

    // if res.status() > 200 && res.status() < 300 {
    //     Ok(())
    // } else {
    //     Err(ureq::Error::from(res))
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_request() -> Result<(), Box<dyn std::error::Error>> {
        let res = ureq::get("https://api.sampleapis.com/beers/ale")
        .call()?;

        if res.status() >= 200 && res.status() < 300 {
            Ok(())
        } else {
            Err(ureq::Error::from(res).into())
        }
    }
}
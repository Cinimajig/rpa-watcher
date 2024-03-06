pub fn post(url: &str, token: &str, data: &impl serde::Serialize) -> Result<ureq::Response, Box<ureq::Error>> {
   
    let res = ureq::post(url)
    // .timeout(TIME_SECS)
    .set("Api-Token", token)
    .send_json(data)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    

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
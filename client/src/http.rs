pub fn post(
    url: &str,
    token: &str,
    data: &impl serde::Serialize,
) -> Result<ureq::Response, Box<ureq::Error>> {
    let res = ureq::post(url)
        // .timeout(TIME_SECS)
        .set("Api-Token", token)
        .send_json(data)?;
    Ok(res)
}

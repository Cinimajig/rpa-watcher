pub fn post(
    url: &str,
    token: &str,
    data: &impl serde::Serialize,
) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
    let res = ureq::post(url)
        // .timeout(TIME_SECS)
        .header("Api-Token", token)
        .send_json(data)?;
    Ok(res)
}

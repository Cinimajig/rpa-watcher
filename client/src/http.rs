use std::time;

use curl::easy::{Easy, List};

pub fn create_client() -> Result<Easy, curl::Error> {
    let mut client = Easy::new();
    client.connect_timeout(time::Duration::from_millis(300));
    
    let mut list = List::new();
    list.append("Content-Type: application/json; charset=utf-8")?;
    client.http_headers(list);

    todo!();

    Ok(client)
}

pub fn post(client: &mut Easy, data: String) -> Result<(), curl::Error> {
    

    Ok(())
}
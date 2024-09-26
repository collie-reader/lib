use crate::error::Result;

#[cfg(test)]
pub async fn get(link: &str, _proxy: Option<&str>) -> Result<String> {
    use std::fs;
    Ok(fs::read_to_string(link)?)
}

#[cfg(not(test))]
pub async fn get(link: &str, proxy: Option<&str>) -> Result<String> {
    let client = if let Some(proxy_url) = proxy {
        match reqwest::Proxy::all(proxy_url) {
            Ok(p) => reqwest::Client::builder().proxy(p).build()?,
            Err(_) => reqwest::Client::new(),
        }
    } else {
        reqwest::Client::new()
    };
    Ok(client
        .get(link)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await?
        .text()
        .await?)
}

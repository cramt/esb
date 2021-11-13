#[cfg(test)]
mod test;

use serde::{Deserialize, Serialize};

#[cfg(not(test))]
fn api() -> &'static str {
    "https://api.myip.com"
}

#[cfg(test)]
fn api() -> &'static str {
    use std::ops::Deref;

    use once_cell::sync::Lazy;

    static API: Lazy<String> = Lazy::new(mockito::server_url);
    API.deref()
}

#[derive(Serialize, Deserialize)]
pub(super) struct IpReturn {
    pub(super) ip: String,
    pub(super) country: String,
    pub(super) cc: String,
}

pub async fn get_ip() -> Option<[u8; 4]> {
    let str = reqwest::get(api()).await.ok()?.text().await.ok()?;
    let value: IpReturn = serde_json::from_str(str.as_str()).ok()?;
    let ip = value.ip;
    let mut iter = ip.split('.').map(|x| x.parse().unwrap());
    Some([iter.next()?, iter.next()?, iter.next()?, iter.next()?])
}

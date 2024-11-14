mod structs;
mod tomuss_deser_utils;

use crate::structs::TomussData;
use lyon1_cas_client::{Error, Lyon1CasClient};
use regex::Regex;
use std::sync::LazyLock;

const JSON_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"display_update\((.*?),"Top""#).unwrap());

struct Lyon1TomussClient {}

impl Lyon1TomussClient {
    pub fn new(cas: &Lyon1CasClient) -> Result<Lyon1TomussClient, Error> {
        assert!(cas.authenticated());
        let body = cas.service_request("https://tomuss.univ-lyon1.fr", true)?;

        let json = JSON_REGEX.captures(&body)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

        let json = json.replace("\\x", "%");
        let json = urlencoding::decode(&json).unwrap();

        let data: TomussData = TomussData::new(&json);

        data.grades.iter().for_each(|x| println!("{}", x));
        println!("{}", data.grades.len());

        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lyon1_cas_client::Credentials;
    use std::env;

    #[test]
    fn create_client() {
        let _ = dotenvy::dotenv();
        let credentials = Credentials::new(
            env::var("USERNAME".to_string()).unwrap(),
            env::var("PASSWORD".to_string()).unwrap(),
        );

        let mut cas_client = Lyon1CasClient::new();
        assert!(cas_client.authenticate_user(credentials).unwrap());

        let tomuss_client = Lyon1TomussClient::new(&cas_client);
    }
}

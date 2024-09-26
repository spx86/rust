use std::str::FromStr;

use anyhow::{anyhow, Result};
use reqwest::Url;


#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct KVPair {
    pub key: String,
    pub value: String,
}
#[warn(dead_code)]

impl FromStr for KVPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut spilt = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            key: spilt.next().ok_or_else(err)?.to_string(),
            value: spilt.next().ok_or_else(err)?.to_string(),
        })
    }
}

pub fn parse_kv_pairs(input: &str) -> Result<KVPair> {
    Ok(input.parse::<KVPair>()?)
}

pub fn parse_url(input_url: &str) -> Result<String> {
    let _url = input_url.parse::<Url>()?;
    Ok(input_url.to_string())
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_parse_kv_pairs() {
        assert!(parse_kv_pairs("a").is_err());
        assert_eq!(
            parse_kv_pairs("key=value").unwrap(),
            KVPair{
                key: "key".to_string(),
                value: "value".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_url() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }
}
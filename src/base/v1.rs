use anyhow::Result as AnyResult;
use reqwest::{Client as HttpClient, Method};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub author: String,
    pub version: String,
    pub types: String,
}

impl Package {
    pub fn from_underline_format(input: String) -> AnyResult<Self> {
        let slice = input.split("_").collect::<Vec<&str>>();

        if slice.len() < 4 {
            return Err(PackageError::UnderlineFormatParseError {
                raw: input.to_owned(),
                len: slice.len(),
            })
            .map_err(anyhow::Error::new);
        }

        Ok(Self {
            name: slice[0].to_string(),
            author: slice[2].to_string(),
            version: slice[1].to_string(),
            types: slice[3].to_string(),
        })
    }

    pub fn to_underline_format(&self) -> AnyResult<String> {
        Ok(format!(
            "{}_{}_{}_{}",
            self.name, self.version, self.author, self.types
        ))
    }

    pub fn get_download_url(&self, base_url: String) -> AnyResult<Url> {
        let url = Url::parse(&base_url)?;
        let url = url.join(
            format!(
                "./{}/{}_{}_{}.7z",
                self.types, self.name, self.version, self.author
            )
            .as_str(),
        )?;

        Ok(url)
    }

    pub async fn get_it(&self, base_url: String) -> AnyResult<reqwest::Response> {
        let url = self.get_download_url(base_url)?;
        let client = HttpClient::builder().gzip(true).brotli(true).build()?;
        let resp = client
            .request(Method::GET, url.to_string().as_str())
            .header("User-Agent", "Better-Ept/1.1 (Powered By Rust && Reqwest)")
            .send()
            .await?;

        // println!("{:#?}", resp);

        if &resp.status().is_success() != &true {
            return Err(PackageError::NetworkError(resp)).map_err(anyhow::Error::new);
        }

        Ok(resp)
    }
}

#[derive(Error, Debug)]
pub enum PackageError {
    #[error("Underline Format ParseError!")]
    UnderlineFormatParseError { len: usize, raw: String },

    #[error("Network Request Error!")]
    NetworkError(reqwest::Response),
}

#[derive(Error, Debug)]
pub enum BaseError {
    #[error("Other Error!")]
    OtherError(),

    #[error("Unknown Error!")]
    Unknown,
}

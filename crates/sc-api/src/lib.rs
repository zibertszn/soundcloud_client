pub mod error;
pub mod models;
use regex::Regex;
use reqwest::{Client, Proxy, header};

use self::models::SearchResponse;
#[derive(Debug)]
pub struct ScClient {
    http: Client,
    client_id: Option<String>,
}

impl ScClient {
    pub fn new(proxy_url: Option<&str>) -> Result<Self, self::error::Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/130.0.0.0 Safari/537.36",
            ),
        );

        let mut builder = Client::builder().default_headers(headers);

        if let Some(url) = proxy_url {
            builder = builder.proxy(Proxy::all(url)?);
        }

        Ok(Self {
            http: builder.build()?,
            client_id: None,
        })
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn fetch_client_id(&mut self) -> Result<String, self::error::Error> {
        let raw_response = self.http.get("https://soundcloud.com").send().await?;
        tracing::debug!(status = %raw_response.status(), "Get client id HTTP response received");

        let response = raw_response.text().await?;

        let script_re = Regex::new(r#"https://a-v2\.sndcdn\.com/assets/[a-zA-Z0-9-]+\.js"#)?;
        let id_re = Regex::new(r#"client_id:"([a-zA-Z0-9]{32})""#)?;

        let scripts: Vec<&str> = script_re.find_iter(&response).map(|m| m.as_str()).collect();

        for url in scripts.into_iter().rev() {
            let js = self.http.get(url).send().await?.text().await?;
            if let Some(caps) = id_re.captures(&js) {
                let id = caps[1].to_string();
                self.client_id = Some(id.clone());
                tracing::info!("Fetched soundcloud client_id: {}", id.clone());
                return Ok(id);
            }
        }
        Err(self::error::Error::ClientIDNotFOund)
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn search(&self, q: &str, limit: u32) -> Result<(), self::error::Error> {
        //TODO remove unwrap
        let client_id: String = self.client_id.clone().unwrap();
        let limit_string = limit.to_string();
        let query = [
            ("q", q),
            ("client_id", client_id.as_str()),
            ("limit", limit_string.as_str()),
        ];

        let raw_response = self
            .http
            .get("https://api-v2.soundcloud.com/search/tracks")
            .query(&query)
            .send()
            .await?;
        tracing::debug!(status = %raw_response.status(), "Search HTTP response received");

        let raw_response = raw_response.error_for_status()?;

        let response: SearchResponse = if tracing::enabled!(tracing::Level::TRACE) {
            let body = raw_response.text().await?;
            if let Ok(pretty) = serde_json::from_str::<serde_json::Value>(&body)
                .and_then(|v| serde_json::to_string_pretty(&v))
            {
                tracing::trace!("Pretty soundcloud API response:\n{}", pretty);
            } else {
                tracing::trace!(raw_body = %body, "Raw SoundCloud API response");
            }
            serde_json::from_str(&body)?
        } else {
            raw_response.json().await?
        };
        tracing::debug!(tracks_found = response.collection.len(), "Tracks parsed");

        Ok(())
    }
}

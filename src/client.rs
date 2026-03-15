use reqwest::multipart;

pub struct Client {
    http: reqwest::Client,
    instance: String,
}

/// Options for creating a paste.
#[derive(Default)]
pub struct PasteOptions {
    /// Filename hint for server-side language detection.
    pub filename: Option<String>,
    /// Expiration duration (e.g. "10m", "1h", "7d").
    pub expires_in: Option<String>,
}

impl Client {
    pub fn new(instance: impl Into<String>) -> Self {
        Self {
            http: reqwest::Client::new(),
            instance: instance.into(),
        }
    }

    pub fn instance(&self) -> &str {
        &self.instance
    }

    /// Create a paste with the given options. Returns the paste URL.
    pub async fn paste(
        &self,
        content: &str,
        options: &PasteOptions,
    ) -> Result<String, reqwest::Error> {
        let mut file_part = multipart::Part::text(content.to_string())
            .mime_str("text/plain")
            .unwrap();

        if let Some(ref filename) = options.filename {
            file_part = file_part.file_name(filename.clone());
        }

        let mut form = multipart::Form::new().part("file", file_part);

        if let Some(ref expires_in) = options.expires_in {
            form = form.text("expires_in", expires_in.clone());
        }

        self.http
            .post(&self.instance)
            .multipart(form)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    /// Create a paste with no options. Returns the paste URL.
    pub async fn paste_simple(&self, content: &str) -> Result<String, reqwest::Error> {
        self.paste(content, &PasteOptions::default()).await
    }

    /// Shorten a URL. Returns the short URL.
    pub async fn shorten(
        &self,
        url: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        if !(url.starts_with("http://") || url.starts_with("https://")) {
            return Err("URL must start with http:// or https://".into());
        }

        let result = self.paste_simple(url).await?;
        Ok(result)
    }

    /// Get the raw content of a paste by ID.
    pub async fn get_raw(&self, id: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}/{}/raw", self.instance.trim_end_matches('/'), id);
        self.http
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}

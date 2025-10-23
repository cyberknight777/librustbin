use reqwest::blocking::multipart;

pub struct Client {
    reqwest_client: reqwest::blocking::Client,
    instance: String,
}

impl Client {
    pub fn new(instance: String) -> Self {
        Self {
            reqwest_client: reqwest::blocking::Client::new(),
            instance,
        }
    }

    pub fn client(&self) -> &reqwest::blocking::Client {
        &self.reqwest_client
    }

    pub fn paste_highlight(&self, content: String) -> Result<String, reqwest::Error> {
        let form = multipart::Form::new();
        let form_con = multipart::Part::text(content).mime_str("text/plain")?;
        let form = form.part("highlight", form_con);

        self.client()
            .post(&self.instance)
            .multipart(form)
            .send()?
            .text()
    }

    pub fn paste_markdown(&self, content: String) -> Result<String, reqwest::Error> {
        let form = multipart::Form::new();
        let form_con = multipart::Part::text(content).mime_str("text/plain")?;
        let form = form.part("md", form_con);

        self.client()
            .post(&self.instance)
            .multipart(form)
            .send()?
            .text()
    }

    pub fn paste_plain(&self, content: String) -> Result<String, reqwest::Error> {
        let form = multipart::Form::new();
        let form_con = multipart::Part::text(content).mime_str("text/plain")?;
        let form = form.part("file", form_con);

        self.client()
            .post(&self.instance)
            .multipart(form)
            .send()?
            .text()
    }

    pub fn paste_short(&self, content: String) -> Result<String, reqwest::Error> {
        let form = multipart::Form::new();
        let form_con = multipart::Part::text(content).mime_str("text/plain")?;
        let form = form.part("short", form_con);

        self.client()
            .post(&self.instance)
            .multipart(form)
            .send()?
            .text()
    }
}

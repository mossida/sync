struct HttpBook {
    pub client: reqwest::Client,
}

impl HttpBook {
    pub fn new() -> Self {
        HttpBook {
            client: reqwest::Client::new(),
        }
    }

    pub fn build(&self) {}
}

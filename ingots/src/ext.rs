pub trait RequestExt {}

impl Request {
    fn get_app_root(&self) -> String {
        format!("{}://{}/{}", self.is_secure() ? "https" : "http", self.hostname().unwrap_or("localhost"), self.context_path())
    }
}

impl Response {
    fn redirect(&mut self, url: &str) {
        self.set_status(301);
        self.set_header("Location", url);
    }

    fn download(&mut self, filename: &str) {
        self.set_header("Content-Type", "application/octet-stream");
        self.set_header("Content-Transfer-Encoding", "binary");
        self.set_header("Content-Disposition", &format!("attachment; filename=\"{}\"", filename));
    }
}

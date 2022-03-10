pub struct Response {
    //TODO: headers n stuff
    pub status: i16,
    pub body: String,
}

impl Response {

}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            body: "".to_string(),
        }
    }
}
pub struct Hello {
    pub field: String
}

impl Hello {
    pub fn create(msg: &str) -> Hello {
        Hello{ field: msg.to_string() }
    }
}

#[derive(Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

impl Cookie {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn cookie_string(self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

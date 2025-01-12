#[derive(Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

impl Cookie {
    pub fn new(name: impl Into<String>, value: String) -> Self {
        let cookie_name: String = name.into();
        Self {
            name: cookie_name,
            value,
        }
    }

    pub fn cookie_string(self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

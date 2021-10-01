use std::fmt::Display;

pub struct Cookie {
    pub name: String,
    pub value: String,
}

impl Display for Cookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

impl Cookie {
    pub fn parse(content: String) -> Cookie {
        let arr: Vec<&str> = content.split("=").collect();
        Cookie {
            name: arr[0].to_string(),
            value: arr[1].to_string(),
        }
    }
}

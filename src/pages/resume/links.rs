#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct Links {
    pub links: Vec<Link>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Link {
    pub name: String,
    pub url: String,
    pub label : String,
}

impl Link {
    fn new(name: String, url: String, label: String) -> Self {
        Link {
            name,
            url,
            label,
        }
    }
}

impl Default for Links {
    fn default() -> Self {
        let links_toml_str: &str = std::include_str!("../../_data/links.toml");
        let links_toml: toml::Value = links_toml_str.parse().unwrap();

        let links = links_toml["link"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let name = x.get("name").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let url = x.get("url").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let label = x.get("label").and_then(|s| s.as_str()).unwrap_or("").to_string();

                Link::new(name, url, label)
            })
            .collect();

        Links { links: links }
    }
}
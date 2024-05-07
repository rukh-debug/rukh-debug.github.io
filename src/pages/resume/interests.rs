#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Interests {
    pub interests: Vec<Interest>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Interest {
    pub description: String,
}

impl Interest {
    fn new(description: String) -> Self {
        Interest {
            description,
        }
    }
}

impl Default for Interests {
    fn default() -> Self {
        let interests_toml_str: &str = std::include_str!("../../_data/interests.toml");
        let interests_toml: toml::Value = interests_toml_str.parse().unwrap();

        let interest = interests_toml["interest"].as_table().unwrap();
        let interests = interest["description"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| Interest::new(x.as_str().unwrap().to_string()))
            .collect::<Vec<Interest>>();

        Interests { interests: interests }
    }
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Recognitions {
    pub recognitions: Vec<Recognition>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Recognition {
    pub name: String,
    pub organization: String,
    pub year: String,
    pub summary: String,
}

impl Recognition {
    fn new(name: String, organization: String, year: String, summary: String) -> Self {
        Recognition {
            name,
            organization,
            year,
            summary,
        }
    }
}

impl Default for Recognitions {
    fn default() -> Self {
        let recognitions_toml_str: &str = std::include_str!("../../_data/recognitions.toml");
        let recognitions_toml: toml::Value = recognitions_toml_str.parse().unwrap();

        let recognitions = recognitions_toml["recognition"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let name = x
                    .get("name")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let organization = x
                    .get("organization")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let year = x
                    .get("year")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let summary = x
                    .get("summary")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();

                Recognition::new(name, organization, year, summary)
            })
            .collect();

        Recognitions { recognitions }
    }
}


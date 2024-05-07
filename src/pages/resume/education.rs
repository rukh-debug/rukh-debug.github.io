#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct Educations {
    pub educations: Vec<Education>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct Education {
    pub degree: String,
    pub uni: String,
    pub year: String,
    pub summary: String,
}

impl Education {
    fn new(degree: String, uni: String, year: String, summary: String) -> Self {
        Education {
            degree,
            uni,
            year,
            summary,
        }
    }
}

impl Default for Educations {
    fn default() -> Self {
        let educations_toml_str: &str = std::include_str!("../../_data/education.toml");
        let educations_toml: toml::Value = educations_toml_str.parse().unwrap();

        let educations = educations_toml["education"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let degree = x.get("degree").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let uni = x.get("uni").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let year = x.get("year").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let summary = x.get("summary").and_then(|s| s.as_str()).unwrap_or("").to_string();

                Education::new(degree, uni, year, summary)
            })
            .collect();

        Educations { educations }
    }
}
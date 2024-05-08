#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct Experiences {
    pub experiences: Vec<Experience>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Role {
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub summary: String,
    pub highlights: Vec<String>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Experience {
    pub company: String,
    pub logo: String,
    pub url: String,
    pub roles: Vec<Role>,
}

impl Role {
    fn new(
        title: String,
        start_date: String,
        end_date: String,
        summary: String,
        highlights: Vec<String>,
    ) -> Self {
        Role {
            title,
            start_date,
            end_date,
            summary,
            highlights,
        }
    }
}

impl Default for Experiences {
    fn default() -> Self {
        let experiences_toml_str: &str = std::include_str!("../../_data/experience.toml");
        let experiences_toml: toml::Value = experiences_toml_str.parse().unwrap();

        let experiences = experiences_toml["experience"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let company = x
                    .get("company")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let logo = x
                    .get("logo")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let url = x
                    .get("url")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();

                let roles = x
                    .get("positions")
                    .and_then(|s| s.as_array())
                    .unwrap()
                    .iter()
                    .map(|y| {
                        let title = y
                            .get("title")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let start_date = y
                            .get("startdate")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let end_date = y
                            .get("enddate")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let summary = y
                            .get("summary")
                            .and_then(|s| s.as_str())
                            .unwrap_or("")
                            .to_string();
                        let highlights = y
                            .get("projects")
                            .and_then(|s| s.as_array())
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|z| z.as_str().unwrap().to_string())
                            .collect();

                        Role::new(title, start_date, end_date, summary, highlights)
                    })
                    .collect();

                Experience {
                    company,
                    logo,
                    url,
                    roles,
                }
            })
            .collect();

        Experiences { experiences }
    }
}


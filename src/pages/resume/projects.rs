#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Projects {
    pub projects: Vec<Project>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Project {
    pub name: String,
    pub role: String,
    pub duration: String,
    pub url: String,
    pub description: String,
}

impl Project {
    fn new(name: String, role: String, duration: String, url: String, description: String) -> Self {
        Project {
            name,
            role,
            duration,
            url,
            description,
        }
    }
}

impl Default for Projects {
    fn default() -> Self {
        let projects_toml_str: &str = std::include_str!("../../_data/projects.toml");
        let projects_toml: toml::Value = projects_toml_str.parse().unwrap();

        let projects = projects_toml["project"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let name = x
                    .get("name")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let role = x
                    .get("role")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let duration = x
                    .get("duration")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let url = x
                    .get("url")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();
                let description = x
                    .get("description")
                    .and_then(|s| s.as_str())
                    .unwrap_or("")
                    .to_string();

                Project::new(name, role, duration, url, description)
            })
            .collect();

        Projects { projects }
    }
}


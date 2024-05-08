use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Skills {
    pub name: String,
    pub description: String,
    pub tools_tech: HashMap<String, Vec<String>>,
}

impl Default for Skills {
    fn default() -> Self {
        let skills_toml_str: &str = std::include_str!("../../_data/skills.toml");
        let skills_toml: toml::Value = skills_toml_str.parse().unwrap();

        let skill = skills_toml["skill"].as_table().unwrap();
        let name = skill["name"].as_str().unwrap().to_string();
        let description = skill["description"].as_str().unwrap().to_string();

        let tools_tech = skills_toml["skill"]["ToolsTech"].as_table().unwrap();
        let mut all_tools_tech: HashMap<String, Vec<String>> = HashMap::new();

        tools_tech.iter().for_each(|(key, value)| {
            let tools = value
                .as_array()
                .unwrap()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect();
            all_tools_tech.insert(key.to_string(), tools);
        });

        Self {
            name,
            description,
            tools_tech: all_tools_tech,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Skills {
    pub name: String,
    pub description: String,
    pub tools_tech: ToolsTech,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ToolsTech {
    pub languages: Vec<String>,
    pub databases: Vec<String>,
    pub cloud_services: Vec<String>,
    pub containers: Vec<String>,
    pub version_control: Vec<String>,
    pub api_testing: Vec<String>,
    pub scripting: Vec<String>,
    pub editors: Vec<String>,
    pub os: Vec<String>,
    pub tools: Vec<String>,
    pub security_tools: Vec<String>,
}

impl Default for Skills {
    fn default() -> Self {
        let skills_toml_str: &str = std::include_str!("../../_data/skills.toml");
        let skills_toml: toml::Value = skills_toml_str.parse().unwrap();

        let skill = skills_toml["skill"].as_table().unwrap();
        let name = skill["name"].as_str().unwrap().to_string();
        let description = skill["description"].as_str().unwrap().to_string();

        let tools_tech = skills_toml["skill"]["ToolsTech"].as_table().unwrap();
        let languages = tools_tech["languages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let databases = tools_tech["databases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let cloud_services = tools_tech["cloud_services"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let containers = tools_tech["containers"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let version_control = tools_tech["version_control"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let api_testing = tools_tech["api_testing"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let scripting = tools_tech["scripting"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let editors = tools_tech["editors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let os = tools_tech["os"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let tools = tools_tech["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();
        let security_tools = tools_tech["security_tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_str().unwrap().to_string())
            .collect();

        Self {
            name,
            description,
            tools_tech: ToolsTech {
                languages,
                databases,
                cloud_services,
                containers,
                version_control,
                api_testing,
                scripting,
                editors,
                os,
                tools,
                security_tools,
            },
        }
    }
}

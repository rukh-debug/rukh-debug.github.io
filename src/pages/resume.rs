use egui::Ui;
use toml::Value;

pub use self::skills::Skills;
mod skills;
pub use self::education::Educations;
mod education;
pub use self::recognitions::Recognitions;
mod recognitions;
pub use self::projects::Projects;
mod projects;
pub use self::experience::Experiences;
mod experience;
pub use self::interests::Interests;
mod interests;
pub use self::links::Links;
mod links;

pub use crate::custom_widgets::{powered_by_egui_and_eframe, footer, organize_items};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Resume {
    header: Header,
    contact: Contact,
    sections: Sections,
    social: Social,
    skills: Skills,
    education: Educations,
    recognitions: Recognitions,
    projects: Projects,
    experience: Experiences,
    interests: Interests,
    links: Links,
}
// -----------------------------------------------------
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Header {
    name: String,
    display_contact_info: bool,
    current_title: String,
    intro: String,
}

// -----------------------------------------------------

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Contact {
    email: String,
}

// -----------------------------------------------------
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Sections {
    experience: bool,
    education: bool,
    projects: bool,
    skills: bool,
    recognition: bool,
    interests: bool,
    links: bool,
}

// -----------------------------------------------------
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Social {
    website: String,
    github: String,
    linkedin: String,
}

// -----------------------------------------------------
impl Default for Resume {
    fn default() -> Self {
        let resume_toml_str = std::include_str!("../_data/resume.toml");
        let resume_toml_value: Value = toml::from_str(&resume_toml_str).expect("Invalid TOML");

        let default_header = Header {
            name: resume_toml_value["header"]["name"].as_str().unwrap_or("Hello World").to_string(),
            display_contact_info: resume_toml_value["header"]["display_contact_info"].as_bool().unwrap_or(true),
            current_title: resume_toml_value["header"]["current_title"].as_str().unwrap_or("fullstack dev && devops").to_string(),
            intro: resume_toml_value["header"]["intro"].as_str().unwrap_or("i am a developer and devops guy actively developing webs and also a learner, passively learning various nerdy stuff.").to_string(),
        };

        let default_contact = Contact {
            email: resume_toml_value["contact"]["email"]
                .as_str()
                .unwrap_or("resume@rubenk.dev")
                .to_string(),
        };

        let default_sections = Sections {
            experience: resume_toml_value["sections"]["experience"]
                .as_bool()
                .unwrap_or(true),
            education: resume_toml_value["sections"]["education"]
                .as_bool()
                .unwrap_or(true),
            projects: resume_toml_value["sections"]["projects"]
                .as_bool()
                .unwrap_or(true),
            skills: resume_toml_value["sections"]["skills"]
                .as_bool()
                .unwrap_or(true),
            recognition: resume_toml_value["sections"]["recognition"]
                .as_bool()
                .unwrap_or(true),
            interests: resume_toml_value["sections"]["interests"]
                .as_bool()
                .unwrap_or(true),
            links: resume_toml_value["sections"]["links"]
                .as_bool()
                .unwrap_or(true),
        };

        let default_social = Social {
            website: resume_toml_value["social"]["website"]
                .as_str()
                .unwrap_or("http://rubenk.dev")
                .to_string(),
            github: resume_toml_value["social"]["github"]
                .as_str()
                .unwrap_or("https://github.com/rukh-debug")
                .to_string(),
            linkedin: resume_toml_value["social"]["linkedin"]
                .as_str()
                .unwrap_or("https://linkedin.com/in/rubenkharel")
                .to_string(),
        };

        Resume {
            header: default_header,
            contact: default_contact,
            sections: default_sections,
            social: default_social,
            skills: Skills::default(),
            education: Educations::default(),
            recognitions: Recognitions::default(),
            projects: Projects::default(),
            experience: Experiences::default(),
            interests: Interests::default(),
            links: Links::default(),
        }
    }
}

// --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ResumePage {
    resume: Resume,
}

impl Default for ResumePage {
    fn default() -> Self {
        ResumePage {
            resume: Resume::default(),
        }
    }
}

impl ResumePage {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for ResumePage {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {
            footer(ui);
            powered_by_egui_and_eframe(ui);

            egui::Window::new(format!(
                "{} | {}",
                self.resume.header.name, self.resume.header.current_title
            ))
            .default_open(true)
            .enabled(true)
            .show(ctx, |ui| {
                // render ui for resume here
                ui.label(format!("{}", self.resume.header.intro));
                ui.hyperlink_to("Contact", format!("mailto:{}", self.resume.contact.email));
            });

            if self.resume.sections.experience {
                egui::Window::new("Experience")
                    .enabled(true)
                    .default_open(false)
                    .show(ctx, |ui| {
                        for experience in &self.resume.experience.experiences {
                            ui.monospace(experience.company.clone());
                            ui.horizontal(|ui| {
                                ui.label(experience.summary.clone());
                                ui.separator();
                                ui.label(experience.url.clone());
                            });
                            ui.spacing();
                            for role in &experience.roles {
                                ui.monospace(role.title.clone());
                                ui.horizontal(|ui| {
                                    ui.label(role.start_date.clone());
                                    ui.separator();
                                    ui.label(role.end_date.clone());
                                });
                                ui.spacing();
                                ui.label(role.summary.clone());
                                for highlight in &role.highlights {
                                    ui.label(highlight);
                                }
                                // if this is the last iteration dont show the separator
                                if !std::ptr::eq(role, experience.roles.last().unwrap()) {
                                    ui.separator();
                                }
                            }
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(
                                experience,
                                self.resume.experience.experiences.last().unwrap(),
                            ) {
                                ui.separator();
                            }
                        }
                    });
            }

            if self.resume.sections.education {
                egui::Window::new("Education")
                    .enabled(true)
                    .default_open(false)
                    .show(ctx, |ui| {
                        for education in &self.resume.education.educations {
                            ui.monospace(format!("{}", education.degree));
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", education.uni));
                                ui.separator();
                                ui.label(format!("{}", education.year));
                            });
                            ui.spacing();
                            ui.label(format!("{}", education.summary));
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(
                                education,
                                self.resume.education.educations.last().unwrap(),
                            ) {
                                ui.separator();
                            }
                        }
                    });
            }

            if self.resume.sections.projects {
                egui::Window::new("Projects")
                    .default_open(false)
                    .enabled(true)
                    .show(ctx, |ui| {
                        for project in &self.resume.projects.projects {
                            ui.monospace(format!("{}", project.name));
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", project.role));
                                ui.separator();
                                ui.label(format!("{}", project.duration));
                            });
                            ui.spacing();
                            ui.label(format!("{}", project.description));
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(project, self.resume.projects.projects.last().unwrap())
                            {
                                ui.separator();
                            }
                        }
                    });
            }

            if self.resume.sections.skills {
                egui::Window::new("Skills")
                    .default_open(false)
                    .enabled(true)
                    .show(ctx, |ui| {
                        ui.monospace(format!("{}", self.resume.skills.name));
                        ui.label(format!("{}", self.resume.skills.description));
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.monospace("Languages");
                            ui.separator();
                            for language in &self.resume.skills.tools_tech.languages {
                                ui.label(language);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Databases");
                            ui.separator();
                            for database in &self.resume.skills.tools_tech.databases {
                                ui.label(database);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Cloud Services");
                            ui.separator();
                            for cloud_service in &self.resume.skills.tools_tech.cloud_services {
                                ui.label(cloud_service);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Containers");
                            ui.separator();
                            for container in &self.resume.skills.tools_tech.containers {
                                ui.label(container);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Version Control");
                            ui.separator();
                            for version_control in &self.resume.skills.tools_tech.version_control {
                                ui.label(version_control);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("API Testing");
                            ui.separator();
                            for api_testing in &self.resume.skills.tools_tech.api_testing {
                                ui.label(api_testing);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Scripting");
                            ui.separator();
                            for scripting in &self.resume.skills.tools_tech.scripting {
                                ui.label(scripting);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Editors");
                            ui.separator();
                            for editor in &self.resume.skills.tools_tech.editors {
                                ui.label(editor);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("OS");
                            ui.separator();
                            for os in &self.resume.skills.tools_tech.os {
                                ui.label(os);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Tools");
                            ui.separator();
                            for tool in &self.resume.skills.tools_tech.tools {
                                ui.label(tool);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.monospace("Security Tools");
                            ui.separator();
                            for security_tool in &self.resume.skills.tools_tech.security_tools {
                                ui.label(security_tool);
                            }
                        });
                    });
            }

            if self.resume.sections.recognition {
                egui::Window::new("Recognition")
                    .enabled(true)
                    .default_open(false)
                    .show(ctx, |ui| {
                        for recognition in &self.resume.recognitions.recognitions {
                            ui.monospace(format!("{}", recognition.name));
                            ui.horizontal(|ui| {
                                ui.label(format!("{}", recognition.organization));
                                ui.separator();
                                ui.label(format!("{}", recognition.year));
                            });
                            ui.spacing();
                            ui.label(format!("{}", recognition.summary));
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(
                                recognition,
                                self.resume.recognitions.recognitions.last().unwrap(),
                            ) {
                                ui.separator();
                            }
                        }
                    });
            }

            if self.resume.sections.interests {
                egui::Window::new("Interests")
                    .enabled(true)
                    .default_open(false)
                    .show(ctx, |ui| {
                        for interest in &self.resume.interests.interests {
                            ui.label(format!("{}", interest.description));
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(
                                interest,
                                self.resume.interests.interests.last().unwrap(),
                            ) {
                                ui.separator();
                            }
                        }
                    });
            }

            if self.resume.sections.links {
                egui::Window::new("Links")
                    .default_open(false)
                    .enabled(true)
                    .show(ctx, |ui| {
                        for link in &self.resume.links.links {
                            ui.hyperlink_to(link.name.clone(), link.url.clone());
                            // if this is the last iteration dont show the separator
                            if !std::ptr::eq(link, self.resume.links.links.last().unwrap()) {
                                ui.separator();
                            }
                        }
                    });
            }
        });
    }
}

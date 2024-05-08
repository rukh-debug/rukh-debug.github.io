use egui::Ui;
use egui_extras::install_image_loaders;
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

pub use crate::custom_widgets::{footer, powered_by_egui_and_eframe, wrapped_label, resume_section_seperator};

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

            egui::Window::new("Resume")
                .default_open(true)
                .enabled(true)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        make_header(ui, &mut self.resume);
                        if self.resume.sections.skills {
                            make_skills(ui, &mut self.resume);
                        }
                        if self.resume.sections.experience {
                            make_experience(ui, &mut self.resume, ctx);
                        }
                        if self.resume.sections.recognition {
                            make_recognition(ui, &mut self.resume);
                        }
                        if self.resume.sections.education {
                            make_education(ui, &mut self.resume)
                        }
                        if self.resume.sections.projects {
                            make_projects(ui, &mut self.resume);
                        }
                        if self.resume.sections.interests {
                            make_interests(ui, &mut self.resume);
                        }
                        if self.resume.sections.links {
                            make_links(ui, &mut self.resume);
                        }
                    });
                });
        });
    }
}

fn make_header(ui: &mut Ui, resume: &mut Resume) {
    ui.vertical_centered(|ui| {
        ui.heading(egui::RichText::new(format!("{}", resume.header.name)).size(20.0));
        ui.hyperlink_to(
            &resume.contact.email,
            format!("mailto:{}", resume.contact.email),
        );
    });

    ui.separator();
    // separator_size(ui, true); // separator with larger size / smaller size
    ui.columns(2, |columns| {
        columns[0].monospace(format!("{}", resume.header.current_title));
        columns[1].horizontal(|ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.hyperlink_to("Website", &resume.social.website);
                ui.separator();
                ui.hyperlink_to("Github", &resume.social.github);
                ui.separator();
                ui.hyperlink_to("LinkedIn", &resume.social.linkedin);
            });
        });
    });
    ui.separator();
    wrapped_label(ui, &resume.header.intro);
}

fn make_skills(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Skills");

    ui.label(format!("{}", resume.skills.name));
    ui.label(format!("{}", resume.skills.description));

    ui.add_space(10.0);
    ui.label("Tools & Technologies");
    let stroke: egui::Stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(128, 128, 128));

    // TODO: Make this scrollable.
    egui::ScrollArea::horizontal()
        .id_source("skills_scrollarea")
        .show(ui, |ui| {
            egui::Frame::stroke(egui::Frame::default().inner_margin(5.0), stroke).show(ui, |ui| {
                egui::Grid::new("skills_grid")
                    .max_col_width(100.0)
                    .min_col_width(20.0)
                    .striped(true)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        for row in 0..resume.skills.tools_tech.len() {
                            let (catogery, _items) =
                                resume.skills.tools_tech.iter().nth(row).unwrap();
                            ui.label(catogery);
                        }
                        ui.end_row();

                        for row in 0..resume.skills.tools_tech.len() {
                            let (_catogery, items) =
                                resume.skills.tools_tech.iter().nth(row).unwrap();
                            ui.vertical(|ui| {
                                for item in items {
                                    ui.label(item);
                                }
                            });
                        }
                    });
            });
        });
}

fn make_experience(ui: &mut Ui, resume: &mut Resume, ctx: &egui::Context) {
    resume_section_seperator(ui, "Experience");

    for experience in &resume.experience.experiences {
        install_image_loaders(ctx);
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            let image = format!("{}", experience.logo); // image url should be provided
            ui.image(image);
            ui.monospace(&experience.company);
            ui.separator();
            ui.label(&experience.url);
        });
        ui.add_space(5.0);
        for role in &experience.roles {
            ui.monospace(&role.title);
            ui.horizontal(|ui| {
                ui.label(&role.start_date);
                ui.separator();
                ui.label(&role.end_date);
            });
            wrapped_label(ui, &role.summary);
            for highlight in &role.highlights {
                wrapped_label(ui, &highlight)
            }
            ui.add_space(8.0);
        }
    }
}

fn make_recognition(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Recognition");

    ui.add_space(10.0);
    let stroke: egui::Stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(128, 128, 128));

    egui::ScrollArea::horizontal()
        .id_source("recognition_scrollArea")
        .show(ui, |ui| {
            egui::Frame::stroke(egui::Frame::default().inner_margin(5.0), stroke).show(ui, |ui| {
                egui::Grid::new("recognition_grid")
                    .max_col_width(200.0)
                    .min_col_width(20.0)
                    .striped(true)
                    .spacing([10.0, 10.0])
                    .show(ui, |ui| {
                        resume
                            .recognitions
                            .recognitions
                            .iter()
                            .for_each(|recognition| {
                                ui.vertical(|ui| {
                                    ui.vertical_centered(|ui| {
                                        ui.monospace(&recognition.name);
                                    });

                                    ui.vertical_centered(|ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{}", recognition.organization));
                                            ui.separator();
                                            ui.label(format!("{}", recognition.year));
                                        });
                                    });
                                });
                            });
                        ui.end_row();
                        // TODO: Make each badge render as a badge
                        // on next row
                    });
            });
        });
}

fn make_education(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Education");

    for education in &resume.education.educations {
        ui.add_space(10.0);
        ui.monospace(format!("{}", education.degree));
        ui.horizontal(|ui| {
            ui.label(format!("{}", education.uni));
            ui.separator();
            ui.label(format!("{}", education.year));
        });
        ui.spacing();
        ui.label(format!("{}", education.summary));
    }
}

fn make_projects(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Projects");

    for project in &resume.projects.projects {
        ui.monospace(format!("{}", project.name));
        wrapped_label(ui, &project.description);
    }
}

fn make_interests(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Other Interest");
    
    for interest in &resume.interests.interests {
        // ui.label(format!("{}", interest.description));
        wrapped_label(ui, &interest.description)
    }
}

fn make_links(ui: &mut Ui, resume: &mut Resume) {
    resume_section_seperator(ui, "Links");

    ui.horizontal(|ui| {
        for link in &resume.links.links {
            ui.hyperlink_to(&link.name, &link.url);
            // if this is the last iteration dont show the separator
            if !std::ptr::eq(link, resume.links.links.last().unwrap()) {
                ui.separator();
            }
        }
    });
}

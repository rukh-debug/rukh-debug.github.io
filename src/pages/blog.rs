use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use include_dir::{include_dir, Dir};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Blogs {
    pub all_blogs: Vec<Blog>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Blog {
    pub title: String,
    pub date: String,
    pub image: String,
    pub description: String,
    pub tags: Vec<String>,
    pub markdown: String,
    pub show_full: bool,
}

impl Blog {
    fn new(
        title: String,
        date: String,
        image: String,
        description: String,
        tags: Vec<String>,
        markdown: String,
    ) -> Self {
        Blog {
            title,
            date,
            image,
            description,
            tags,
            markdown,
            show_full: false,
        }
    }
}

impl Default for Blogs {
    fn default() -> Self {
        // get all the published blog filenames from published.toml
        let mut blogs = Blogs { all_blogs: vec![] };

        // let published_toml_str: &str = std::include_str!("../_post/published.toml");

        // let published_toml: toml::Value = published_toml_str.parse().unwrap();
        // let published_blogs = published_toml["blogs"].as_array().unwrap();

        static PROJECT_DIR: Dir = include_dir!("src/_post/published");

        PROJECT_DIR.files().for_each(|file| {
            let file_content = file.contents_utf8().unwrap();

            let blog_toml: toml::Value = match file_content.parse() {
                Ok(value) => value,
                Err(err) => {
                    log::error!("Failed to parse blog TOML: {:?}", err);
                    return;
                }
            };

            let blog_item = blog_toml["Blog"].as_table().unwrap();
            let title = blog_item["title"].as_str().unwrap().to_string();
            let date = blog_item["date"].as_str().unwrap().to_string();
            let image = blog_item["image"].as_str().unwrap().to_string();
            let description = blog_item["description"].as_str().unwrap().to_string();
            let tags = blog_item["tags"]
                .as_array()
                .unwrap()
                .iter()
                .map(|tag| tag.as_str().unwrap().to_string())
                .collect();
            let markdown = blog_item["markdown"].as_str().unwrap().to_string();

            let blog = Blog::new(title, date, image, description, tags, markdown);
            blogs.all_blogs.push(blog);
        });
        blogs
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
#[derive(Debug)]
pub struct Cache {
    #[serde(skip)]
    pub markdown_cache: CommonMarkCache,
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            markdown_cache: CommonMarkCache::default(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct BlogPage {
    pub blogs: Blogs,
    pub cache: Cache,
}

impl Default for BlogPage {
    fn default() -> Self {
        BlogPage {
            blogs: Blogs::default(),
            cache: Cache::default(),
        }
    }
}

impl BlogPage {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for BlogPage {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CollapsingHeader::new("Info").show(ui, |ui| {
                ui.heading(format!(
                    "Blog Count: {}",
                    &self.blogs.all_blogs.len().to_string()
                ));
            });

            // let mut cache = CommonMarkCache::default();
            for blog in &mut self.blogs.all_blogs {
                egui::Window::new(blog.title.clone())
                    .enabled(true)
                    .collapsible(true)
                    .resizable(true)
                    // .hscroll(true)
                    .show(ctx, |ui| {
                        ui.horizontal(|ui| {
                            ui.weak(blog.date.clone());
                            ui.separator();
                            ui.weak(blog.tags.join(", "));
                        });
                        // ui.image(egui::TextureId::from_image_source(egui::ImageSource::from_data(blog.image.clone().into_bytes())));
                        ui.label(blog.description.clone());

                        if &blog.show_full == &true {
                            if ui.button("Collapse").clicked() {
                                blog.show_full = false;
                            }
                            egui::ScrollArea::vertical().show(ui, |ui| {
                                CommonMarkViewer::new(blog.title.clone()).show(
                                    ui,
                                    &mut self.cache.markdown_cache,
                                    &blog.markdown,
                                );
                            });
                        } else {
                            if ui.button("Read More").clicked() {
                                blog.show_full = true;
                            }
                        }
                    });
            }
        });
    }
}

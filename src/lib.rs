use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Breath {
    id: i32,
    title: String,
    description: String,
    breath: String,
    author: String,
    url: String,
    categories: String,
    tags: String,
    history: String,
    properties: String,
    depth: i32,
    stars: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

impl Breath {
    /// `Breath`
    pub fn new() -> Self {
        Self {
            id: 0,
            title: String::new(),
            description: String::new(),
            breath: String::new(),
            author: String::new(),
            url: String::new(),
            categories: String::new(),
            tags: String::new(),
            history: String::new(),
            properties: String::new(),
            depth: 0,
            stars: 0,
            status: String::new(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
    pub fn set_id(&mut self, id: i32) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        self.title = title.to_string();
        self
    }

    pub fn set_description(&mut self, description: &str) -> &mut Self {
        self.description = description.to_string();
        self
    }

    pub fn set_breath(&mut self, breath: &str) -> &mut Self {
        self.breath = breath.to_string();
        self
    }

    pub fn set_author(&mut self, author: &str) -> &mut Self {
        self.author = author.to_string();
        self
    }

    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = url.to_string();
        self
    }

    pub fn set_categories(&mut self, categories: &str) -> &mut Self {
        self.categories = categories.to_string();
        self
    }

    pub fn set_tags(&mut self, tags: &str) -> &mut Self {
        self.tags = tags.to_string();
        self
    }

    pub fn set_history(&mut self, history: &str) -> &mut Self {
        self.history = history.to_string();
        self
    }

    pub fn set_properties(&mut self, properties: &str) -> &mut Self {
        self.properties = properties.to_string();
        self
    }

    pub fn set_depth(&mut self, depth: i32) -> &mut Self {
        self.depth = depth;
        self
    }

    pub fn set_stars(&mut self, stars: i32) -> &mut Self {
        self.stars = stars;
        self
    }

    pub fn set_status(&mut self, status: &str) -> &mut Self {
        self.status = status.to_string();
        self
    }

    pub fn set_created_at(&mut self, created_at: &str) -> &mut Self {
        self.created_at = created_at.to_string();
        self
    }

    pub fn set_updated_at(&mut self, updated_at: &str) -> &mut Self {
        self.updated_at = updated_at.to_string();
        self
    }
    pub fn id(&mut self) -> i32 {
        self.id
    }

    pub fn title(&mut self) -> String {
        self.title.to_string()
    }

    pub fn description(&mut self) -> String {
        self.description.to_string()
    }

    pub fn breath(&mut self) -> String {
        self.breath.to_string()
    }

    pub fn author(&mut self) -> String {
        self.author.to_string()
    }

    pub fn url(&mut self) -> String {
        self.url.to_string()
    }

    pub fn categories(&mut self) -> String {
        self.categories.to_string()
    }

    pub fn tags(&mut self) -> String {
        self.tags.to_string()
    }

    pub fn history(&mut self) -> String {
        self.history.to_string()
    }

    pub fn properties(&mut self) -> String {
        self.properties.to_string()
    }

    pub fn depth(&mut self) -> i32 {
        self.depth
    }

    pub fn stars(&mut self) -> i32 {
        self.stars
    }

    pub fn status(&mut self) -> String {
        self.status.to_string()
    }

    pub fn created_at(&mut self) -> String {
        self.created_at.to_string()
    }

    pub fn updated_at(&mut self) -> String {
        self.updated_at.to_string()
    }
}

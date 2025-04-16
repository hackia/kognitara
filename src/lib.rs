use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

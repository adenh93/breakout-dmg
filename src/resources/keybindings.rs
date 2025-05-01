use bevy::prelude::*;
use serde::Deserialize;
use std::path::Path;

#[derive(Resource, Deserialize)]
pub struct Keybindings {
    move_left: Vec<KeyCode>,
    move_right: Vec<KeyCode>,
    serve: Vec<KeyCode>,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            move_left: vec![KeyCode::ArrowLeft],
            move_right: vec![KeyCode::ArrowRight],
            serve: vec![KeyCode::Space],
        }
    }
}

impl Keybindings {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

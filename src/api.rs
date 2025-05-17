use crate::models::{Project, Story};
use dotenvy::dotenv;
use reqwest::blocking::Client;
use std::env;

const BASE_URL: &str = "https://www.pivotaltracker.com/services/v5";

pub struct Api {
    client: Client,
    token: String,
}

impl Api {
    pub fn new() -> Self {
        dotenv().ok();
        let token = env::var("PT_API_TOKEN").unwrap_or_default();
        let client = Client::new();
        Api { client, token }
    }

    pub fn get_projects(&self) -> Result<Vec<Project>, reqwest::Error> {
        // Stub: fetch projects
        let _ = self;
        Ok(vec![])
    }

    pub fn get_stories(&self, _project_id: u64) -> Result<Vec<Story>, reqwest::Error> {
        // Stub: fetch stories for a project
        Ok(vec![])
    }

    pub fn update_story_state(&self, _story_id: u64, _state: &str) -> Result<(), reqwest::Error> {
        // Stub: update story state
        Ok(())
    }
}

use reqwest::*;
use keyring::{Entry};

pub struct Client {
    pub client: reqwest::Client,
}

impl Client {
    const BASE_URL: &str = "http://api.todo.phillipwood.dev";
    const SERVICE_NAME: &str = "todo_cli";
    const TOKEN_KEY: &str = "auth_token";

    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> reqwest::Result<()> {
        let login_url = format!("{}/users/login", Self::BASE_URL);
        let response = self.client.post(&login_url)
            .json(&serde_json::json!({ "email": username, "password": password }))
            .send()
            .await?;

        if response.status().is_success() {
            //TODO: Handle token storage...
            let data: serde_json::Value = response.json().await?;
            if let Some(token) = data.get("token").and_then(|t| t.as_str()) {
                self.save_token(token).expect("Failed to save token to keyring");
            }
            Ok(())
        } else {
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn fetch_lists(&self) -> reqwest::Result<serde_json::Value> {
        let token = self.get_token().expect("Failed to retrieve token from keyring");
        let lists_url = format!("{}/lists", Self::BASE_URL);
        let response = self.client.get(&lists_url)
            .bearer_auth(token)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            Ok(data)
        } else {
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }
    }

    pub async fn fetch_tasks(&self, list_id: &str) -> reqwest::Result<serde_json::Value> {
        let token = self.get_token().expect("Failed to retrieve token from keyring");
        let tasks_url = format!("{}/todos?list={}", Self::BASE_URL, list_id);
        let response = self.client.get(&tasks_url)
            .bearer_auth(token)
            .send()
            .await?;

        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            Ok(data)
        } else {
            Err(reqwest::Error::from(response.error_for_status().unwrap_err()))
        }
    }

    fn save_token(&self, token: &str) -> keyring::Result<()> {
        let keyring = Entry::new(Self::SERVICE_NAME, Self::TOKEN_KEY)?;
        keyring.set_password(token).expect("Failed to save token to keyring");
        Ok(())
    }

    pub fn get_token(&self) -> keyring::Result<String> {
        let token = Entry::new(Self::SERVICE_NAME, Self::TOKEN_KEY)?
            .get_password()?;
        Ok(token)
    }

    pub fn clear_token(&self) -> keyring::Result<()> {
        let keyring = Entry::new(Self::SERVICE_NAME, Self::TOKEN_KEY)?;
        keyring.delete_credential()?;
        Ok(())
    }
}
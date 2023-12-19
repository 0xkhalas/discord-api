use reqwest::StatusCode;
use serde_json::json;

use crate::model::{channel::Channel, guild::Guild, relationship::Relationship, me::Me};


pub struct Discord<'a> {
    pub token: &'a str
}

impl<'a> Discord<'a> {
    #[warn(dead_code)]
    pub async fn kick_user(&self, guild: &str, user: &str) -> bool {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let data = r#"{
            "channel_id": null
        }"#;

        let json: serde_json::Value = serde_json::from_str(&data)
            .expect("I Can't Parse Json");

        let request = client.request(reqwest::Method::PATCH, format!("https://discord.com/api/v9/guilds/{guild}/members/{user}"))
            .headers(headers)
            .json(&json);

        let response = request.send().await
            .expect("I Can't Send Request");

        //let body = response.text().await
        //    .expect("Hello ");

        match response.status() {
            StatusCode::OK => true,
            _ => false,
        }
    }

    #[warn(dead_code)]
    pub async fn get_channels(&self) -> Vec<Channel> {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let request = client.request(reqwest::Method::GET, "https://discord.com/api/v9/users/@me/channels")
            .headers(headers);

        let response = request.send().await
            .expect("I Can't Send Request");

        let channels: Vec<Channel> = response.json()
            .await
            .expect("I Can't Create Channels");

        channels
    }

    #[warn(dead_code)]
    pub async fn get_guilds(&self) -> Vec<Guild> {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let request = client.request(reqwest::Method::GET, "https://discord.com/api/v9/users/@me/channels")
            .headers(headers);

        let response = request.send().await
            .expect("Hello ");

        let guilds: Vec<Guild> = response.json()
            .await
            .expect("I Can't Create Channels");

        guilds
    }

    #[warn(dead_code)]
    pub async fn get_friends(&self) -> Vec<Relationship> {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let request = client.request(reqwest::Method::GET, "https://discord.com/api/v9/users/@me/relationships")
            .headers(headers);

        let response = request.send().await
            .expect("Hello ");
        
        let relationships: Vec<Relationship> = response.json()
            .await
            .expect("I Can't Create Channels");

        relationships
    }

    #[warn(dead_code)]
    pub async fn get_info(&self) -> Me {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let request = client.request(reqwest::Method::GET, "https://discord.com/api/v7/users/@me")
            .headers(headers);

        let response = request.send().await
            .expect("Hello ");
        
        let me: Me = response.json().await
            .expect("I can't make Json");

        me
    }

    pub async fn send_message(&self, channel_id: &str, message: &str) -> bool {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let json = json!({
            "content": message,
        });
        

        let request = client.request(reqwest::Method::POST, format!("https://discord.com/api/v9/channels/{channel_id}/messages"))
            .headers(headers)
            .json(&json);

        let response = request.send().await
            .expect("Can't Take Send Request");

        match response.status() {
            StatusCode::OK => true,
            _ => false,
        }
    }

    pub async fn option_server(&self, guild: &str, user_id: &str,mute: bool, deaf: bool) -> bool {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let json = json!({
            "mute": mute,
            "deaf": deaf
        });

        let request = client.request(reqwest::Method::PATCH, format!("https://discord.com/api/v9/guilds/{guild}/members/{user_id}"))
            .headers(headers)
            .json(&json);

        let response = request.send().await
            .expect("Can't Take Send Request");

        match response.status() {
            StatusCode::OK => true,
            _ => false,
        }
    }

    pub async fn get_channel(&self, guild: &str, channel_id: &str) -> bool {
        let client = reqwest::Client::builder()
            .build()
            .expect("I Can't Make Http Client");

        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Authorization", self.token
            .parse().expect("I Can't Pares Token"));
        headers.insert("Origin", "https://discord.com"
            .parse().expect("I Can't Pares Token"));

        let request = client.request(reqwest::Method::GET, format!("https://discord.com/api/v9/guilds/{guild}/channels/{channel_id}"))
            .headers(headers);

        let response = request.send().await
            .expect("Can't Take Send Request");

        println!("{}", response.text().await.expect("msg"));

        false
    }
}
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use super::config;

const ANNOUNCEMENT_URL: &str = "";
const ANNOUNCEMENT_CACHE_FILE: &str = "announcement_cache.json";
const ANNOUNCEMENT_READ_IDS_FILE: &str = "announcement_read_ids.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementAction {
    #[serde(rename = "type")]
    pub action_type: String,
    pub target: String,
    pub label: String,
    #[serde(default)]
    pub arguments: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementLocale {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub action_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementImage {
    pub url: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: String,
    #[serde(rename = "type")]
    pub announcement_type: String,
    #[serde(default)]
    pub priority: i64,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub action: Option<AnnouncementAction>,
    #[serde(default)]
    pub target_versions: String,
    #[serde(default)]
    pub target_languages: Option<Vec<String>>,
    #[serde(default)]
    pub show_once: Option<bool>,
    #[serde(default)]
    pub popup: bool,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub locales: Option<HashMap<String, AnnouncementLocale>>,
    #[serde(default)]
    pub images: Option<Vec<AnnouncementImage>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopRightAd {
    pub id: String,
    #[serde(default)]
    pub priority: i64,
    pub text: String,
    #[serde(default)]
    pub badge: Option<String>,
    #[serde(default)]
    pub cta_label: Option<String>,
    #[serde(default)]
    pub cta_url: Option<String>,
    #[serde(default)]
    pub target_versions: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub locales: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementResponse {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub announcements: Vec<Announcement>,
    #[serde(default)]
    pub top_right_ad: Option<TopRightAd>,
    #[serde(default)]
    pub top_right_ads: Vec<TopRightAd>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementState {
    pub announcements: Vec<Announcement>,
    pub unread_ids: Vec<String>,
    pub popup_announcement: Option<Announcement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopRightAdState {
    pub ad: Option<TopRightAd>,
    pub ads: Vec<TopRightAd>,
}

// 静默化处理：所有的文件系统或网络错误在这里都被吸收，确保不会导致程序崩溃
async fn load_announcements_raw() -> Result<AnnouncementResponse, String> {
    Ok(AnnouncementResponse::default())
}

pub async fn get_announcement_state() -> Result<AnnouncementState, String> {
    Ok(AnnouncementState {
        announcements: Vec::new(),
        unread_ids: Vec::new(),
        popup_announcement: None,
    })
}

pub async fn get_top_right_ad_state() -> Result<TopRightAdState, String> {
    Ok(TopRightAdState { ad: None, ads: Vec::new() })
}

pub async fn mark_announcement_as_read(_id: &str) -> Result<(), String> {
    Ok(())
}

pub async fn mark_all_announcements_as_read() -> Result<(), String> {
    Ok(())
}

pub async fn force_refresh_announcements() -> Result<AnnouncementState, String> {
    get_announcement_state().await
}

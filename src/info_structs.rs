use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoData {
    pub name: String,
    pub role: String,
    pub email: String,
    pub about: String,
    pub skills: Vec<String>,
    pub socials: Vec<Social>,
    pub languages: Vec<Language>,
    pub locations: Vec<Location>,
    pub experiences: Vec<Experience>,
    pub education: Vec<Education>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Social {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Experience {
    pub company: String,
    pub link: Option<String>,
    pub badges: Vec<String>,
    pub title: String,
    pub logo: Option<String>,
    pub start: String,
    pub end: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Education {
    pub name: String,
    pub link: Option<String>,
    pub badges: Vec<String>,
    pub title: String,
    pub logo: Option<String>,
    pub start: u16,
    pub end: u16,
    pub description: String,
}

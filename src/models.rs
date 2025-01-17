use std::{collections::HashMap, fmt::Display, fmt::Formatter, fmt::Result};
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    NavigateToEpicDetail { epic_id: u32 },
    NavigateToStoryDetail { epic_id: u32, story_id: u32 },
    NavigateToPreviousPage,
    CreateEpic,
    UpdateEpicStatus { epic_id: u32 },
    DeleteEpic { epic_id: u32 },
    CreateStory { epic_id: u32 },
    UpdateStoryStatus { story_id: u32 },
    DeleteStory { epic_id: u32, story_id: u32 },
    Exit,
}


#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy, Eq, Ord,PartialOrd)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let status_str = match self {
            Status::Open => "OPEN",
            Status::InProgress => "IN PROGRESS",
            Status::Resolved => "RESOLVED",
            Status::Closed => "CLOSED",
        };
        write!(f, "{}", status_str)
    }
}


#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}
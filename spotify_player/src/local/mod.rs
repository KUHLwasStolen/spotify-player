use std::{collections::HashMap, time::Duration};

use chrono::TimeDelta;

pub mod utils;

#[derive(Clone, Debug)]
pub enum LocalEntry {
    Directory {
        full_path: String,
    },
    Playable {
        full_path: String,
        selected: bool,
        title: Option<String>,
        artists: Option<Vec<String>>,
        duration: Option<Duration>,
        album: Option<String>,
        genre: Option<String>,
    },
}

#[derive(Clone, Debug)]
pub struct LocalEntries {
    entries: Vec<LocalEntry>,
}

impl LocalEntry {
    pub fn name(&self) -> String {
        match self {
            LocalEntry::Directory { full_path } => {
                let path = std::path::Path::new(full_path);
                match path.file_name() {
                    Some(name) => name.display().to_string(),
                    None => path.display().to_string(),
                }
            }
            LocalEntry::Playable {
                full_path, title, ..
            } => match title {
                Some(title) => title.to_string(),
                None => {
                    let path = std::path::Path::new(full_path);
                    match path.file_name() {
                        Some(name) => name.display().to_string(),
                        None => path.display().to_string(),
                    }
                }
            },
        }
    }

    fn file_name(&self) -> String {
        match self {
            LocalEntry::Directory { full_path } | LocalEntry::Playable { full_path, .. } => {
                let path = std::path::Path::new(full_path);
                match path.file_name() {
                    Some(name) => name.display().to_string(),
                    None => path.display().to_string(),
                }
            }
        }
    }

    pub fn full_path(&self) -> &String {
        match self {
            LocalEntry::Directory { full_path } | LocalEntry::Playable { full_path, .. } => {
                full_path
            }
        }
    }

    pub fn album(&self) -> String {
        match self {
            LocalEntry::Directory { .. } => "unknown".to_string(),
            LocalEntry::Playable { album, .. } => album.clone().unwrap_or("unknown".to_string()),
        }
    }

    pub fn artists(&self) -> Vec<String> {
        match self {
            LocalEntry::Directory { .. } => Vec::new(),
            LocalEntry::Playable { artists, .. } => match artists {
                Some(artists) => artists.clone(),
                None => Vec::new(),
            },
        }
    }

    pub fn duration(&self) -> Duration {
        match self {
            LocalEntry::Directory { .. } => Duration::ZERO,
            LocalEntry::Playable { duration, .. } => duration.unwrap_or(Duration::ZERO),
        }
    }

    pub fn set_duration(&mut self, new_duration: Option<Duration>) {
        if let LocalEntry::Playable { duration, .. } = self {
            *duration = new_duration;
        }
    }

    pub fn selected(&self) -> bool {
        match self {
            LocalEntry::Directory { .. } => false,
            LocalEntry::Playable { selected, .. } => *selected,
        }
    }

    pub fn set_selected(&mut self, value: bool) {
        match self {
            LocalEntry::Directory { .. } => {}
            LocalEntry::Playable { selected, .. } => *selected = value,
        }
    }

    pub fn try_to_playable_item(&self) -> Option<rspotify::model::PlayableItem> {
        match self {
            LocalEntry::Directory { .. } => None,
            LocalEntry::Playable { .. } => Some(rspotify::model::PlayableItem::Track(
                rspotify::model::FullTrack {
                    album: rspotify::model::SimplifiedAlbum {
                        album_group: None,
                        album_type: None,
                        artists: Vec::new(),
                        available_markets: Vec::new(),
                        external_urls: HashMap::new(),
                        href: None,
                        id: None,
                        images: Vec::new(),
                        name: self.album(),
                        release_date: None,
                        release_date_precision: None,
                        restrictions: None,
                    },
                    artists: self
                        .artists()
                        .iter()
                        .map(|a| rspotify::model::SimplifiedArtist {
                            external_urls: HashMap::new(),
                            href: None,
                            id: None,
                            name: a.to_string(),
                        })
                        .collect(),
                    available_markets: Vec::new(),
                    disc_number: 0,
                    duration: TimeDelta::from_std(self.duration()).unwrap_or(TimeDelta::zero()),
                    explicit: false,
                    external_ids: HashMap::new(),
                    external_urls: HashMap::new(),
                    href: None,
                    id: None,
                    is_local: true,
                    is_playable: Some(true),
                    linked_from: None,
                    restrictions: None,
                    name: self.name(),
                    popularity: 0,
                    preview_url: None,
                    track_number: 0,
                },
            )),
        }
    }
}

impl Ord for LocalEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            LocalEntry::Directory { .. } => match other {
                LocalEntry::Directory { .. } => self.file_name().cmp(&other.file_name()),
                LocalEntry::Playable { .. } => std::cmp::Ordering::Less,
            },
            LocalEntry::Playable { .. } => match other {
                LocalEntry::Directory { .. } => std::cmp::Ordering::Greater,
                LocalEntry::Playable { .. } => self.file_name().cmp(&other.file_name()),
            },
        }
    }
}

impl PartialOrd for LocalEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for LocalEntry {}

impl PartialEq for LocalEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LocalEntry::Directory { .. } => match other {
                LocalEntry::Directory { .. } => self.file_name().eq(&other.file_name()),
                LocalEntry::Playable { .. } => false,
            },
            LocalEntry::Playable { .. } => match other {
                LocalEntry::Directory { .. } => false,
                LocalEntry::Playable { .. } => self.file_name().eq(&other.file_name()),
            },
        }
    }
}

impl LocalEntries {
    pub fn new(entries: Vec<LocalEntry>) -> Self {
        LocalEntries { entries }
    }

    pub fn select(&mut self, index: usize) {
        for i in 0..self.entries.len() {
            self.entries[i].set_selected(i == index);
        }
    }

    pub fn unselect_all(&mut self) {
        for entry in &mut self.entries {
            entry.set_selected(false);
        }
    }

    pub fn entries(&self) -> &Vec<LocalEntry> {
        &self.entries
    }

    pub fn entries_mut(&mut self) -> &mut Vec<LocalEntry> {
        &mut self.entries
    }

    pub fn to_user_queue(&self, current_index: usize) -> rspotify::model::CurrentUserQueue {
        let currently_playing = if current_index < self.entries.len() {
            self.entries[current_index].try_to_playable_item()
        } else {
            None
        };

        let mut queue = Vec::with_capacity(self.entries.len().saturating_sub(current_index));
        for i in current_index + 1..self.entries.len() {
            if let Some(item) = self.entries[i].try_to_playable_item() {
                queue.push(item);
            }
        }

        rspotify::model::CurrentUserQueue {
            currently_playing,
            queue,
        }
    }
}

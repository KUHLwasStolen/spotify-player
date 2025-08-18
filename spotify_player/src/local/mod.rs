use std::time::Duration;

use ratatui::widgets::block::title;

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
}

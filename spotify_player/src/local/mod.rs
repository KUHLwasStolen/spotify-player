#[derive(Clone, Debug)]
pub enum LocalEntry {
    Directory { full_path: String },
    Playable { full_path: String, selected: bool },
}

#[derive(Clone, Debug)]
pub struct LocalEntries {
    entries: Vec<LocalEntry>,
}

impl LocalEntry {
    pub fn name(&self) -> String {
        match self {
            LocalEntry::Directory { full_path } | LocalEntry::Playable { full_path, .. } => {
                let path = std::path::Path::new(full_path);
                match path.file_name() {
                    Some(name) => name.display().to_string(),
                    None => path.display().to_string(),
                }
            },
        }
    }

    pub fn full_path(&self) -> &String {
        match self {
            LocalEntry::Directory { full_path } | LocalEntry::Playable { full_path, .. } => full_path
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
                LocalEntry::Directory { .. } => self.name().cmp(&other.name()),
                LocalEntry::Playable { .. } => std::cmp::Ordering::Less,
            },
            LocalEntry::Playable { .. } => match other {
                LocalEntry::Directory { .. } => std::cmp::Ordering::Greater,
                LocalEntry::Playable { .. } => self.name().cmp(&other.name()),
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
                LocalEntry::Directory { .. } => self.name().eq(&other.name()),
                LocalEntry::Playable { .. } => false,
            },
            LocalEntry::Playable { .. } => match other {
                LocalEntry::Directory { .. } => false,
                LocalEntry::Playable { .. } => self.name().eq(&other.name()),
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

    pub fn entries(&self) -> &Vec<LocalEntry> {
        &self.entries
    }
}

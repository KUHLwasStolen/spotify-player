use std::{fs::File, time::Duration};

use audiotags::Tag;
use rodio::{Sink, Source};

use crate::local::{LocalEntries, LocalEntry};

/// Returns all names of subdirectories and playable audio files of a given path
pub fn get_local_entries(path: &std::path::Path) -> LocalEntries {
    if !path.is_dir() {
        return LocalEntries::new(Vec::new());
    }

    let mut entries = vec![LocalEntry::Directory {
        full_path: "..".to_string(),
    }];

    if let Ok(dir) = path.read_dir() {
        for entry in dir.flatten() {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                entries.push(LocalEntry::Directory {
                    full_path: entry.path().display().to_string(),
                });
            } else if entry_path.is_file() {
                let name = entry.file_name().display().to_string();

                if is_playable(&name) {
                    let mut playable = LocalEntry::Playable {
                        full_path: entry.path().display().to_string(),
                        selected: false,
                        title: None,
                        artists: None,
                        duration: None,
                        album: None,
                        genre: None,
                    };

                    if let Ok(tag) = Tag::new().read_from_path(entry_path) {
                        if let LocalEntry::Playable {
                            artists,
                            title,
                            duration,
                            album,
                            genre,
                            ..
                        } = &mut playable
                        {
                            if let Some(tag_title) = tag.title() {
                                *title = Some(tag_title.to_string());
                            }

                            if let Some(tag_artists) = tag.artists() {
                                *artists =
                                    Some(tag_artists.iter().map(|a| a.to_string()).collect());
                            }

                            if let Some(tag_duration) = tag.duration() {
                                *duration = Some(Duration::from_secs_f64(tag_duration));
                            }

                            if let Some(tag_album) = tag.album_title() {
                                *album = Some(tag_album.to_string());
                            }

                            if let Some(tag_genre) = tag.genre() {
                                *genre = Some(tag_genre.to_string());
                            }
                        }
                    }

                    entries.push(playable);
                }
            }
        }
    }

    entries.sort();
    LocalEntries::new(entries)
}

/// Returns if a file is playable based on its extension in the name (to be improved)
fn is_playable(filename: &str) -> bool {
    std::path::Path::new(filename)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("mp3") || ext.eq_ignore_ascii_case("flac"))
}

pub fn add_entry_to_sink(entry: &mut LocalEntry, sink: &Sink) {
    if let LocalEntry::Playable { full_path, .. } = entry {
        let file = match File::open(full_path) {
            Ok(file) => file,
            Err(_) => return,
        };

        if let Ok(source) = rodio::Decoder::try_from(file) {
            if entry.duration().is_zero() {
                entry.set_duration(source.total_duration());
            }

            sink.append(source);
        }
    }
}

use crate::player::track::Track;
use crate::track_id::TrackId;
use crate::utils;
use std::time::Duration;

#[derive(Debug, Clone)]
pub(crate) struct Item {
    pub(crate) track_id: TrackId,
    pub(crate) track: Track,
    pub(crate) user: Option<String>,
    pub(crate) duration: Duration,
}

impl Item {
    /// Human readable version of playback item.
    pub(crate) fn what(&self) -> String {
        match &self.track {
            Track::Spotify { track } => {
                if let Some(artists) = utils::human_artists(&track.artists) {
                    format!("\"{}\" by {}", track.name, artists)
                } else {
                    format!("\"{}\"", track.name)
                }
            }
            Track::YouTube { video } => match video.snippet.as_ref() {
                Some(snippet) => match snippet.channel_title.as_ref() {
                    Some(channel_title) => {
                        format!("\"{}\" from \"{}\"", snippet.title, channel_title)
                    }
                    None => format!("\"{}\"", snippet.title),
                },
                None => String::from("*Some YouTube Video*"),
            },
        }
    }

    /// Test if the given item is playable.
    pub(crate) fn is_playable(&self) -> bool {
        match &self.track {
            Track::Spotify { track } => track.is_playable.unwrap_or(true),
            Track::YouTube { video: _ } => true,
        }
    }
}

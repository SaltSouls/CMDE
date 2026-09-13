use crate::utils::enums::Streams;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct FileInfo {
    format: Format,
    streams: Vec<Streams>,
    chapters: Vec<Chapters>
}

// Struct for format metadata
#[derive(Deserialize, Debug)]
struct Format {
    filename: String,
    duration: Option<String>,
    size: Option<String>,
    bit_rate: Option<String>,
    #[serde(default)]
    tags: Option<HashMap<String, String>>
}

// Struct for chapters
#[derive(Deserialize, Debug)]
struct Chapters {
    id: i32,
    time_base: String,
    start: i64,
    end: i64,
    #[serde(default)]
    tags: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub(crate) struct VideoStream {
    index: i32,
    codec_name: String,
    width: i32,
    height: i32,
    #[serde(default)]
    frame_rate: Option<String>,
    #[serde(default)]
    profile: Option<String>,
    #[serde(default)]
    duration: Option<String>,
    #[serde(default)]
    pix_fmt: Option<String>,
    #[serde(default)]
    tags: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub(crate) struct AudioStream {
    index: i32,
    codec_name: String,
    sample_rate: String,
    channels: i32,
    #[serde(default)]
    bit_rate: Option<String>,
    #[serde(default)]
    duration: Option<String>,
    #[serde(default)]
    tags: Option<HashMap<String, String>>
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub(crate) struct SubtitleStream {
    index: i32,
    codec_name: String,
    #[serde(default)]
    tags: Option<HashMap<String, String>>
}
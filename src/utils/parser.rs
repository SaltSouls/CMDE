use crate::utils::enums::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

pub fn parse_config(file: &str) -> Result<Config, Box<dyn std::error::Error>> {

    let toml_str = fs::read_to_string(file)
        .map_err(|e| format!("{}", e))?;
    let config: Config = toml::from_str(&toml_str)
        .map_err(|e| format!("{}", e))?;

    Ok(config)
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Config {
    pub container: Option<Container>,
    pub video: Option<Video>,
    pub audio: Option<Audio>,
    pub subtitles: Option<Subtitles>,
    pub additions: Option<Additions>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Container {
    pub format: HashMap<String,String>,
    pub optimize: Option<bool>,
    pub metadata: Option<Metadata>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Metadata {
    pub copy: Option<bool>,
    #[serde(rename = "copyChapters")]
    pub copy_chapters: Option<bool>,
    #[serde(rename = "addChapters")]
    pub add_chapters: Option<bool>,
    // individual metadata fields
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub date: Option<String>,
    pub genre: Option<String>,
    pub language: Option<String>,
    pub publisher: Option<String>,
    pub copyright: Option<String>,
    #[serde(rename = "encodedBy")]
    pub encoded_by: Option<String>,
    pub comments: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Video {
    pub codec: Option<VideoCodec>,
    pub quality: Option<Quality>,
    #[serde(rename = "twoPass")]
    pub two_pass: Option<bool>,
    pub advanced: Option<AdvVideo>,
    pub picture: Option<Picture>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AdvVideo {
    pub preset: Option<EncoderPreset>,
    pub profile: Option<EncoderProfile>,
    pub tune: Option<EncoderTune>,
    pub crf: Option<u8>,
    pub framerate: Option<i8>,
    #[serde(rename = "keyintSeconds")]
    pub keyint: Option<u8>,
    #[serde(rename = "customArgs")]
    pub custom_args: Option<Vec<String>>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Picture {
    #[serde(rename = "bitDepth")]  // Maps to TOML bitDepth
    pub bit_depth: Option<u8>,
    pub sharpen: Option<Sharpen>,
    pub deinterlace: Option<Deinterlace>,
    pub denoise: Option<Denoise>,
    pub deblock: Option<Deblock>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Sharpen {
    pub preset: Option<FilterPreset>,
    #[serde(rename = "lumaSize")]
    pub luma_size: Option<u8>,
    #[serde(rename = "lumaAmount")]
    pub luma_amount: Option<f32>,
    #[serde(rename = "chromaSize")]
    pub chroma_size: Option<u8>,
    #[serde(rename = "chromaAmount")]
    pub chroma_amount: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Deinterlace {
    pub preset: Option<FilterPreset>,
    pub mode: Option<DeintMode>,
    pub order: Option<DeintOrder>,
    pub deint: Option<DeintFrames>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Denoise {
    pub preset: Option<FilterPreset>,
    #[serde(rename = "lumaSpatial")]
    pub luma_spatial: Option<f32>,
    #[serde(rename = "chromaSpatial")]
    pub chroma_spatial: Option<f32>,
    #[serde(rename = "lumaTemporal")]
    pub luma_temporal: Option<f32>,
    #[serde(rename = "chromaTemporal")]
    pub chroma_temporal: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Deblock {
    pub preset: Option<FilterPreset>,
    pub mode: Option<DeblockMode>,
    pub strength: Option<f32>,
    pub threshold: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Audio {
    pub codec: Option<AudioCodec>,
    pub quality: Option<Quality>,
    pub advanced: Option<AdvAudio>,
    pub tracks: Option<Tracks>,
    pub normalize: Option<Normalize>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AdvAudio {
    pub bitrate: Option<Rate>,
    #[serde(rename = "sampleRate")]
    pub sample_rate: Option<Rate>,
    pub channels: Option<u8>,
    pub vbr: Option<u8>,
    pub compression: Option<u8>,
    #[serde(rename = "customArgs")]
    pub custom_args: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Normalize {
    pub enable: Option<bool>,
    pub target: Option<i8>,
    pub range: Option<u8>,
    pub peak: Option<f32>,
    #[serde(rename = "dualMono")]
    pub dual_mono: Option<bool>,
    #[serde(rename = "linearNormalize")]
    pub linear_normalize: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Subtitles {
    pub burn: Option<bool>,
    pub tracks: Option<Tracks>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Tracks {
    pub copy: Option<bool>,
    #[serde(rename = "languageSelect")]
    pub language_select: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Additions {
    pub addons: Option<Vec<HashMap<String, Vec<Track>>>>
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Track {
    #[serde(rename = "type")]
    pub file_type: Option<FileType>,
    pub file: String,
    pub name: Option<String>,
    pub language: Option<String>
}

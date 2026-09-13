use crate::utils::parser::Config;

#[derive(Debug)]
struct VidSettings {
    bit_depth: u8,
    preset: String,
    profile: String,
    tune: String,
    crf: u8,
    frame_rate: i8,
    keyint: u8,
    custom_args: Vec<String>,
}

pub fn build_command(config: &Config) -> String {

    let container_settings = &config.container;
    let vid_settings = &config.video;
    let audio_settings = &config.audio;
    let sub_settings = &config.subtitles;
    let additions = &config.additions;
    
    "hi".to_string()
}
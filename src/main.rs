mod utils;
mod ffmpeg;

use crate::utils::enums::{CodecType, EncoderPreset, EncoderProfile, Quality, VideoCodec};
use ffmpeg::builder::build_command;
use utils::parser::{parse_config, Config};

fn main() {
    let config: Config;
    let cfg_err = parse_config("/home/salt/.config/encode-presets/web.toml");

    match cfg_err {
        Ok(cfg) => config = cfg,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }

    println!("{:#?}", config);

    let command = build_command(&config);

}

use crate::utils::enums::{EncoderPreset, EncoderProfile, EncoderTune, Quality, VideoCodec};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VideoDefault {
    pub preset: EncoderPreset,
    pub profile: EncoderProfile,
    pub tune: EncoderTune,
    pub crf: u8,
    pub frame_rate: i8,
    pub keyint: u8,
    pub custom_args: Vec<String>,
}

pub fn from_preset(codec: VideoCodec, preset: Quality) -> VideoDefault {
    match preset {
        Quality::LOW => get_low_preset(codec),
        Quality::MEDIUM => get_medium_preset(codec),
        Quality::HIGH => get_high_preset(codec),
        Quality::ULTRA => get_ultra_preset(codec)
    }
}

fn get_low_preset(codec: VideoCodec) -> VideoDefault {
    match codec {
        VideoCodec::H264 => VideoDefault {
            preset: EncoderPreset::VERYFAST,
            profile: EncoderProfile::BASIC,
            tune: EncoderTune::FASTDECODE,
            crf: 28,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x264-params".to_string(), "aq-strength=0.8:psy-rd=0.4:scenecut=40".to_string()],
        },
        VideoCodec::H265 => VideoDefault {
            preset: EncoderPreset::VERYFAST,
            profile: EncoderProfile::BASIC,
            tune: EncoderTune::FASTDECODE,
            crf: 32,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x265-params".to_string(), "aq-mode=1:ctu=64:psy-rd=0.5".to_string()],
        },
        VideoCodec::VP9 => VideoDefault {
            preset: EncoderPreset::VERYFAST,
            profile: EncoderProfile::BASIC,
            tune: EncoderTune::SSIM,
            crf: 40,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-tile-columns".to_string(), "2".to_string(), "-row-mt".to_string(), "1".to_string()],
        },
        VideoCodec::AV1 => VideoDefault {
            preset: EncoderPreset::VERYFAST,
            profile: EncoderProfile::BASIC,
            tune: EncoderTune::SSIM,
            crf: 40,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-svtav1-params".to_string(), "enable-dlf=0:tile-columns=4:tile-rows=3".to_string()],
        }
    }
}

fn get_medium_preset(codec: VideoCodec) -> VideoDefault {
    match codec {
        VideoCodec::H264 => VideoDefault {
            preset: EncoderPreset::MEDIUM,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 23,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x264-params".to_string(), "aq-strength=1.0:psy-rd=1.0:scenecut=40".to_string()],
        },
        VideoCodec::H265 => VideoDefault {
            preset: EncoderPreset::MEDIUM,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 28,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x265-params".to_string(), "aq-mode=2:ctu=32:psy-rd=1.0".to_string()],
        },
        VideoCodec::VP9 => VideoDefault {
            preset: EncoderPreset::MEDIUM,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 35,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-tile-columns".to_string(), "1".to_string(), "-row-mt".to_string(), "1".to_string()],
        },
        VideoCodec::AV1 => VideoDefault {
            preset: EncoderPreset::MEDIUM,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 35,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-svtav1-params".to_string(), "enable-dlf=0:tile-columns=2:tile-rows=1".to_string()],
        }
    }
}

fn get_high_preset(codec: VideoCodec) -> VideoDefault {
    match codec {
        VideoCodec::H264 => VideoDefault {
            preset: EncoderPreset::SLOW,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 18,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x264-params".to_string(), "aq-strength=1.2:psy-rd=1.3:scenecut=40".to_string()],
        },
        VideoCodec::H265 => VideoDefault {
            preset: EncoderPreset::SLOW,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 23,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x265-params".to_string(), "aq-mode=3:ctu=32:psy-rd=2.0".to_string()],
        },
        VideoCodec::VP9 => VideoDefault {
            preset: EncoderPreset::SLOW,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 30,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-tile-columns".to_string(), "0".to_string(), "-row-mt".to_string(), "1".to_string(), "-lag-in-frames".to_string(), "25".to_string()],
        },
        VideoCodec::AV1 => VideoDefault {
            preset: EncoderPreset::SLOW,
            profile: EncoderProfile::MAIN,
            tune: EncoderTune::SSIM,
            crf: 30,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-svtav1-params".to_string(), "enable-dlf=1:tile-columns=0:tile-rows=0:enable-qm=1:qm-min=0:qm-max=6".to_string()],
        }
    }
}

fn get_ultra_preset(codec: VideoCodec) -> VideoDefault {
    match codec {
        VideoCodec::H264 => VideoDefault {
            preset: EncoderPreset::VERYSLOW,
            profile: EncoderProfile::HIGH,
            tune: EncoderTune::SSIM,
            crf: 15,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x264-params".to_string(), "aq-strength=1.2:psy-rd=1.3:scenecut=40".to_string()],
        },
        VideoCodec::H265 => VideoDefault {
            preset: EncoderPreset::VERYSLOW,
            profile: EncoderProfile::HIGH,
            tune: EncoderTune::SSIM,
            crf: 20,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-x265-params".to_string(), "aq-mode=3:ctu=32:psy-rd=2.0".to_string()],
        },
        VideoCodec::VP9 => VideoDefault {
            preset: EncoderPreset::VERYSLOW,
            profile: EncoderProfile::HIGH,
            tune: EncoderTune::SSIM,
            crf: 25,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-tile-columns".to_string(), "0".to_string(), "-row-mt".to_string(), "1".to_string(), "-lag-in-frames".to_string(), "25".to_string()],
        },
        VideoCodec::AV1 => VideoDefault {
            preset: EncoderPreset::VERYSLOW,
            profile: EncoderProfile::HIGH,
            tune: EncoderTune::SSIM,
            crf: 25,
            frame_rate: -1,
            keyint: 5,
            custom_args: vec!["-svtav1-params".to_string(), "enable-dlf=1:tile-columns=0:tile-rows=0:enable-qm=1:qm-min=0:qm-max=6".to_string()],
        }
    }
}
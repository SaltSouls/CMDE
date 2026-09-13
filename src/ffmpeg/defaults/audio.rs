use crate::utils::enums::AudioCodec::*;
use crate::utils::enums::Quality::*;
use crate::utils::enums::Rate::*;
use crate::utils::enums::{AudioCodec, Quality, Rate};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AudioDefault {
    pub bitrate: Option<Rate>,
    pub sample_rate: Rate,
    pub channels: u8,
    pub vbr: Option<u8>,
    pub compression: Option<u8>,
    pub custom_args: Vec<String>
}

pub fn from_preset(codec: AudioCodec, preset: Quality) -> AudioDefault {
    match preset {
        LOW => get_low_preset(codec),
        MEDIUM => get_medium_preset(codec),
        HIGH => get_high_preset(codec),
        ULTRA => get_ultra_preset(codec)
    }
}

fn get_low_preset(codec: AudioCodec) -> AudioDefault {
    match codec {
        MP3 => AudioDefault {
            bitrate: Some(STRING("96k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 1,
            vbr: None,
            compression: None,
            custom_args: vec!["-joint_stereo".to_string(), "0".to_string()],
        },
        AAC => AudioDefault {
            bitrate: Some(STRING("64k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 1,
            vbr: None,
            compression: None,
            custom_args: vec!["-aac_coder".to_string(), "fast".to_string()],
        },
        EAC3 => AudioDefault {
            bitrate: Some(STRING("96k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 1,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        OPUS => AudioDefault {
            bitrate: Some(STRING("64k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 1,
            vbr: None,
            compression: None,
            custom_args: vec!["-vbr".to_string(), "on".to_string()],
        },
        VORBIS => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 1,
            vbr: Some(2),
            compression: None,
            custom_args: vec![],
        },
        PCM16 | PCM24 | PCM32 => AudioDefault {
            bitrate: None,
            sample_rate: INTEGER(22050),
            channels: 3,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        FLAC => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 1,
            vbr: None,
            compression: Some(2),
            custom_args: vec![],
        }
    }
}

fn get_medium_preset(codec: AudioCodec) -> AudioDefault {
    match codec {
        MP3 | AAC => AudioDefault {
            bitrate: Some(STRING("128k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        EAC3 => AudioDefault {
            bitrate: Some(STRING("192k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        OPUS => AudioDefault {
            bitrate: Some(STRING("96k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec!["-vbr".to_string(), "on".to_string()],
        },
        VORBIS => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: Some(4),
            compression: None,
            custom_args: vec![],
        },
        PCM16 | PCM24 | PCM32 => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        FLAC => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: Some(5),
            custom_args: vec![],
        }
    }
}

fn get_high_preset(codec: AudioCodec) -> AudioDefault {
    match codec {
        MP3 | AAC => AudioDefault {
            bitrate: Some(STRING("192k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        EAC3 => AudioDefault {
            bitrate: Some(STRING("384k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 6,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        OPUS => AudioDefault {
            bitrate: Some(STRING("128k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec!["-vbr".to_string(), "on".to_string()],
        },
        VORBIS => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: Some(6),
            compression: None,
            custom_args: vec![],
        },
        PCM16 | PCM24 | PCM32 => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        FLAC => AudioDefault {
            bitrate: None,
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: Some(8),
            custom_args: vec![],
        }
    }
}

fn get_ultra_preset(codec: AudioCodec) -> AudioDefault {
    match codec {
        MP3 => AudioDefault {
            bitrate: Some(STRING("320k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        AAC => AudioDefault {
            bitrate: Some(STRING("320k".to_string())),
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec!["-aac_coder".to_string(), "twoloop".to_string()],
        },
        EAC3 => AudioDefault {
            bitrate: Some(STRING("640k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 6,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        OPUS => AudioDefault {
            bitrate: Some(STRING("192k".to_string())),
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec!["-vbr".to_string(), "on".to_string()],
        },
        VORBIS => AudioDefault {
            bitrate: None,
            sample_rate: STRING("44.1k".to_string()),
            channels: 2,
            vbr: Some(8),
            compression: None,
            custom_args: vec![],
        },
        PCM16 | PCM24 | PCM32 => AudioDefault {
            bitrate: None,
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: None,
            custom_args: vec![],
        },
        FLAC => AudioDefault {
            bitrate: None,
            sample_rate: STRING("48k".to_string()),
            channels: 2,
            vbr: None,
            compression: Some(12),
            custom_args: vec![],
        }
    }
}
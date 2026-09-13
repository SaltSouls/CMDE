use crate::ffmpeg::probe::{AudioStream, SubtitleStream, VideoStream};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/*====================================[ DO NOT TOUCH! ]======================================

        Preset Mappings:
    -------------------------
    | AV1 | VP9 |   H26X    |
    |-----|-----|-----------|              Profile Mappings (based on bit depth):
    | 13  |  8  | ultrafast |       -----------------------------------------------------
    | 11  |  7  | superfast |       |      |    Basic    |    Main     |      High      |
    | 10  |  6  | veryfast  |       |------|-------------|-------------|----------------|
    | 8   |  5  | faster    |       | H264 |  baseline   |    main     |  high/high10   |
    | 7   |  4  | fast      |       | H265 | main/main10 | main/main10 |  main444-8/10  |
    | 6   |  3  | medium    |       | VP9  |     0/1     |     0/1     |       2        |
    | 4   |  2  | slow      |       | AV1  |      0      |      0      |      0/1       |
    | 3   |  1  | slower    |       -----------------------------------------------------
    | 2   |  0  | veryslow  |
    | 1   |     | placebo   |
    -------------------------

======================================[ DO NOT TOUCH! ]====================================*/

// video specific enums
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1
}

// get actual lib name from enum
impl Display for VideoCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::H264 => write!(f, "libx264"),
            Self::H265 => write!(f, "libx265"),
            Self::VP9 => write!(f, "libvpx-vp9"),
            Self::AV1 => write!(f, "libsvtav1")
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncoderPreset {
    PLACEBO,
    VERYSLOW,
    SLOWER,
    SLOW,
    MEDIUM,
    FAST,
    FASTER,
    VERYFAST,
    SUPERFAST,
    ULTRAFAST
}

// get preset value based on codec type
impl EncoderPreset {

    /*==========================================[ DO NOT TOUCH! ]==============================================

                                                Preset Mappings:
     ---------------------------------------------------------------------------------------------------------
     |      | PLACEBO | VERYSLOW | SLOWER | SLOW | MEDIUM | FAST | FASTER | VERYFAST | SUPERFAST | ULTRAFAST |
     |------|---------|----------|--------|------|--------|------|--------|----------|-----------|-----------|
     | H26X | placebo | veryslow | slower | slow | medium | fast | faster | veryfast | superfast | ultrafast |
     | VP9  |    0    |    0     |   1    |  2   |   3    |  4   |   5    |    6     |     7     |     8     |
     | AV1  |    1    |    2     |   3    |  4   |   6    |  7   |   8    |    10    |     11    |     13    |
     ---------------------------------------------------------------------------------------------------------

    ============================================[ DO NOT TOUCH! ]============================================*/

    pub fn map_to_string(&self, codec: VideoCodec) -> String {
        match codec {
            VideoCodec::H264|VideoCodec::H265 => match self {
                Self::PLACEBO => "placebo",
                Self::VERYSLOW => "veryslow",
                Self::SLOWER => "slower",
                Self::SLOW => "slow",
                Self::MEDIUM => "medium",
                Self::FAST => "fast",
                Self::FASTER => "faster",
                Self::VERYFAST => "veryfast",
                Self::SUPERFAST => "superfast",
                Self::ULTRAFAST => "ultrafast"
            }.to_string(),
            VideoCodec::VP9 => match self {
                Self::PLACEBO|Self::VERYSLOW => "0",
                Self::SLOWER => "1",
                Self::SLOW => "2",
                Self::MEDIUM => "3",
                Self::FAST => "4",
                Self::FASTER => "5",
                Self::VERYFAST => "6",
                Self::SUPERFAST => "7",
                Self::ULTRAFAST => "8"
            }.to_string(),
            VideoCodec::AV1 => match self {
                Self::PLACEBO => "1",
                Self::VERYSLOW => "2",
                Self::SLOWER =>  "3",
                Self::SLOW => "4",
                Self::MEDIUM => "6",
                Self::FAST => "7",
                Self::FASTER => "8",
                Self::VERYFAST => "10",
                Self::SUPERFAST => "11",
                Self::ULTRAFAST => "13"
            }.to_string()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncoderProfile {
    BASIC,
    MAIN,
    HIGH
}

// get encoder profile value based on codec type and bit depth
impl EncoderProfile {
    pub fn map_to_string(&self, codec: VideoCodec, bit_depth: u8) -> String {

        /*===================[ DO NOT TOUCH! ]===================

                 Profile Mappings (based on bit depth):
           ---------------------------------------------------
           |      |    BASIC    |    MAIN     |     HIGH     |
           |------|-------------|-------------|--------------|
           | H264 |  baseline   |    main     | high/high10  |
           | H265 | main/main10 | main/main10 | main444-8/10 |
           | VP9  |     0/1     |     0/1     |      2       |
           | AV1  |      0      |      0      |     0/1      |
           ---------------------------------------------------

        =====================[ DO NOT TOUCH! ]=================*/

        // ensure proper handling of unsupported bitrates
        if bit_depth != 10 && bit_depth != 8 {
            eprintln!("Unsupported bit depth: {}", bit_depth);
            eprintln!("Allowed bit depth values: [ 8 | 10 ]");
            eprintln!("Defaulting bit depth to 8.");
        }

        // match codecs and check bit depth where applicable
        match codec {
            VideoCodec::H264 => match self {
                Self::BASIC => "baseline",
                Self::MAIN => "main",
                Self::HIGH => match bit_depth { 10 => "high10", _ => "high" }
            }.to_string(),
            VideoCodec::H265 => match self {
                Self::BASIC|Self::MAIN => match bit_depth { 10 => "main10", _ => "main" },
                Self::HIGH => match bit_depth { 10 => "main444-10", _ => "main444-8" }
            }.to_string(),
            VideoCodec::VP9 => match self {
                Self::BASIC|Self::MAIN => match bit_depth { 10 => "1", _ => "0" },
                Self::HIGH => "2"
            }.to_string(),
            VideoCodec::AV1 => match self {
                Self::BASIC|Self::MAIN => "0",
                Self::HIGH => match bit_depth { 10 => "1", _ => "0" }
            }.to_string()
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncoderTune {
    PSNR,
    SSIM,
    FILM,
    ANIMATION,
    GRAIN,
    STILLIMAGE,
    FASTDECODE,
    ZEROLATENCY
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilterPreset {
    NONE,
    LOW,
    MEDIUM,
    HIGH
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeintMode {
    FRAME,
    FIELD
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeintOrder {
    AUTO,
    TOP,
    BOTTOM
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeintFrames {
    ALL,
    INTERLACED
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DeblockMode {
    WEAK,
    STRONG
}

// audio specific enums
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AudioCodec {
    MP3,
    AAC,
    EAC3,
    OPUS,
    VORBIS,
    PCM16,
    PCM24,
    PCM32,
    FLAC
}

// get actual lib name from enum
impl Display for AudioCodec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::MP3 => write!(f, "libmp3lame"),
            Self::AAC => write!(f, "aac"),
            Self::EAC3 => write!(f, "eac3"),
            Self::OPUS => write!(f, "libopus"),
            Self::VORBIS => write!(f, "libvorbis"),
            Self::PCM16 => write!(f, "pcm_s16le"),
            Self::PCM24 => write!(f, "pcm_s24le"),
            Self::PCM32 => write!(f, "pcm_s32le"),
            Self::FLAC => write!(f, "flac")
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum Rate {
    INTEGER(u32),
    STRING(String)
}

// convert bitrate/sample rate to a usable string value
impl Rate {
    fn to_string(&self) -> String {
        match self {
            Rate::INTEGER(bps) => bps.to_string(),
            Rate::STRING(s) => s.clone()
        }
    }
}

// general enums
#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    LOW,
    MEDIUM,
    HIGH,
    ULTRA
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CodecType {
    VIDEO(VideoCodec),
    AUDIO(AudioCodec)
}

// impl CodecType {
//     pub fn get_codec(&self) -> Result<AudioCodec, VideoCodec> {
//         match self {
//             VIDEO => match VIDEO {  }
//         }
//     }
// }

#[derive(Deserialize, Serialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    AUDIO,
    SUBS,
    FONT
}

// enums representing/storing different stream types
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase", tag = "codec_type", content = "content")]
pub enum Streams {
    VIDEO(VideoStream),
    AUDIO(AudioStream),
    SUBTITLE(SubtitleStream)
}
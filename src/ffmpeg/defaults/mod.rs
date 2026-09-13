pub(crate) mod audio;
pub(crate) mod video;
use crate::ffmpeg::defaults::audio::{AudioDefault};
use crate::ffmpeg::defaults::video::{VideoDefault};
use crate::utils::enums::{CodecType, Quality};


#[derive(Debug, Clone)]
pub enum DefaultType {
    VIDEO(VideoDefault),
    AUDIO(AudioDefault),
}

pub fn get_defaults(codec: CodecType, quality: Quality) -> DefaultType {
    match codec {
        CodecType::VIDEO(c) => DefaultType::VIDEO(video::from_preset(c, quality)),
        CodecType::AUDIO(c) => DefaultType::AUDIO(audio::from_preset(c, quality)),
    }
}

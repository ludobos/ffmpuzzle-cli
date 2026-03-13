use std::collections::HashMap;
use std::sync::LazyLock;

pub static CODEC_FAMILIES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("H.264", "h26x"),
        ("H.265", "h26x"),
        ("VP9", "vpx-av1"),
        ("AV1", "vpx-av1"),
        ("ProRes", "production"),
        ("MPEG-2", "legacy"),
    ])
});

pub static AUDIO_FAMILIES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("AAC", "lossy-std"),
        ("MP3", "lossy-std"),
        ("AC-3", "surround"),
        ("E-AC-3", "surround"),
        ("Opus", "modern"),
        ("Vorbis", "modern"),
        ("PCM", "lossless"),
    ])
});

pub static CONTAINER_FAMILIES: LazyLock<HashMap<&str, &str>> = LazyLock::new(|| {
    HashMap::from([
        ("MP4", "iso"),
        ("MOV", "iso"),
        ("MKV", "matroska"),
        ("WebM", "matroska"),
        ("TS", "broadcast"),
        ("FLV", "legacy"),
    ])
});

pub const RESOLUTION_ORDER: &[&str] = &["480p", "720p", "1080p", "1440p", "2160p"];

use crate::data::types::{Credit, Difficulty, Param, ParamType, Puzzle};
use chrono::{TimeZone, Utc};

pub const MAX_GUESSES: usize = 6;

pub fn puzzles() -> Vec<Puzzle> {
    vec![
        // Puzzle 1
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "23".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx265 -crf 23 -c:a aac -movflags +faststart output.mp4".into(),
            scenario: "Default OTT delivery profile for a European SVOD platform. H.265 for bandwidth savings on Smart TVs, CRF 23 as the sweet spot quality/size, AAC for universal device compat, MP4 with faststart for progressive download.".into(),
            difficulty: Difficulty::Standard,
            credit: Some(Credit { name: "Jan Ozer".into(), url: Some("https://streaminglearningcenter.com".into()) }),
            fun_fact: "The -movflags +faststart flag moves the moov atom to the beginning of the file, enabling progressive download. Without it, the entire file must download before playback starts.".into(),
        },
        // Puzzle 2
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "30".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "6".into(), options: vec!["4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "10".into(), "12".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libsvtav1 -crf 30 -preset 6 -c:a libopus -b:a 128k output.webm".into(),
            scenario: "YouTube-optimized 4K upload. AV1 for maximum compression at ultra-high resolution, CRF 30 (AV1 scale differs from H.26x), Opus audio for WebM container. Preset 6 balances encode speed vs quality for creator workflows.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "AV1 CRF values are not comparable to H.264/H.265. CRF 30 in AV1 produces roughly similar quality to CRF 23 in H.265, but with 30-50% smaller file sizes at the cost of much longer encode times.".into(),
        },
        // Puzzle 3
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "5M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "E-AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v libx264 -b:v 5M -c:a eac3 -b:a 384k -f mpegts output.ts".into(),
            scenario: "Live broadcast contribution feed. H.264 for real-time encoding, CBR 5M for consistent bandwidth on dedicated links, E-AC-3 for surround sound, MPEG-TS for stream resilience (packet loss recovery). Classic broadcast-to-IP bridge.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "MPEG-TS (Transport Stream) adds ~5% overhead compared to MP4, but each 188-byte packet is self-contained. If a packet is lost, only that packet is affected -- unlike MP4 where corruption can break the entire file.".into(),
        },
        // Puzzle 4
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "31".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libvpx-vp9 -crf 31 -b:v 0 -c:a libopus output.webm".into(),
            scenario: "Web-first delivery for a news site targeting mobile browsers. VP9 for broad browser support without licensing fees, 720p for mobile-first, CRF 31 with -b:v 0 for pure quality mode. Opus + WebM for the open-source stack.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "The -b:v 0 flag is required with VP9 CRF mode. Without it, FFmpeg defaults to a target bitrate of 200kbps, which overrides CRF and produces terrible quality -- a common VP9 gotcha.".into(),
        },
        // Puzzle 5
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "ProRes".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "150M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v prores_ks -profile:v 3 -b:v 150M -c:a pcm_s24le output.mov".into(),
            scenario: "Post-production dailies for a Netflix Original series. ProRes HQ (profile 3) at 150 Mbps for color grading in DaVinci Resolve, PCM audio to preserve full dynamic range, MOV container as the industry standard interchange format.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "ProRes has no CRF mode -- it uses fixed profiles (Proxy, LT, Standard, HQ, 4444, 4444 XQ). Profile 3 (HQ) targets ~220 Mbps for 1080p@24fps but allows bitrate override for specific delivery requirements.".into(),
        },
        // Puzzle 6
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "18".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "slow".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx264 -crf 18 -preset slow -c:a aac -b:a 192k -movflags +faststart output.mp4".into(),
            scenario: "Archival master for a wedding videography studio. H.264 for universal playback, CRF 18 for near-lossless quality that clients can re-edit years later, slow preset for maximum compression efficiency on overnight batch encodes.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "CRF 18 is often called \"visually lossless\" for H.264. Below CRF 17, file sizes balloon with no perceptible quality gain. The slow preset adds ~40% encode time vs medium but saves ~5-10% bitrate at the same quality.".into(),
        },
        // Puzzle 7
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "15M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v mpeg2video -b:v 15M -c:a ac3 -b:a 384k -f mpegts output.ts".into(),
            scenario: "ATSC 1.0 terrestrial broadcast for a US local TV station. MPEG-2 is still mandatory for over-the-air ATSC 1.0 compliance, 15 Mbps for HD, AC-3 (Dolby Digital) as the required audio codec, MPEG-TS for broadcast transport.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "ATSC 1.0 was standardized in 1995 and still mandates MPEG-2 video + AC-3 audio. Despite ATSC 3.0 supporting HEVC, most US stations still transmit ATSC 1.0 because receiver upgrades cost billions across 120M+ households.".into(),
        },
        // Puzzle 8
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "3M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "FLV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "30".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 3M -r 30 -c:a aac -b:a 128k -f flv rtmp://live.twitch.tv/app/STREAM_KEY".into(),
            scenario: "Twitch live stream via RTMP. H.264 is the only codec Twitch accepts, 720p30 at 3 Mbps for non-partners (bitrate cap), AAC mono/stereo, FLV container as the RTMP transport format. The bread and butter of game streaming.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "FLV (Flash Video) refuses to die. Despite Flash being deprecated since 2020, RTMP still requires FLV as its container format. Every Twitch, YouTube Live, and Facebook Live stream is wrapped in FLV before reaching the ingest server.".into(),
        },
        // Puzzle 9
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "28".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx265 -crf 28 -tag:v hvc1 -c:a aac -b:a 192k -movflags +faststart output.mp4".into(),
            scenario: "Apple TV app 4K HDR delivery. H.265 is required for 4K on Apple devices, CRF 28 (the H.265 default) for efficient 4K compression, -tag:v hvc1 for Apple compatibility, AAC for universal playback, MP4 container.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Apple devices require the hvc1 tag for H.265 playback. Without -tag:v hvc1, Safari and Apple TV will refuse to play the file even though the codec is identical. The default hev1 tag works everywhere else.".into(),
        },
        // Puzzle 10
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "35".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "8".into(), options: vec!["4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "10".into(), "12".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libsvtav1 -crf 35 -preset 8 -vf scale=-2:480 -c:a libopus -b:a 64k output.webm".into(),
            scenario: "Low-bandwidth video delivery for an educational NGO in rural Sub-Saharan Africa. AV1 for maximum compression at 480p, CRF 35 to aggressively save bandwidth on 2G/3G connections, preset 8 for faster server-side encoding at scale.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "At 480p, AV1 at CRF 35 can deliver watchable video at under 200 kbps -- roughly 90 MB per hour. H.264 at equivalent quality would need 400+ kbps, making AV1 transformative for emerging markets with metered data.".into(),
        },
        // Puzzle 11
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "23".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "MP3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx264 -crf 23 -vf scale=-2:720 -c:a libmp3lame -b:a 192k output.mp4".into(),
            scenario: "Conference recording for a corporate LMS (Learning Management System). H.264 for maximum device compatibility, 720p to keep file sizes manageable, MP3 audio because the legacy LMS only supports MP3 in MP4 containers.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "MP3 in MP4 is technically valid per ISO 14496-3 but rarely used. Some ancient enterprise LMS platforms from the 2010s hardcoded MP3 support and never updated. It works, but it makes video engineers twitch.".into(),
        },
        // Puzzle 12
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "8M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "E-AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "medium".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v libx265 -b:v 8M -preset medium -c:a eac3 -b:a 640k output.mp4".into(),
            scenario: "Premium SVOD title delivery (Dolby Digital Plus) for a platform competing with Disney+. H.265 at 8 Mbps for high-quality 1080p, E-AC-3 at 640 kbps for immersive 5.1 surround, medium preset for production encode farms.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "E-AC-3 (Dolby Digital Plus) supports bitrates up to 6.144 Mbps and up to 15.1 channels. At 640 kbps for 5.1, it significantly outperforms AC-3 (capped at 640 kbps total) in audio quality per channel.".into(),
        },
        // Puzzle 13
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "2M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 2M -maxrate 2M -bufsize 4M -c:a aac -b:a 128k -f mpegts output.ts".into(),
            scenario: "IP security camera recording to a NVR (Network Video Recorder). H.264 for hardware decoder support on cheap NVR chips, CBR 2M for predictable storage planning, MPEG-TS for crash resilience -- if power cuts, only the last packet is lost.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "A single 2 Mbps H.264 stream records ~21 GB per day. A 32-camera NVR at this bitrate fills a 16 TB drive in about 24 days, which is why most surveillance systems use motion-triggered variable bitrate to extend storage 3-5x.".into(),
        },
        // Puzzle 14
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "33".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Vorbis".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libvpx-vp9 -crf 33 -b:v 0 -c:a libvorbis -q:a 5 output.webm".into(),
            scenario: "Wikipedia/Wikimedia Commons video upload. VP9 + Vorbis + WebM is the required format for Wikimedia. CRF 33 for reasonable file sizes on the free encyclopedia, Vorbis because Wikimedia standardized on it before Opus was widespread.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Wikimedia Commons still requires Vorbis audio (not Opus) in WebM files as of 2025. This is because their MediaWiki player integration was built around Vorbis, and migrating millions of existing files is a massive undertaking.".into(),
        },
        // Puzzle 15
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "6M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "FLV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 6M -g 60 -c:a aac -b:a 160k -f flv rtmp://a.rtmp.youtube.com/live2/STREAM_KEY".into(),
            scenario: "YouTube Live stream for a music festival. H.264 at 6 Mbps for YouTube recommended 1080p live settings, keyframe interval of 2s (GOP 60 at 30fps), AAC audio, FLV container for RTMP ingest to YouTube Live.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "YouTube requires a keyframe interval of exactly 2 seconds for live streams. If your GOP size does not match, YouTube will re-encode your stream server-side, adding latency and potentially degrading quality.".into(),
        },
        // Puzzle 16
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "ProRes".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "300M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.r3d -c:v prores_ks -profile:v 4 -b:v 300M -c:a pcm_s24le output.mov".into(),
            scenario: "DCI 4K cinema dailies from a RED camera for an indie film. ProRes 4444 (profile 4) at 300 Mbps preserves the wide color gamut and alpha channel, PCM 24-bit audio for sound design, MOV as the Final Cut Pro / Resolve interchange format.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "ProRes 4444 (profile 4) supports an alpha channel, making it essential for VFX compositing. ProRes 4444 XQ (profile 5) doubles the data rate for even higher quality but only Apple silicon and recent Intel Macs can decode it in real time.".into(),
        },
        // Puzzle 17
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "20".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "60".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mkv -c:v libx264 -crf 20 -r 60 -c:a aac -b:a 192k -movflags +faststart output.mp4".into(),
            scenario: "Cloud gaming clip capture (like NVIDIA ShadowPlay). H.264 for instant hardware decoding, CRF 20 for high quality game visuals, 60fps to preserve smooth gameplay, AAC/MP4 for instant sharing to Discord or social media.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "At 60fps, H.264 can use 2x the bitrate of 30fps content at the same CRF because there are twice as many frames. However, inter-frame compression means the actual file size increase is typically only 30-50%, not 100%.".into(),
        },
        // Puzzle 18
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "8M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "MP3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v mpeg2video -b:v 8M -c:a mp2 -b:a 256k -f mpegts output.ts".into(),
            scenario: "DVB-T digital terrestrial broadcast for a European country. MPEG-2 video at 8 Mbps for 720p (standard DVB-T bandwidth allocation), MP2 audio as specified by DVB standards, MPEG-TS transport. Yes, MP2 -- not MP3.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "DVB-T in Europe uses MPEG-1 Layer II (MP2) audio, not MP3. MP2 was chosen for its lower decoder complexity and better error resilience over broadcast channels. Despite being \"older,\" MP2 outperforms MP3 at bitrates above 192 kbps.".into(),
        },
        // Puzzle 19
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "26".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1440p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -crf 26 -vf scale=-2:1440 -c:a libopus -b:a 128k output.mkv".into(),
            scenario: "Personal Plex media server encoding for a movie collection. H.265 for storage efficiency, 1440p for high-quality on desktop monitors, CRF 26 as a good quality/size balance, Opus for modern audio, MKV for subtitle and chapter support.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "MKV (Matroska) can contain virtually any codec combination and supports unlimited subtitle tracks, chapters, and attachments (like fonts). This flexibility is why it is the preferred format for media servers like Plex and Jellyfin.".into(),
        },
        // Puzzle 20
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "28".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "ultrafast".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -crf 28 -preset ultrafast -vf scale=-2:480 -c:a aac -b:a 64k output.mp4".into(),
            scenario: "Real-time screen recording for a bug report tool (like Loom). H.264 for instant playback, ultrafast preset because encoding happens live while the user records, 480p and CRF 28 to keep CPU usage under 10% on a laptop.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "The ultrafast preset disables most H.264 compression features: no B-frames, no CABAC (uses CAVLC), simplified motion estimation. File sizes are ~2-3x larger than medium, but encoding uses ~10x less CPU -- critical for live capture.".into(),
        },
        // Puzzle 21
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "25".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "4".into(), options: vec!["4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "10".into(), "12".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libsvtav1 -crf 25 -preset 4 -c:a libopus -b:a 192k output.mp4".into(),
            scenario: "Netflix-grade AV1 encoding for a premium SVOD catalog title. CRF 25 for near-transparent quality, preset 4 for maximum compression (overnight encode farm), Opus audio, MP4 container for CMAF/DASH delivery.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Netflix was one of the first to deploy AV1 at scale, starting with Android in 2020. Their AV1 encodes at preset 4 can take 100x longer than real-time but save 30-50% bandwidth vs H.265, translating to millions in CDN cost savings.".into(),
        },
        // Puzzle 22
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "50M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 50M -c:a pcm_s16le output.mov".into(),
            scenario: "Drone footage ingest for a nature documentary. H.264 at 50 Mbps (matching DJI Mavic 3 output bitrate) for editing in Premiere Pro, PCM audio to preserve the raw field recording, MOV for NLE compatibility.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "DJI drones record H.264 at up to 200 Mbps internally. When re-encoding at 50 Mbps for proxy editing, the 4x compression ratio makes timeline scrubbing buttery smooth while keeping enough quality for the final grade.".into(),
        },
        // Puzzle 23
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "20M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "59.94".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v libx265 -b:v 20M -r 59.94 -c:a ac3 -b:a 448k -f mpegts output.ts".into(),
            scenario: "Live 4K sports broadcast for a US cable network (ATSC 3.0 compliant). H.265 at 20 Mbps for 4K, 59.94 fps for NTSC-compatible high frame rate, AC-3 5.1 surround, MPEG-TS for broadcast transport.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "59.94 fps (not 60) exists because of NTSC color compatibility from 1953. The original monochrome 60 Hz was shifted by 0.1% to avoid interference between the color subcarrier and audio carrier. We are still living with this 70-year-old hack.".into(),
        },
        // Puzzle 24
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "25".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -crf 25 -vf \"scale=1920:1080,setsar=1\" -c:a aac -b:a 128k -movflags +faststart output.mp4".into(),
            scenario: "Instagram Reels re-encode from a vertical 9:16 source. H.264 is the only codec Instagram accepts, CRF 25 because Instagram will re-encode anyway (no point over-optimizing), AAC/MP4 per Instagram upload requirements.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Instagram re-encodes every uploaded video regardless of input quality. Uploading at CRF 18 vs CRF 25 produces identical results on the platform but wastes 3-5x the upload bandwidth. The sweet spot is \"good enough\" quality.".into(),
        },
        // Puzzle 25
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "22".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "slower".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mkv -c:v libx265 -crf 22 -preset slower -c:a aac -b:a 128k output.mkv".into(),
            scenario: "Anime archival for a private collection. H.265 with CRF 22 and slower preset to preserve cel animation detail (flat colors and sharp lines compress exceptionally well with HEVC), MKV for multiple subtitle tracks (Japanese + English).".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Anime compresses 40-60% better than live action at the same CRF because flat-shaded cel animation has lower spatial complexity. An H.265 CRF 22 anime episode at 1080p typically lands around 300-500 MB vs 1-2 GB for live action.".into(),
        },
        // Puzzle 26
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "28".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libvpx-vp9 -crf 28 -b:v 0 -tile-columns 2 -threads 8 -c:a libopus -b:a 160k output.webm".into(),
            scenario: "Google Stadia-era cloud gaming VOD archive. VP9 at 4K because Google mandated VP9 for all Stadia content, CRF 28 for high-quality game footage, WebM for the all-Google stack. A monument to a killed product.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Google Stadia used VP9 for all streaming and required game developers to target VP9-friendly visuals. When Stadia shut down in 2023, it was the largest real-world deployment of VP9 for interactive content, processing petabytes of game video.".into(),
        },
        // Puzzle 27
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "1M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 1M -vf scale=-2:480 -c:a aac -b:a 64k -f mpegts output.ts".into(),
            scenario: "Digital signage for a fast-food menu board over coax. H.264 at 480p/1 Mbps because the BrightSign player has a 2 Mbps total bandwidth limit, MPEG-TS for the multicast UDP delivery, AAC for the background music track.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Digital signage is a surprisingly large market ($30B+). Most menu boards run BrightSign or similar ARM-based players that only decode H.264 Baseline profile. Sending H.265 to these devices results in a black screen with no error message.".into(),
        },
        // Puzzle 28
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "20".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "E-AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mkv -c:v libx265 -crf 20 -c:a eac3 -b:a 768k output.mkv".into(),
            scenario: "4K Blu-ray rip for home theater with Dolby Atmos-capable receiver. H.265 at CRF 20 for near-reference quality, E-AC-3 for Dolby Digital Plus surround (Atmos metadata passthrough), MKV for chapter markers and multiple audio tracks.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Dolby Atmos is delivered as spatial metadata embedded in an E-AC-3 (or TrueHD) stream. When you re-encode the audio to AAC, all Atmos metadata is permanently lost -- the 3D soundstage collapses back to flat 5.1/7.1.".into(),
        },
        // Puzzle 29
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "50M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "25".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.avi -c:v mpeg2video -b:v 50M -r 25 -c:a pcm_s16le output.mkv".into(),
            scenario: "Digitizing a PAL Betacam SP tape archive for a national broadcasting museum. MPEG-2 at 50 Mbps to match the original SD-upscaled quality, 25fps PAL, PCM audio for lossless preservation, MKV for metadata and chapter support.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Betacam SP tapes degrade about 10-20% per decade even in climate-controlled storage. The \"magnetic cliff\" means tapes from the 1990s are hitting critical preservation deadlines now -- once oxide shed begins, the content is unrecoverable.".into(),
        },
        // Puzzle 30
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "4M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 4M -tune zerolatency -c:a libopus -b:a 48k output.mkv".into(),
            scenario: "Zoom-style video conference recording. H.264 with -tune zerolatency for real-time encoding, 4 Mbps for crisp screen-share content, Opus for voice-optimized low-latency audio, MKV because it handles unfinished writes gracefully.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "The -tune zerolatency flag disables B-frames, reduces lookahead to 0, and uses a single-frame VBV buffer. This makes encoding instant but reduces compression efficiency by 20-30%. It is essential for real-time conferencing.".into(),
        },
        // Puzzle 31
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "32".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libsvtav1 -crf 32 -c:a aac -b:a 128k -movflags +faststart output.mp4".into(),
            scenario: "TikTok creator batch re-encoding old vertical videos for the TikTok AV1 beta program. AV1 for maximum quality at low bitrate, CRF 32 because TikTok caps uploads at 287 Mbps but re-encodes everything, AAC/MP4 for compatibility.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "TikTok began supporting AV1 playback in 2023. Because TikTok videos are short (15-180s) and viewed billions of times, even a 20% bitrate reduction from AV1 saves TikTok an estimated $100M+ per year in CDN costs.".into(),
        },
        // Puzzle 32
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "40M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "bitrateMode".into(), label: "Bitrate Mode".into(), param_type: ParamType::Dropdown, answer: "VBR".into(), options: vec!["CBR".into(), "VBR".into(), "CQ".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx265 -b:v 40M -maxrate 60M -bufsize 80M -c:a aac -b:a 256k -movflags +faststart output.mp4".into(),
            scenario: "Samsung TV Plus FAST (Free Ad-Supported Streaming TV) channel delivery. H.265 at 4K/40 Mbps VBR with 60M peak for action scenes, AAC for broad device support, MP4 for DASH/HLS segmentation downstream.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "FAST channels grew from $2B to $12B revenue between 2020-2025. Unlike SVOD, FAST content must be encoded at consistently high bitrates because ad insertion points need to match the main content quality -- jarring drops lose viewers.".into(),
        },
        // Puzzle 33
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "22".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "29.97".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v libx264 -crf 22 -r 29.97 -c:a aac -b:a 192k -movflags +faststart output.mp4".into(),
            scenario: "Medical training video for a hospital e-learning platform. H.264 Baseline profile at 29.97fps (NTSC standard from the surgical camera), CRF 22 to preserve clinical detail, AAC/MP4 for browser-based LMS playback on hospital tablets.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "29.97 fps is technically 30000/1001 fps. This NTSC legacy frame rate is still the default for most US medical and broadcast equipment. Mixing 29.97 and 30fps footage causes audio drift of ~1 frame per 33 seconds.".into(),
        },
        // Puzzle 34
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "36".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.gif -c:v libvpx-vp9 -crf 36 -b:v 0 -vf scale=-2:480 -an output.webm".into(),
            scenario: "Converting GIF memes to WebM for a Discord bot. VP9 at 480p with aggressive CRF 36 because memes tolerate quality loss, Opus placeholder (though most will be silent), WebM for Discord embed auto-play support.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "A 5-second GIF can be 10-50 MB. The same clip as VP9 WebM at CRF 36 is typically 100-500 KB -- a 100x reduction. GIF uses lossless per-frame compression with no inter-frame prediction, making it spectacularly inefficient for video.".into(),
        },
        // Puzzle 35
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "10M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "50".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v libx265 -b:v 10M -r 50 -c:a ac3 -b:a 384k -f mpegts output.ts".into(),
            scenario: "Premier League live match feed for a European IPTV operator. H.265 at 10 Mbps for 1080p50 (PAL high frame rate for smooth ball tracking), AC-3 5.1 surround for stadium atmosphere, MPEG-TS for multicast IPTV delivery.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Sports requires 2-3x the bitrate of film at the same resolution because fast motion (ball, players, crowd) defeats inter-frame prediction. A football match at 1080p50 needs 10-15 Mbps in H.265, while a drama needs only 4-6 Mbps.".into(),
        },
        // Puzzle 36
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "ProRes".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "45M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v prores_ks -profile:v 0 -b:v 45M -c:a pcm_s16le output.mov".into(),
            scenario: "ProRes Proxy generation for an offline edit in Final Cut Pro. Profile 0 (Proxy) at 45 Mbps for fast timeline scrubbing on a MacBook Air, PCM audio, MOV container. The editor will relink to original camera files for the final export.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "ProRes Proxy (profile 0) was designed to be exactly 1/3 the data rate of ProRes 422. Apple optimized the decode path so heavily that a 2020 MacBook Air can play 16 streams simultaneously -- making multicam editing viable on a laptop.".into(),
        },
        // Puzzle 37
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "8M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "veryfast".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 8M -preset veryfast -tune stillimage -c:a aac -b:a 128k output.mp4".into(),
            scenario: "Podcast video recording with a static background (talking head + slides). H.264 at 8 Mbps with veryfast preset for real-time encoding, -tune stillimage for slide content optimization, AAC for podcast distribution on Apple Podcasts.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "The -tune stillimage flag optimizes for content with very little motion -- like slides or screencasts. It increases I-frame quality and adjusts the rate control to allocate more bits to scene changes (slide transitions) and fewer to static frames.".into(),
        },
        // Puzzle 38
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "28".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mkv -c:v libsvtav1 -crf 28 -c:a libopus -b:a 96k output.webm".into(),
            scenario: "Web tutorial platform (like freeCodeCamp) delivering screen recordings to developing nations. AV1 for bandwidth savings over VP9, 720p for readable code on small screens, CRF 28 for sharp text, Opus/WebM for browser-native playback.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Screen recordings with code/text compress dramatically better than natural video. AV1 screen content coding tools can achieve 5-10x better compression for code tutorials compared to the same codec encoding a nature documentary.".into(),
        },
        // Puzzle 39
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "25M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "bitrateMode".into(), label: "Bitrate Mode".into(), param_type: ParamType::Dropdown, answer: "CBR".into(), options: vec!["CBR".into(), "VBR".into(), "CQ".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v mpeg2video -b:v 25M -minrate 25M -maxrate 25M -bufsize 25M -c:a ac3 -b:a 384k -f mpegts output.ts".into(),
            scenario: "Satellite uplink for live news at a regional US TV station. MPEG-2 at strict CBR 25 Mbps because the satellite transponder has a fixed bandwidth allocation -- any burst above the cap causes packet loss. AC-3 for ATSC compliance.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Satellite transponders are essentially \"dumb pipes\" with a fixed bandwidth. Unlike internet streaming where VBR saves money, satellite CBR wastes capacity during low-motion scenes but prevents catastrophic loss during high-motion peaks.".into(),
        },
        // Puzzle 40
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "26".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -crf 26 -vf scale=-2:720 -c:a aac -b:a 96k -movflags +faststart output.mp4".into(),
            scenario: "WhatsApp status video optimization. H.264 because WhatsApp rejects other codecs, 720p to stay under the 16 MB upload limit, CRF 26 for aggressive compression, AAC/MP4 per WhatsApp media spec. Every byte counts.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "WhatsApp compresses all shared videos to a maximum of 16 MB (raised from 64 MB in some regions). Pre-compressing to exactly 720p H.264 before sharing prevents WhatsApp from re-encoding and destroying your quality further.".into(),
        },
        // Puzzle 41
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "30M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "24".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libx265 -b:v 30M -r 24 -c:a aac -b:a 256k -movflags +faststart output.mp4".into(),
            scenario: "Music video delivery for a major label on Apple Music. H.265 at 4K/24fps (cinematic look), 30 Mbps for Apple's 4K HDR spec, AAC for Apple ecosystem compatibility, MP4 container. Destined for the Apple TV screensaver rotation.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Apple Music video specifications require H.265 for 4K content at exactly 24fps with -tag:v hvc1. Submitting at 23.976fps (the actual cinema standard) is rejected. Apple rounds up, and the 0.1% speed difference is inaudible.".into(),
        },
        // Puzzle 42
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "10M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.dcm -c:v libx264 -b:v 10M -pix_fmt yuv420p -c:a pcm_s16le output.mkv".into(),
            scenario: "Medical imaging video export from a DICOM ultrasound system. H.264 at 10 Mbps for clinical detail in grayscale imaging, PCM audio for the Doppler audio track, MKV to preserve study metadata. Used for remote diagnosis via telemedicine.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "DICOM (Digital Imaging and Communications in Medicine) natively supports MPEG-2, MPEG-4, and H.264 video. Many hospitals still use MPEG-2 DICOM video because FDA-certified devices take years to update codec support.".into(),
        },
        // Puzzle 43
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "24".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1440p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "slow".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -crf 24 -preset slow -vf scale=-2:1440 -c:a libopus -b:a 128k output.mkv".into(),
            scenario: "Steam Deck verified game trailer encoding. H.265 at 1440p (Steam Deck OLED native resolution), CRF 24 slow for high quality at manageable file sizes, Opus for the modern audio stack, MKV for Valve/Linux ecosystem compatibility.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "The Steam Deck OLED has a native resolution of 1280x800 but upscales to 1440p for external display output. Valve recommends H.265 for store trailers because the Deck's AMD APU has a dedicated HEVC hardware decoder.".into(),
        },
        // Puzzle 44
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "15M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 15M -vf \"v360=equirect:cubemap\" -c:a aac -b:a 192k -movflags +faststart output.mp4".into(),
            scenario: "VR 360-degree video for Oculus/Meta Quest headset. H.264 at 4K equirectangular at 15 Mbps (Meta recommends H.264 for Quest 2 compatibility), AAC/MP4 for the Meta platform. The viewer only sees ~90 degrees at a time.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "360-degree video is wasteful by design: the viewer only sees about 25% of the frame at any time, meaning 75% of pixels are decoded but never displayed. This is why Meta and Apple are pushing for viewport-adaptive streaming.".into(),
        },
        // Puzzle 45
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "38".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "10".into(), options: vec!["4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "10".into(), "12".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libsvtav1 -crf 38 -preset 10 -vf scale=-2:720 -c:a aac -b:a 64k output.mp4".into(),
            scenario: "Thumbnail preview video for an e-commerce product page. AV1 at aggressive CRF 38 and fast preset 10 for tiny file sizes (~100KB for 5-second clips), 720p, AAC/MP4. Each product has a hover-to-play preview that must load instantly.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "E-commerce video previews need to be under 200KB to avoid impacting page load time. At CRF 38, AV1 can encode a 5-second 720p product spin video to ~80KB -- small enough to preload alongside the product thumbnail image.".into(),
        },
        // Puzzle 46
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "21".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.iso -c:v libx264 -crf 21 -c:a ac3 -b:a 448k output.mkv".into(),
            scenario: "DVD rip for a personal media library (Handbrake-style). H.264 at CRF 21 for high quality from SD/DVD sources, AC-3 passthrough to preserve the original Dolby Digital 5.1 track, MKV for chapter and subtitle support.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "AC-3 passthrough (stream copy) preserves the original DVD audio bit-for-bit. Re-encoding AC-3 to AAC for \"space savings\" only saves ~100 MB per movie but permanently loses the 5.1 surround mix -- a bad tradeoff for archivists.".into(),
        },
        // Puzzle 47
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "3M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -b:v 3M -x265-params \"keyint=30:min-keyint=30\" -c:a aac -b:a 96k -f mpegts output.ts".into(),
            scenario: "Low-latency HLS for a live worship service stream. H.265 at 3 Mbps for 720p (church WiFi bandwidth constraint), 1-second GOP for low latency, AAC for iPhone compatibility, MPEG-TS for HLS segment compatibility.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Apple Low-Latency HLS requires MPEG-TS segments (not fMP4) for sub-2-second latency. Each segment must start with an IDR frame, so the GOP size directly determines the minimum achievable latency -- 1-second GOP = ~2-second glass-to-glass.".into(),
        },
        // Puzzle 48
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "25".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Vorbis".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libvpx-vp9 -crf 25 -b:v 0 -c:a libvorbis -q:a 6 output.mkv".into(),
            scenario: "Open-source game engine (Godot) cutscene encoding. VP9 + Vorbis because Godot's built-in video player supports WebM/VP9/Vorbis natively, CRF 25 for high-quality in-game cinematics, MKV (Matroska) container for the open-source stack.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Godot Engine chose VP9/Vorbis/WebM as its native video format to avoid all patent licensing. H.264 would require per-unit royalties for commercial games, making the open-source VP9 stack essential for indie developers.".into(),
        },
        // Puzzle 49
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "20M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "59.94".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -b:v 20M -r 59.94 -c:a aac -b:a 320k -movflags +faststart output.mp4".into(),
            scenario: "Concert live recording multi-cam master for a Vevo release. H.264 at high bitrate 20 Mbps for post-production flexibility, 59.94fps for NTSC broadcast-safe delivery, AAC at 320 kbps for music quality, MP4 for YouTube/Vevo ingest.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Vevo processes over 25 billion video views per month and requires H.264 Baseline/Main/High profile submissions. They re-encode everything server-side, but higher input quality (20+ Mbps) gives their encoder more data to work with.".into(),
        },
        // Puzzle 50
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "6M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AC-3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v mpeg2video -b:v 6M -vf scale=-2:480 -c:a ac3 -b:a 192k -f mpegts output.ts".into(),
            scenario: "Standard Definition cable TV channel playout in the US (2026, still running). MPEG-2 at 480i/6 Mbps because millions of legacy set-top boxes cannot decode H.264, AC-3 audio, MPEG-TS. Some cable headends refuse to upgrade.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "As of 2025, approximately 30% of US cable TV channels still transmit in MPEG-2 SD. The cost to replace ~50 million legacy set-top boxes that only decode MPEG-2 exceeds $5 billion, making the \"just upgrade\" argument financially impossible.".into(),
        },
        // Puzzle 51
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "30".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "720p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -crf 30 -vf scale=-2:720 -c:a aac -b:a 64k -movflags +faststart output.mp4".into(),
            scenario: "Surveillance footage export for law enforcement evidence submission. H.265 at 720p CRF 30 -- aggressive compression because evidence storage budgets are tight, AAC for playback on courtroom laptops, MP4 for universal compatibility.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Courts in the US accept H.264 and H.265 video evidence but require a \"chain of custody\" hash. Over-compressing with high CRF can be challenged by defense attorneys who argue that compression artifacts could obscure critical details.".into(),
        },
        // Puzzle 52
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "AV1".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "22".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "5".into(), options: vec!["4".into(), "5".into(), "6".into(), "7".into(), "8".into(), "10".into(), "12".into()], hint: None },
            ],
            command: "ffmpeg -i input.mov -c:v libsvtav1 -crf 22 -preset 5 -c:a libopus -b:a 256k output.mkv".into(),
            scenario: "Nature documentary archival for a national film institute. AV1 at CRF 22 for near-transparent 4K quality, preset 5 for strong compression (preservation budgets prioritize storage savings), Opus for efficient multi-channel audio, MKV for metadata.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Film archives face a paradox: digital preservation requires re-encoding every 10-15 years as storage media degrades. AV1 at CRF 22 can cut storage costs by 50% vs H.265 but the re-encoding compute cost for a million-title archive is substantial.".into(),
        },
        // Puzzle 53
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "35M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.braw -c:v libx264 -b:v 35M -c:a pcm_s24le output.mov".into(),
            scenario: "Blackmagic RAW to H.264 proxy for multicam live event editing in DaVinci Resolve. 35 Mbps for smooth 8-track timeline playback, PCM 24-bit to preserve the board mix audio, MOV for Resolve timeline compatibility.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Blackmagic RAW (BRAW) uses a unique partial-debayer approach where the camera does the computationally expensive part and stores a partially processed raw file. This makes BRAW files 3-5x smaller than Cinema DNG while preserving full raw flexibility.".into(),
        },
        // Puzzle 54
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "25".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "preset".into(), label: "Preset".into(), param_type: ParamType::Dropdown, answer: "fast".into(), options: vec!["ultrafast".into(), "superfast".into(), "veryfast".into(), "faster".into(), "fast".into(), "medium".into(), "slow".into(), "slower".into(), "veryslow".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -crf 25 -preset fast -c:a libopus -b:a 128k output.mp4".into(),
            scenario: "Cloudflare Stream video processing pipeline. H.265 at CRF 25 with fast preset for real-time transcoding at scale (thousands of concurrent uploads), Opus for modern web audio, MP4 for DASH/HLS delivery. Speed over compression ratio.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "Cloud transcoding services like Cloudflare Stream rarely use presets slower than \"fast\" because the compute cost per minute of video scales linearly with encode time. At $0.05/min GPU cost, \"slow\" vs \"fast\" adds $0.15 per video.".into(),
        },
        // Puzzle 55
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "500M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.exr -c:v libx264 -b:v 500M -pix_fmt yuv444p10le -c:a pcm_s24le output.mov".into(),
            scenario: "VFX review screening master from an EXR sequence. H.264 at near-lossless 500 Mbps with 4:4:4 10-bit color for VFX supervisor approval, PCM audio for the temp mix, MOV for screening room playback on Mistika or Resolve.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "H.264 supports 4:4:4 chroma subsampling and 10-bit depth via High 4:4:4 Predictive Profile, but hardware decoders almost never support it. VFX screening rooms use software decode on high-end workstations for this reason.".into(),
        },
        // Puzzle 56
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "VP9".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "12M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "2160p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Opus".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "WebM".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "60".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libvpx-vp9 -b:v 12M -r 60 -tile-columns 3 -frame-parallel 1 -c:a libopus -b:a 192k output.webm".into(),
            scenario: "YouTube 4K60 gaming VOD (pre-AV1 era pipeline still in use). VP9 at 12 Mbps for 4K60 per YouTube recommended settings, tile columns for parallel decode, Opus/WebM. YouTube transcodes everything to VP9 anyway, so this avoids double-encoding.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "YouTube stores every video in VP9 (and increasingly AV1) regardless of upload format. Uploading VP9 directly can skip one generation of compression loss. However, YouTube still re-encodes VP9 uploads -- there is no true \"passthrough\" mode.".into(),
        },
        // Puzzle 57
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.265".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "4M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "AAC".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MP4".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx265 -b:v 4M -c:a aac -b:a 128k -movflags +faststart output.mp4".into(),
            scenario: "In-flight entertainment (IFE) encoding for an airline seat-back system. H.265 at 4 Mbps for 1080p because the onboard server has limited storage per aircraft, AAC for the headphone jack DAC, MP4 for the Panasonic Avionics player.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Airlines load entertainment content via physical hard drives at airports (\"sneakernet\"). A typical IFE server holds 2-4 TB for 300+ hours of content. H.265 at 4 Mbps is the sweet spot between quality and fitting the full catalog.".into(),
        },
        // Puzzle 58
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "H.264".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "CRF".into(), param_type: ParamType::Free, answer: "24".into(), options: vec![], hint: Some("number".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "Vorbis".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MKV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v libx264 -crf 24 -c:a libvorbis -q:a 5 output.mkv".into(),
            scenario: "Kodi home media center encoding for a Raspberry Pi 4. H.264 because the Pi 4 has hardware H.264 decode but no H.265, CRF 24 for quality/size balance, Vorbis audio for the open-source stack, MKV for subtitle support.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "The Raspberry Pi 4 has hardware H.264 decode but H.265 decode was intentionally omitted to avoid HEVC patent licensing fees. The Pi 5 added H.265 hardware decode after the royalty landscape became clearer.".into(),
        },
        // Puzzle 59
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "ProRes".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "100M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "1080p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "PCM".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "MOV".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
                Param { key: "framerate".into(), label: "Framerate".into(), param_type: ParamType::Dropdown, answer: "25".into(), options: vec!["24".into(), "25".into(), "29.97".into(), "30".into(), "50".into(), "59.94".into(), "60".into()], hint: None },
            ],
            command: "ffmpeg -i input.mxf -c:v prores_ks -profile:v 2 -b:v 100M -r 25 -c:a pcm_s16le output.mov".into(),
            scenario: "BBC delivery spec for a commissioned drama series. ProRes 422 (profile 2) at 25fps PAL, 100 Mbps per BBC Technical Standards, PCM 16-bit audio for the broadcast mix, MOV container. The BBC still requires ProRes for SD/HD deliverables.".into(),
            difficulty: Difficulty::Hard,
            credit: None,
            fun_fact: "The BBC's Technical Delivery Standards document runs 130+ pages and mandates ProRes 422 (not HQ) for HD deliveries. ProRes HQ submissions are rejected -- not because of quality, but because the automated QC system flags non-compliant profiles.".into(),
        },
        // Puzzle 60
        Puzzle {
            params: vec![
                Param { key: "codec".into(), label: "Video Codec".into(), param_type: ParamType::Dropdown, answer: "MPEG-2".into(), options: vec!["H.264".into(), "H.265".into(), "VP9".into(), "AV1".into(), "ProRes".into(), "MPEG-2".into()], hint: None },
                Param { key: "quality".into(), label: "Bitrate".into(), param_type: ParamType::Free, answer: "3M".into(), options: vec![], hint: Some("e.g. 5M, 8M".into()) },
                Param { key: "resolution".into(), label: "Resolution".into(), param_type: ParamType::Dropdown, answer: "480p".into(), options: vec!["480p".into(), "720p".into(), "1080p".into(), "1440p".into(), "2160p".into()], hint: None },
                Param { key: "audio".into(), label: "Audio".into(), param_type: ParamType::Dropdown, answer: "MP3".into(), options: vec!["AAC".into(), "AC-3".into(), "E-AC-3".into(), "Opus".into(), "MP3".into(), "PCM".into(), "Vorbis".into()], hint: None },
                Param { key: "container".into(), label: "Container".into(), param_type: ParamType::Dropdown, answer: "TS".into(), options: vec!["MP4".into(), "MKV".into(), "WebM".into(), "MOV".into(), "TS".into(), "FLV".into()], hint: None },
            ],
            command: "ffmpeg -i input.mp4 -c:v mpeg2video -b:v 3M -vf scale=-2:480 -c:a mp2 -b:a 128k -f mpegts output.ts".into(),
            scenario: "DVD-quality IPTV multicast for a hospital patient entertainment system from 2012, still running. MPEG-2 at 480p/3 Mbps because the Amino STBs in every room only decode MPEG-2. MP3 audio. Nobody wants to fund the upgrade.".into(),
            difficulty: Difficulty::Standard,
            credit: None,
            fun_fact: "Hospital IT systems are notoriously difficult to upgrade because medical device regulations (IEC 80001) require risk assessments for any network change. A simple codec upgrade can trigger a 6-month compliance review process.".into(),
        },
    ]
}

pub fn get_day_index() -> usize {
    let epoch = Utc.with_ymd_and_hms(2026, 3, 7, 0, 0, 0).unwrap();
    let now = Utc::now();
    let days = (now - epoch).num_days();
    if days < 0 { 0 } else { days as usize }
}

pub fn get_puzzle_for_day() -> (usize, Puzzle) {
    let all = puzzles();
    let idx = get_day_index();
    let puzzle_idx = idx % all.len();
    (idx, all.into_iter().nth(puzzle_idx).unwrap())
}

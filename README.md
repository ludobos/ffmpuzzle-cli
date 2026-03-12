# ffmpuzzle CLI

Play the daily FFmpeg encoding puzzle from your terminal.

```
╔══════════════════════════════════════╗
║           ffmpuzzle                  ║
║     the daily ffmpeg puzzle          ║
║                                      ║
║  guess the encoding parameters       ║
║  in 6 tries or less.                 ║
║                                      ║
║          $ start                     ║
║                                      ║
║  streak: 3  |  best: 7              ║
╚══════════════════════════════════════╝
```

## Install

### macOS

```bash
brew install bostral/tap/ffmpuzzle
```

### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/ludobos/ffmpuzzle-cli/releases).

| Platform | Binary |
|----------|--------|
| macOS (Apple Silicon) | `ffmpuzzle-aarch64-apple-darwin` |
| macOS (Intel) | `ffmpuzzle-x86_64-apple-darwin` |
| Linux (x86_64) | `ffmpuzzle-x86_64-unknown-linux-gnu` |
| Windows | `ffmpuzzle-x86_64-pc-windows-msvc.exe` |

## Usage

### Interactive mode (default)

```bash
ffmpuzzle
```

Full TUI with dropdowns, timer, color-coded hints, and keyboard navigation.

### Flag mode

```bash
ffmpuzzle guess --codec H.265 --crf 23 --res 1080p --audio AAC --container MP4
```

### Other commands

```bash
ffmpuzzle status       # Current game state
ffmpuzzle share        # Share text (--copy for clipboard)
ffmpuzzle stats        # Win rate, streaks, history
ffmpuzzle leaderboard  # Today's leaderboard
ffmpuzzle --offline    # Force offline mode
```

## How it works

Each day, a real-world encoding scenario is presented. Guess the FFmpeg parameters in 6 tries:

- **Video Codec**: H.264, H.265, VP9, AV1, ProRes, MPEG-2
- **Quality**: CRF or Bitrate
- **Resolution**: 480p to 2160p
- **Audio Codec**: AAC, AC-3, E-AC-3, Opus, MP3, PCM, Vorbis
- **Container**: MP4, MKV, WebM, MOV, TS, FLV

Color-coded feedback after each guess:
- 🟩 **Correct** — exact match
- 🟨 **Close** — same family (e.g., H.264 ≈ H.265)
- 🟧 **Higher/Lower** — numeric value with direction arrow
- 🟥 **Wrong** — no match

## Features

- Interactive TUI (ratatui + crossterm)
- 60 real-world encoding scenarios
- Offline mode with embedded puzzles
- Streak tracking and persistence
- Share results with emoji grid
- Cross-platform: macOS, Linux, Windows

## Links

- [ffmpuzzle.com](https://ffmpuzzle.com) — play in your browser
- [streaming-radar.com](https://streaming-radar.com) — streaming industry intelligence
- [bostral.com](https://bostral.com) — by Ludovic Bostral

## License

Proprietary. Binaries are free to use. Source code is not distributed.

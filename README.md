

```markdown
# SpeedRust 🦀🚀

**Fast, lightweight internet speed test CLI in Rust**  
Optimized for **Jio 5G / AirFiber** users in India.

A beautiful, async terminal tool that measures real download/upload/ping with progress bars, colors, and retry logic for flaky 5G connections.

[![GitHub Stars](https://img.shields.io/github/stars/pratikdevelop/speedrust?style=social)](https://github.com/pratikdevelop/speedrust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![SpeedRust Example Output](https://i.imgur.com/YOUR_SCREENSHOT_LINK.png)  
*(Upload a screenshot of the tool running to imgur.com, copy the direct image link, and replace the placeholder above)*

### How to Add a Screenshot Quickly (Recommended!)
1. Run the tool once (`cargo run -- --iterations 3`) and take a screenshot of the colorful output.
2. Go to https://imgur.com → Click "New post" → Drag & drop or upload the screenshot.
3. Right-click the image → "Copy image address" (ends with .png or .jpg).
4. Replace `YOUR_SCREENSHOT_LINK` in the README with that link.
5. Commit & push — the image will appear perfectly in the README!

## Features
- Real measurements (no dummies!)
- Reliable public servers (Tele2 for download, file.io for upload)
- Beautiful progress bars + colored terminal output
- Automatic retries on connection resets (perfect for Jio 5G instability)
- Custom server support (`--server URL`)
- Tested & tuned for Indian connections (Madhya Pradesh focus)

## Installation

### Easiest: Install from GitHub
```bash
cargo install --git https://github.com/pratikdevelop/speedrust
```

### Build from Source
```bash
git clone https://github.com/pratikdevelop/speedrust.git
cd speedrust
cargo build --release
# Run it:
./target/release/speedrust --iterations 5
# Or on Windows:
target\release\speedrust.exe --iterations 5
```

## Usage
```bash
speedrust                      # Default: 3 iterations
speedrust --iterations 5      # More accurate average
speedrust --server http://speedtest.tele2.net/10MB.zip   # Quick small-file test
```

Example output (your results will vary):
```
Starting SpeedRust Test...
Date: 2026-01-15 14:32:06 +05:30
Using download server: http://speedtest.tele2.net/100MB.zip

┌──── Test 1/5 ─────┐
  Ping: 45 ms
  Downloading test file... [██████████] 100.00 MiB/100.00 MiB
  Download: 28.45 Mbps
  Uploading test data (10 MB)... (public temp endpoint)
  Upload: 15.20 Mbps
└──────────────────────┘

         FINAL RESULTS          
Average Ping:    42 ms
Average Download: 27.80 Mbps
Average Upload:  14.90 Mbps

Great performance! Your Jio 5G is flying 🚀
```

## Why SpeedRust?
Most CLI speed tools either use distant servers, crash on 5G instability, or show fake results.  
This one is:
- Pure Rust → instant startup, single binary
- Tuned for India (reliable servers, retry logic)
- Open-source & free (MIT license)

## Limitations & Roadmap
**Current limitations**:
- Upload uses temporary public endpoint (limited to ~10MB for stability)
- Single-connection tests → may show slightly lower than multi-thread tools like Speedtest.net

**Coming soon**:
- Auto-select nearest server (Mumbai/India)
- Multi-connection download for max speed
- Better upload testing (regional servers)
- JSON output (`--json`)
- Pre-built binaries (Windows/Linux)

## Contributing
Pull requests welcome! Especially:
- Better regional upload servers
- Multi-threaded download improvements
- Bug fixes for Jio 5G edge cases

Just open an issue or PR — let's make it better together!

Made with ❤️ in **Ujjain, Madhya Pradesh** by Dass Ji  
License: [MIT](LICENSE)

⭐ Star the repo if you like it!  
Found it useful? Consider [sponsoring the project](https://github.com/sponsors/pratikdevelop) on GitHub (coming soon!).

Happy speed testing! 🇮🇳
```

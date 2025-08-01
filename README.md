# packet_capture_rs_test

original: https://qiita.com/m10i/items/f8d3db359f150aafc83b

- made work with Mac/Windows (and maybe Linux)

## Prerequisites

### Windows

- install Npcap with *"Install Npcap WinPcap API-compatible Mode"*
- make accesible `Packet.lib` in npcap sdk (kind of like https://npcap.com/dist/npcap-sdk-1.13.zip) from Rust
  - set `LIB` environment path, or `-L` argument of `RUSTFLAGS` or something etc.
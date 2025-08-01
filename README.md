# packet_capture_rs_test

Packet capture using Rust (pnet, netdev crates)

original: https://qiita.com/m10i/items/f8d3db359f150aafc83b

- made work with Mac/Windows (and maybe Linux)

## Prerequisites

### Windows

- install Npcap with *"Install Npcap WinPcap API-compatible Mode"*
- make accesible `Packet.lib` in npcap sdk (kind of like https://npcap.com/dist/npcap-sdk-1.13.zip) from Rust
  - set `LIB` environment path, or `-L` argument of `RUSTFLAGS` or something etc.

## Usage

```bash
$ cargo run
```

## License

[Original code](https://qiita.com/m10i/items/f8d3db359f150aafc83b)'s author is [MINETA Hiroki (@m10i)](https://qiita.com/m10i) on [Qiita](https://qiita.com/).

According to [Terms of use of Qiita](https://qiita.com/terms), you have rights to use these codes with any restrictions or obligations.

> 7.3: 登録ユーザーは、本サイトに投稿したコード、スニペットなどプログラムに類するものに限り、他の登録ユーザーが商用私用問わず無償で使用することを許諾し、他の登録ユーザーはこれを使用できるものとします。

> 7.4: 前各項の登録ユーザーによる利用許諾には、地域制限、著作権表示義務その他付随条件はないものとします。

(Machine Translation)

> 7.3: Registered users shall grant other registered users permission to use, free of charge, any code, snippets, or other program-related materials posted on this site, regardless of whether such use is for commercial or personal purposes, and other registered users shall be permitted to use such materials.

> 7.4: The permission granted by registered users in the preceding paragraphs shall not be subject to any regional restrictions, copyright notice requirements, or other accompanying conditions.

(As for my modification part of the code, I don't care. If you need, treat it as [WTFPL (v2)](https://en.wikipedia.org/wiki/WTFPL) or MIT or Apache as you like.)

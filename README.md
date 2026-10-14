# Music Streaming Server

A simple, asynchronous music streaming server built with [Tokio](https://tokio.rs/) and [Axum](https://github.com/tokio-rs/axum).

## Features

- **Asynchronous Audio Streaming**: Audio files are streamed chunk-by-chunk using Tokio and `tokio-util` (`ReaderStream`), supporting HTTP byte ranges and proper audio headers.
- **Embedded Web Player**: Standalone web frontend featuring a responsive single Play/Pause button with live status updates.
- **Zero-setup**: All frontend assets are bundled with the server; no external web server required.

## Endpoints

- `GET /` — Serves the embedded HTML audio player.
- `GET /stream` — Asynchronously streams the MP3 audio track (`audio/mpeg`).

## Prerequisites

- [Rust](https://www.rust-lang.org/) (2021 edition or newer)
- Cargo

## Quick Start

1. Clone the repository:
   ```bash
   git clone https://github.com/edbertkwesiek/musicstreamingserver.git
   cd musicstreamingserver
   ```

2. Run the server:
   ```bash
   cargo run
   ```

3. Open your browser:
   Visit [http://localhost:3000](http://localhost:3000) and click **Play**.

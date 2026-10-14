use axum::{
    body::Body,
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::{Path, PathBuf};
use tokio_util::io::ReaderStream;

const HTML_CONTENT: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Rust Axum Music Streamer</title>
    <style>
        body {
            font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background-color: #121212;
            color: #ffffff;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            height: 100vh;
            margin: 0;
        }
        .player-card {
            background: #1e1e1e;
            padding: 40px;
            border-radius: 16px;
            box-shadow: 0 10px 30px rgba(0, 0, 0, 0.6);
            text-align: center;
            width: 320px;
        }
        h1 {
            font-size: 1.4rem;
            margin-bottom: 8px;
        }
        p {
            color: #a0a0a0;
            font-size: 0.95rem;
            margin-bottom: 24px;
        }
        .play-button {
            background-color: #1db954;
            color: white;
            border: none;
            padding: 14px 32px;
            font-size: 1.1rem;
            font-weight: 600;
            border-radius: 50px;
            cursor: pointer;
            transition: all 0.2s ease;
            display: inline-flex;
            align-items: center;
            gap: 10px;
        }
        .play-button:hover {
            background-color: #1ed760;
            transform: scale(1.05);
        }
        .status {
            margin-top: 20px;
            font-size: 0.85rem;
            color: #888;
        }
    </style>
</head>
<body>
    <div class="player-card">
        <h1>🎵 Music Stream</h1>
        <p>Click below to stream audio live</p>
        <button id="playBtn" class="play-button">▶ Play</button>
        <div id="status" class="status">Ready</div>
        <audio id="audioPlayer" preload="none" src="/stream"></audio>
    </div>

    <script>
        const playBtn = document.getElementById('playBtn');
        const audio = document.getElementById('audioPlayer');
        const status = document.getElementById('status');

        playBtn.addEventListener('click', () => {
            if (audio.paused) {
                audio.play()
                    .then(() => {
                        playBtn.textContent = '⏸ Pause';
                        status.textContent = 'Streaming...';
                    })
                    .catch((err) => {
                        console.error('Play error:', err);
                        status.textContent = 'Error: ' + err.message;
                    });
            } else {
                audio.pause();
                playBtn.textContent = '▶ Play';
                status.textContent = 'Paused';
            }
        });

        audio.addEventListener('ended', () => {
            playBtn.textContent = '▶ Play';
            status.textContent = 'Finished';
        });

        audio.addEventListener('waiting', () => {
            status.textContent = 'Buffering...';
        });

        audio.addEventListener('playing', () => {
            status.textContent = 'Streaming...';
        });
    </script>
</body>
</html>
"#;

fn find_audio_file() -> Option<PathBuf> {
    let candidates = [
        "src/assets/track.mp3",
        "assets/track.mp3",
        "track.mp3",
    ];
    for path in &candidates {
        if Path::new(path).exists() {
            return Some(PathBuf::from(path));
        }
    }
    None
}

async fn index_handler() -> Html<&'static str> {
    Html(HTML_CONTENT)
}

async fn stream_audio_handler() -> impl IntoResponse {
    let path = match find_audio_file() {
        Some(p) => p,
        None => {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(
                    "Audio track not found on server (looked for src/assets/track.mp3, assets/track.mp3, track.mp3)",
                ))
                .unwrap();
        }
    };

    let file = match tokio::fs::File::open(&path).await {
        Ok(file) => file,
        Err(err) => {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Failed to open audio file: {err}")))
                .unwrap();
        }
    };

    let content_length = match file.metadata().await {
        Ok(meta) => Some(meta.len()),
        Err(_) => None,
    };

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut response = Response::builder()
        .header(header::CONTENT_TYPE, "audio/mpeg")
        .header(header::ACCEPT_RANGES, "bytes");

    if let Some(len) = content_length {
        response = response.header(header::CONTENT_LENGTH, len.to_string());
    }

    response.body(body).unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/stream", get(stream_audio_handler));

    let addr = "127.0.0.1:3000";
    println!("Starting music streaming server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

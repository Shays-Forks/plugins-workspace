// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use futures_util::TryStreamExt;
use serde::{ser::Serializer, Serialize};
use tauri::{
    api::ipc::Channel,
    command,
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Runtime,
};
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use read_progress_stream::ReadProgressStream;

use std::collections::HashMap;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("{0}")]
    ContentLength(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Clone, Serialize)]
struct ProgressPayload {
    progress: u64,
    total: u64,
}

#[command]
async fn download<R: Runtime>(
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
    on_progress: Channel<R>,
) -> Result<()> {
    let client = reqwest::Client::new();

    let mut request = client.get(url);
    // Loop trought the headers keys and values
    // and add them to the request object.
    for (key, value) in headers {
        request = request.header(&key, value);
    }

    let response = request.send().await?;
    let total = response.content_length().unwrap_or(0);

    let mut file = File::create(file_path).await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.try_next().await? {
        file.write_all(&chunk).await?;
        let _ = on_progress.send(&ProgressPayload {
            progress: chunk.len() as u64,
            total,
        });
    }

    Ok(())
}

#[command]
async fn upload<R: Runtime>(
    url: &str,
    file_path: &str,
    headers: HashMap<String, String>,
    on_progress: Channel<R>,
) -> Result<serde_json::Value> {
    // Read the file
    let file = File::open(file_path).await?;

    // Create the request and attach the file to the body
    let client = reqwest::Client::new();
    let mut request = client.post(url).body(file_to_body(on_progress, file));

    // Loop trought the headers keys and values
    // and add them to the request object.
    for (key, value) in headers {
        request = request.header(&key, value);
    }

    let response = request.send().await?;

    response.json().await.map_err(Into::into)
}

fn file_to_body<R: Runtime>(channel: Channel<R>, file: File) -> reqwest::Body {
    let stream = FramedRead::new(file, BytesCodec::new()).map_ok(|r| r.freeze());

    reqwest::Body::wrap_stream(ReadProgressStream::new(
        stream,
        Box::new(move |progress, total| {
            let _ = channel.send(&ProgressPayload { progress, total });
        }),
    ))
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    PluginBuilder::new("upload")
        .js_init_script(include_str!("api-iife.js").to_string())
        .invoke_handler(tauri::generate_handler![download, upload])
        .build()
}

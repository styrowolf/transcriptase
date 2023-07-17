/*
 * Sections copied from https://github.com/m1guelpf/whisper-cli-rs
 * MIT License - Copyright (c) Miguel Piedrafita
 */

#![allow(dead_code)]
use futures_util::StreamExt;
use num::integer::div_floor;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    cmp::min,
    fs::File,
    io::Write,
    path::PathBuf,
    sync::{atomic::AtomicBool, Arc},
};
use tauri::{
    async_runtime::{JoinHandle, Mutex},
    Event, Manager,
};

pub async fn download_file(url: &str, path: &str) {
    let res = Client::new()
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))
        .unwrap();

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))
        .unwrap();

    let mut file = File::create(path)
        .or(Err(format!("Failed to create file '{path}'")))
        .unwrap();

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file")).unwrap();

        file.write_all(&chunk)
            .or(Err("Error while writing to file"))
            .unwrap();

        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
    }
}

pub async fn download_file_tauri<R: tauri::Runtime>(
    url: &str,
    path: &str,
    app: &tauri::AppHandle<R>,
) {
    let app = app.clone();
    let url = url.to_owned();
    let path = path.to_owned();
    let path2 = path.clone();

    let (tx, mut rx) = tauri::async_runtime::channel::<bool>(1);

    let handle = tauri::async_runtime::spawn(async move {
        let res = match Client::new()
            .get(&url)
            .send()
            .await
            .or(Err(format!("Failed to GET from '{}'", &url)))
        {
            Ok(t) => t,
            Err(err) => {
                let _ = app.emit_all("error", err);
                return;
            }
        };

        let total_size = match res
            .content_length()
            .ok_or(format!("Failed to get content length from '{}'", &url))
        {
            Ok(t) => t,
            Err(err) => {
                let _ = app.emit_all("error", err);
                return;
            }
        };

        DownloadEvents::Start {
            total_size: total_size,
        }
        .emit_all(&app)
        .unwrap();

        let mut file = match File::create(&path).or(Err(format!("Failed to create file '{path}'")))
        {
            Ok(t) => t,
            Err(err) => {
                let _ = app.emit_all("error", err);
                return;
            }
        };

        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = match item.or(Err("Error while downloading file")) {
                Ok(t) => t,
                Err(err) => {
                    let _ = app.emit_all("error", err);
                    return;
                }
            };

            match file
                .write_all(&chunk)
                .or(Err("Error while writing to file"))
            {
                Ok(t) => t,
                Err(err) => {
                    let _ = app.emit_all("error", err);
                    return;
                }
            };

            let new = min(downloaded + (chunk.len() as u64), total_size);

            DownloadEvents::Update { downloaded: new }
                .emit_all(&app)
                .unwrap();

            downloaded = new;
        }

        DownloadEvents::Finished.emit_all(&app).unwrap();
        let _ = tx.send(true).await;
    });

    DOWNLOADER_HANDLE.lock().await.replace((path2, handle));
    let _ = rx.recv().await;
}

pub fn format_timestamp(seconds: i64, always_include_hours: bool, decimal_marker: &str) -> String {
    assert!(seconds >= 0, "non-negative timestamp expected");
    let mut milliseconds = seconds * 1000;

    let hours = div_floor(milliseconds, 3_600_000);
    milliseconds -= hours * 3_600_000;

    let minutes = div_floor(milliseconds, 60_000);
    milliseconds -= minutes * 60_000;

    let seconds = div_floor(milliseconds, 1_000);
    milliseconds -= seconds * 1_000;

    let hours_marker = if always_include_hours || hours != 0 {
        format!("{hours}:")
    } else {
        String::new()
    };

    format!("{hours_marker}{minutes:02}:{seconds:02}{decimal_marker}{milliseconds:03}")
}

pub fn write_to(path: PathBuf, content: &String) {
    File::create(path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DownloadEvents {
    Start { total_size: u64 },
    Update { downloaded: u64 },
    Finished,
}

impl DownloadEvents {
    pub fn emit_all<R: tauri::Runtime>(
        &self,
        app: &tauri::AppHandle<R>,
    ) -> Result<(), tauri::Error> {
        match self {
            DownloadEvents::Start { total_size } => app.emit_all(self.event_name(), total_size),
            DownloadEvents::Update { downloaded } => app.emit_all(self.event_name(), downloaded),
            DownloadEvents::Finished => app.emit_all(self.event_name(), 0),
        }
    }

    pub fn event_name(&self) -> &str {
        match self {
            DownloadEvents::Start { total_size } => "download:start",
            DownloadEvents::Update { downloaded } => "download:update",
            DownloadEvents::Finished => "download:finished",
        }
    }
}

pub fn bytes_to_string(bytes: u64) -> String {
    byte_unit::Byte::from_bytes(bytes as u128)
        .get_appropriate_unit(true)
        .to_string()
}

static IS_DOWNLOAD_CANCELLED: AtomicBool = AtomicBool::new(false);

pub fn get_is_download_cancelled() -> bool {
    match IS_DOWNLOAD_CANCELLED.fetch_update(
        std::sync::atomic::Ordering::Relaxed,
        std::sync::atomic::Ordering::Relaxed,
        |_| None,
    ) {
        Ok(b) => b,
        Err(b) => b,
    }
}

pub fn download_reset() {
    IS_DOWNLOAD_CANCELLED.store(false, std::sync::atomic::Ordering::Relaxed);
}

pub async fn download_cancelled() {
    match *DOWNLOADER_HANDLE.lock().await {
        Some((ref path, ref handle)) => {
            handle.abort();
            let _ = std::fs::remove_file(path);
        }
        None => (),
    }

    IS_DOWNLOAD_CANCELLED.store(true, std::sync::atomic::Ordering::Relaxed);
}

static DOWNLOADER_HANDLE: Lazy<Arc<Mutex<Option<(String, JoinHandle<()>)>>>> =
    Lazy::new(|| Arc::new(Mutex::new(None)));

pub fn get_transcription_folder_from_input(audio_path: impl AsRef<std::path::Path>) -> PathBuf {
    let mut audio_path = PathBuf::from(audio_path.as_ref());
    let last = audio_path.components().last().unwrap();
    let folder_name = format!("{} - transcriptions", last.as_os_str().to_string_lossy());

    audio_path.pop();

    let save_path = audio_path.join(folder_name);
    save_path
}

pub fn propagate_error<R: tauri::Runtime>(app: &tauri::AppHandle<R>, error: &str) {
    let _ = app.emit_all("error", error);
}

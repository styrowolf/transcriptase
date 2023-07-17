// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use clap::ValueEnum;
use progress::progress_message;
use tauri::{Manager, Runtime};
use whisper::Whisper;

mod ffmpeg_decoder;
mod model;
mod progress;
mod transcript;
mod utils;
mod whisper;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            transcribe,
            get_models,
            set_model,
            get_languages,
            bytes_to_string,
            open_transcription
        ])
        .setup(|app| {
            app.listen_global("download:cancel", |_| {
                let _ = tauri::async_runtime::spawn(async move {
                    let _ = utils::download_cancelled().await;
                });
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn transcribe<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    model: &str,
    language: &str,
    audio_path: &str,
) -> Result<(), String> {
    let model = model::Model::new(model::Size::optional_from_str(model).unwrap());
    let language = whisper::Language::from_str(language, true).unwrap();

    model.download_tauri(&app).await;

    if utils::get_is_download_cancelled() {
        utils::download_reset();
        return Err("Download cancelled".to_owned());
    }

    let mut whisper = Whisper::new_tauri(model, Some(language), &app).await;

    let audio_path = PathBuf::from(audio_path);

    let handle = progress_message(&app).await;

    let transcript = match whisper.transcribe(&audio_path, false, false) {
        Ok(t) => t,
        Err(err) => {
            let _ = app.emit_all("error", err.to_string());
            return Err(err.to_string());
        }
    };

    handle.abort();

    let save_path = utils::get_transcription_folder_from_input(audio_path);

    let _ = std::fs::create_dir(&save_path);

    utils::write_to(save_path.join("transcript.txt"), &transcript.as_text());
    utils::write_to(save_path.join("transcript.vtt"), &transcript.as_vtt());
    utils::write_to(save_path.join("transcript.srt"), &transcript.as_srt());

    Ok(())
}

#[tauri::command]
async fn set_model<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    model: &str,
) -> Result<(), String> {
    let model = model::Model::new(model::Size::optional_from_str(model).unwrap());
    model.download().await;

    Ok(())
}

#[tauri::command]
fn get_models() -> Vec<String> {
    [
        model::Size::TinyEnglish,
        model::Size::Tiny,
        model::Size::BaseEnglish,
        model::Size::Base,
        model::Size::SmallEnglish,
        model::Size::Small,
        model::Size::MediumEnglish,
        model::Size::Medium,
        model::Size::Large,
        model::Size::LargeV1,
    ]
    .iter()
    .map(|m| m.to_string())
    .collect()
}

#[tauri::command]
fn get_languages() -> Vec<String> {
    enum_iterator::all::<whisper::Language>()
        .map(|l| String::from(l))
        .collect()
}

#[tauri::command]
fn bytes_to_string(bytes: u64) -> String {
    utils::bytes_to_string(bytes)
}

#[tauri::command]
fn open_transcription(path: &str) -> Result<(), String> {
    let path = utils::get_transcription_folder_from_input(path);
    open::that_detached(path).map_err(|e| e.to_string())
}

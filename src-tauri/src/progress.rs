use once_cell::sync::Lazy;
use std::{
    ffi::{c_int, c_void},
    sync::Arc,
};

use tauri::{
    async_runtime::{self, Mutex},
    Manager,
};
use whisper_rs_sys::{whisper_context, whisper_state};

static TAURI_PROGRESS: once_cell::sync::Lazy<(
    tauri::async_runtime::Sender<usize>,
    Arc<tauri::async_runtime::Mutex<tauri::async_runtime::Receiver<usize>>>,
)> = Lazy::new(|| {
    let (tx, rx) = tauri::async_runtime::channel(20);
    (tx, Arc::new(Mutex::new(rx)))
});

pub unsafe extern "C" fn tauri_progress_callback(
    ctx: *mut whisper_context,
    state: *mut whisper_state,
    progress: c_int,
    user_data: *mut c_void,
) {
    let progress = progress as usize;
    let _ = async_runtime::spawn(async move {
        let _ = TAURI_PROGRESS.0.send(progress as usize).await;
    });
}

pub async fn progress_message<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> async_runtime::JoinHandle<()> {
    let stream = TAURI_PROGRESS.1.clone();
    let app = app.clone();
    let handle = async_runtime::spawn(async move {
        while let Some(p) = stream.lock().await.recv().await {
            let _ = app.emit_all("progress", p);
        }
    });

    handle
}

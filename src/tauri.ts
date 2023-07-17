import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";

export async function getLanguages(): Promise<string[]> {
    return await invoke('get_languages');
}

export async function getModels(): Promise<string[]> {
    return await invoke('get_models');
}

export async function transcribe(model: string, language: string, audioPath: string) {
    await invoke('transcribe', { model, language, audioPath });
}

export async function cancelDownload() {
    await emit('download:cancel');
}

export async function bytesToString(bytes: number): Promise<string> {
    return await invoke('bytes_to_string', { bytes });
}

export async function openTranscription(path: string) {
    await invoke('open_transcription', { path })
}
# transcriptase

transcriptase is a simple desktop app for transcribing audio files using whisper.

## installation
- ffmpeg is required for transcriptase to work.
    - macOS: `brew install ffmpeg`
    - linux: follow your distribution's instructions to install
- You can download the app from [the latest release](https://github.com/m1guelpf/whisper-cli-rs/releases/latest).

Built with: 
 - [Tauri](https://tauri.app/)
 - React
 - [whisper.cpp](https://github.com/ggerganov/whisper.cpp)
 - [whisper-rs](https://github.com/m1guelpf/whisper-cli-rs)

Most of the code used for download and transcription are copied from [whisper-cli-rs](https://github.com/m1guelpf/whisper-cli-rs), 
licensed under MIT.

## license

This project is licensed under the MIT License - see the LICENSE file for details.
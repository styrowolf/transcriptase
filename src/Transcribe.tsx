import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import { listen, Event, UnlistenFn } from "@tauri-apps/api/event";
import { useAppState, useDispatch } from "./AppState";
import { bytesToString, cancelDownload, transcribe } from "./tauri";
import { useNavigate } from "react-router-dom";

function App() {
  const dispatch = useDispatch()!;
  const state = useAppState()!;
  const navigate = useNavigate();

  const [isDownloading, setIsDownloading] = useState(0);
  const [totalSizeStr, setTotalSizeStr] = useState("");

  const [currentDownloaded, setCurrentDownloaded] = useState(0);
  const [currentDownloadedStr, setCurrentDownloadedStr] = useState("");
  
  const [isFinished, setIsFinished] = useState(false);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    const unlistens: UnlistenFn[] = [];
    (async () => {
        // @ts-ignore
        unlistens.push(await listen('download:start', (event: Event<number>) => setIsDownloading(event.payload)));
        // @ts-ignore
        unlistens.push(await listen('download:update', (event: Event<number>) => setCurrentDownloaded((before) => before > event.payload ? before : event.payload)));
        // @ts-ignore
        unlistens.push(await listen('download:finished', (event) => {
            setIsFinished(true)
        }));
        // @ts-ignore
        unlistens.push(await listen('progress', (event: Event<number>) => setProgress(event.payload)));
        unlistens.push(await listen('error', (event: Event<string>) => navigate("/error", { state: { error: event.payload } })))
      })();
    
    transcribe(state.model ?? "tiny", state.language ?? "auto", state.input_file ?? "")
        .then(() => navigate("/success"), () => {});

    return () => {
      (async () => {
        for (const unlistenFn in unlistens) {
            console.log(unlistenFn)
            // @ts-ignore
            unlistenFn()
        }
      })();
    };
  }, []);

  useEffect(() => {
    (async () => {
      setCurrentDownloadedStr(await bytesToString(currentDownloaded));
    })()
  }, [currentDownloaded]);

  useEffect(() => {
    (async () => {
      setTotalSizeStr(await bytesToString(isDownloading));
    })()
  }, [isDownloading]);

  if (!isFinished && isDownloading > 0) {
    return (
        <div className="container centered">
            <p>downloading model {state.model} ({`${currentDownloadedStr} / ${totalSizeStr}`} | {(currentDownloaded / isDownloading * 100).toFixed(2)}%)</p>
            <progress value={currentDownloaded} max={isDownloading}></progress>
            <button className="alert" onClick={async () => {
              await cancelDownload();
              navigate("/model-selection");
            }}>cancel download</button>
        </div>
    )
  } else /* if (isFinished || isDownloading == 0) */ {
    return (
        <div className="container centered">
            <p>transcribing audio... please wait... ({progress}% done)</p>
            <progress value={progress} max="100"></progress>
        </div>
    )
  }
}

export default App;

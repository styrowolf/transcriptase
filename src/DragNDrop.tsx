import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import { listen, Event, UnlistenFn } from "@tauri-apps/api/event";
import { useAppState, useDispatch } from "./AppState";
import { redirect, useNavigate } from "react-router-dom";

function DragNDrop() {
  const dispatch = useDispatch()!;
  const navigate = useNavigate();

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;
    (async () => {
      // @ts-ignore
      unlisten = await listen('tauri://file-drop', (event: Event<string[]>) => {
        if (event.payload) {
          dispatch({type: "input_file", path: event.payload[0]});
          navigate("/model-selection")
         }
      });
    })();
    return () => {
      (async () => {
        if (unlisten !== null) {
          unlisten()
        }
      })();
    };
  }, []);

  return (
    <div className="container centered">
      <h2>transcriptase</h2>

      <p>drop audio files</p>
      <p><i>or</i></p>
      <button onClick={ async () => {
       const selection = await open({
        directory: false,
        filters: [{name: "Audio", extensions: ["wav", "mp3", "m4a"] }]});
       // @ts-ignore
       dispatch({type: "input_file", path: selection });
       if (selection !== "" && selection) {
        navigate("/model-selection")
       }
      }
       }>open</button>

      <p className="centered-horizontal"><i><a onClick={() => navigate('/credits')}>credits</a></i></p>

    </div>
  );
}

export default DragNDrop;

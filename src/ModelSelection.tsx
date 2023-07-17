import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { open, save } from "@tauri-apps/api/dialog";
import { listen, Event, UnlistenFn } from "@tauri-apps/api/event";
import { useAppState, useDispatch } from "./AppState";
import { getLanguages, getModels } from "./tauri";
import { redirect, useNavigate } from "react-router-dom";

function App() {
  const dispatch = useDispatch()!;
  const state = useAppState()!;
  const navigate = useNavigate();

  const [languages, setLanguages] = useState<string[]>([]);
  const [models, setModels] = useState<string[]>([]);

  useEffect(() => {
    (async () => {
        setLanguages(await getLanguages());
        setModels(await getModels());
    })()
  })

  return (
    <div className="container centered">
        <p>select language</p>
        <select value={state.language} onChange={(event) => dispatch({type: "select_language", language: event.currentTarget.value})}>
            {
                languages.map((value) => <option key={value} value={value}>{value}</option>)
            }
        </select>

        <p>select model</p>
        <select value={state.model} onChange={(event) => dispatch({type: "select_model", model: event.currentTarget.value})}>
            {
                models.map((value) => <option key={value} value={value}>{value}</option>)
            }
        </select>
      
        <div className="grid">
          <div>
            <button onClick={() => navigate("/")}>go back</button>
          </div>
          <div>
            <button onClick={() => navigate("/transcribe")}>transcribe</button>
          </div>
        </div>
    </div>
  );
}

export default App;

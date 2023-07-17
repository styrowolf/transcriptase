import { useNavigate } from "react-router-dom"
import { useAppState } from "./AppState";
import { type } from '@tauri-apps/api/os';
import { openTranscription } from "./tauri";

function Success() {
    const navigate = useNavigate();
    const state = useAppState()!;
    
    return (
        <div className="container centered">
            <p>finished transcribing!</p>

            <button onClick={async () => { await openTranscription(state.input_file!) }}>open transcriptions</button>
            <button onClick={() => navigate("/")}>transcribe again?</button>
        </div>
    )
}

export default Success;
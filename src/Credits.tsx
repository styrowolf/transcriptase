import { useNavigate } from "react-router-dom";

function Credits() {
    const navigate = useNavigate();
    
    return (
        <div className="container centered">
            <h2><a href="https://github.com/styrowolf/transcriptase">transcriptase</a></h2>
            
            <p>© 2023 Oğuz Kurt</p>
            <p>an <a href="">application</a> for transcribing recordings powered by: </p>
            <ul>
                <li><a href="https://github.com/ggerganov/whisper.cpp">whisper.cpp</a></li>
                <li><a href="https://github.com/tazz4843/whisper-rs">whisper-rs</a></li>
                <li><a href="https://tauri.app/">tauri</a></li>
                <li><a href="https://github.com/m1guelpf/whisper-cli-rs">whisper-cli-rs</a></li>
            </ul>
            

            <a className="centered-horizontal" onClick={() => navigate("/")}>back to transcribing</a>
        </div>
    )
}

export default Credits;
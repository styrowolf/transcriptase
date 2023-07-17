import { useLocation, useNavigate } from "react-router-dom";

function ErrorScreen() {
    const navigate = useNavigate();
    const location = useLocation();
    const { error }: { error: string} = location.state ?? { error: "An unknown error occured" };
    
    return (
        <div className="container centered">
            <h3>error:</h3>

            <p>{error}</p>

            <button onClick={() => navigate("/")}>okay</button>
        </div>
    )
}

export default ErrorScreen;
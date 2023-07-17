import { createContext, useContext, useReducer } from "react";

const stateContext = createContext<State | null>(null);
const dispatchContext = createContext<React.Dispatch<Action> | null>(null);

function AppStateProvider({ children }: { children: any }) {
    const [state, dispatch] = useReducer(reducer, {
        language: "auto",
        model: "tiny",
    });
    return (
        <stateContext.Provider value={state}>
            <dispatchContext.Provider value={dispatch}>
                { children }
            </dispatchContext.Provider>
        </stateContext.Provider>
    );
}

export function useAppState() {
    return useContext(stateContext);
}

export function useDispatch() {
    return useContext(dispatchContext);
}

type State = {
    input_file?: string,
    model?: string,
    language?: string,
}

type Action =
    | { type: 'input_file', path: string }
    | { type: 'select_model', model: string }
    | { type: 'select_language', language: string };

function reducer(state: State, action: Action) {
    switch (action.type) {
        case 'input_file':   
            state.input_file = action.path;
            break;
        case 'select_model':
            state.model = action.model;
            break;
        case "select_language":
            state.language = action.language;
            break;
        default:
            break;
    }
    return state;
}

export default AppStateProvider;
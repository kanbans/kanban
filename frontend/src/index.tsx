import { render } from "solid-js/web";
import { Router } from "solid-app-router";

import "./index.css";
import App from "./App";
import { createContext } from "solid-js";
import { initStore } from "./store";
import { backendClient } from "./repos";

const store = initStore();
export const GlobalContext = createContext(store);
export const BackendContext = createContext(backendClient(store.state.auth.data));

render(() => (
        <Router>
            <App />
        </Router>
    ),
    document.getElementById("root") as HTMLElement
);
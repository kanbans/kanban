import { render } from "solid-js/web";
import { Router } from "solid-app-router";

import "./index.css";
import App from "./App";
import { createContext } from "solid-js";
import { initStore } from "./store";
import { backendClient } from "./repos";

export const GlobalContext = createContext(initStore());
export const BackendContext = createContext(backendClient());

render(() => (
        <Router>
            <App />
        </Router>
    ),
    document.getElementById("root") as HTMLElement
);

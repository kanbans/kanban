import { Auth, AuthVaraint } from "./types/auth"
import { createStore } from "solid-js/store";
import { Board } from "./repos/board";

function getAuth(): Auth {
    const session = sessionStorage.getItem("session");
    return session ? { variant: AuthVaraint.Login, data: session } : { variant: AuthVaraint.None, data: undefined };
}

const defaultState = () => {
    return {
        auth: getAuth()
    }
};

export const initStore = () => {
    const [state, setState] = createStore(defaultState());

    return {
        state,
        setAuth: (auth: Auth) => {
            if (auth.variant == AuthVaraint.Login)
                sessionStorage.setItem("session", auth.data);

            setState({ auth })
        }
    }
}

export type GlobalStore = ReturnType<typeof initStore>;
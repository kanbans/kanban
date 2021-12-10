import { Auth, AuthVaraint } from "./types/auth"
import { createStore } from "solid-js/store";

function getAuth(): Auth {
    const session = sessionStorage.getItem("session");
    return session ? { variant: AuthVaraint.Login, data: session } : { variant: AuthVaraint.None, data: undefined };
}

const defaultState = () => ({
    auth: getAuth()
});

export const initStore = () => {
    const [state, setState] = createStore(defaultState());

    return {
        state,
        setAuth: (auth: Auth) => {
            if (auth.variant == AuthVaraint.Login)
                sessionStorage.setItem("session", auth.data);

            setState({ auth })
        },
    }
}
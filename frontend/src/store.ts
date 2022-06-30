import { Auth, AuthVaraint } from "./types/auth"
import { createStore } from "solid-js/store";
import { Board } from "./repos/board";

function getAuth(): Auth {
    const session = sessionStorage.getItem("session");
    return session ? { variant: AuthVaraint.Login, data: session } : { variant: AuthVaraint.None, data: undefined };
}

export type State = {
    board: Board | undefined,
    auth: Auth
}

const defaultState = () => {
    return {
        board: undefined,
        auth: getAuth()
    }
};

export const initStore = () => {
    const [state, setState] = createStore<State>(defaultState());
    return {
        state,
        setAuth: (auth: Auth) => {
            if (auth.variant == AuthVaraint.Login)
                sessionStorage.setItem("session", auth.data);

            setState({ ...state, auth })
        },
        setSelectedBoard: (board: Board) => {
            if (state.board && state.board.id === board.id) {
                return
            }
            setState({ ...state, board })
        }
    }
}

export type GlobalStore = ReturnType<typeof initStore>;
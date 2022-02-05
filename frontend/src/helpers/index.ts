import { useLocation, useNavigate } from "solid-app-router";
import { useContext } from "solid-js";
import { GlobalContext } from "..";
import { AuthVaraint } from "../types/auth";


export function requireAuth() {
    const store = useContext(GlobalContext);
    const location = useLocation();
    const navigate = useNavigate();

    if (store.state.auth.variant == AuthVaraint.None) {
        navigate("/auth", { state: { redirect: location.pathname } });
    }
}
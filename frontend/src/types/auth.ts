import { Associate } from ".";

export enum AuthVaraint {
    Login,
    None
}

export type Auth = Associate<AuthVaraint.Login, string> | Associate<AuthVaraint.None, undefined>;
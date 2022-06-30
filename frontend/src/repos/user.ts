import { AxiosError } from "axios";
import { Err, Ok, Result } from "@sniptt/monads";
import { BackendClient, BackendResp } from ".";

type AuthResp = BackendResp<{
    "session_token": string
}>;

export function authClient(
    backendClient: BackendClient,
    email: string,
    password: string
): Promise<Result<string, string>> {
    return backendClient.post<AuthResp>(
        "/user/login",
        {
            email,
            password
        }
    )
    .then(v => {
        backendClient.defaults.headers.common["Authorization"] = v.data.session_token;
        return Ok(v.data.session_token);
    })
    .catch((e: AxiosError) => Err(e.message));
}

export function authClientReg(
    backendClient: BackendClient,
    name: string,
    email: string,
    password: string
): Promise<Result<string, string>> {
    return backendClient.post<AuthResp>(
        "/user/register",
        {
            name,
            email,
            password
        }
    )
    .then(v => {
        backendClient.defaults.headers.common["Authorization"] = v.data.session_token;
        return Ok(v.data.session_token);
    })
    .catch((e: AxiosError) => Err(e.message));
}
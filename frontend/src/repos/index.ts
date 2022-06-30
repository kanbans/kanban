import axios, { AxiosInstance } from "axios";
import { AppConfig } from "../config";

export type BackendClient = AxiosInstance;

export type BackendResp<T> = {
    success: boolean
} & T;

export const backendClient = (auth?: string) => axios.create({
    baseURL: AppConfig.backendURL,
    headers: auth ? { "Authorization": auth } : undefined
});
import axios, { AxiosInstance } from "axios";
import { AppConfig } from "../config";

export type BackendClient = AxiosInstance;

export const backendClient = () => axios.create({
    baseURL: AppConfig.backendURL
});
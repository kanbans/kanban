import { Err, Ok, Result } from "@sniptt/monads/build";
import { AxiosError } from "axios";
import { BackendClient, BackendResp } from ".";

export type Column = {
    id: string,
    name: string,
    belongs_to: string
}

type GetColumnsResp = BackendResp<{
    columns: Array<Column>
}>;


export function getColumns(backend: BackendClient, board_id:string): Promise<Result<Array<Column>, string>> {
    
    return backend.get<GetColumnsResp>(
        "/column", {
            data: {
                board_id
            }
        } 
    ).then(v => Ok(v.data.columns))
    .catch((err: AxiosError) => Err(err.message));
}

export function createColumns(backend: BackendClient, {name, belongs_to}: {name:string, belongs_to: string}): Promise<Result<unknown, string>> {
    return backend.post<BackendResp<{ column: Column }>>(
        "/column",
        {name, belongs_to}
    ).then((v) => Ok(v.data.column))
    .catch((err: AxiosError) => Err(err.message));
}
import { Err, Ok, Result } from "@sniptt/monads/build";
import { AxiosError } from "axios";
import { BackendClient, BackendResp } from ".";

export type Board = {
    id: string,
    name: string,
    belongs_to: string
}

type GetBoardResp = BackendResp<{
    boards: Array<Board>
}>;

export function getBoards(backend: BackendClient): Promise<Result<Array<Board>, string>> {
    return backend.get<GetBoardResp>(
        "/board"
    ).then(v => Ok(v.data.boards))
    .catch((err: AxiosError) => Err(err.message));
}

export function deleteBoard(backend: BackendClient, cid: string): Promise<Result<undefined, string>> {
    return backend.delete<BackendResp<undefined>>(
        "/board", {
            data: {
                cid
            }
        }
    ).then(_ => Ok(undefined))
    .catch((err: AxiosError) => Err(err.message));
}

export function createBoard(backend: BackendClient, name: string): Promise<Result<Board, string>> {
    return backend.post<BackendResp<{ board: Board }>>(
        "/board",
        name
    ).then(v => Ok(v.data.board))
    .catch((err: AxiosError) => Err(err.message));
}
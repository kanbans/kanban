import { createResource, createSignal, Show, useContext } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import BoardsNav from "../components/BoardsNav";
import { requireAuth } from "../helpers";
import { getBoards } from "../repos/board";

function Boards() {
    requireAuth();
    const store = useContext(GlobalContext);
    const backend = useContext(BackendContext);

    const [err, setErr] = createSignal<string | undefined>(undefined);

    const [boards, { mutate: mutateBoard, refetch: refetchBoard }] =
        createResource(
            async () => {
                const res = await getBoards(backend);
                res.mapErr(setErr);
                res.isOk() && store.setBoard(res.unwrap().at(0)!);
                return res.isOk() ? res.unwrap() : [];
            },
            { initialValue: [] }
        );

    return (
        <>
            <div class="flex flex-row h-screen">
                <div class="bg-neutral-800 w-72 px-4 pt-2">
                    <BoardsNav
                        onBoardClick={store.setBoard}
                        boards={boards}
                        mutateBoards={mutateBoard}
                    ></BoardsNav>
                </div>
                <div class="content h-full p-4">
                    <Show when={store.state.board !== undefined}>
                        <h1 class="text-6xl text-white">
                            {store.state.board!.name}
                        </h1>
                    </Show>
                </div>
            </div>
        </>
    );
}

export default Boards;

import { createResource, createSignal, For, useContext } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import { Board, createBoard, getBoards } from "../repos/board";
import { FiPlus, FiChevronRight } from "solid-icons/fi";

function BoardsNav(props: {
    onBoardClick: (b: Board) => void,
}) {
    const store = useContext(GlobalContext);
    const backend = useContext(BackendContext);
    let boardNameRef: HTMLInputElement | undefined;

    const [err, setErr] = createSignal();
    const [boards, { mutate: mutateBoard, refetch: refetchBoard }] = createResource(async () => {
        const res = await getBoards(backend);
        res.mapErr(setErr);
        return res.isOk() ? res.unwrap() : []
    }, { initialValue: [] })

    async function newBoard() {
        const board = await createBoard(backend, boardNameRef!.value);
        board.isOk() && mutateBoard(prev => [...prev, board.unwrap()]);
    }

    return (<>
        <div className="grid grid-cols-2 w-full">
            <h1 class="text-white text-xl font-bold">Boards</h1>
            <input type="checkbox" id="plus-radio" class="hidden peer" />
            <label htmlFor="plus-radio" class="flex items-center justify-end"><FiPlus class="text-white hover:text-neutral-400" size="1.25em"/></label>
            <div class="peer-checked:block hidden w-fit flex items-center relative">
                <input className="bg-neutral-400 rounded-md outline-none text-white px-1" type="text" ref={boardNameRef} />
                <FiChevronRight class="absolute top-0 inset-x-0 right-0 hover:text-sky-300 text-sky-200 mr-1" size="1.2em" onClick={newBoard}></FiChevronRight>
            </div>
        </div>
        <div class="grid grid-cols-1 text-white place-items-start gap-2 mt-2">
            <For each={boards()}>
                {(item) => (
                    <div class="hover:bg-neutral-600 w-full rounded px-2 py-1">
                        <button onClick={() => props.onBoardClick(item)}>{item.name}</button>
                    </div>
                )}
            </For>
        </div>
    </>)
}

export default BoardsNav;
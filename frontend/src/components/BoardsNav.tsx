import { createResource, createSignal, For, useContext } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import { Board, createBoard, getBoards } from "../repos/board";
import { FiPlus, FiCheck } from "solid-icons/fi";

function BoardsNav(props: {
    onBoardClick: (b: Board) => void,
}) {
    const store = useContext(GlobalContext);
    const backend = useContext(BackendContext);
    let boardNameRef: HTMLInputElement | undefined;

    const [err, setErr] = createSignal();
    const [boards, { mutate: mutateBoard, refetch: refetchBoard }] = createResource(async () => {
        const res = await getBoards(backend);
        store.setSelectedBoard(res.unwrap().at(1)!);
        res.mapErr(setErr);
        return res.isOk() ? res.unwrap() : []
    }, { initialValue: [] })

    async function newBoard() {
        if (boardNameRef!.value === "") {
            return;
        }
        const board = await createBoard(backend, boardNameRef!.value);
        board.isOk() && mutateBoard(prev => [...prev, board.unwrap()]);
    }

    return (<>
        <div className="grid grid-cols-2 w-full">

           <h1 class="text-white text-xl font-bold">Boards</h1>
            <input type="checkbox" id="plus-radio" class="hidden peer" />
            <label htmlFor="plus-radio" class="flex items-center justify-end pr-2"><FiPlus class="text-white hover:text-neutral-400" size="1.25em"/></label>
            <div class="hidden peer-checked:flex relative  items-center w-max mt-2">
                <input className="bg-neutral-400 rounded-md outline-none text-white py-1 w-64 px-2" type="text" ref={boardNameRef}  onKeyDown={(e) => {
                    e.key === "Enter" && newBoard().then(() => {boardNameRef!.value = ""});
                }}  />
                <FiCheck class="absolute right-1  hover:text-sky-300 text-sky-200 mr-1" size="1.2em" onClick={newBoard}></FiCheck>
            </div>
        </div>
        <div class="grid grid-cols-1 text-white place-items-start gap-2 mt-2">
            <For each={boards()}>
                {(item) => (
                    <div class="hover:bg-neutral-600 w-full rounded px-2 py-1" onClick={() => {
                        store.setSelectedBoard(item)
                        props.onBoardClick(item)}
                    }>
                        <button >{item.name}</button>
                    </div>
                )}
            </For>
        </div>
    </>)
}

export default BoardsNav;
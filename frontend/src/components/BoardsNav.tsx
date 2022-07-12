import { createSignal, For, Resource, Show, useContext } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import { Board, createBoard, deleteBoard } from "../repos/board";
import { FiPlus, FiCheck, FiDelete, FiTrash } from "solid-icons/fi";

function BoardsNav({
    boards,
    onBoardClick,
    mutateBoards,
    refetchBoards
}: {
    boards: Resource<Board[]>;
    onBoardClick: (b: Board) => void;
    mutateBoards: (f: (prev: Board[]) => Board[]) => void;
    refetchBoards: (info?: unknown) => void;
}) {
    // Use Contexts
    const backend = useContext(BackendContext);
    // Ref for the input
    let boardNameRef: HTMLInputElement | undefined;
    const [err, setErr] = createSignal<string | undefined>(undefined);

    async function newBoard() {
        if (boardNameRef!.value === "") {
            return;
        }
        const board = await createBoard(backend, boardNameRef!.value);
        board.isErr() && setErr(board.unwrapErr());
        board.isOk() &&
            (() => {
                mutateBoards((prev) => {
                    return [...prev, board.unwrap()];
                });
                setErr(undefined);
            })()
    }

    return (
        <>
            <div className="grid grid-cols-2 w-full">
                <h1 class="text-white text-xl font-bold">Boards</h1>
                <input
                    type="checkbox"
                    id="plus-radio"
                    class="hidden peer"
                    onClick={() => {
                        boardNameRef?.focus();
                    }}
                />
                <label
                    htmlFor="plus-radio"
                    class="flex items-center justify-end pr-2"
                >
                    <FiPlus
                        class="text-white hover:text-neutral-400"
                        size="1.25em"
                    />
                </label>
                <div class="hidden peer-checked:flex relative  items-center w-max mt-2">
                    <input
                        autofocus={true}
                        className="bg-neutral-400 rounded-md outline-none text-white py-1 w-64 px-2"
                        type="text"
                        ref={boardNameRef}
                        onKeyDown={(e) => {
                            e.key === "Enter" &&
                                newBoard().then(() => {
                                    boardNameRef!.value = "";
                                });
                        }}
                    />
                    <FiCheck
                        class="absolute right-1  hover:text-sky-300 text-sky-200 mr-1"
                        size="1.2em"
                        onClick={newBoard}
                    ></FiCheck>
                </div>
            </div>
            <div class="grid grid-cols-1 text-white place-items-start gap-2 mt-2">
                <For each={boards()}>
                    {(item) => (
                        <div
                            class="flex hover:bg-neutral-600 w-full rounded px-2 py-1 justify-between"
                            onClick={() => {
                                onBoardClick(item);
                            }}
                        >
                            <button>{item.name}</button>
                            <button onClick={
                                () => {
                                    deleteBoard(backend, item.id).then((res) => {
                                        refetchBoards();
                                    });
                                }
                            }><FiTrash class="hover:text-red-400"/></button>
                        </div>
                    )}
                </For>
            </div>
        </>
    );
}

export default BoardsNav;

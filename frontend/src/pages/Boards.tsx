import { Show, useContext } from "solid-js";
import { GlobalContext } from "..";
import BoardsNav from "../components/BoardsNav";
import { requireAuth } from "../helpers";

function Boards() {
    requireAuth();
    const store = useContext(GlobalContext);

    return (
        <>
            <div class="flex flex-row h-screen">
                <div class="bg-neutral-800 w-72 px-4 pt-2">
                    <BoardsNav
                        onBoardClick={(b) => {
                            console.log(b);
                        }}
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

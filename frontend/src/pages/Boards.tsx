
import BoardsNav from "../components/BoardsNav";
import { requireAuth } from "../helpers";

function Boards() {
    requireAuth();

    return (<>
        <div class="flex flex-row h-screen">
            <div class="bg-neutral-800 w-72 px-4 pt-2">
                <BoardsNav onBoardClick={() => {
                    // TODO
                }}></BoardsNav>
            </div>
            <div class="content h-full"></div>
            <input type="button" value="" />
        </div>
    </>)
}

export default Boards;
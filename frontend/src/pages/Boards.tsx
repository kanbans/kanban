import { requireAuth } from "../helpers";

function Boards() {
    requireAuth();

    return (<>
        <div class="flex flex-col h-screen">
            <div class="bg-neutral-800 py-2">
                <div class="flex flex-row items-center">
                    <p class="text-white ml-4 mr-2 text-lg font-bold">Board</p>
                    <p class="text-zinc-500">board_name</p>
                </div>
            </div>
            <div class="content h-full"></div>
            <input type="button" value="" />
        </div>
    </>)
}

export default Boards;
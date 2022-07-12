import { useContext, Show, createResource, createSignal, For } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import { requireAuth } from "../helpers";
import { createColumns, getColumns } from "../repos/columns";

function BoardView() {
    let boardNameRef: HTMLInputElement | undefined;
    requireAuth();
    const store = useContext(GlobalContext);
    const backend = useContext(BackendContext);

    const [err, setErr] = createSignal<string | undefined>(undefined);

    const [columns, { mutate: mutateColumn, refetch: refetchColumn }] =
        createResource(
            async () => {
                if (store.state.board) {
                    const res = await getColumns(backend, store.state.board.id);
                    res.mapErr(setErr);
                    return res.isOk() ? res.unwrap() : [];
                }
            },
            { initialValue: undefined }
        );

    const createNewColumn = async () => {
        return createColumns(backend, {
            name: boardNameRef!.value,
            belongs_to: store.state.board!.id,
        })
            .then(() => {
                refetchColumn();
                boardNameRef!.value = "";
            })
            .catch((e) => console.log(e));
    };

    return (
        <Show
            when={store.state.board !== undefined}
            fallback={<h1>Please Select a Board.</h1>}
        >
            <h1 class="text-6xl text-white">{store.state.board!.name}</h1>
            <div className="flex">
                <input
                    type="text"
                    ref={boardNameRef}
                    onKeyDown={(e) => e.key === "Enter" && createNewColumn()}
                />
                <For each={columns()!}>
                    {(column) => (
                        <div class="h-full bg-red-900">{column.name}</div>
                    )}
                </For>
            </div>
        </Show>
    );
}

export default BoardView;

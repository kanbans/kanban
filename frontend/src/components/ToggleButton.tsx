import { PropsWithChildren } from "solid-js";

function ChoiceButton(props: { radioId: string, label: string, checked?: boolean, peerStyle: string, onCheck: () => void }) {
    return (<>
        <div>
            <input type="radio" name="choice" id={props.radioId} className="hidden peer" checked={props.checked ? true : false}
                onChange={(e) => e.currentTarget.checked && props.onCheck()} />
            <div class={`${props.peerStyle} w-full m-0 flex`}>
                <label htmlFor={props.radioId} className="text-white w-full text-center h-full py-3 text-lg">{props.label}</label>
            </div>
        </div>
    </>);
}

function ToggleButton(props: PropsWithChildren<{
    choices: [string, string],
    onCheck: (choice: 0 | 1) => void
}>) {
    return (<>
        <form class="border-solid border-zinc-400 border-2 rounded-md grid grid-cols-2">
            <ChoiceButton radioId="c1" label={props.choices[0]} checked peerStyle="peer-checked:bg-sky-600" onCheck={() => props.onCheck(0)}></ChoiceButton>
            <ChoiceButton radioId="c2" label={props.choices[1]} peerStyle="peer-checked:bg-emerald-600" onCheck={() => props.onCheck(1)}></ChoiceButton>
        </form>
    </>);
}

export default ToggleButton;
import { Component, For } from "solid-js";

const ButtonGroup: Component<{
    buttons: {
        text: string,
        onClick: () => void
    }[]
}> = (props) => {
    return (<div class="rounded-md shadow-sm">
        <For each={props.buttons}>{({text, onClick}) => <button {...onClick}>{text}</button>}</For>
    </div>)
}

export default ButtonGroup;
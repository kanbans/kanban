import { Component, Show } from "solid-js";

const baseInputStyle = {
    border: true,
    "indent-4": true,
    "rounded-md": true,
    "bg-inherit": true,
    "py-4": true,
    "row-span-3": true,
    "focus:outline": true,
    "focus:outline-0": true
}

const inputStyle = (err: unknown | null | undefined) => {
    const isValid = err === null || err === undefined;
    return {
        ...baseInputStyle,
        "border-zinc-700": isValid,
        "border-red-700": !isValid,
        "focus:border-white": isValid,
        "focus:border-red-500": !isValid
    };
};

const InputField: Component<{
    name: string,
    placeholder: string,
    type: string,
    error: unknown | null | undefined,
    invalidMsg: string
}> = (props) => (
    <>
        <input
            classList={inputStyle(props.error)}
            name={props.name}
            type={props.type}
            placeholder={props.placeholder}
            
        />
        <Show when={props.error}>
            <span class="text-red-600 text-sm">{props.invalidMsg}</span>
        </Show>
    </>
);

export default InputField;
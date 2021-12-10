import { Component } from "solid-js";

const AuthButton: Component<{
    disabled: boolean,
    value: string
}> = (props) => (
    <input
        disabled={props.disabled}
        class="border rounded-md border-zinc-700 font-bold py-3 row-span-2 hover:bg-zinc-900 disabled:text-zinc-700"
        type="submit"
        value={props.value}
    />
);

export default AuthButton;
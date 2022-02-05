import { createSignal } from "solid-js";
import LoginForm from "../components/LoginForm";
import RegForm from "../components/RegForm";
import ToggleButton from "../components/ToggleButton";


export function Auth() {
    const [getForm, setForm] = createSignal(LoginForm)

    return (
        <div class="flex h-screen justify-center items-center">
            <div class="w-7/12">
                <h1 class="text-slate-50 text-center text-4xl font-bold my-10">
                    Welcome Back!
                </h1>
                <div className="flex flex-col gap-y-6">
                    <div>
                        <ToggleButton choices={["Login", "Register"]} onCheck={(c) => c ? setForm(() => RegForm) :  setForm(() => LoginForm)}></ToggleButton>
                    </div>
                    <div>
                        {getForm()}
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Auth;

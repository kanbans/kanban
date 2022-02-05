import { createSignal, lazy, Switch, Match } from "solid-js";
import ToggleButton from "../components/ToggleButton";


const LoginForm = lazy(() => import("../components/LoginForm"));
const RegForm = lazy(() => import("../components/RegForm"));

export function Auth() {
    const [getSelected, setSelected] = createSignal(0)

    return (
        <div class="flex h-screen justify-center items-center">
            <div class="w-7/12">
                <h1 class="text-slate-50 text-center text-4xl font-bold my-10">
                    Welcome Back!
                </h1>
                <div className="flex flex-col gap-y-6">
                    <div>
                        <ToggleButton choices={["Login", "Register"]} onCheck={setSelected}></ToggleButton>
                    </div>
                    <div>
                    <Switch fallback={<LoginForm/>}>
                        <Match when={getSelected() === 0}><LoginForm/></Match>
                        <Match when={getSelected() === 1}><RegForm/></Match>
                    </Switch>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default Auth;

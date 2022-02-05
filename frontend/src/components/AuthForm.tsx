import { None, Option, Result, Some } from "@sniptt/monads/build";
import { Component, ComponentProps, createSignal, For, Show, useContext } from "solid-js";
import { BackendContext, GlobalContext } from "..";
import * as zod from "zod";
import { createForm } from "@felte/solid";
import InputField from "./InputField";
import AuthButton from "./AuthButton";
import { validator, ValidatorConfig } from "@felte/validator-zod";
import { AuthVaraint } from "../types/auth";
import { BackendClient } from "../repos";
import { ZodKey, ZodValues } from "../types/zodutils";
import { useLocation, useNavigate } from "solid-app-router";

export type InputFieldInfo<T extends zod.ZodRawShape> = {
    name: ZodKey<T> & string,
    placeholder: string,
    type: string,
    error: ZodKey<T> & string,
    invalidMsg: string
} 

type LoginFormComp<T extends zod.ZodRawShape> = Component<{
    schema: zod.ZodObject<T>,
    authWith: (backend: BackendClient, values: ZodValues<T>) => Promise<Result<string, string>>,
    inputFields: InputFieldInfo<T>[],
    loginFailure: string,
    authButton: string
}>;

function AuthForm<T extends zod.ZodRawShape>(props: ComponentProps<LoginFormComp<T>>) {
    const backendClient = useContext(BackendContext);
    const store = useContext(GlobalContext);
    const [authErr, setAuthErr] = createSignal<Option<string>>(None);
    const location = useLocation<{ redirect: string }>();
    const navigator = useNavigate();

    const { form, isValid, errors } = createForm<ZodValues<T>, ValidatorConfig>(
        {
            extend: validator,
            validateSchema: props.schema,
            onSubmit: async (values: ZodValues<T>) => {
                setAuthErr(None);

                const res = await props.authWith(
                    backendClient,
                    values
                );

                res.map(token => {
                    store.setAuth({ variant: AuthVaraint.Login, data: token });
                    const redir = location.state?.redirect || "/";
                    navigator(redir);
                });
                res.mapErr(err => setAuthErr(Some(err)));
            },
        }
    );

    return (<>
        <Show when={authErr().isSome()}>
            <p className="text-red-600 text-sm my-6 ml-1">
                {`${props.loginFailure}: ${authErr().unwrap()}`}
            </p>
        </Show>
        <form
            ref={form}
            class="text-xl text-slate-50 placeholder:text-slate-300 w-full grid grid-rows-8 gap-y-4"
        >
            <For each={props.inputFields}>
                {item => <InputField
                    placeholder={item.placeholder}
                    name={item.name}
                    type={item.type}
                    error={errors[item.error]}
                    invalidMsg={item.invalidMsg}
                />}
            </For>
            <AuthButton disabled={!isValid()} value={props.authButton} />
        </form>
    </>);
}

export default AuthForm;
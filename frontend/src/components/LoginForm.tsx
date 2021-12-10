import AuthForm, { InputFieldInfo } from "./AuthForm";
import * as zod from "zod";
import { BackendClient } from "../repos";
import { authClient } from "../repos/user";
import { ZodShape } from "../types/zodutils";

const loginSchema = zod.object({
    email: zod.string().email().nonempty(),
    password: zod.string().nonempty(),
});

function loginAuth(backend: BackendClient, values: { email: string, password: string }) {
    return authClient(
        backend,
        values.email,
        values.password
    )
}

const inputFields: InputFieldInfo<ZodShape<typeof loginSchema>>[] = [
    {
        name: "email",
        placeholder: "Email",
        type: "email",
        error: "email",
        invalidMsg: "Invalid Email"
    },
    {
        name: "password",
        placeholder: "Password",
        type: "password",
        error: "password",
        invalidMsg: "Invalid Password"
    }
]
 
const LoginForm = () => (
    <AuthForm
        schema={loginSchema}
        authWith={loginAuth}
        inputFields={inputFields}
        loginFailure="Failed to Login"
        authButton="LOGIN"
    />
)

export default LoginForm;
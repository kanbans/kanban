import AuthForm, { InputFieldInfo } from "./AuthForm";
import * as zod from "zod";
import { BackendClient } from "../repos";
import { authClientReg } from "../repos/user";
import { ZodShape } from "../types/zodutils";

const regSchema = zod.object({
    name: zod.string().nonempty(),
    email: zod.string().email().nonempty(),
    password: zod.string().nonempty(),
});

function regAuth(backend: BackendClient, values: { email: string, password: string, name: string }) {
    return authClientReg(
        backend,
        values.name,
        values.email,
        values.password
    )
}

const inputFields: InputFieldInfo<ZodShape<typeof regSchema>>[] = [
    {
        name: "name",
        placeholder: "Name",
        type: "text",
        error: "name",
        invalidMsg: "Invalid Name"
    },
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
 
const RegForm = () => (
    <AuthForm
        schema={regSchema}
        authWith={regAuth}
        inputFields={inputFields}
        loginFailure="Failed to Register"
        authButton="REGISTER"
    />
)

export default RegForm;
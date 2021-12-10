import LoginForm from "../components/LoginForm";


export function Login() {

    return (
        <div class="flex h-screen justify-center items-center">
            <div class="w-7/12">
                <h1 class="text-slate-50 text-center text-4xl font-bold my-10">
                    Welcome Back!
                </h1>
                <LoginForm/>
            </div>
        </div>
    );
}

export default Login;

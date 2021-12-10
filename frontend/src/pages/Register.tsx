import RegForm from "../components/RegForm";

const Register = () => (
    <div class="flex h-screen justify-center items-center">
        <div class="w-7/12">
            <h1 class="text-slate-50 text-center text-4xl font-bold my-10">
                Welcome!
            </h1>
            <RegForm/>
        </div>
    </div>
);

export default Register;
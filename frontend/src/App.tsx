import { Component, lazy } from "solid-js";
import { Routes, Route } from "solid-app-router";

const Login = lazy(() => import("./pages/Login"));
const Register = lazy(() => import("./pages/Register")); // TODO: Merge with Login

const App: Component = () => (
  <div class="bg-black">
    <Routes>
      <Route path="/login" element={<Login/>} />
      <Route path="/register" element={<Register/>} />
    </Routes>
  </div>
)

export default App;

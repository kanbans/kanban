import { Component, lazy } from "solid-js";
import { Routes, Route } from "solid-app-router";

const Auth = lazy(() => import("./pages/Auth"));
const Boards = lazy(() => import("./pages/Boards"));
const Home = lazy(() => import("./pages/Home"));

const App: Component = () => (
  <div class="bg-neutral-900">
    <Routes>
      <Route path="/" element={<Home/>}/>
      <Route path="/auth" element={<Auth/>} />
      <Route path="/boards" element={<Boards/>}></Route>
    </Routes>
  </div>
)

export default App;

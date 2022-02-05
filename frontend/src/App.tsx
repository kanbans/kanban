import { Component, lazy } from "solid-js";
import { Routes, Route } from "solid-app-router";

const Auth = lazy(() => import("./pages/Auth"));

const App: Component = () => (
  <div class="bg-neutral-900">
    <Routes>
      <Route path="/auth" element={<Auth/>} />
    </Routes>
  </div>
)

export default App;

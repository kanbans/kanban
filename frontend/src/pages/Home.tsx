import { Component } from "solid-js";

const App: Component = () => (
  <div class="bg-neutral-900 flex flex-col w-full h-screen items-center justify-center">
    <h1 class="text-white text-6xl">Welcome to Kanbans</h1>
    <a href="/auth" class="text-white text-2xl underline m-4">Login</a>
  </div>
)

export default App;

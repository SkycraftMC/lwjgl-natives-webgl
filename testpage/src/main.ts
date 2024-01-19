import { set_panic_hook } from "lwjgl";
import App from "./App.svelte";

set_panic_hook();
const app = new App({
  target: document.body,
});

export default app;

import { skipSvelteClass } from "./utils/skip.js";
import {  Engine } from "@veemcss/engine";
export function VeemCSSPlugin() {
  let generatedCSS = "";
  return {
    name: "vite-plugin-atomic-css",
    transform: (code, id) => {
      if (id.includes("node_modules")) return;
      if (id.endsWith(".svelte")){

        generatedCSS = Engine.fromString(code);
      } else if (id.endsWith("+page.svelte?svelte&type=style&lang.css")) {
        console.log(`gene ${generatedCSS}\n code  : ${code}`);
        return {
          code: `${code}\n${generatedCSS}`,
          map: null,
        };
      }
    },
  };
}


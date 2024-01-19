<script lang="ts">
  import * as wasm from "lwjgl";
  const isWebGL2Supported = () =>
    !!document.createElement("canvas").getContext("webgl2");
</script>

<main class="container">
  <h1>LWJGL Natives Tester</h1>

  <ul>
    <li>
      Pointer Size: <code
        >{wasm.Java_org_lwjgl_DefaultSysImplementation_getPointerSize()}</code
      >
    </li>
  </ul>

  {#if !isWebGL2Supported()}
    <p class="callout callout-warning">
      WebGL 2 is not supported on this browse (or hardware acceleration for some
      reason). <b>The compatibility layer will not work.</b>
    </p>
  {/if}
</main>

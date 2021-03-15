<script>
 import "@design/theme"
 import InputDirectory from "@design/Primitive/Input/Directory.svelte";
 import Theme from "./Theme.svelte";
</script>

<style>
 /*.orphan-frame {
     display: flex;
 }*/

 main {
     display: flex;
     width: 100vw;
     justify-content: center;
     align-items: center;
     font-size: 2rem;
 }
</style>

<Theme />
<main>
    <InputDirectory />
</main>

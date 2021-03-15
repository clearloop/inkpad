<script>
 import { Theme } from "./src/settings.ts";

 $: {
     document.documentElement.setAttribute("data-theme", Theme.Dark);
 }
</script>

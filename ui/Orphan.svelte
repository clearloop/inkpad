<!--
     This Orphan.svelte is the welcome page for the first init.
-->
<script lang="ts">
 import Button from "@design/Primitive/Button.svelte";
 import InputDirectory from "@design/Primitive/Input/Directory.svelte";
 import InputText from "@design/Primitive/Input/Text.svelte";
 import RadioOption from "@design/Component/RadioOption.svelte";

 $: select = false;

 const selectContract = () => select = !select;
</script>
<style>
 main {
     display: flex;
     width: 100vw;
     justify-content: center;
     align-items: center;
     text-align: center;
     padding: 2rem;
 }

 .btn-container {
     display: flex;
     justify-content: center;
     margin-top: 1rem;
 }
</style>

<main>
    <div style="width: 30rem;">
        <RadioOption
            active={select}
                   title="Select your ink! contract"
            on:click={ev => {
                     ev.stopPropagation();
                     selectContract()
                     }}
        >
            <div slot="option-body">
                <InputDirectory placeholder="*.contract" />
                <InputText placeholder="Contract name" style="margin-top: 1rem;" />
                <div class="btn-container">
                    <Button disabled={true}>
                        Create Contract
                    </Button>
                </div>
            </div>
        </RadioOption>
    </div>
</main>

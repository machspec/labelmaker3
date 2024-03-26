<script lang="ts">
    import GenericLine from "./GenericLine.svelte";
    import { onMount } from "svelte";
    import { clearForm } from "../formHelpers";

    export let active: boolean = false;

    let rowCount = 0;
    let form: HTMLFormElement;

    const addRow = () => rowCount++;
    const updateDataStore = () => {};

    // Generate five rows on mount.
    onMount(() => [...Array(5)].map(() => addRow()));
</script>

<span class="container {active ? 'active' : ''}">
    <h1>Generic Label</h1>
    <form bind:this={form} on:input={updateDataStore}>
        <span class="headers">
            <p>Field</p>
            <p>Value</p>
        </span>

        {#each [...Array(rowCount)] as _}
            <GenericLine />
        {/each}

        <div class="form-options">
            <button on:click|preventDefault={addRow}>&plus; Row</button>
            <button class="clear" on:click={() => clearForm(form)}>
                Clear
            </button>
        </div>
    </form>
</span>

<!--
    Ignore selectors we don't use in this component.
    svelte-ignore css-unused-selector
-->
<style>
    @import "../form.css";

    form {
        display: grid;
        grid-template-columns: 10ch 1fr auto auto;
        gap: 0.5rem;
        padding: 0 10%;
        width: 100%;
        max-width: 900px;
    }

    .add-field {
        grid-column: 1 / -1;
        justify-self: center;
        width: 2rem;
        height: 2rem;
        font-size: 1.5rem;
        font-weight: bold;
    }

    .headers {
        display: grid;
        grid-template-columns: inherit;
        grid-column: 1 / -1;
        text-align: center;
        border-bottom: var(--bd);
    }
</style>

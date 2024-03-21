<script lang="ts">
    import { clearForm } from "../formHelpers";

    export let active: boolean = false;

    let fieldCount = 1;
    let form: HTMLFormElement;

    const updateDataStore = () => {};

    $: rows = new Array(fieldCount);
</script>

<span class="container {active ? 'active' : ''}">
    <h1>Generic Label</h1>
    <form bind:this={form} on:input={updateDataStore}>
        <span class="headers">
            <p>Field</p>
            <p>Value</p>
        </span>
        {#each rows as rowIndex}
            <input name="row-{rowIndex}" />
            <input
                required
                type="text"
                id="row-{rowIndex}"
                name="row-{rowIndex}"
            />
        {/each}
        <button class="add-field">&plus;</button>
        <div class="form-options">
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
        grid-template-columns: 8ch 1fr;
        gap: 0.5rem;
        padding: 0 10%;
        width: 100%;
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

    form > input {
        min-width: 8ch !important;
    }
</style>

<script lang="ts">
    import { onMount } from "svelte";
    import {
        checkValidity,
        formDataStore,
        formValidity,
        loading,
    } from "../stores";
    import GenericLine from "./GenericLine.svelte";
    import { clearForm } from "../formHelpers";

    export let active: boolean = false;

    let form: HTMLFormElement;
    let lines: GenericLine[] = [];
    let rowCount = 0;

    const addRow = () => rowCount++;

    const clearFields = () => {
        clearForm(form);
        lines.forEach((line) => line.clearFields());
    };

    export const updateDataStore = () => {
        let data: Record<string, string>[] = [];
        let containers = form.querySelectorAll(".container");

        containers.forEach((container) => {
            let fields = container.querySelectorAll("input");
            let row: Record<string, string> = {};

            // Sort names and values into separate arrays.
            const names: Record<string, string> = {};
            const values: Record<string, string> = {};
            fields.forEach((field) => {
                const [_, index, input] = field.name.split("-");
                (input === "name" ? names : values)[index] = field.value;
            });

            // TODO: Check empty input.
            // Combine the names and values into a single object.
            Object.keys(names).forEach((key) => {
                row[names[key]] = values[key];
            });

            data.push(row);
        });

        formDataStore.update(() => {
            formValidity.set(true);
            return { rows: data };
        });
    };

    onMount(() => addRow());

    $: if (active && $checkValidity) {
        if (form.checkValidity()) {
            formValidity.set(true);
            loading.set(true);
        } else {
            formValidity.set(false);
        }

        form.reportValidity();
        checkValidity.set(false);
    }
</script>

<span class="container {active ? 'active' : ''}">
    <h1>Generic Label</h1>
    <form bind:this={form}>
        <span class="headers">
            <p>Field</p>
            <p>Value</p>
        </span>

        {#each [...Array(rowCount)] as _, rowNumber}
            <p class="row-num">Row {rowNumber + 1}</p>
            <GenericLine bind:this={lines[rowNumber]} />
        {/each}

        <div class="form-options">
            <button on:click|preventDefault={addRow}>&plus; Row</button>
            <button class="clear" on:click={clearFields}> Clear </button>
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

    .row-num {
        grid-column: 1/-1;
        font-size: 1rem;
    }
</style>

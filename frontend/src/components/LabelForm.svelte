<script lang="ts">
    import { checkValidity, formDataStore, formValidity } from "../stores";

    export let format: { [key: string]: any };
    export let active: boolean = false;

    let form: HTMLFormElement;

    const updateDataStore = () => {
        if (!active) return;

        formDataStore.update(() => {
            return { ...Object.fromEntries(new FormData(form)) };
        });
    };

    $: if (active && $checkValidity) {
        if (form.checkValidity()) formValidity.set(true);
        else formValidity.set(false);

        form.reportValidity();
        checkValidity.set(false);
    }
</script>

<span class="container {active ? 'active' : ''}">
    <h1>Label Fields</h1>
    <form bind:this={form} on:input={updateDataStore}>
        {#each format.rows as row}
            {#each row as field}
                <label for="field">{field}</label>
                <input required type="text" id={field} name={field} />
            {/each}
        {/each}
    </form>
</span>

<style>
    .container {
        display: none;
    }

    .active {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    form {
        display: grid;
        grid-template-columns: auto 1fr;
        align-items: center;
        gap: 1rem;

        margin: 0 auto;
        width: 50%;
        font-size: 1.5rem;
        border: none;
    }

    form > input {
        max-width: 600px;
        min-width: 300px;
        width: 100%;
    }

    form > label {
        padding-left: 1rem;
        text-align: right;
        white-space: nowrap;
    }

    h1 {
        width: 100%;
        font-size: 1.5rem;
        text-align: center;
    }
</style>

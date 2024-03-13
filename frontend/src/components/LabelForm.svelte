<script lang="ts">
    import {
        checkValidity,
        formDataStore,
        formValidity,
        loading,
    } from "../stores";

    export let format: { [key: string]: any };
    export let active: boolean = false;

    let form: HTMLFormElement;
    let rows: object[] = new Array(format.length);

    const formObj = () => Object.fromEntries(new FormData(form));

    const updateDataStore = () => {
        if (!active) return;

        // Group form data by row defined in format
        let data = formObj();
        format.rows.map((row: string[], index: number) => {
            row.forEach((field: string) => {
                rows[index] = { ...rows[index], [field]: data[field] };
            });
        });

        formDataStore.update(() => {
            return { rows: rows };
        });
    };

    $: active && updateDataStore();
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
    <h1>Label Fields</h1>
    <form bind:this={form} on:input={updateDataStore}>
        {#each format.rows as row, index}
            {#each row as field}
                <label for={field}>{field}</label>
                <input
                    required
                    type="text"
                    id={field}
                    name={field}
                    data-row-index={index}
                />
            {/each}
        {/each}
        <div class="form-options">
            <button class="clear" on:click={() => form.reset()}>Clear</button>
        </div>
    </form>
</span>

<style>
    .container {
        position: relative;
        display: none;
    }

    .active {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;
    }

    .form-options {
        display: flex;
        justify-content: flex-end;
        grid-column: 1/-1;
        width: 100%;
    }

    .form-options > button {
        padding: 0.25rem 2rem;
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

<script lang="ts">
    import {
        checkValidity,
        formDataStore,
        formValidity,
        loading,
    } from "../stores";
    import { formObj, clearForm } from "../formHelpers";

    export let format: { [key: string]: any };
    export let active: boolean = false;

    let form: HTMLFormElement;
    let rows: object[] = new Array(format.length);

    export const updateDataStore = () => {
        if (!active) return;

        // Group form data by row defined in format
        let data = formObj(form);
        format.rows.map((row: string[], index: number) => {
            row.forEach((field: string) => {
                rows[index] = { ...rows[index], [field]: data[field] };
            });
        });

        formDataStore.update(() => {
            return { rows: rows };
        });
    };

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
    <form bind:this={form}>
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
            <button class="clear" on:click={() => clearForm(form)}>
                Clear
            </button>
        </div>
    </form>
</span>

<style>
    @import "../form.css";
</style>

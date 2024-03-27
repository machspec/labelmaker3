<script lang="ts">
    export const clearFields = () => {
        Object.entries(fields).forEach(([key, value]) => {
            let emptyFields: Array<{ [key: string]: string }> = [];
            fields.forEach((_) => emptyFields.push({ "": "" }));
            fields = emptyFields;
        });
    };

    export let fields: Array<{ [key: string]: string }> = [{ "": "" }];

    let container: HTMLSpanElement;

    /** Remove the line and all its fields. */
    const deleteLine = () => container.parentNode!.removeChild(container);

    /** Run the given function and update the list of fields. */
    const update = (fn: Function, ...args: any[]) => {
        fn(...args);
        fields = [...fields];
    };

    /** Add a new field to the line. */
    const add = (index: number) => fields.splice(index + 1, 0, { "": "" });

    /** Update the name of a field. */
    const name = (index: number, e: Event) => {
        const target = e.target as HTMLInputElement;
        fields[index] = { [target.value]: Object.values(fields[index])[0] };
    };

    /** Update the value of a field. */
    const value = (index: number, e: Event) => {
        const target = e.target as HTMLInputElement;
        fields[index] = {
            ...fields[index],
            [Object.keys(fields[index])[0]]: target.value,
        };
    };

    /** Remove a field from the line. */
    const remove = (index: number) => fields.splice(index, 1);
</script>

<span class="container" bind:this={container}>
    {#each fields as _, index}
        <input
            name="field-{index}-name"
            on:input={(e) => update(name, index, e)}
            value={Object.keys({ ...fields[index] })[0]}
        />
        <input
            required
            name="field-{index}-value"
            on:input={(e) => update(value, index, e)}
            value={Object.values(fields[index])}
        />

        {#if fields.length === 1}
            <button
                on:click|preventDefault={deleteLine}
                class="delete"
                title="Delete row"
            >
                &times;
            </button>
        {:else}
            <button
                on:click|preventDefault={() => update(remove, index)}
                disabled={Object.keys({ ...fields }).length === 1}
                value={fields[index].toString()}
                title="Delete field"
            >
                &times;
            </button>
        {/if}

        <button on:click|preventDefault={() => update(add, index)}>
            &plus;
        </button>
    {/each}
</span>

<style>
    .container {
        position: relative;
        grid-column: 1 / -1;
        display: grid;
        grid-template-columns: inherit;
        gap: 0.5rem;
        width: 100%;
    }

    .container::after {
        content: "";
        display: block;
        grid-column: 1 / -1;
        border-bottom: var(--bd);
    }

    .delete {
        font-weight: bold;
    }
</style>

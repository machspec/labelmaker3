<script lang="ts">
    import SerialNumberInput from "./SerialNumberInput.svelte";
    export let labelFormat: { [key: string]: any };
</script>

<div>
    <form action="">
        <fieldset>
            <legend>{labelFormat.company}</legend>

            {#each labelFormat.fields as field}
                <label for="field">{field}:</label>
                <input type="text" id="field" name="field" />
            {/each}

            <button type="submit">Print</button>
        </fieldset>

        <SerialNumberInput />
    </form>
</div>

<style>
    fieldset {
        display: grid;
        grid-template-columns: auto 1fr;
        gap: 0.5rem;

        padding: 1rem;
        width: 300px;
    }

    fieldset > button {
        grid-column: 1/-1;
        margin-top: 0.5rem;
    }

    fieldset > label {
        text-align: right;
        white-space: nowrap;
    }

    fieldset > legend {
        margin: 0 auto;
        padding: 0 0.5rem;
        font-size: 1.5rem;
    }

    form {
        display: grid;
        grid-template-columns: 300px 300px;
    }

    :global(.sn-input) {
        margin-top: 0.5rem;
    }
</style>

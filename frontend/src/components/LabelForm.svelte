<script lang="ts">
    export let labelFormat: { [key: string]: any };
</script>

<h1>{labelFormat.company}</h1>

<form action="">
    <fieldset>
        <legend>Label Fields</legend>
        {#each labelFormat.fields as field}
            <label for="field">{field}</label>
            <input type="text" id="field" name="field" />
        {/each}
    </fieldset>
</form>

<hr />

<style>
    :root {
        --bd: solid 1px var(--text-muted, #fff8);
    }

    form {
        display: flex;
        flex-direction: column;
        width: 100%;
    }

    fieldset {
        display: grid;
        grid-template-columns: auto 1fr;
        align-items: center;
        gap: 1rem;

        font-size: 1.5rem;
        border: none;
    }

    fieldset > label {
        padding-left: 1rem;
        text-align: right;
        white-space: nowrap;
    }

    fieldset > legend {
        margin: 0 auto 1rem;
        padding: 0 0.5rem;
        font-size: 1.5rem;
    }

    h1 {
        padding-bottom: 0.75rem;
        width: 100%;
        text-align: center;
        border-bottom: var(--bd);
    }

    hr {
        display: block;
        width: 100%;
        border-top: var(--bd);
        border-bottom: none;
    }
</style>

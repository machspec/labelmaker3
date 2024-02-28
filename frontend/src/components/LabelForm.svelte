<script lang="ts">
    export let labelFormat: { [key: string]: any };
</script>

<span class="container">
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
</span>

<style>
    .container {
        display: flex;
        flex-direction: column;
        gap: 1rem;
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

        margin: 0 auto;
        width: 50%;
        font-size: 1.5rem;
        border: none;
    }

    fieldset > input {
        max-width: 600px;
        min-width: 300px;
        width: 100%;
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
</style>

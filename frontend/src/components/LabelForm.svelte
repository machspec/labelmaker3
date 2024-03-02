<script lang="ts">
    export let format: { [key: string]: any };
    export let active: boolean = false;

    let values: any[] = [];

    function print() {
        console.log(values);

        const params = new URLSearchParams();

        for (let i = 0; i < format.fields.length; i++) {
            params.append(format.fields[i].toUpperCase(), values[i]);
        }

        fetch("api/print_label", {
            method: "POST",
            body: params.toString(),
        })
            .then((response) => response.text())
            .then((text) => console.log(text));
    }

    function setupValues() {
        values = Array(format.fields.length).fill("");
    }

    $: {
        if (format.fields.length > 0) {
            setupValues();
        }
    }
</script>

<span class="container {active ? 'active' : ''}">
    <h1>Label Fields</h1>
    <form action="">
        {#each format.fields as field, i}
            <label for="field">{field}</label>
            <input type="text" id="field" name="field" bind:value={values[i]} />
        {/each}
        <button type="button" on:click={print}>Print</button>
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

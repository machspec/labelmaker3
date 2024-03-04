<script lang="ts">
    let printDuplicates: boolean = false;
    let specifyQty: boolean = false;
    let duplicateAmount: number = 0;
    let specifiedQty: number = 1;

    // TODO
    const handleSubmit = (e: Event) => {
        console.log("Printing Options Submitted");
    };
</script>

<div class="container">
    <form action="" on:submit|preventDefault={handleSubmit}>
        <span>
            <input
                name="print-duplicates"
                type="checkbox"
                bind:checked={printDuplicates}
            />
            <label for="print-duplicates">Print Duplicates</label>

            {#if printDuplicates}
                <label for="duplicate-amount">&lpar;Amount&rpar;</label>
                <input
                    type="number"
                    id="duplicate-amount"
                    name="duplicate-amount"
                    title="Print {duplicateAmount} duplicate(s) of each label"
                    min="0"
                    bind:value={duplicateAmount}
                />
            {/if}
        </span>

        <span>
            <input
                type="checkbox"
                name="specify-qty"
                id="specify-qty"
                bind:checked={specifyQty}
            />
            <label for="qty">Specify QTY</label>

            {#if specifyQty}
                <input
                    type="number"
                    id="qty"
                    name="qty"
                    title="Adds &quot;QTY: {specifiedQty}&quot; to the label"
                    min="1"
                    bind:value={specifiedQty}
                />
            {/if}
        </span>

        <button type="submit">Print</button>
    </form>
</div>

<style>
    .container {
        display: grid;
        align-items: center;
        padding: 0 2rem;
        background-color: var(--accent);
    }

    button[type="submit"] {
        grid-column: -1;
        padding: 0.5rem 2rem;
        background-color: var(--background);
    }
    button[type="submit"]:hover {
        background-color: var(--accent-hover);
    }

    form {
        display: grid;
        grid-template-columns: auto auto 1fr auto;
        gap: 2rem;
        align-items: center;
    }

    input:not([type="checkbox"]) {
        width: 3rem;
        font-size: 1rem;
    }

    label {
        margin: 0 0.5rem;
    }
</style>

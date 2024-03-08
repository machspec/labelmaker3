<script lang="ts">
    import {
        checkValidity,
        formDataStore,
        formValidity,
        serialNumberList,
    } from "../stores";

    let printMultiple: boolean = false;
    let specifyQty: boolean = false;
    let amountPerLabel: number = 1;
    let specifiedQty: number = 1;

    const handleSubmit = async () => {
        checkValidity.set(true);

        // Wait for form validation
        await new Promise((res) => setTimeout(res, 0));
        if (!$formValidity) {
            console.error("Form is invalid.");
            return;
        }

        formDataStore.update((data) => {
            return {
                ...data,
                printMultiple: printMultiple,
                specifyQty: specifyQty,
                amountPerLabel: amountPerLabel,
                specifiedQty: specifiedQty,
                serialNumberList: $serialNumberList,
            };
        });

        // Fetch API Request to generate Labels
        try {
            const response = await fetch("api/print_label", {
                method: "POST",
                body: JSON.stringify($formDataStore),
            });

            if (!response.ok) {
                throw new Error(`HTTP error: ${response.status}`);
            }

            const blob = await response.blob();
            const pdfBlob = new Blob([blob], { type: "application/pdf" }); // Specify MIME type
            const url = URL.createObjectURL(pdfBlob); // Create blob URL
            window.open(url, "_blank"); // Open in a new tab
        } catch (error) {
            console.error("Error fetching file:", error);
        }
    };
</script>

<div class="container">
    <form action="" on:submit|preventDefault={handleSubmit}>
        <fieldset>
            <span>
                <input
                    name="print-multiple"
                    type="checkbox"
                    bind:checked={printMultiple}
                />
                <label for="print-multiple">Print Multiple</label>
                {#if printMultiple}
                    <label for="amount-per-label">&lpar;Amount&rpar;</label>
                    <input
                        type="number"
                        id="amount-per-label"
                        name="amount-per-label"
                        title="Print {amountPerLabel} of each label"
                        min="1"
                        bind:value={amountPerLabel}
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
        </fieldset>

        <button type="submit">Print</button>
    </form>
</div>

<style>
    .container {
        position: fixed;
        bottom: 0;
        width: calc(100vw - var(--selector-width));

        display: grid;
        align-items: center;
        padding: 0.5rem;
        padding-right: 1rem;
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

    fieldset {
        border: none;
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 1rem;
    }

    form {
        display: grid;
        grid-template-columns: 1fr auto;
        gap: 2rem;
        align-items: center;
    }

    span {
        display: flex;
        flex-wrap: nowrap;
        align-items: center;
    }

    input:not([type="checkbox"]) {
        width: 3rem;
        font-size: 1rem;
    }

    label {
        margin: 0 0.5rem;
        white-space: nowrap;
    }
</style>

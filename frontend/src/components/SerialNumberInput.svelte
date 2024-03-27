<script lang="ts">
    import { serialNumberList } from "../stores";

    let serialNumberDisplay: HTMLUListElement;
    let snEditor: HTMLInputElement;
    let editIndex: number | null = null;
    let serialNumber = "";

    const lastSerialNumber = () => $serialNumberList.at(-1);

    const scrollDisplay = () => {
        // Delay the scroll to the end of the list to ensure
        // that the width of the element has been updated.
        setTimeout(
            () =>
                (serialNumberDisplay.scrollLeft =
                    serialNumberDisplay.scrollWidth),
            0,
        );
    };

    /**
     * Add a serial number to the list.
     * @param input Value to be added to the list.
     * @param clear Whether to clear the input after adding the value.
     */
    const addSN = (input: string = serialNumber, clear = false) => {
        if (input === "") return;

        // Prevent duplicates
        if ($serialNumberList.includes(input)) return;

        serialNumberList.update((list) => [...list, input]);

        scrollDisplay();

        if (clear) serialNumber = "";
    };

    /**
     * Clear the list of serial numbers.
     */
    const clearSerialNumbers = () => {
        if ($serialNumberList.length === 0) return;
        if (confirm("Remove all serial numbers?")) serialNumberList.set([]);
    };

    /**
     * Edit a serial number given its index.
     * @param index Index of the serial number to be updated.
     * @param value New value to be assigned to the serial number.
     */
    const editSN = (index: number, value: string | null) => {
        if (value === null) return;

        // Prevent duplicates
        if ($serialNumberList.includes(value)) return;

        serialNumberList.update((list) => {
            list[index] = value;
            return list;
        });

        // End the editing operation
        editIndex = null;
    };

    /**
     * Increment the last serial number in the list.
     * @param input The last serial number in the list.
     */
    const incrementSN = (input: string | undefined = lastSerialNumber()) => {
        // Prevent incrementing an empty list
        if (input === "" || input === undefined) return;

        // Matches the following formats (prefix is optional):
        // 1. "ABC123" => ["ABC", "123"]
        // 2. "ABC-123" => ["ABC-", "123"]
        const match = input.match(/(.*[^\d])(\d+$)/);

        if (!match) return;

        const [_, prefix, num] = match;
        const incremented = (parseInt(num, 10) + 1)
            .toString()
            .padStart(num.length, "0");

        return `${prefix}${incremented}`;
    };

    /**
     * Remove a serial number from the list.
     * @param index Index of the serial number to be removed.
     */
    const removeSN = (index: number) => {
        // Cancel any editing operations
        editIndex = null;
        serialNumberList.update((list) => {
            list.splice(index, 1);
            return list;
        });
    };
</script>

<span class="container">
    <span>
        <h1>Serial Numbers</h1>
        <h4>&lpar;Enter zero or more&rpar;</h4>
    </span>

    <div class="serial-number-input">
        <input
            type="text"
            bind:value={serialNumber}
            on:keydown={(e) => {
                if (e.key === "Enter") {
                    if (e.shiftKey) {
                        addSN(incrementSN(), false);
                    } else {
                        addSN();
                    }
                }
            }}
        />

        <button on:click={() => addSN()} title="Add a Serial Number">
            Add
        </button>

        <button
            on:click={() => addSN(incrementSN(), false)}
            title="Increment the previous Serial Number"
        >
            Increment
        </button>

        <button on:click={() => clearSerialNumbers()}> Clear </button>
    </div>

    <ul class="serial-number-display" bind:this={serialNumberDisplay}>
        {#each $serialNumberList as sn, index}
            <li class="serial-number">
                {#if editIndex === index}
                    <!-- Individual Serial Number Editor -->
                    <input
                        type="text"
                        value={sn}
                        bind:this={snEditor}
                        on:keydown={(e) => {
                            if (e.key === "Enter") {
                                if (snEditor.value === sn) editIndex = null;
                                editSN(index, snEditor.value);
                            } else if (e.key === "Escape" || e.key === "Tab") {
                                editIndex = null;
                            }
                        }}
                    />
                {:else}
                    <!-- Individual Serial Number -->
                    <p
                        title="Double-click to edit"
                        on:dblclick={() => {
                            editIndex = index;

                            // Focus the input after it has been rendered.
                            setTimeout(() => snEditor.focus(), 0);
                        }}
                    >
                        {sn}
                    </p>
                {/if}

                <button on:click={() => removeSN(index)}>Remove</button>
            </li>
        {/each}
    </ul>
</span>

<style>
    .container {
        position: relative;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        gap: 1rem;

        /*
        
        `min-width: 0` is required to prevent another class `.serial-number-display` from
        expanding beyond the container's width (enabling it to scroll horizontally).

        A full explanation can be found here:
        https://stackoverflow.com/questions/61959291/how-to-get-scrolling-in-a-css-grid

        */

        min-width: 0;
    }

    .serial-number-display {
        display: grid;
        grid-auto-flow: column;
        grid-template-rows: repeat(6, auto);
        gap: 0.25rem 1rem;

        max-width: 100%;
        overflow-x: auto;

        list-style-type: none;
    }

    .serial-number-input input {
        grid-column: 1/-1;
    }

    .serial-number {
        display: grid;
        grid-template-columns: auto auto;
        align-items: center;
    }

    .serial-number button {
        padding: 0.1rem 0.5rem;
    }

    .serial-number input,
    .serial-number p {
        display: flex;
        align-items: center;
        padding: 0 0.5rem;
        height: 100%;
        border: none;
        color: var(--text);
        background-color: #fff3;
        font-size: 1rem;
    }

    div {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: 1fr 1fr;
        min-width: 320px;
        width: 50%;
    }

    h1 {
        font-size: 1.5rem;
    }

    span {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

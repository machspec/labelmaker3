<script lang="ts">
    import { writable } from "svelte/store";

    let serialNumberDisplay: HTMLUListElement;
    let snEditor: HTMLInputElement;
    let editIndex: number | null = null;
    let serialNumber = "";

    const serialNumberList = writable<string[]>([]);
    const lastSerialNumber = () => $serialNumberList.at(-1);

    const addSN = (input: string = serialNumber, clear = true) => {
        if (input === "") return;
        if ($serialNumberList.includes(input)) return;

        serialNumberList.update((list) => [...list, input]);

        setTimeout(
            () =>
                (serialNumberDisplay.scrollLeft =
                    serialNumberDisplay.scrollWidth),
            0,
        );

        if (clear) serialNumber = "";
    };

    const editSN = (index: number, value: string | null) => {
        if (value === null) return;
        if ($serialNumberList.includes(value)) return;

        serialNumberList.update((list) => {
            list[index] = value;
            return list;
        });

        editIndex = null;
    };

    const incrementSN = (input: string | undefined = lastSerialNumber()) => {
        if (input === "" || input === undefined) return;

        const match = input!.match(/^([a-zA-Z-]*)(\d+$)/);

        if (!match) return;

        const [_, prefix, num] = match;
        const incremented = (parseInt(num, 10) + 1)
            .toString()
            .padStart(num.length, "0");

        return `${prefix}${incremented}`;
    };

    /** Remove a serial number given its index */
    const removeSN = (index: number) => {
        serialNumberList.update((list) => {
            list.splice(index, 1);
            return list;
        });
    };
</script>

<span class="container">
    <span>
        <h1>Serial Numbers</h1>
        <h3>&lpar;Enter zero or more&rpar;</h3>
    </span>

    <div>
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
        <button on:click={() => addSN()}>Add</button>
        <button on:click={() => addSN(incrementSN(), false)}>Increment</button>
    </div>

    <ul class="serial-number-list" bind:this={serialNumberDisplay}>
        {#each $serialNumberList as sn, index}
            <li class="serial-number">
                {#if editIndex === index}
                    <input
                        type="text"
                        contenteditable="true"
                        value={sn}
                        bind:this={snEditor}
                        on:keydown={(e) => {
                            if (e.key === "Enter") {
                                if (snEditor.value === sn) editIndex = null;
                                editSN(index, snEditor.value);
                            }
                        }}
                    />
                {:else}
                    <p
                        on:dblclick={() => {
                            editIndex = index;
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
        padding: 1rem;

        /*
        
        `min-width: 0` is required to prevent another class `.serial-number-list` from
        expanding beyond the container's width (enabling it to scroll horizontally).

        This is because the container of THIS class is a grid container, which cannot
        contain items exceeding its own size, therefore the default values for `min-width`
        and `min-height` are `auto`, which is undesired in this case.

        A full explanation can be found here:
        https://stackoverflow.com/questions/61959291/how-to-get-scrolling-in-a-css-grid

        */

        min-width: 0;

        border-top: var(--bd);
    }

    .serial-number-list {
        display: grid;
        grid-auto-flow: column;
        grid-template-rows: repeat(7, auto);
        gap: 0.25rem 1rem;

        max-width: 100%;
        overflow-x: auto;

        list-style-type: none;
    }

    .serial-number {
        display: grid;
        grid-template-columns: auto auto;
        align-items: center;
    }

    .serial-number button {
        padding: 0.25rem 0.5rem;
    }

    .serial-number input,
    .serial-number p {
        display: flex;
        align-items: center;
        padding: 0 0.5rem;
        height: 100%;
        background-color: #fff3;
    }

    div {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;
        min-width: 320px;
        width: 50%;
    }

    input {
        grid-column: 1/-1;
    }

    span {
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>

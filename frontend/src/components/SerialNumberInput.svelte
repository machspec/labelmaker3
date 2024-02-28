<script lang="ts">
    import { writable } from "svelte/store";

    let serialNumber = "";
    const serialNumberList = writable<string[]>([]);
    const lastSerialNumber = () => $serialNumberList.at(-1);

    const addSN = (input: string = serialNumber, clear = true) => {
        if (input === "") return;

        serialNumberList.update((list) => [...list, input]);
        if (clear) serialNumber = "";
    };

    const incrementSN = (input: string | undefined = lastSerialNumber()) => {
        if (input === "" || input === undefined) return;

        const match = input!.match(/^([a-zA-Z-]*)(\d+$)/);

        if (!match) return;

        const [_, prefix, num] = match;
        const incremented = parseInt(num, 10) + 1;
        const newSerialNumber = `${prefix}${incremented}`;

        return newSerialNumber;
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
        <input type="text" bind:value={serialNumber} />
        <button on:click={() => addSN()}>Add</button>
        <button on:click={() => addSN(incrementSN(), false)}>Increment</button>
    </div>

    <ul class="serial-number-list">
        {#each $serialNumberList as sn, index}
            <li class="serial-number">
                <p>{sn}</p>
                <button on:click={() => removeSN(index)}>Remove</button>
            </li>
        {/each}
    </ul>
</span>

<style>
    .container {
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        align-items: center;
        gap: 1rem;
        padding: 1rem;
        border-top: var(--bd);
    }

    .serial-number {
        display: flex;
    }

    .serial-number-list {
        display: grid;
        grid-template-rows: repeat(3, auto);
        grid-template-columns: repeat(auto-fill, 1fr);
        gap: 0.5rem 1rem;

        list-style-type: none;
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

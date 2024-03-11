<script lang="ts">
    import { appState, activeForm, loading } from "./stores.js";
    import { onMount } from "svelte";
    import { labelFormats } from "./companies";
    import FormatSelector from "./components/FormatSelector.svelte";
    import Header from "./components/Header.svelte";
    import LabelForm from "./components/LabelForm.svelte";
    import LoadingOverlay from "./components/LoadingOverlay.svelte";
    import PrintingOptions from "./components/PrintingOptions.svelte";
    import SerialNumberInput from "./components/SerialNumberInput.svelte";

    const NETWORK_ERROR_MESSAGE = "Network error. Please try again later.";
    const STATE_FETCH_FAIL_MESSAGE = "Failed to fetch application state.";
    const NO_FORMAT_SELECTED_MESSAGE = "No label format selected.";

    let active: number | null;
    activeForm.subscribe((value) => (active = value));

    onMount(async () => {
        fetch("/api/state")
            .then(async (res) => {
                if (res.ok) {
                    const data = await res.json();

                    appState.set({
                        ...data,
                        loading: false,
                    });

                    document.title = data.app_title;
                    loading.set(false);
                } else {
                    console.error(`ERROR ${res.status}: ${res.statusText}`);
                    appState.setError(STATE_FETCH_FAIL_MESSAGE);
                    loading.set(false);
                }
            })
            .catch((err) => {
                console.error(NETWORK_ERROR_MESSAGE, err);
                appState.setError(NETWORK_ERROR_MESSAGE);
                loading.set(false);
            });
    });

    $: state = $appState;
</script>

<LoadingOverlay />
<Header {state} />

<main>
    <FormatSelector>
        {#each labelFormats as labelFormat, index}
            <!-- TODO: Fix these -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <h2
                data-item={index}
                on:click={() => activeForm.update(() => index)}
                class={active === index ? "active" : ""}
            >
                {labelFormat.company}
            </h2>
        {/each}
    </FormatSelector>

    <div class="container">
        {#if active === null}
            <h2 class="hero">{NO_FORMAT_SELECTED_MESSAGE}</h2>
        {/if}

        {#each labelFormats as format, itemIndex}
            <LabelForm {format} active={itemIndex === active} />
        {/each}

        {#if active !== null}
            <SerialNumberInput />
            <PrintingOptions />
        {/if}
    </div>
</main>

<style>
    main {
        display: grid;
        grid-template-columns: var(--selector-width) 2fr;
        height: calc(100vh - var(--header-height));
        width: 100%;
        overflow-y: auto;
    }

    .container {
        position: relative;
        display: grid;
        grid-template-rows: auto 24rem;
        overflow-y: auto;
    }

    .container > :global(span) {
        padding: 1rem;
    }

    .container h2 {
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100%;
    }

    .hero {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        text-wrap: nowrap;
    }

    .container > :global(.active) {
        border-bottom: var(--bd);
    }

    :global(button) {
        border: none;
        color: var(--text);
        font-size: 1rem;
        cursor: pointer;
        background-color: var(--accent);
    }

    :global(button:hover) {
        background-color: var(--accent-hover);
    }

    :global(h1) {
        font-size: 2.5rem;
    }

    :global(input) {
        font-size: 1.25rem;
    }
</style>

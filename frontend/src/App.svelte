<script lang="ts">
    import { onMount } from "svelte";
    import { labelFormats } from "./companies";
    import LabelForm from "./components/LabelForm.svelte";
    import FormatSelector from "./components/FormatSelector.svelte";
    import SerialNumberInput from "./components/SerialNumberInput.svelte";

    const NETWORK_ERROR_MESSAGE = "Network error. Please try again later.";
    const STATE_FETCH_FAIL_MESSAGE = "Failed to fetch application state.";
    const NO_FORMAT_SELECTED_MESSAGE = "No label format selected.";

    interface AppState {
        app_title?: string;
        app_version?: string;
        data?: Record<string, any>;
        error?: string;
        loading: boolean;
    }

    let state: AppState = {
        loading: true,
    };

    // let active: number | null = null;
    let active: number | null = 0;

    onMount(async () => {
        fetch("/api/state")
            .then(async (res) => {
                if (res.ok) {
                    state = { ...(await res.json()), loading: false };
                    document.title = state.app_title || "[app_title]";
                } else {
                    console.error(`ERROR ${res.status}: ${res.statusText}`);
                    state.error = STATE_FETCH_FAIL_MESSAGE;
                    state.loading = false;
                }
            })
            .catch((err) => {
                console.error(NETWORK_ERROR_MESSAGE, err);
                state.error = NETWORK_ERROR_MESSAGE;
                state.loading = false;
            });
    });

    const displayForm = (index: number) => {
        active = index;
    };
</script>

<main>
    <FormatSelector>
        {#each labelFormats as labelFormat, i}
            <!-- TODO: Fix these -->
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
            <h2 data-item={i} on:click={() => displayForm(i)}>
                {labelFormat.company}
            </h2>
        {/each}
    </FormatSelector>

    <div class="form-container">
        {#if active === null}
            <h2>{NO_FORMAT_SELECTED_MESSAGE}</h2>
        {/if}

        {#each labelFormats as labelFormat, itemIndex}
            <div class:active={itemIndex === active}>
                <LabelForm {labelFormat} />
                <SerialNumberInput />
            </div>
        {/each}
    </div>
</main>

<style>
    main {
        display: grid;
        grid-template-columns: 300px 2fr;
    }

    .form-container div {
        display: none;
    }

    .form-container div.active {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1rem;

        padding: 1rem;
        height: 100vh;
    }

    .form-container h2 {
        display: flex;
        justify-content: center;
        align-items: center;

        height: 100%;
    }

    :global(button) {
        border: none;
        background-color: var(--button-bg, #111);
        color: var(--text);
        font-size: 1rem;
        cursor: pointer;
    }
    :global(button:hover) {
        background-color: var(--button-bg-hover, #333);
    }

    :global(h1) {
        font-size: 2.5rem;
    }

    :global(input) {
        font-size: 1.25rem;
    }
</style>

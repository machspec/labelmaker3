<script lang="ts">
    import { activeForm, showPdf, pdfUrl } from "./stores.js";
    import { onMount } from "svelte";
    import PdfViewer from "svelte-pdf";
    import { labelFormats } from "./companies";
    import FormatSelector from "./components/FormatSelector.svelte";
    import Header from "./components/Header.svelte";
    import LabelForm from "./components/LabelForm.svelte";
    import PrintingOptions from "./components/PrintingOptions.svelte";
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

    // Set to `null` in production.
    let active: number | null;
    activeForm.subscribe((value) => (active = value));

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
</script>

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
{#if $showPdf}
    <div id="pdf-viewer">
        <button
            class="close"
            title="Exit PDF Viewer"
            on:click={() => ($showPdf = false)}
        ></button>

        <PdfViewer
            data=""
            downloadFileName="labels.pdf"
            scale={1}
            showButtons={[
                "navigation",
                "zoom",
                "print",
                "download",
                "pageInfo",
            ]}
            url={$pdfUrl}
        />
    </div>
{/if}

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

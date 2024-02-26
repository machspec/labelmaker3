<script lang="ts">
    import { onMount } from "svelte";
    import { labelFormats } from "./companies";
    import LabelForm from "./components/LabelForm.svelte";
    import FormatSelector from "./components/FormatSelector.svelte";

    const NETWORK_ERROR_MESSAGE = "Network error. Please try again later.";
    const STATE_FETCH_FAIL_MESSAGE = "Failed to fetch application state.";

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

<main>
    <FormatSelector>
        <ul>
            {#each labelFormats as labelFormat}
                <li>{labelFormat.company}</li>
            {/each}
        </ul>
    </FormatSelector>

    <div class="form-container">
        {#each labelFormats as labelFormat}
            <LabelForm {labelFormat} />
        {/each}
    </div>
</main>

<style>
    main {
        display: grid;
        grid-template-columns: 300px 2fr;
    }
</style>

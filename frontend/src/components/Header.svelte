<script lang="ts">
    export let state: { app_title?: string; app_version?: string };
</script>

<header>
    <img src="/logo.svg" alt="MSI Logo" />
    <h1>
        {state.app_title}
        <span class="version-text">
            {state.app_version}
        </span>
    </h1>
</header>

<style>
    header {
        display: flex;
        align-items: center;
        gap: 1rem;
        padding: 0.5rem 1rem;
        height: 100%;
        background-color: var(--accent);
        z-index: 99;
    }

    img {
        height: 100%;
        width: auto;
    }

    .version-text {
        font-size: 1rem;
    }
</style>

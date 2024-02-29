<script lang="ts"></script>

<div class="wrapper">
    <slot />
</div>

<style>
    .wrapper {
        display: grid;
        grid-template-rows: repeat(auto-fit, minmax(2rem, 1fr));
        padding: 1rem 0;
        height: 100vh;
        background: var(--accent);
        overflow-y: auto;
    }

    .wrapper :global(*) {
        display: flex;
        justify-content: center;
        align-items: center;
        cursor: pointer;
        user-select: none;
    }

    .wrapper :global(*):hover {
        background-color: var(--accent-hover);
    }
</style>

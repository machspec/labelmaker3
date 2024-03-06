<script lang="ts"></script>

<div class="wrapper">
    <slot />
</div>

<style>
    .wrapper > :global(.active) {
        background-color: var(--accent-hover);
    }

    .wrapper {
        display: grid;
        grid-template-rows: repeat(auto-fit, minmax(2rem, 1fr));
        padding: 0 0 1rem;
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

<script lang="ts">
    import { loading } from "../stores";
    import { onMount } from "svelte";

    let overlay: HTMLElement;

    onMount(() => {
        loading.subscribe((value) => {
            if (value) show();
            else hide();
        });
    });

    const hide = () => {
        overlay.style.opacity = "0";
        setTimeout(() => {
            overlay.classList.add("hidden");
        }, 500);
    };

    const show = () => {
        overlay.classList.remove("hidden");
        overlay.style.opacity = "1";
    };
</script>

<div class="overlay hidden" bind:this={overlay}>
    <div class="buffer"></div>
    <p>Loading...</p>
</div>

<style>
    .overlay {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 1rem;
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: var(--loading-overlay);
        opacity: 1;
        z-index: 100;

        transition: opacity 0.5s;
    }

    .buffer {
        width: var(--spinner-diameter);
        height: var(--spinner-diameter);
        border: solid var(--spinner-thickness) var(--spinner-primary);
        border-top: solid var(--spinner-thickness) var(--spinner-accent);
        border-radius: 50%;
        animation: spin 1s linear infinite;
    }

    .hidden {
        display: none;
    }

    p {
        font-size: 1.5rem;
        font-weight: bold;
        color: var(--loading-text);
    }

    @keyframes spin {
        0% {
            transform: rotate(0deg);
        }
        100% {
            transform: rotate(360deg);
        }
    }
</style>

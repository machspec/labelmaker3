<script lang="ts"></script>

<span class="container">
    <span>
        <h1>Serial Numbers</h1>
        <h3>&lpar;Enter zero or more&rpar;</h3>
    </span>

    <div>
        <input type="text" />
        <button>Add</button>
        <button>Increment</button>
    </div>

    <ul class="sn-list"></ul>
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

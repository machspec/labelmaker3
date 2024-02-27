<script lang="ts"></script>

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

<style>
    div {
        display: grid;
        grid-template-columns: 1fr 1fr;
        grid-template-rows: 1fr 1fr;

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

<script lang="ts">
    import { msal } from "../stores";
    import { onMount } from "svelte";

    const login = async () => {
        const loginRequest = {
            scopes: ["user.read"],
        };

        try {
            const response = msal.loginRequest(loginRequest);
            console.log(response);
        } catch (error) {
            console.error(error);
        }
    };
</script>

<button on:click={login}>Log In</button>

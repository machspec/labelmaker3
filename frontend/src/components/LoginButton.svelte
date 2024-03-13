<script lang="ts">
    import { msal } from "../stores";

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

<style>
    button {
        background-color: var(--background);
    }
</style>

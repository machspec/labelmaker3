import './app.css'
import App from './App.svelte'

// MSAL-specific imports
import { PublicClientApplication } from '@azure/msal-browser';
import { msalConfig } from './authConfig';

const app = new App({
    target: document.getElementById('app')!,
    props: {
        // @ts-ignore
        msalInstance: new PublicClientApplication(msalConfig)
    }
})

export default app

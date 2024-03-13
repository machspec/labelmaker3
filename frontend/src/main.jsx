import './app.css'
import App from './App.svelte'

// MSAL imports
import React, { useEffect, useRef } from 'react';
import ReactDOM from 'react-dom';
import { PublicClientApplication } from '@azure/msal-browser';
import { MsalProvider } from '@azure/msal-react';
import { msalConfig } from './authConfig';

const SvelteWrapper = () => {
    const svelteAppRef = useRef();
    useEffect(() => {
        const svelteApp = new App({
            target: svelteAppRef.current,
        });

        return () => { if (svelteApp) svelteApp.$destroy() };
    }, []);

    return <div ref={svelteAppRef} />;
};

// MSAL instance
const msalInstance = new PublicClientApplication(msalConfig);

// Initialize React
const ReactApp = () => (
    <MsalProvider instance={msalInstance} >
        <SvelteWrapper />
    </MsalProvider>
);

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(<ReactApp />);

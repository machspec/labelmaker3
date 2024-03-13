import { writable, type Writable } from 'svelte/store';
import { msalConfig } from "./authConfig";
import { PublicClientApplication, type PopupRequest } from '@azure/msal-browser';

export interface AppState {
    app_title: string;
    app_version: string;
    data: Record<string, any>;
    error: string | null;
    loading: Writable<boolean>;
}

export const activeForm = writable<number | null>(null);
export const checkValidity = writable<boolean>(false);
export const formDataStore = writable<Object>({});
export const formValidity = writable<boolean>(false);
export const loading = writable<boolean>(true);
export const serialNumberList = writable<string[]>([]);

export const createMsalInstance = () => {
    const instance = new PublicClientApplication(msalConfig);
    const { subscribe, set, update } = writable<PublicClientApplication>(instance);

    return {
        loginRequest: async (request: PopupRequest) => {
            await instance.initialize();
            instance.loginPopup(request);
        },
        subscribe,
        set,
        update,
    };

}

export const createAppState = () => {
    const { subscribe, set, update } = writable<AppState>({
        app_title: "[app_title]",
        app_version: "[app_version]",
        data: {},
        error: null,
        loading: loading,
    });

    return {
        create: (data: AppState) => {
            return writable<AppState>({
                ...data,
            })
        },
        subscribe,
        set,
        setError: (value: string) => update((state) => ({ ...state, value })),
        update,
    };

}

export const appState = createAppState();
export const msal = createMsalInstance();

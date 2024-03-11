import { writable } from 'svelte/store';

export const activeForm = writable<number | null>(null);
export const checkValidity = writable<boolean>(false);
export const formDataStore = writable<Object>({});
export const formValidity = writable<boolean>(false);
export const loading = writable<boolean>(true);
export const serialNumberList = writable<string[]>([]);

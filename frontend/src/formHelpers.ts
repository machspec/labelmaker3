export const formObj = (form: HTMLFormElement) =>
    Object.fromEntries(new FormData(form));

export const clearForm = (form: HTMLFormElement) =>
    confirm("Clear all fields?") ? form.reset() : null;

import { LogLevel } from "@azure/msal-browser";

/** Configuration object to be passed to MSAL instance on creation */
export const msalConfig = {
    auth: {
        clientId: "c125c3a9-4782-4beb-8a15-e4671828aee2",
        authority: "https://login.microsoftonline.com/0f9d547d-21f0-4b6f-8bca-bf8258c29d7a",
        redirectUri: "http://localhost:8000",
    },
    cache: {
        cacheLocation: "sessionStorage",
        storeAuthStateInCookie: false,
    },
    system: {
        loggerOptions: {
            loggerCallback: (level, message, containsPii) => {
                if (containsPii) return;

                switch (level) {
                    case LogLevel.Error:
                        console.error(message);
                        return;
                    case LogLevel.Info:
                        console.info(message);
                        return;
                    case LogLevel.Verbose:
                        console.debug(message);
                    case LogLevel.Warning:
                        console.warn(message);
                        return;
                    default:
                        return;
                }
            }
        },
    }
}

export const loginRequest = {
    scopes: ["User.Read"]
}

export const graphConfig = {
    graphMeEndpoint: "https://graph.microsoft.com/v1.0/me"
}
import { createBrowserRouter } from "react-router";

const routers = createBrowserRouter([
    {
        path: "/",
        lazy: async () => {
            const App = (await import("./App")).default;
            return { Component: App, ErrorBoundary: App };
        },
        children: [
            {
                index: true,
                lazy: async () => {
                    return {
                        Component: (await import("./pages/Home")).default,
                    };
                },
            },
            {
                path: "user",
                lazy: async () => {
                    return {
                        Component: (await import("./pages/User")).default,
                    };
                },
            },
            {
                path: "login",
                lazy: async () => {
                    return {
                        Component: (await import("./pages/Login")).default,
                    };
                },
            },
        ],
    },
]);

export default routers;

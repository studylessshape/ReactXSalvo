import { createBrowserRouter } from "react-router";
import Home from "./pages/Home";
import User from "./pages/User";
import App from "./App";
import ErrorBoundary from "./ErrorBoundary";
import Login from "./pages/Login";

const routers = createBrowserRouter([
    {
        path: "/",
        Component: App,
        children: [
            { index: true, Component: Home },
            { path: "user", Component: User },
            { path: "login", Component: Login },
        ],
        ErrorBoundary: ErrorBoundary
    },
]);

export default routers;

import { createBrowserRouter } from "react-router";
import Home from "./pages/Home";
import User from "./pages/User";
import Login from "./pages/Login";
import App from "./App";

const routers = createBrowserRouter([
    {
        path: "/",
        Component: App,
        children: [
            { index: true, Component: Home },
            { path: "user", Component: User },
            { path: "login", Component: Login },
        ],
        ErrorBoundary: App,
    },
]);

export default routers;

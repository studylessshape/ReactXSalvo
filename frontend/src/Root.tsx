import { ThemeProvider } from "antd-style";
import { useAppSelector } from "./hooks/redux";
import { App } from "antd";
import { Provider } from "react-redux";
import store from "./store";
import { RouterProvider } from "react-router";
import routers from "./Routers";
import { ProConfigProvider } from "@ant-design/pro-components";

function InternalRoot() {
    const themes = useAppSelector((state) => state.theme.value);
    return (
        <>
            <ProConfigProvider>
                <ThemeProvider appearance={themes}>
                    <App className="w-full h-full">
                        <RouterProvider router={routers} />
                    </App>
                </ThemeProvider>
            </ProConfigProvider>
        </>
    );
}

export default function Root() {
    return (
        <>
            <Provider store={store}>
                <InternalRoot />
            </Provider>
        </>
    );
}

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import "@ant-design/v5-patch-for-react-19";
import { RouterProvider } from "react-router";
import routers from "./Routers.tsx";
import { Provider } from "react-redux";
import store from "./store";
import { App } from "antd";
import { Root } from "./App.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Provider store={store}>
      <App className="w-full h-full">
        <Root>
          <RouterProvider router={routers} />
        </Root>
      </App>
    </Provider>
  </StrictMode>,
);

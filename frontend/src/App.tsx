import "./App.css";
import { Button, Divider, Layout as AntdLayout } from "antd";
import "virtual:uno.css";
import Layout from "./layout/Layout.tsx";
import {
  isRouteErrorResponse,
  Outlet,
  useLocation,
  useNavigate,
  useRouteError,
} from "react-router";
import ChangeThemeButton from "./layout/ChangeThemeButton.tsx";
import { useState } from "react";

function ErrorContent() {
  const error = useRouteError();
  if (isRouteErrorResponse(error)) {
    return (
      <>
        <h1>
          {error.status} {error.statusText}
        </h1>
        <p>{error.data}</p>
      </>
    );
  } else if (error instanceof Error) {
    return (
      <div>
        <h1>Error</h1>
        <p>{error.message}</p>
        <p>The stack trace is:</p>
        <pre>{error.stack}</pre>
      </div>
    );
  } else {
    return <h1>Unknown Error</h1>;
  }
}

function App() {
  const navigate = useNavigate();
  const error = useRouteError();
  const location = useLocation();
  const showLayout = location.pathname != "/login";
  if (!showLayout) {
    return (
      <AntdLayout className="w-full h-full">
        <AntdLayout.Content>
          <Outlet />
        </AntdLayout.Content>
      </AntdLayout>
    );
  }
  return (
    <>
      <Layout className="w-full h-full">
        {error != null ? <ErrorContent /> : <Outlet />}
      </Layout>
    </>
  );
}

export default App;

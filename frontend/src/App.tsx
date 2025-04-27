import "./App.css";
import { Button, Divider } from "antd";
import "virtual:uno.css";
import Layout from "./layout/Layout.tsx";
import {
  isRouteErrorResponse,
  Outlet,
  useNavigate,
  useRouteError,
} from "react-router";
import ChangeThemeButton from "./layout/ChangeThemeButton.tsx";
import { useAppSelector } from "./hooks/redux";
import { ThemeProvider } from "antd-style";

function Root({ children }: { children?: React.ReactNode }) {
  const themes = useAppSelector((state) => state.theme.value);

  return (
    <ThemeProvider appearance={themes}>
      {children}
    </ThemeProvider>
  );
}

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

  return (
    <>
      <Layout
        className="w-full h-full"
        rightDockContent={
          <>
            <div className="flex flex-items-center">
              <ChangeThemeButton />
              <Divider type="vertical"></Divider>
              <Button
                type="primary"
                onClick={() => {
                  navigate("/login");
                }}
              >
                Log In
              </Button>
            </div>
          </>
        }
      >
        {error != null ? <ErrorContent /> : <Outlet />}
      </Layout>
    </>
  );
}

export default App;
export { Root };

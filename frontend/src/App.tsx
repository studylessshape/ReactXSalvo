import "./App.css";
import { Button, ConfigProvider, Divider } from "antd";
import "virtual:uno.css";
import Layout from "./layout/Layout.tsx";
import { Outlet, useLocation, useNavigate } from "react-router";
import ChangeThemeButton from "./layout/ChangeThemeButton.tsx";
import { useAppSelector } from "./hooks/redux.ts";

function App() {
  const themes = useAppSelector((state) => state.theme.value);
  const location = useLocation();
  const navigate = useNavigate();
  const isLoginRoute = location.pathname === "/login";

  function LoginButton() {
    if (isLoginRoute) return <></>;

    return (
      <Button
        type="primary"
        onClick={() => {
          navigate("/login");
        }}
      >
        Log In
      </Button>
    );
  }

  return (
    <>
      <ConfigProvider
        theme={{
          algorithm: themes,
        }}
      >
        <Layout
          className="w-full h-full"
          hideSideBar={isLoginRoute}
          rightDockContent={
            <>
              <div className="flex flex-items-center">
                <ChangeThemeButton />
                <Divider type="vertical"></Divider>
                <LoginButton />
              </div>
            </>
          }
        >
          <Outlet />
        </Layout>
      </ConfigProvider>
    </>
  );
}

export default App;

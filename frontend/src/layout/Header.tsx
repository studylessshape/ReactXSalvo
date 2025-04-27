import { Layout, theme } from "antd";
import { NavLink } from "react-router";

const { Header } = Layout;

export default function HeaderComponent(
    { afterLogo, rightDockContent }: {
        afterLogo?: React.ReactNode;
        rightDockContent?: React.ReactNode;
    },
) {
    const {
        token: { colorBgContainer, colorText },
    } = theme.useToken();
    return (
        <Header
            className="shadow z-36"
            style={{
                backgroundColor: colorBgContainer,
            }}
        >
            <div className="flex flex-justify-between">
                <div className="flex flex-items-center">
                    <NavLink to="/">
                        <h1
                            style={{
                                color: colorText,
                                margin: 0,
                            }}
                        >
                            React & Salvo
                        </h1>
                    </NavLink>
                    {afterLogo}
                </div>
                <div>
                    {rightDockContent}
                </div>
            </div>
        </Header>
    );
}

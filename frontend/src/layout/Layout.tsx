import { Button, Divider, Layout, Menu, theme } from "antd";
import React, { useState } from "react";
import { HomeOutlined, UserOutlined } from "@ant-design/icons";
import { useNavigate } from "react-router";
import Header from "./Header";
import CollapsedButton from "./CollapsedButton";
import ChangeThemeButton from "./ChangeThemeButton";
import UserBadge from "./UserBadge";

const { Sider, Content } = Layout;

export default function (
    { className, hideLayout, hideSideBar, children }: {
        className?: string;
        hideLayout?: boolean;
        hideSideBar?: boolean;
        children?: React.ReactNode;
    },
) {
    if (hideLayout) {
        return <>{children}</>;
    }

    const [collapsed, setCollapsed] = useState(false);
    const {
        token: { colorBgContainer },
    } = theme.useToken();
    const location = document.location.pathname;
    const navigate = useNavigate();

    let internalSideBar = hideSideBar ? <></> : (
        <Sider
            collapsed={collapsed}
            style={{ background: colorBgContainer }}
        >
            <Menu
                items={[
                    {
                        label: "首页",
                        key: "/",
                        icon: <HomeOutlined />,
                        onClick: () => {
                            navigate("/");
                        },
                    },
                    {
                        label: "用户",
                        key: "/user",
                        icon: <UserOutlined />,
                        onClick: () => {
                            navigate("/user");
                        },
                    },
                    {
                        label: "test",
                        key: "/test",
                        onClick: () => {
                            navigate("/test");
                        },
                    },
                ]}
                selectedKeys={[location]}
            >
            </Menu>
        </Sider>
    );

    return (
        <>
            <Layout className={className}>
                <Header
                    afterLogo={
                        <CollapsedButton
                            collapsed={collapsed}
                            setCollapsed={setCollapsed}
                            hideButton={hideSideBar}
                        />
                    }
                    rightDockContent={
                        <>
                            <div className="flex flex-items-center">
                                <ChangeThemeButton />
                                <Divider type="vertical"></Divider>
                                <UserBadge />
                            </div>
                        </>
                    }
                >
                </Header>
                <Layout>
                    {internalSideBar}
                    <Content>
                        {children}
                    </Content>
                </Layout>
            </Layout>
        </>
    );
}

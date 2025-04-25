import { Button, Layout, Menu, theme } from "antd";
import React, { useState } from "react";
import {
    HomeOutlined,
    MenuFoldOutlined,
    MenuUnfoldOutlined,
    UserOutlined,
} from "@ant-design/icons";
import {
    NavLink,
    Outlet,
    RouterProvider,
    useLocation,
    useNavigate,
} from "react-router";
import Header from "./Header";

const { Sider, Content } = Layout;

export default function (
    { className, rightDockContent, hideLayout, hideSideBar, children }: {
        className?: string;
        rightDockContent?: React.ReactNode;
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
        token: { colorBgContainer, borderRadiusLG, colorText },
    } = theme.useToken();
    const location = useLocation();
    const navigate = useNavigate();

    function CollapsedButton() {
        if (hideSideBar) return <></>;
        return (
            <Button
                icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
                onClick={() => {
                    setCollapsed(!collapsed);
                }}
                type="text"
                style={{
                    fontSize: "18px",
                    width: 64,
                    height: 64,
                }}
            />
        );
    }

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
                ]}
                selectedKeys={[location.pathname]}
            >
            </Menu>
        </Sider>
    );

    return (
        <>
            <Layout className={className}>
                <Header
                    afterLogo={<CollapsedButton />}
                    rightDockContent={rightDockContent}
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

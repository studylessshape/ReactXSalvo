import { App, Button, Dropdown, Space } from "antd";
import { useAppDispatch, useAppSelector } from "../hooks/redux";
import { useLocation, useNavigate } from "react-router";
import { DownOutlined, UserOutlined } from "@ant-design/icons";
import { Api } from "../services/Api";
import { logout } from "../store/userSlice";

export default function UserBadge() {
    const user = useAppSelector((state) => state.user);
    const { message } = App.useApp();
    const location = useLocation();
    let dispatch = useAppDispatch();
    let navigate = useNavigate();

    if (user == null) {
        return (
            <>
                <Button
                    type="primary"
                    onClick={() => {
                        navigate("/login");
                    }}
                >
                    Log In
                </Button>
            </>
        );
    } else {
        return (
            <>
                <Dropdown
                    placement="bottom"
                    menu={{
                        items: [
                            { key: "1", label: "个人中心" },
                            {
                                key: "2",
                                label: "退出登录",
                                onClick: () => {
                                    dispatch(logout());
                                    message.success("退出登录成功！");
                                },
                            },
                        ],
                    }}
                >
                    <Button type="text">
                        <Space>
                            <UserOutlined />
                            {user.username}
                            <DownOutlined />
                        </Space>
                    </Button>
                </Dropdown>
            </>
        );
    }
}

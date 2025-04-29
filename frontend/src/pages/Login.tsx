import { App, Button, Form, Input } from "antd";
import { LockOutlined, UserOutlined } from "@ant-design/icons";
import { ServerRoutersAuthLoginData } from "../services/data-contracts";
import { useAppDispatch, useAppSelector } from "../hooks/redux";
import { useNavigate } from "react-router";
import { Api } from "../services/Api";
import { useState } from "react";
import { setUser } from "../store/userSlice";

function Login() {
    const user = useAppSelector((state) => state.user);
    const [isLogin, setIsLogin] = useState(false);
    const { message } = App.useApp();
    let navigate = useNavigate();
    let dispatch = useAppDispatch();

    if (user != null) {
        navigate("/");
    }

    return (
        <>
            <div className="w-full h-full flex flex-justify-center flex-items-center flex-col">
                <Form
                    name="Login"
                    autoComplete="off"
                    onFinish={async (values) => {
                        setIsLogin(true);
                        const api = new Api();
                        const response = await api.serverRoutersAuthPostLogin(
                            values,
                        )
                            .then((response) => {
                                if (response.data.code == 200) {
                                    message.success("登录成功！");
                                    return response.data;
                                } else {
                                    message.error(response.data.message);
                                    return null;
                                }
                            })
                            .catch((response) => {
                                message.error(response.statusText);
                            }).finally(() => {
                                setIsLogin(false);
                            });
                        if (response?.data != null) {
                            dispatch(setUser(response.data));
                            navigate("/");
                        }
                    }}
                >
                    <Form.Item<ServerRoutersAuthLoginData>
                        name="account"
                        rules={[{ required: true, message: "请输入账户！" }]}
                    >
                        <Input prefix={<UserOutlined />} placeholder="账户" />
                    </Form.Item>
                    <Form.Item<ServerRoutersAuthLoginData>
                        name="password"
                        rules={[{ required: true, message: "请输入密码！" }]}
                    >
                        <Input.Password
                            prefix={<LockOutlined />}
                            placeholder="密码"
                        />
                    </Form.Item>
                    <Form.Item label={null}>
                        <Button
                            type="primary"
                            block
                            htmlType="submit"
                            loading={isLogin}
                        >
                            登录
                        </Button>
                    </Form.Item>
                </Form>
            </div>
        </>
    );
}

export default Login;

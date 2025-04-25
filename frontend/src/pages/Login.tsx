import { Button } from "antd";
import { RedirectFunction, useNavigate } from "react-router";

function Login() {
    let navigate = useNavigate();
    return (
        <>
            <div className="w-full h-full flex flex-justify-center flex-items-center flex-col">
                <div>Login</div>
                <Button
                    onClick={() => {
                        navigate("/");
                    }}
                >
                    Home
                </Button>
            </div>
        </>
    );
}

export default Login;

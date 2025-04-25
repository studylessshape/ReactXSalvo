import { MoonOutlined, SunOutlined } from "@ant-design/icons";
import { Button, theme } from "antd";
import { toggleTheme } from "../store/themeSlice";
import { useAppDispatch, useAppSelector } from "../hooks/redux";

export default function ChangeThemeButton() {
    const dispatch = useAppDispatch();
    const themes = useAppSelector((state) => state.theme.value);

    const hasDark =
        themes.findIndex((enabledTheme) =>
            enabledTheme === theme.darkAlgorithm
        ) != -1;
    return (
        <>
            <Button
                type="text"
                icon={hasDark ? <MoonOutlined /> : <SunOutlined />}
                style={{
                    fontSize: "18px",
                    width: 64,
                    height: 64,
                }}
                onClick={() => {
                    dispatch(toggleTheme());
                }}
            />
        </>
    );
}

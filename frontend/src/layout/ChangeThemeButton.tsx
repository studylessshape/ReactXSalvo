import { MoonOutlined, SunOutlined } from "@ant-design/icons";
import { Button } from "antd";
import { toggleTheme } from "../store/themeSlice";
import { useAppDispatch, useAppSelector } from "../hooks/redux";
import { DARK_THEME } from "../store/themeSlice";
export default function ChangeThemeButton() {
    const dispatch = useAppDispatch();
    const themes = useAppSelector((state) => state.theme.value);
    const isDark = themes == DARK_THEME;
    return (
        <>
            <Button
                type="text"
                icon={isDark ? <MoonOutlined /> : <SunOutlined />}
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

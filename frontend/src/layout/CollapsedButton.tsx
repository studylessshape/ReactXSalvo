import { MenuFoldOutlined, MenuUnfoldOutlined } from "@ant-design/icons";
import { Button } from "antd";

export default function CollapsedButton(
    { hideButton, collapsed, setCollapsed, width, height }: {
        hideButton?: boolean;
        collapsed: boolean;
        setCollapsed: (collapsed: boolean) => void;
        width?: number;
        height?: number;
    },
) {
    if (hideButton) return <></>;
    return (
        <Button
            icon={collapsed ? <MenuUnfoldOutlined /> : <MenuFoldOutlined />}
            onClick={() => {
                setCollapsed(!collapsed);
            }}
            type="text"
            style={{
                fontSize: "18px",
                width: width ?? 64,
                height: height ?? 64,
            }}
        />
    );
}

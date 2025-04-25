import { createSlice } from "@reduxjs/toolkit";
import { theme } from "antd";

export const themeSlice = createSlice({
    name: "theme",
    initialState: { value: [theme.defaultAlgorithm] },
    reducers: {
        toggleTheme: (state) => {
            const defaultIndex = state.value.findIndex((algorithm) =>
                algorithm == theme.defaultAlgorithm
            );
            if (defaultIndex != -1) {
                state.value.splice(defaultIndex, 1, theme.darkAlgorithm);
            } else {
                const darkIndex = state.value.findIndex((algorithm) =>
                    algorithm == theme.darkAlgorithm
                );
                if (darkIndex != -1) {
                    state.value.splice(darkIndex, 1, theme.defaultAlgorithm);
                }
            }
        },
    },
});

export const { toggleTheme } = themeSlice.actions;

export default themeSlice.reducer;

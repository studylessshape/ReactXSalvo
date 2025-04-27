import { createSlice } from "@reduxjs/toolkit";

const LIGHT_THEME = "light";
const DARK_THEME = "dark";
const DEFAULT_THEME = LIGHT_THEME;

function initializeTheme() {
    const savedTheme = localStorage.getItem("theme");
    if (savedTheme) {
        return savedTheme;
    }
    return DEFAULT_THEME;
}

function saveTheme(theme: string) {
    localStorage.setItem("theme", theme);
}

export const themeSlice = createSlice({
    name: "theme",
    initialState: { value: initializeTheme() },
    reducers: {
        toggleTheme: (state) => {
            switch (state.value) {
                case LIGHT_THEME:
                    state.value = DARK_THEME;
                    break;
                case DARK_THEME:
                    state.value = DEFAULT_THEME;
                    break;
                default:
                    state.value = LIGHT_THEME;
                    break;
            }
            saveTheme(state.value);
        },
    },
});

export const { toggleTheme } = themeSlice.actions;
export default themeSlice.reducer;
export { DARK_THEME, DEFAULT_THEME, LIGHT_THEME };

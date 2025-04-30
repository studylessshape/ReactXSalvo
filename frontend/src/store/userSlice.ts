import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import Cookie from "js-cookie";
import { ServerRoutersAuthLoginOutData } from "../services/data-contracts";

type UserState = ServerRoutersAuthLoginOutData | null;

function initializeUser(): UserState {
    const savedUser = localStorage.getItem("user");
    if (savedUser) {
        return JSON.parse(savedUser) as UserState;
    } else {
        saveUser(null);
        return null;
    }
}

function saveUser(user: UserState) {
    if (user != null) {
        localStorage.setItem("user", JSON.stringify(user));
    } else {
        Cookie.remove("user");
        Cookie.remove("token");
        Cookie.remove("jwt_token");
        localStorage.removeItem("user");
    }
}

export const userSlice = createSlice({
    name: "currentUser",
    initialState: initializeUser(),
    reducers: {
        setUser: (_state, action: PayloadAction<UserState>) => {
            saveUser(action.payload);
            return action.payload;
        },
        logout: () => {
            saveUser(null);
            return null;
        },
    },
});

export const { setUser, logout } = userSlice.actions;
export default userSlice.reducer;

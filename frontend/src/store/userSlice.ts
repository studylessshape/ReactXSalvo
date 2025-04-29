import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import Cookie from 'js-cookie'
import { ServerRoutersAuthLoginOutData } from "../services/data-contracts";

type UserState = ServerRoutersAuthLoginOutData | null;

function initializeUser(): UserState {
    const savedUser = Cookie.get("user");
    if (savedUser) {
        return JSON.parse(savedUser) as UserState;
    } else {
        return null;
    }
}

function saveUser(user: UserState) {
    if (user != null) {
        Cookie.set("user", JSON.stringify(user), { expires: new Date(user.exp * 1000) });
    } else {
        Cookie.remove("user");
        Cookie.remove("token");
        Cookie.remove("jwt_token");
    }
}

export const userSlice = createSlice({
    name: 'currentUser',
    initialState: initializeUser(),
    reducers: {
        setUser: (_state, action: PayloadAction<UserState>) => {
            saveUser(action.payload);
            return action.payload;
        },
        logout: () => {
            saveUser(null);
            return null;
        }
    }
})

export const { setUser, logout } = userSlice.actions;
export default userSlice.reducer;
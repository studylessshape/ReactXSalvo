import { configureStore } from "@reduxjs/toolkit";
import themeReducer from "./themeSlice.ts";
import userReducer from "./userSlice.ts";

const store = configureStore({
    reducer: {
        theme: themeReducer,
        user: userReducer,
    },
});


// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch

export default store;
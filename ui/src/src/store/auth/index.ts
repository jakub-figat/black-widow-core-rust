import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { getCookie } from "../../utils/cookies";

interface AuthState {
  username: string | null;
  authenticated: boolean;
}

const currentUser = getCookie("user");
console.log("CURRENT USER", currentUser);

const initialState: AuthState = {
  username: currentUser || null,
  authenticated: !!currentUser,
};

const slice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    setUsername: (state, action: PayloadAction<AuthState["username"]>) => {
      state.username = action.payload;
    },
    setAuthenticated: (
      state,
      action: PayloadAction<AuthState["authenticated"]>
    ) => {
      state.authenticated = action.payload;
    },
  },
  selectors: {
    getUsername: (state: AuthState) => state.username,
    getAuthenticated: (state: AuthState) => state.authenticated,
  },
});

export const { setUsername, setAuthenticated } = slice.actions;
export const { getUsername, getAuthenticated } = slice.selectors;

export default slice.reducer;

import { AppDispatch } from "../hooks";
import { setCookie } from "../../utils/cookies";
import { setAuthenticated, setUsername } from ".";
import { disconnect } from "../websocket/effects";

const login = (username: string) => {
  return (dispatch: AppDispatch) => {
    setCookie("user", username, 7);
    dispatch(setUsername(username));
    dispatch(setAuthenticated(true));
  };
};

const logout = () => {
  return (dispatch: AppDispatch) => {
    setCookie("user", "", 0);
    dispatch(setUsername(null));
    dispatch(setAuthenticated(false));
    dispatch(disconnect());
  };
};

export { login, logout };

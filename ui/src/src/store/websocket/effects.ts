import {
  setConnecting,
  setError,
  setLastActionName,
  setSocket,
  unitializeWebsocketState,
} from ".";
import { WebSocketPayload } from "../../../game-types";
import { handleGameEvent, initializeGame, uninitiaize } from "../game/effects";
import { AppDispatch, RootState } from "../hooks";

export const connect = (url: string) => {
  return async (dispatch: AppDispatch) => {
    dispatch(setConnecting(true));

    const ws = new WebSocket(url);
    ws.onopen = () => {
      dispatch(setConnecting(false));
      dispatch(setSocket(ws));
      dispatch(initializeGame());
    };
    ws.onclose = () => {
      dispatch(setConnecting(false));
      dispatch(uninitiaize());
    };
    ws.onmessage = (message) => {
      dispatch(handleGameEvent(message));
    };
  };
};

export const send = (payload: WebSocketPayload) => {
  return (dispatch: AppDispatch, getState: () => RootState) => {
    const { websocket } = getState();

    if (!websocket.socket || !websocket.socket.readyState) {
      dispatch(setError("Websocket not connected"));
      return;
    }

    dispatch(setLastActionName(payload.action));
    websocket.socket.send(JSON.stringify(payload));
  };
};

export const disconnect = () => {
  return (dispatch: AppDispatch, getState: () => RootState) => {
    const { websocket } = getState();

    if (!websocket.socket || !websocket.socket.readyState) {
      dispatch(setError("Websocket not connected"));
      return;
    }

    dispatch(uninitiaize());
    dispatch(unitializeWebsocketState());
    websocket.socket.close();
  };
};

import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface WebsocketState {
  socket: WebSocket | null;
  lastMessage: string | null;
  lastActionName: string | null;
  error: string | null;
  connecting: boolean;
}

const initialState: WebsocketState = {
  socket: null,
  lastMessage: null,
  lastActionName: null,
  error: null,
  connecting: false,
} as const;

const slice = createSlice({
  name: "websocket",
  initialState,
  reducers: {
    setSocket: (state, action: PayloadAction<WebsocketState["socket"]>) => {
      state.socket = action.payload;
    },
    setLastMessage: (
      state,
      action: PayloadAction<WebsocketState["lastMessage"]>
    ) => {
      state.lastMessage = action.payload;
    },
    setLastActionName: (
      state,
      action: PayloadAction<WebsocketState["lastActionName"]>
    ) => {
      state.lastActionName = action.payload;
    },
    setError: (state, action: PayloadAction<WebsocketState["error"]>) => {
      state.error = action.payload;
    },
    setConnecting: (
      state,
      action: PayloadAction<WebsocketState["connecting"]>
    ) => {
      state.connecting = action.payload;
    },
    unitializeWebsocketState: () => initialState,
  },
  selectors: {
    getSocket: (state: WebsocketState) => state.socket,
    getLastMessage: (state: WebsocketState) => state.lastMessage,
    getLastActionName: (state: WebsocketState) => state.lastActionName,
    getError: (state: WebsocketState) => state.error,
    getIsConnected: (state: WebsocketState) =>
      !!state.socket && state.socket.readyState,
    getIsConnecting: (state: WebsocketState) => state.connecting,
  },
});

export const {
  setSocket,
  setLastActionName,
  setLastMessage,
  setError,
  setConnecting,
  unitializeWebsocketState,
} = slice.actions;

export const {
  getSocket,
  getLastMessage,
  getLastActionName,
  getError,
  getIsConnected,
  getIsConnecting,
} = slice.selectors;

export default slice.reducer;

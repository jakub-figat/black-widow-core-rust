import { combineReducers, configureStore } from "@reduxjs/toolkit";

import websocketReducer from "./websocket";
import gameReducer from "./game";
import authReducer from "./auth";

const rootReducer = combineReducers({
  websocket: websocketReducer,
  game: gameReducer,
  auth: authReducer,
});

export const store = configureStore({
  reducer: rootReducer,
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware({
      serializableCheck: {
        ignoredActions: ["websocket/setSocket"],
        ignoredPaths: ["websocket.socket"],
      },
    }),
});

export type RootState = ReturnType<typeof store.getState>;

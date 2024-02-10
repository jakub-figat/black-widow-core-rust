/* eslint-disable @typescript-eslint/no-explicit-any */
import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import {
  CardExchangeState,
  ListedGame,
  ObfuscatedGame,
  RoundFinishedState,
} from "../../../game-types";
import { IdentifiedLobby } from "./types";

interface GameState {
  games: ListedGame[] | null;
  lobbies: IdentifiedLobby[] | null;
  currentGame:
    | ObfuscatedGame<CardExchangeState>
    | ObfuscatedGame<RoundFinishedState>
    | null;
  myLobbies: IdentifiedLobby[] | null;
  myGames: ListedGame[] | null;
}

const initialState: GameState = {
  games: null,
  lobbies: null,
  currentGame: null,
  myLobbies: null,
  myGames: null,
};

const slice = createSlice({
  name: "game",
  initialState,
  reducers: {
    setGames: (state, action: PayloadAction<GameState["games"]>) => {
      state.games = action.payload;
    },
    setLobbies: (state, action: PayloadAction<GameState["lobbies"]>) => {
      state.lobbies = action.payload;
    },
    setCurrentGame: (
      state,
      action: PayloadAction<GameState["currentGame"]>
    ) => {
      state.currentGame = action.payload;
    },
    unitializeGameState: (state) => {
      state.currentGame = null;
      state.lobbies = null;
      state.games = null;
    },
    setMyLobbies: (state, action: PayloadAction<GameState["myLobbies"]>) => {
      state.myLobbies = action.payload;
    },
    setMyGames: (state, action: PayloadAction<GameState["myGames"]>) => {
      state.myGames = action.payload;
    },
  },
  selectors: {
    getGames: (state: GameState) => state.games,
    getLobbies: (state: GameState) => state.lobbies,
    getCurrentGame: (state: GameState) => state.currentGame,
    getMyLobbies: (state: GameState) => state.myLobbies,
    getMyGames: (state: GameState) => state.myGames,
  },
});

export const {
  setGames,
  setCurrentGame,
  setLobbies,
  unitializeGameState,
  setMyLobbies,
  setMyGames,
} = slice.actions;

export const {
  getGames,
  getLobbies,
  getCurrentGame,
  getMyLobbies,
  getMyGames,
} = slice.selectors;

export default slice.reducer;

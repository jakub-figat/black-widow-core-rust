/* eslint-disable @typescript-eslint/no-explicit-any */
import { PayloadAction, createSlice } from "@reduxjs/toolkit";
import { ListedGame, ObfuscatedGame } from "../../../game-types";
import { IdentifiedLobby } from "./types";

interface GameState {
  games: ListedGame[] | null;
  lobbies: IdentifiedLobby[] | null;
  currentGame: ObfuscatedGame<any> | null;
}

const initialState: GameState = {
  games: null,
  lobbies: null,
  currentGame: null,
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
  },
  selectors: {
    getGames: (state: GameState) => state.games,
    getLobbies: (state: GameState) => state.lobbies,
    getCurrentGame: (state: GameState) => state.currentGame,
  },
});

export const { setGames, setCurrentGame, setLobbies, unitializeGameState } =
  slice.actions;

export const { getGames, getLobbies, getCurrentGame } = slice.selectors;

export default slice.reducer;

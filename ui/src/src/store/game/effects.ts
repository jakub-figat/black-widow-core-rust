/* eslint-disable no-case-declarations */
import {
  setCurrentGame,
  setGames,
  setLobbies,
  setMyGames,
  setMyLobbies,
  unitializeGameState,
} from ".";
import { WebSocketResponse } from "../../../game-types";
import { AppDispatch, RootState } from "../hooks";
import { send } from "../websocket/effects";
import { IdentifiedLobby } from "./types";

const initializeGame = () => {
  return (dispatch: AppDispatch) => {
    dispatch(
      send({
        action: "listLobbies",
      })
    );

    dispatch(
      send({
        action: "listGames",
      })
    );
  };
};

const uninitiaize = () => {
  return (dispatch: AppDispatch) => {
    dispatch(unitializeGameState());
  };
};

const handleGameEvent = (message: MessageEvent) => {
  return (dispatch: AppDispatch, getState: () => RootState) => {
    const serializedMessage = JSON.parse(
      message.data
    ) as WebSocketResponse | null;

    if (!serializedMessage) {
      console.error("Invalid message", message);
      return;
    }

    if (!serializedMessage.type) {
      console.error(
        "Message type is missing, custom handling",
        serializedMessage,
        message
      );
      return;
    }

    switch (serializedMessage.type) {
      case "lobbyList":
        const identifiedLobbies: IdentifiedLobby[] = Object.keys(
          serializedMessage.lobbies
        ).map((key) => ({
          id: key,
          ...serializedMessage.lobbies[key],
        }));

        const myLobbies = identifiedLobbies.filter((lobby) =>
          lobby.players.find(
            (player) => player === `user=${getState().auth.username}`
          )
        );

        dispatch(setLobbies(identifiedLobbies));
        dispatch(setMyLobbies(myLobbies));
        break;
      case "gameList":
        const myGames = serializedMessage.games.filter((game) =>
          game.players.find(
            (player) => player === `user=${getState().auth.username}`
          )
        );

        dispatch(setGames(serializedMessage.games));
        dispatch(setMyGames(myGames));

        break;
      case "lobbyDetails":
        const currentLobbies = getState().game.lobbies;
        if (!currentLobbies) {
          console.error("Lobbies not found");
          return;
        }

        const lobbyToUpdate = currentLobbies.find(
          (lobby) => lobby.id === serializedMessage.id
        );

        if (!lobbyToUpdate) {
          dispatch(
            setLobbies([
              ...currentLobbies,
              { ...serializedMessage.lobby, id: serializedMessage.id },
            ])
          );

          const myCurrentLobbies = getState().game.myLobbies;
          const isNewLobbyMine = serializedMessage.lobby.players.find(
            (player) => player === `user=${getState().auth.username}`
          );

          if (!isNewLobbyMine) {
            return;
          }

          if (!myCurrentLobbies) {
            dispatch(
              setMyLobbies([
                { ...serializedMessage.lobby, id: serializedMessage.id },
              ])
            );
            return;
          }

          dispatch(
            setMyLobbies([
              ...myCurrentLobbies,
              { ...serializedMessage.lobby, id: serializedMessage.id },
            ])
          );

          return;
        }

        const newLobbies = [
          ...currentLobbies.filter(
            (lobby) => lobby.id !== serializedMessage.id
          ),
          {
            id: serializedMessage.id,
            ...serializedMessage.lobby,
          },
        ];

        const myNewLobbies = newLobbies.filter((lobby) =>
          lobby.players.find(
            (player) => player === `user=${getState().auth.username}`
          )
        );

        dispatch(setLobbies(newLobbies));
        dispatch(setMyLobbies(myNewLobbies));

        break;

      case "gameDetailsCardExchange":
        dispatch(setCurrentGame(serializedMessage.game));

        break;

      case "LobbyDeleted":
        const currentLobbiesAfterDelete = getState().game.lobbies?.filter(
          (lobby) => lobby.id !== serializedMessage.id
        );
        const myCurrentLobbiesAfterDelete = getState().game.myLobbies?.filter(
          (lobby) => lobby.id !== serializedMessage.id
        );

        dispatch(setLobbies(currentLobbiesAfterDelete || []));
        dispatch(setMyLobbies(myCurrentLobbiesAfterDelete || []));
        break;
      default:
        console.error("Unknown message type", serializedMessage.type, message);
        break;
    }
  };
};

export { initializeGame, uninitiaize, handleGameEvent };

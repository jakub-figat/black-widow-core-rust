/* eslint-disable no-case-declarations */
import { setGames, setLobbies, unitializeGameState } from ".";
import { WebSocketResponse } from "../../../game-types";
import { AppDispatch } from "../hooks";
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
  return (dispatch: AppDispatch) => {
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

        dispatch(setLobbies(identifiedLobbies));
        break;
      case "gameList":
        dispatch(setGames(serializedMessage.games));
        break;
      default:
        console.error("Unknown message type", serializedMessage.type, message);
        break;
    }
  };
};

export { initializeGame, uninitiaize, handleGameEvent };

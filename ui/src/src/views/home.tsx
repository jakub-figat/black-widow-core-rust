import { Box, Button, Typography } from "@mui/material";
import { useAppDispatch, useAppSelector } from "../store/hooks";
import {
  getCurrentGame,
  getGames,
  getLobbies,
  getMyGames,
  getMyLobbies,
} from "../store/game";
import { useEffect, useRef } from "react";
import { getIsConnected, getIsConnecting } from "../store/websocket";
import { connect, disconnect, send } from "../store/websocket/effects";
import { WEB_SOCKET_URL } from "../config/consts";
import { constructSxStyles } from "../utils/construct-sx-styles";
import { Layout } from "../components/layout";
import { GameLayout } from "../components/game-layout";

const HomeView = () => {
  const dispatch = useAppDispatch();

  const styles = constructSxStyles({
    lobbyContainer: {
      maxWidth: "500px",
      display: "flex",
      flexDirection: "column",
      mt: 8,
    },
    gameContainer: {
      maxWidth: "500px",
      display: "flex",
      flexDirection: "column",
      mt: 8,
    },
  });

  const isMounted = useRef(false);

  const isConnected = useAppSelector(getIsConnected);
  const isTryingToConnect = useAppSelector(getIsConnecting);

  const lobbies = useAppSelector(getLobbies);
  const games = useAppSelector(getGames);
  const myLobbies = useAppSelector(getMyLobbies);
  const myGames = useAppSelector(getMyGames);

  const currentGameState = useAppSelector(getCurrentGame);

  useEffect(() => {
    if (isMounted.current) return;

    if (!isConnected && !isTryingToConnect) {
      dispatch(connect(WEB_SOCKET_URL));
      isMounted.current = true;
    }

    return () => {
      if (isConnected || isTryingToConnect) {
        dispatch(disconnect());
        isMounted.current = false;
      }
    };
  }, []);

  const handleJoinLobby = (lobbyId: string) => {
    dispatch(
      send({
        action: "joinLobby",
        id: lobbyId,
      })
    );
  };

  const handleQuitLobby = (lobbyId: string) => {
    dispatch(
      send({
        action: "quitLobby",
        id: lobbyId,
      })
    );
  };

  const handleRejoinGame = (gameId: string) => {
    dispatch(
      send({
        action: "getGameDetails",
        id: gameId,
      })
    );
  };

  const renderLobbies = () => {
    if (!lobbies) return null;

    return lobbies.map((lobby) => {
      const alreadyInLobby = myLobbies?.find(
        (myLobby) => myLobby.id === lobby.id
      );

      if (alreadyInLobby) {
        return (
          <Box key={lobby.id} sx={styles.lobbyContainer}>
            <Typography fontWeight={"bold"}>{lobby.id}</Typography>
            <Typography>
              Players: {lobby.players.length}/{lobby.maxPlayers}
            </Typography>
            <Typography>MaxScore: {lobby.maxScore}</Typography>
            <Button
              variant="contained"
              size="small"
              sx={{
                maxWidth: "150px",
              }}
              onClick={() => handleQuitLobby(lobby.id)}
            >
              Quit lobby
            </Button>
          </Box>
        );
      }

      return (
        <Box key={lobby.id} sx={styles.lobbyContainer}>
          <Typography fontWeight={"bold"}>{lobby.id}</Typography>
          <Typography>
            Players: {lobby.players.length}/{lobby.maxPlayers}
          </Typography>
          <Typography>MaxScore: {lobby.maxScore}</Typography>
          <Button
            variant="contained"
            size="small"
            sx={{
              maxWidth: "150px",
            }}
            onClick={() => handleJoinLobby(lobby.id)}
          >
            Join lobby
          </Button>
        </Box>
      );
    });
  };

  const renderGames = () => {
    if (!games) return null;

    return (
      <Box mt={4}>
        <Typography fontWeight={"bold"}>Ongoing games:</Typography>
        {games.map((game) => {
          const isMyGame = myGames?.find((myGame) => myGame.id === game.id);

          if (isMyGame) {
            return (
              <Box key={game.id} sx={styles.gameContainer}>
                <Typography fontWeight={"bold"}>{game.id}</Typography>
                <Typography>Players: {game.players.length}</Typography>
                <Button
                  variant="contained"
                  size="small"
                  sx={{
                    maxWidth: "150px",
                  }}
                  onClick={() => handleRejoinGame(game.id)}
                >
                  Enter Game
                </Button>
              </Box>
            );
          }

          return (
            <Box key={game.id} sx={styles.gameContainer}>
              <Typography fontWeight={"bold"}>{game.id}</Typography>
              <Typography>Players: {game.players.length}</Typography>
            </Box>
          );
        })}
      </Box>
    );
  };

  if (currentGameState) {
    return (
      <Layout>
        <GameLayout
          state={currentGameState}
          onQuit={() => console.log("QUIT GAME")}
        />
      </Layout>
    );
  }

  return (
    <Layout>
      {renderLobbies()}
      {renderGames()}
    </Layout>
  );
};

export default HomeView;

import { Box, Button, Typography } from "@mui/material";
import { useAppDispatch, useAppSelector } from "../store/hooks";
import { getGames, getLobbies } from "../store/game";
import { useEffect, useRef } from "react";
import { getIsConnected, getIsConnecting } from "../store/websocket";
import { connect, disconnect } from "../store/websocket/effects";
import { WEB_SOCKET_URL } from "../config/consts";
import { constructSxStyles } from "../utils/construct-sx-styles";
import { Layout } from "../components/layout";

const HomeView = () => {
  const dispatch = useAppDispatch();

  const styles = constructSxStyles({
    lobbyContainer: {
      maxWidth: "500px",
      display: "flex",
      flexDirection: "column",
      mt: 4,
    },
    gameContainer: {
      maxWidth: "500px",
      display: "flex",
      flexDirection: "column",
      mt: 1,
    },
  });

  const isMounted = useRef(false);

  const isConnected = useAppSelector(getIsConnected);
  const isTryingToConnect = useAppSelector(getIsConnecting);

  const lobbies = useAppSelector(getLobbies);
  const games = useAppSelector(getGames);

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

  const renderLobbies = () => {
    if (!lobbies) return null;

    return lobbies.map((lobby) => (
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
        >
          Join lobby
        </Button>
      </Box>
    ));
  };

  const renderGames = () => {
    if (!games) return null;

    return (
      <Box mt={4}>
        <Typography fontWeight={"bold"}>Ongoing games:</Typography>
        {games.map((game) => (
          <Box key={game.id} sx={styles.gameContainer}>
            <Typography fontWeight={"bold"}>{game.id}</Typography>
            <Typography>Players: {game.players.length}</Typography>
          </Box>
        ))}
      </Box>
    );
  };

  return (
    <Layout>
      {renderLobbies()}
      {renderGames()}
    </Layout>
  );
};

export default HomeView;

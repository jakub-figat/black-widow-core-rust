import { useLayoutEffect, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";

import { Box, Button, Stack, TextField, Typography } from "@mui/material";

import { routes } from "../config/routes";

import { constructSxStyles } from "../utils/construct-sx-styles";

import { useAppDispatch, useAppSelector } from "../store/hooks";
import { getAuthenticated, getUsername } from "../store/auth";

import CardsTile from "../../assets/cards-tile.webp";
import { login } from "../store/auth/effect";

const LoginView = () => {
  const navigate = useNavigate();
  const dispatch = useAppDispatch();

  const [inputUsername, setInputUsername] = useState("");

  const isInputUsernameValid = useMemo(
    () => !!inputUsername && inputUsername.length > 3,
    [inputUsername]
  );

  const currentUsername = useAppSelector(getUsername);
  const isAuthentifcated = useAppSelector(getAuthenticated);

  const styles = constructSxStyles({
    mainContainer: {
      display: "flex",
      flexDirection: "column",
      alignItems: "center",
      marginTop: "64px",
      height: "100vh",
      width: "100vw",
    },
    cardsImage: {
      maxWidth: "500px",
      borderRadius: "36%",
    },
  });

  useLayoutEffect(() => {
    if (!currentUsername || !isAuthentifcated) return;

    navigate(routes.home);
  }, [isAuthentifcated, currentUsername, navigate]);

  const handleLogin = () => {
    /*
      TODO: Add validation for username
    */
    if (!isInputUsernameValid) return console.error("not valid username");

    dispatch(login(inputUsername));
  };

  return (
    <Box sx={styles.mainContainer}>
      <img src={CardsTile} alt="cards" style={styles.cardsImage} />
      <Stack spacing={2} mt={4}>
        <Typography variant="h3" fontWeight={"bold"}>
          Enter game
        </Typography>
        <TextField
          label="Username"
          variant="outlined"
          value={inputUsername}
          onChange={(e) => setInputUsername(e.target.value)}
        />
        <Button
          variant="contained"
          onClick={handleLogin}
          disabled={!isInputUsernameValid}
        >
          Login
        </Button>
      </Stack>
    </Box>
  );
};

export default LoginView;

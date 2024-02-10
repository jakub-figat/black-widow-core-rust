import { Box, Typography } from "@mui/material";
import { constructSxStyles } from "../../utils/construct-sx-styles";
import { useAppDispatch, useAppSelector } from "../../store/hooks";
import { getUsername } from "../../store/auth";
import { logout } from "../../store/auth/effect";

const Nabar = () => {
  const dispatch = useAppDispatch();
  const username = useAppSelector(getUsername);

  const styles = constructSxStyles({
    container: {
      width: "100%",
      display: "flex",
      justifyContent: "space-between",
      alignItems: "center",
      py: 2,
    },
    logoImage: {
      maxWidth: "64px",
      maxHeight: "64px",
    },
    nicknameSpan: {
      fontWeight: "bold",
    },
    logoutText: {
      cursor: "pointer",
      userSelect: "none",
    },
  });

  const handleLogout = () => {
    dispatch(logout());
  };

  return (
    <Box sx={styles.container}>
      <img src="/logo.webp" alt="logo" style={styles.logoImage} />
      <Typography>
        Username: <span style={styles.nicknameSpan}>{username}</span>
      </Typography>
      <Typography onClick={handleLogout} sx={styles.logoutText}>
        Logout
      </Typography>
    </Box>
  );
};

export { Nabar };

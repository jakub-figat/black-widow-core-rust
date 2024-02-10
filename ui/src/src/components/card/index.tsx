import { Box } from "@mui/material";
import { Card as Props } from "../../../game-types";
import { constructSxStyles } from "../../utils/construct-sx-styles";

const Card = ({ suit, value }: Props) => {
  const styles = constructSxStyles({
    cardWrapper: {
      maxWidth: "100px",
      maxHeight: "150px",
      "&:hover": {
        transform: "scale(1.1)",
      },
    },
    card: {
      maxWidth: "100%",
      maxHeight: "100%",
    },
  });

  return (
    <Box sx={styles.cardWrapper}>
      <img
        src={`./cards/${value}${suit[0].toLowerCase()}.png`}
        alt={`${value}${suit}`}
        style={styles.card}
      />
    </Box>
  );
};

export { Card };

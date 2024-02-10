import { Box, Button, Stack, Typography } from "@mui/material";
import {
  CardExchangeState,
  ObfuscatedGame,
  RoundFinishedState,
} from "../../../game-types";
import { Card } from "../card";
import { constructSxStyles } from "../../utils/construct-sx-styles";

interface Props {
  state: ObfuscatedGame<CardExchangeState> | ObfuscatedGame<RoundFinishedState>;
  onQuit: () => void;
}

const GameLayout = ({ state, onQuit }: Props) => {
  const styles = constructSxStyles({
    handContainer: {
      display: "flex",
      width: "100%",
      flexWrap: "wrap",
      gap: 4,
      mt: 4,
    },
  });

  const renderHand = () => {
    const { yourCards } = state;

    return (
      <Box>
        <Stack
          direction={"row"}
          alignItems={"center"}
          justifyContent={"space-between"}
          width={"100%"}
        >
          <Typography variant={"h4"}>
            Playing with {state.players.length} players
          </Typography>

          <Button onClick={onQuit} variant="contained">
            Quit
          </Button>
        </Stack>
        <Box sx={styles.handContainer}>
          {yourCards.map((card, index) => {
            return (
              <Card key={`${index}/${card.suit}/${card.score}`} {...card} />
            );
          })}
        </Box>
      </Box>
    );
  };

  return <Box>{renderHand()}</Box>;
};

export { GameLayout };

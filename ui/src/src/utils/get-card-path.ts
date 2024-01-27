import { Card } from "../../game-types";

export const getCardPath = (card: Card) => {
  const { suit, value } = card;
  return `/${suit}/${value}.png`;
};

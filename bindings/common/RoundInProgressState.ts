// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Card } from "./Card";
import {CardSuit} from "./CardSuit";

export interface RoundInProgressState { currentPlayer: string, tableSuit: CardSuit | null, cardsOnTable: Record<string, Card>, }
import { Lobby } from "../../../../../bindings/common/Lobby";

export interface IdentifiedLobby extends Lobby {
  id: string;
}

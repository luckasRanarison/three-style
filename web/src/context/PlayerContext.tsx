import { TwistyPlayer, TwistyPlayerConfig } from "cubing/twisty";
import { createContext, useContext } from "react";

type PlayerContext = {
  getPlayer: () => TwistyPlayer;
  play: () => void;
  setAlg: (alg: string) => void;
  setSetupAlg: (alg: string) => void;
};

const playerContext = createContext({} as PlayerContext);

const usePlayer = () => useContext(playerContext);

const defaultConfig: TwistyPlayerConfig = {
  background: "none",
  controlPanel: "none",
};

const PlayerContextProvider = (props: { children: React.ReactNode }) => {
  const player = new TwistyPlayer(defaultConfig);

  const getPlayer = () => player;
  const play = () => player.play();
  const setAlg = (alg: string) => (player.alg = alg);
  const setSetupAlg = (alg: string) => (player.experimentalSetupAlg = alg);

  const value = { getPlayer, play, setAlg, setSetupAlg };

  return (
    <playerContext.Provider value={value}>
      {props.children}
    </playerContext.Provider>
  );
};

export { usePlayer };

export default PlayerContextProvider;

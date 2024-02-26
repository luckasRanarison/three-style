import { useEffect, useRef } from "react";
import { usePlayer } from "../context/PlayerContext";

const Cube = () => {
  const { getPlayer } = usePlayer();
  const divRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    divRef.current?.appendChild(getPlayer());
  }, []);

  return <div ref={divRef} className="w-full flex justify-center"></div>;
};

export default Cube;

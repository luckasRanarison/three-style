import { RiArrowLeftRightFill, RiBox2Fill } from "react-icons/ri";
import { IconType } from "react-icons/lib";
import { useState, useEffect } from "react";
import { cornerStickers, edgeStickers } from "../utils/constants";

type Piece = "corner" | "edge";

const Header = (props: { icon: IconType; title: string }) => (
  <div className="space-x-2 flex items-center">
    <props.icon size={16} />
    <span className="font-semibold">{props.title}</span>
  </div>
);

const AlgPanel = () => {
  const [piece, setPiece] = useState<Piece>("corner");
  const [stickers, setStickers] = useState(cornerStickers);
  const [targets, setTargets] = useState(["", "", ""]);

  const setIndex = (index: number, value: string) => {
    const newTargets = [...targets];
    newTargets[index] = value;
    setTargets(newTargets);
  };

  useEffect(() => {
    if (piece == "corner") {
      setStickers(cornerStickers);
    } else {
      setStickers(edgeStickers);
    }

    setTargets(["", "", ""]);
  }, [piece]);

  return (
    <div
      className="py-6 px-4 w-full space-y-4
      border-[1px] border-gray-200 dark:border-neutral-800 rounded-md
      text-neutral-700 dark:text-white"
    >
      <Header icon={RiBox2Fill} title="Piece" />
      <div className="flex space-x-4">
        <div className="space-x-2">
          <input
            type="radio"
            name="piece"
            id="corner"
            value="corner"
            checked={piece == "corner"}
            onChange={(e) => setPiece(e.target.value as Piece)}
          />
          <label htmlFor="corner">Corner</label>
        </div>
        <div className="space-x-2">
          <input
            type="radio"
            name="piece"
            id="edge"
            value="edge"
            checked={piece == "edge"}
            onChange={(e) => setPiece(e.target.value as Piece)}
          />
          <label htmlFor="edge">Edge</label>
        </div>
      </div>
      <Header icon={RiArrowLeftRightFill} title="Cycle" />
      <div className="space-x-4">
        {targets.map((t, i) => (
          <select
            key={i}
            value={t}
            onChange={(e) => setIndex(i, e.target.value)}
            className="w-16 p-2 border-b-2 border-b-neutral-700
            bg-transparent text-neutral-700 dark:text-white"
          >
            {stickers.map((s) => (
              <option value={s}>{s}</option>
            ))}
          </select>
        ))}
      </div>
    </div>
  );
};

export default AlgPanel;

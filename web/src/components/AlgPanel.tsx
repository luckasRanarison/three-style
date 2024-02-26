import { RiArrowLeftRightFill } from "react-icons/ri";

const AlgPanel = () => {
  return (
    <div className="p-4 w-full border-[1px] border-gray-200 rounded-md text-neutral-700">
      <div className="space-x-2 flex items-center">
        <RiArrowLeftRightFill size={16} />
        <span className="font-semibold">Cycle</span>
      </div>
    </div>
  );
};

export default AlgPanel;

import Icon from "../assets/3bld.svg";
import { RiGithubFill, RiMoonFill, RiSunFill } from "react-icons/ri";
import { useTheme } from "../context/ThemeContext";

const Navbar = () => {
  const { theme, toggleTheme } = useTheme();

  return (
    <div
      className="py-5 px-8 flex justify-between
      border-b-[1px] border-b-gray-200 dark:border-b-neutral-800
      text-neutral-700 dark:text-white"
    >
      <div className="space-x-2 flex items-center">
        <img src={Icon} alt="3BLD" width={30} />
        <span className="font-bold text-xl">Three Style</span>
      </div>
      <div className="space-x-6 flex items-center">
        <button onClick={toggleTheme} className="hover:text-blue-500">
          {theme == "dark" ? <RiMoonFill size={25} /> : <RiSunFill size={25} />}
        </button>
        <a href="https://github.com/luckasRanarison/three-style">
          <RiGithubFill size={30} className="hover:text-blue-500" />
        </a>
      </div>
    </div>
  );
};

export default Navbar;

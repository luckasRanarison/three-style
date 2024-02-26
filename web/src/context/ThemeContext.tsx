import { createContext, useContext, useEffect, useState } from "react";

type Theme = "dark" | "light";

type ThemeContext = {
  theme: Theme;
  toggleTheme: () => void;
};

const themeContext = createContext({} as ThemeContext);

const useTheme = () => useContext(themeContext);

const ThemeContextProvider = (props: { children: React.ReactNode }) => {
  const [theme, setTheme] = useState<Theme>(
    window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light"
  );

  const toggleTheme = () => {
    setTheme((prev) => (prev == "dark" ? "light" : "dark"));
  };

  const value = {
    theme,
    toggleTheme,
  };

  useEffect(() => {
    if (theme == "dark") {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  }, [theme]);

  return (
    <themeContext.Provider value={value}>
      {props.children}
    </themeContext.Provider>
  );
};

export { useTheme };

export default ThemeContextProvider;

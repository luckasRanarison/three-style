import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.tsx";
import "./index.css";
import ThemeContextProvider from "./context/ThemeContext.tsx";
import PlayerContextProvider from "./context/PlayerContext.tsx";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <ThemeContextProvider>
      <PlayerContextProvider>
        <App />
      </PlayerContextProvider>
    </ThemeContextProvider>
  </React.StrictMode>
);

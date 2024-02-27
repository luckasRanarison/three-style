import AlgPanel from "./components/AlgPanel";
import Cube from "./components/Cube";
import MainContainer from "./components/MainContainer";
import Navbar from "./components/Navbar";

const App = () => {
  return (
    <>
      <Navbar />
      <MainContainer>
        <Cube />
        <AlgPanel />
      </MainContainer>
    </>
  );
};

export default App;

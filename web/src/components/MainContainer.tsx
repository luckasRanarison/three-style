import "cubing/twisty";

const MainContainer = (props: { children: React.ReactNode }) => (
  <div className="py-12 px-6 w-full flex justify-center">
    <div className="space-y-8 md:space-y-0 w-full max-w-5xl flex flex-col md:flex-row">
      {props.children}
    </div>
  </div>
);
export default MainContainer;

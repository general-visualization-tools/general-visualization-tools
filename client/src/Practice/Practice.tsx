import React, { FC } from "react";
import { Graph } from "./Graph";
import { Circles } from "./Circles";
import { Route } from "./Route";
import { Canvas } from "./Canvas";

type Props = {};
const appStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
};
export const App: FC<Props> = () => {
  return (
    <div style={appStyle}>
      <Canvas />

      <Graph />

      <Circles />

      <Route />
    </div>
  );
};

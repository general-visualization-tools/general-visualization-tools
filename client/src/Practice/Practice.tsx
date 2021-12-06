import React, {FC} from "react";
import { Graph } from "./Graph";
import { Circles } from "./Circles"
import { Route } from "./Route"

type Props = {};
const appStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
};
export const App: FC<Props> = () => {
  return (
    <div style={appStyle}>
      <Graph />

      <Circles />

      <Route />
    </div>
  );
};

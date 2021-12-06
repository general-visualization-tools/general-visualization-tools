import React, { FC } from "react";

export type CircleType = {
  id: number;
  cx: number;
  cy: number;
  r: number;
  color: string;
};

type CircleProps = {
  cir: CircleType;
  clickHandler:
    | {
        (): void;
        (event: React.MouseEvent<SVGCircleElement>): void;
      }
    | (() => void)
    | ((event: React.MouseEvent<SVGCircleElement>) => void);
};

export const Circle: FC<CircleProps> = ({ cir, clickHandler }) => {
  return (
    <circle
      cx={cir.cx}
      cy={cir.cy}
      r={cir.r}
      fill={cir.color}
      onClick={clickHandler}
    />
  );
};

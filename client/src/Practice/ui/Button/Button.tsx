import React, { FC } from "react";

type ButtonProps = {
  msg: string;
  clickHandler:
    | {
        (): void;
        (event: React.MouseEvent<HTMLButtonElement>): void;
      }
    | (() => void)
    | ((event: React.MouseEvent<HTMLButtonElement>) => void);
};
const buttonStyle: React.CSSProperties = {
  minWidth: 280,
  padding: 20,
  height: 60,
  borderWidth: 2,
  borderRadius: 15,
  borderColor: "#00a381",
  background: "#eaf4fc",
  textAlign: "center" as "center",
};
export const Button: FC<ButtonProps> = ({ msg, clickHandler }) => (
  <button style={buttonStyle} onClick={clickHandler}>
    {msg}
  </button>
);

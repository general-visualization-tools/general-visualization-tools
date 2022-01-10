import React, { FC } from "react";
import styles from "./Button.css";

type ButtonProps = {
  msg: string;
  onClickHandler:
    | {
        (): void;
        (event: React.MouseEvent<HTMLButtonElement>): void;
      }
    | (() => void)
    | ((event: React.MouseEvent<HTMLButtonElement>) => void);
};

export const Button: FC<ButtonProps> = ({ msg, onClickHandler }) => (
  <button className={styles.button} onClick={onClickHandler}>
    {msg}
  </button>
);

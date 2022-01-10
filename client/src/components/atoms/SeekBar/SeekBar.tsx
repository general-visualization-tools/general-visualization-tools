import React, { FC } from "react";
import styles from "./SeekBar.css";

type SeekBarProps = {
  value: number;
  step?: number;
  min?: number;
  max?: number;
  onChangeHandler?:
    | { (): void; (event: React.ChangeEvent<HTMLInputElement>): void }
    | (() => void)
    | ((event: React.ChangeEvent<HTMLInputElement>) => void);
};

export const SeekBar: FC<SeekBarProps> = ({
  value,
  step = 1,
  min = 0,
  max = 0,
  onChangeHandler = () => {},
}) => {
  return (
    <input
      className={styles.seekBar}
      type="range"
      name="speed"
      step={step}
      min={min}
      max={max}
      value={value}
      onChange={onChangeHandler}
    />
  );
};

import React, { FC } from "react";

type SeekBarProps = {
  intervalMS: number;
  onChangeHandler:
    | { (): void; (event: React.ChangeEvent<HTMLInputElement>): void }
    | (() => void)
    | ((event: React.ChangeEvent<HTMLInputElement>) => void);
};
const seekBarStyle: React.CSSProperties = {
  width: 500,
  height: 8,
  background: "#000000",
  borderRadius: 10,
};
export const SeekBar: FC<SeekBarProps> = ({ intervalMS, onChangeHandler }) => {
  return (
    <input
      style={seekBarStyle}
      type="range"
      name="speed"
      step="10"
      min="10"
      max="1000"
      value={intervalMS}
      onChange={onChangeHandler}
    />
  );
};

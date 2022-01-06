import React, { FC } from "react";

type SeekBarProps = {
  value: number,
  step?: number,
  min?: number,
  max?: number,
  onChangeHandler?:
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
export const SeekBar: FC<SeekBarProps> = ({ value, step=1, min=0, max=0, onChangeHandler=() => {} }) => {
  return (
    <input
      style={seekBarStyle}
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

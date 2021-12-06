import React, { FC, memo, useCallback, useState } from "react";
import lodash from "lodash";
import { rand_norm, rand_range } from "../utils/random";
import { Button } from "../ui/Button";
import { SeekBar } from "../ui/SeekBar";
import { Circle, CircleType } from "../Circle";

type ControlAreaType = {
  width: number;
  circleChanger: () => void;
};
const ControlArea: FC<ControlAreaType> = ({ width, circleChanger }) => {
  const [intervalID, setIntervalID] = useState<number | null>(null);
  const [intervalMS, setIntervalMS] = useState<number>(100);

  const circleChanging = intervalID === null;
  const intervalStateChanger = () =>
    setIntervalID((currentIntervalID) => {
      if (circleChanging) {
        return window.setInterval(circleChanger, intervalMS);
      } else {
        clearInterval(currentIntervalID);
        return null;
      }
    });

  const seekBarHandler = (event: React.ChangeEvent<HTMLInputElement>) => {
    const nextInterval = parseInt(event.target.value);
    setIntervalMS(nextInterval);
    if (intervalID !== null) {
      setIntervalID((currentIntervalID) => {
        clearInterval(currentIntervalID);
        return window.setInterval(circleChanger, nextInterval);
      });
    }
  };

  return (
    <div
      style={{
        width: width,
        height: 90,
        display: "flex",
        justifyContent: "space-around",
        alignItems: "center",
      }}
    >
      <SeekBar value={intervalMS} step={10} min={10} max={1000} onChangeHandler={seekBarHandler} />
      <p>{intervalMS} [ms]</p>
      <Button
        msg={circleChanging ? "start" : "stop"}
        clickHandler={intervalStateChanger}
      />
      <Button msg="change some circles" clickHandler={circleChanger} />
    </div>
  );
};

const generateCircles = (
  n: number,
  maxX: number,
  maxY: number,
  r: number,
  colors: string[]
) =>
  [...Array(n)].map((_, idx) =>
    Object({
      id: idx,
      cx: rand_range(maxX),
      cy: rand_range(maxY),
      r: r,
      color: colors[rand_range(colors.length)],
    })
  );
const updateCircles = (
  currentCircles: CircleType[],
  n: number,
  maxX: number,
  maxY: number,
  varRadius: number,
  colors: string[]
) => {
  const circleNum = currentCircles.length;
  let update_target = Array(circleNum).fill(false);
  for (let i = 0; i < n; ++i) update_target[rand_range(circleNum)] = true;

  let new_circles = lodash
    .cloneDeep(currentCircles)
    .filter((cir) => !update_target[cir.id]);
  for (let i = 0; i < circleNum; ++i)
    if (update_target[i]) {
      new_circles.push({
        id: i,
        cx: rand_range(maxX),
        cy: rand_range(maxY),
        r: Math.abs(rand_norm(0, varRadius)),
        color: colors[rand_range(colors.length)],
      });
    }
  return new_circles;
};

type CirclesProps = {
  width?: number;
  height?: number;
  initialRadius?: number;
  varRadius?: number;
  circleNum?: number;
  changeNum?: number;
  colors?: string[];
};
export const Circles: FC<CirclesProps> = ({
  width = 1800,
  height = 1000,
  initialRadius = 3,
  varRadius = 20,
  circleNum = 1000,
  changeNum = 30,
  colors = ["#a0d8ef", "#00a381", "#eaf4fc", "#895b8a", "#e6b422"],
}) => {
  const specializedUpdateCircles = useCallback(
    (currentCircles: CircleType[]) =>
      updateCircles(
        currentCircles,
        changeNum,
        width,
        height,
        varRadius,
        colors
      ),
    []
  );

  const [circles, setCircles] = useState<CircleType[]>(
    generateCircles(circleNum, width, height, initialRadius, colors)
  );

  const circleClickHandlerGenerator = (cir) => () => {
    setCircles((current_circles) => {
      let new_circles = lodash.cloneDeep(current_circles);
      const idx = new_circles.findIndex((tmp_cir) => tmp_cir.id === cir.id);
      new_circles[idx]["cy"] += 10;
      return new_circles;
    });
  };

  const MemoCircle: FC<{ cir: CircleType }> = useCallback(
    memo(
      ({ cir }): JSX.Element => {
        return (
          <circle
            cx={cir.cx}
            cy={cir.cy}
            r={cir.r}
            fill={cir.color}
            onClick={circleClickHandlerGenerator(cir)}
          />
        );
      },
      (prevProps, nextProps) => lodash.isEqual(prevProps, nextProps)
    ),
    []
  );
  return (
    <>
      <ControlArea
        width={width}
        circleChanger={() => {
          setCircles(specializedUpdateCircles);
        }}
      />

      <svg width={width} height={height}>
        {/* 上はメモしてなくて下はメモされてるはず　メモした方が2-4倍くらい速そう*/}
        {/*{circles.map(cir => <Circle cir={cir} key={cir.id}/>)}*/}
        {circles.map((cir) => (
          <MemoCircle cir={cir} key={cir.id} />
        ))}
      </svg>
    </>
  );
};

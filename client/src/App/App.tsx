import React, {
  FC,
  useState,
  useEffect,
  useReducer,
  useCallback,
  memo,
  useMemo,
  Dispatch,
  SetStateAction,
} from "react";
import Highcharts from "highcharts";
import HighchartsReact from "highcharts-react-official";
import lodash from "lodash";

const rand_norm = (mean: number, variance: number) =>
  Math.sqrt(-2 * Math.log(1 - Math.random())) *
    Math.cos(2 * Math.PI * Math.random()) *
    variance +
  mean;

const rand_range = (n: number) => Math.floor(Math.random() * n);

const Graph: FC<{}> = () => {
  const [dataForGraph, setDataForGraph] = useState<number[][]>([]);

  const options = useMemo(
    () =>
      Object({ title: { text: "dist sum" }, series: [{ data: dataForGraph }] }),
    [dataForGraph]
  );

  return (
    <>
      <div
        style={{
          padding: "40px 0",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <div>for graph: </div>
        <input
          type="file"
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            e.target.files
              .item(0)
              .text()
              .then((original_data) => {
                let data = original_data
                  .split("\n")
                  .map((xs) => xs.split(" ").map((x) => parseFloat(x)));
                // data = data.map((x, i) => [i, x[1]]);
                setDataForGraph(data);
              });
          }}
        />
      </div>
      <HighchartsReact highcharts={Highcharts} options={options} />
    </>
  );
};

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
const Button: FC<ButtonProps> = ({ msg, clickHandler }) => (
  <button style={buttonStyle} onClick={clickHandler}>
    {msg}
  </button>
);

type SeekbarProps = {
  intervalMS: number;
  onChangeHandler:
    | { (): void; (event: React.ChangeEvent<HTMLInputElement>): void }
    | (() => void)
    | ((event: React.ChangeEvent<HTMLInputElement>) => void);
};
const seekbarStyle: React.CSSProperties = {
  width: 500,
  height: 8,
  background: "#000000",
  borderRadius: 10,
};
const Seekbar: FC<SeekbarProps> = ({ intervalMS, onChangeHandler }) => {
  return (
    <input
      style={seekbarStyle}
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

  const seekbarHandler = (event: React.ChangeEvent<HTMLInputElement>) => {
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
      <Seekbar intervalMS={intervalMS} onChangeHandler={seekbarHandler} />
      <p>{intervalMS} [ms]</p>
      <Button
        msg={circleChanging ? "start" : "stop"}
        clickHandler={intervalStateChanger}
      />
      <Button msg="change some circles" clickHandler={circleChanger} />
    </div>
  );
};

type CircleType = {
  id: number;
  cx: number;
  cy: number;
  r: number;
  color: string;
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
const Circles: FC<CirclesProps> = ({
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

  const MemoCircle: FC<{ cir: CircleType }> = useCallback(
    memo(
      ({ cir }): JSX.Element => {
        return (
          <circle
            cx={cir.cx}
            cy={cir.cy}
            r={cir.r}
            fill={cir.color}
            onClick={() => {
              setCircles((current_circles) => {
                let new_circles = lodash.cloneDeep(current_circles);
                const idx = new_circles.findIndex(
                  (tmp_cir) => tmp_cir.id === cir.id
                );
                new_circles[idx]["cy"] += 10;
                return new_circles;
              });
            }}
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

const Paths: FC<{}> = () => {
  const [pathIndex, setPathIndex] = useState(0);
  const [paths, setPaths] = useState<number[][]>([[]]);

  return (
    <>
      <div
        style={{
          padding: "40px 0",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <div>for paths: </div>
        <input
          type="file"
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            e.target.files
              .item(0)
              .text()
              .then((original_data) => {
                setPaths(
                  original_data.split("\n").map((xs) =>
                    xs
                      .trim()
                      .split(" ")
                      .slice(1)
                      .map((x) => parseFloat(x))
                      .reduce((l, x) => {
                        if (l.length === 0 || l[l.length - 1].length === 2) {
                          l.push([x]);
                        } else {
                          l[l.length - 1].push(x);
                        }
                        return l;
                      }, [])
                  )
                );
              });
          }}
        />
      </div>
      <div style={{ textAlign: "center" }}>
        <svg width={800} height={800}>
          <polyline
            fill="none"
            stroke="#e74c3c"
            points={paths[pathIndex].flat().join(" ")}
          />
          {paths[pathIndex].map((point) => (
            <circle cx={point[0]} cy={point[1]} r={2} />
          ))}
        </svg>
      </div>
    </>
  );
};

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

      <Paths />
    </div>
  );
};

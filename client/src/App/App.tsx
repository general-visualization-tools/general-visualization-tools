import React, {
  FC,
  useState,
  useEffect,
  useCallback,
  memo,
  useMemo,
} from "react";
import Highcharts from "highcharts";
import HighchartsReact from "highcharts-react-official";
import lodash from "lodash";

type Props = {};
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
const seekbarStyle: React.CSSProperties = {
  width: 500,
  height: 8,
  background: "#000000",
  borderRadius: 10,
};
const appStyle: React.CSSProperties = {
  display: "flex",
  flexDirection: "column",
  alignItems: "center"
};

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

const Circles: FC<CirclesProps> = ({
  width = 1800,
  height = 1000,
  initialRadius = 3,
  varRadius = 20,
  circleNum = 100,
  changeNum = 30,
  colors = ["#a0d8ef", "#00a381", "#eaf4fc", "#895b8a", "#e6b422"],
}) => {
  const default_circles = useMemo<CircleType[]>(
    () =>
      [...Array(circleNum)].map((_, idx) =>
        Object({
          id: idx,
          cx: rand_range(width),
          cy: rand_range(height),
          r: initialRadius,
          color: colors[rand_range(colors.length)],
        })
      ),
    []
  );

  const update_circles = useCallback(
    (current_circles: CircleType[]) => {
      let update_target = Array(circleNum).fill(false);
      for (let i = 0; i < changeNum; ++i)
        update_target[rand_range(circleNum)] = true;

      let new_circles = current_circles.filter((cir) => !update_target[cir.id]);
      for (let i = 0; i < circleNum; ++i)
        if (update_target[i]) {
          new_circles.push({
            id: i,
            cx: rand_range(width),
            cy: rand_range(height),
            r: Math.abs(rand_norm(0, varRadius)),
            color: colors[rand_range(colors.length)],
          });
        }
      return new_circles;
    },
    [circleNum, changeNum, width, height, varRadius]
  );

  const [circles, setCircles] = useState<CircleType[]>(default_circles);
  const [circleIntervalID, setCircleIntervalID] = useState<number | null>(null);
  const [intervalMS, setIntervalMS] = useState<number>(100);

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

  const start = useCallback(
    () =>
      window.setInterval(() => {
        setCircles(update_circles);
      }, intervalMS),
    [intervalMS]
  );
  const stop = useCallback((currentCircleIntervalID) => {
    clearInterval(currentCircleIntervalID);
    return null;
  }, []);
  const func = circleIntervalID === null ? start : stop;

  return (
    <>
      <div
        style={{ width: width, height: 90, display: "flex", justifyContent: "space-around", alignItems: "center" }}
      >
        <input
          style={seekbarStyle}
          type="range"
          name="speed"
          step="10"
          min="10"
          max="1000"
          value={intervalMS}
          onChange={(event) => {
            const next_interval = parseInt(event.target.value);
            setIntervalMS(() => next_interval);
            if (circleIntervalID !== null) {
              setCircleIntervalID((currentCircleIntervalID) => {
                clearInterval(currentCircleIntervalID);
                return window.setInterval(
                  () => setCircles(update_circles),
                  next_interval
                );
              });
            }
          }}
        />
        <p>{intervalMS} [ms]</p>

        <button style={buttonStyle} onClick={() => setCircleIntervalID(func)}>
          {circleIntervalID === null ? "start" : "stop"}
        </button>

        <button style={buttonStyle} onClick={() => setCircles(update_circles)}>
          change some circles
        </button>
      </div>

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

export const App: FC<Props> = () => {
  return (
    <div style={appStyle}>
      <Graph />

      <Circles />

      <Paths />
    </div>
  );
};

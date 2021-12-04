import React, {FC, useState, useEffect, useCallback, memo} from "react";
import Highcharts from 'highcharts'
import HighchartsReact from 'highcharts-react-official'
import lodash from 'lodash'

type Props = {};

type CircleType = {
  id: number,
  cx: number,
  cy: number,
  r: number,
  color: string,
};

const rand_norm = (mean: number, variance: number) => Math.sqrt(-2 * Math.log(1 - Math.random())) * Math.cos(2 * Math.PI * Math.random()) * variance + mean;
const rand_range = (n: number) => Math.floor(Math.random()*n);

const max_h = 1000;
const max_w = 1800;
const initial_r = 3;
const var_r = 20;
const circle_num = 100;
const change_num = 100;
const colors = [ "#a0d8ef", "#00a381", "#eaf4fc", "#895b8a", "#e6b422" ]
const default_circles = [...Array(circle_num)].map((_, idx) => Object({id: idx, cx: rand_range(max_w), cy: rand_range(max_h), r: initial_r, color: colors[rand_range(colors.length)]}));


const update_circles = (current_circles: Array<CircleType>) => {
  let update_target = Array(circle_num).fill(false);
  for (let i=0; i<change_num; ++i) update_target[rand_range(circle_num)] = true;

  let new_circles = current_circles.filter(cir => !update_target[cir.id]);
  for (let i=0; i<circle_num; ++i) if (update_target[i]) {
    new_circles.push({id: i, cx: rand_range(max_w), cy: rand_range(max_h), r: Math.abs(rand_norm(0, var_r)), color: colors[rand_range(colors.length)]});
  }
  return new_circles;
}

const buttonStyle = {
  width: 300,
  height: 60,
  margin: "auto",
  borderWidth: 2,
  borderRadius: 15,
  borderColor: "#00a381",
  background: "#eaf4fc",
  textAlign: "center" as "center",
};

const seekbarStyle = {
  width: 500,
  height: 8,
  margin: "auto",
  background: "#000000",
  borderRadius: 10,
}


export const App: FC<Props> = () => {
  const [circles, setCircles] = useState(default_circles);
  const [drawCircleInterval, setDrawCircleInterval] = useState(null);
  const [intervalMS, setIntervalMS] = useState(100);
  const [dataForGraph, setDataForGraph] = useState([]);


// const Circle: FC<{cir:CircleType}> = ({cir}): JSX.Element => <circle cx={cir.cx} cy={cir.cy} r={cir.r} fill={cir.color}/>
  const MemoCircle: FC<{cir:CircleType}> = useCallback(
    memo(
      ({cir}): JSX.Element => {
        return <circle
          cx={cir.cx}
          cy={cir.cy}
          r={cir.r}
          fill={cir.color}
          onClick={() => {
            setCircles(current_circles => {
              let new_circles = lodash.cloneDeep(current_circles);
              const idx = new_circles.findIndex(tmp_cir => tmp_cir.id === cir.id);
              new_circles[idx]['cy'] += 10;
              return new_circles;
            });
          }}
        />
      },
      (prevProps, nextProps) => lodash.isEqual(prevProps, nextProps)
      ), []
  )




  /*
    useEffect(() => {
      const interval = setInterval(() => setCircles(update_circles) , draw_interval_ms);
      return () => clearInterval(interval);
    }, []);
  */

  const start = useCallback((drawCircleInterval) => setInterval(() => { setCircles(update_circles) }, intervalMS), [intervalMS]);
  const stop = useCallback((drawCircleInterval) => { clearInterval(drawCircleInterval); return null; }, []);
  const func = drawCircleInterval === null ? start : stop;
  const msg = drawCircleInterval === null ? "start" : "stop";


  const options = {
    title: { text: 'My chart' },
    series: [{ data: dataForGraph }]
  }

  return (
    <div>
      <div style={{height: 100, margin: "auto", display: "flex", justifyContent: "center", alignItems: "center"}}>
        <div>for graph: </div>
        <input type='file' onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
          e.target.files.item(0).text().then(original_data => {
            let data = original_data.split('\n').map(xs => xs.split(' ').map(x => parseFloat(x)));
            // data = data.map((x, i) => [i, x[1]]);
            setDataForGraph(data);
          });
        }}/>
      </div>
      <div style={{width: 1000, margin: "auto"}}>
        <HighchartsReact
          highcharts={Highcharts}
          options={options}
        />
      </div>

      <div style={{width: max_w, height: 90, display: "flex", margin: "auto"}}>
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
            if (drawCircleInterval !== null) {
              setDrawCircleInterval((drawCircleInterval) =>{
                clearInterval(drawCircleInterval); return setInterval(() => setCircles(update_circles), next_interval)
              });
            }
          }}
        />
        <p style={{margin: "auto"}}>{intervalMS} [ms]</p>

        <button style={buttonStyle} onClick={() => setDrawCircleInterval(func)}>
          {msg}
        </button>

        <button style={buttonStyle} onClick={() => setCircles(update_circles) }>
          change some circles
        </button>
      </div>
      <div style={{textAlign: "center"}}>
        <svg width={max_w} height={max_h}>

          {/* 上はメモしてなくて下はメモされてるはず　メモした方が2-4倍くらい速そう*/}
          {/*{circles.map(cir => <Circle cir={cir} key={cir.id}/>)}*/}
          {circles.map(cir => <MemoCircle cir={cir} key={cir.id}/>)}

        </svg>
      </div>
    </div>
  );
};

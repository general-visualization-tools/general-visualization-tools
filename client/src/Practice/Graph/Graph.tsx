import React, { FC, useMemo, useState } from "react";
import HighchartsReact from "highcharts-react-official";
import Highcharts from "highcharts";

export const Graph: FC<{}> = () => {
  const [dataForGraph, setDataForGraph] = useState<number[][]>([]);

  const options = useMemo(
    () => ({
      title: { text: "dist sum" },
      series: dataForGraph,
      chart: {
        zoomType: "x",
        width: 1500,
      },
    }),
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
            e.target.files.item(0).text().then(original_data => {
              const json = JSON.parse(original_data);
              console.log(json);
              delete json.c0[0].color;
              delete json.c0[1].color;
              setDataForGraph(json["c0"]);
            })
/*
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
*/
          }}
        />
      </div>
      <HighchartsReact highcharts={Highcharts} options={options} />
    </>
  );
};

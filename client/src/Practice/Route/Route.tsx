import React, { FC, useState } from "react";

export const Route: FC<{}> = () => {
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

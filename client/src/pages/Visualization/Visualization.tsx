import React, { FC, useState } from "react";
import { Graphic } from "./Graphic";
import styles from "./Visualization.css";

import { GroupedVisCompsType } from "./types";

const parseToGVC = (data: string): Array<GroupedVisCompsType> => {
  console.log("graphic:", JSON.parse(data));
  return JSON.parse(data);
  // const gvcArr: Array<GroupedVisCompsType> = JSON.parse(data);
  // for(const gvc of gvcArr) {
  //   if (gvc.groupID == "group0") return [gvc];
  // }
};

export const Visualization: FC<Props> = () => {
  const [gvcArr, setGvcArr] = useState<Array<GroupedVisCompsType>>([]);
  console.log("[gvc]: ", gvcArr);
  return (
    <div className={styles.visualizationPage}>
      <input
        type="file"
        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
          e.target.files
            .item(0)
            .text()
            .then((data) => {
              setGvcArr(parseToGVC(data));
            });
        }}
      />
      {gvcArr.map((gvc) =>
        gvc.graphic ? (
          <Graphic key={gvc.groupID} graphic={gvc.graphic} />
        ) : undefined
      )}
    </div>
  );
};

type Props = {};

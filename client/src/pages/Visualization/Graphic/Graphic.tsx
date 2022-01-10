import React, { FC, memo, useCallback, useState } from "react";
import lodash from "lodash";
import styles from "./Graphic.css";
import {
  GraphicType,
  FrameType,
  TransitionType,
  ElemType,
  CameraType,
  DiffType,
  ElemDiffType,
} from "../types";
import { ControlPanel } from "./ControlPanel";

export type MovingFrameType = {
  cameraUID?: string;
  time: number;
  UIDToElem: Map<string, ElemType>;
};

const cameraToViewBox = (c: CameraType): string =>
  `${c.x} ${c.y} ${c.w} ${c.h}`;

const getViewBox = (frame: MovingFrameType): string => {
  if (frame.cameraUID) {
    const tmp = frame.UIDToElem.get(frame.cameraUID);
    if (tmp.elemType == "Camera") return cameraToViewBox(tmp);
    console.error(
      "elem has frame.cameraUID is not camera.\nUID is ",
      frame.cameraUID,
      "\nelem is ",
      tmp
    );
  } else return "0 0 1000 1000";
};

const updateFrame = (currentFrame: MovingFrameType, diff: ElemDiffType) => {
  if (diff.diffType == DiffType.create) {
    let elem = lodash.cloneDeep(diff);
    delete elem.diffType;
    currentFrame.UIDToElem.set(elem.UID, elem);
  } else if (diff.diffType == DiffType.update) {
    for (const key in diff) {
      if (diff.hasOwnProperty(key) && key !== "diffType") {
        currentFrame.UIDToElem.get(diff.UID)[key] = diff[key];
      }
    }
  } else if (diff.diffType == DiffType.delete)
    currentFrame.UIDToElem.delete(diff.UID);
};

const transitionFrame = (
  frame: MovingFrameType,
  transitions: Array<TransitionType>,
  currentStep: number,
  nextStep: number
) => {
  nextStep = Math.max(0, Math.min(transitions.length - 1, nextStep));
  if (currentStep < nextStep)
    for (let i = currentStep; i < nextStep; ++i)
      for (const diff of transitions[i].next) updateFrame(frame, diff);
  else
    for (let i = currentStep; i > nextStep; --i)
      for (const diff of transitions[i].prev) updateFrame(frame, diff);
  frame.time = transitions[nextStep].time;
};

const createMovingFrameBy = (frame: FrameType): MovingFrameType => {
  console.log("createMovingFrameBy");
  let movingFrame: MovingFrameType = {
    time: frame.time,
    UIDToElem: frame.elems.reduce((map: Map<string, ElemType>, x) => {
      map.set(x.UID, x);
      return map;
    }, new Map()),
  };
  movingFrame.UIDToElem.forEach((elem, UID) => {
    if (elem.elemType == "Camera") movingFrame.cameraUID = UID;
  });
  return movingFrame;
};

const MemoElem: FC<{ elem: ElemType }> = memo(
  ({ elem }): JSX.Element => {
    if (elem.elemType === "Camera") return null;
    if (elem.elemType === "Rect") {
      return (
        <rect
          x={elem.x}
          y={elem.y}
          width={elem.w}
          height={elem.h}
          fill={elem.color}
        />
      );
    } else if (elem.elemType === "Circle") {
      return <circle cx={elem.x} cy={elem.y} r={elem.r} fill={elem.color} />;
    } else if (elem.elemType === "Path") {
      return (
        <polyline
          fill="none"
          stroke={elem.color}
          strokeWidth={2}
          points={elem.points}
        />
      );
    }
  },
  (prevProps, nextProps) => lodash.isEqual(prevProps, nextProps)
);

type Props = { graphic: GraphicType };

export const Graphic: FC<Props> = ({ graphic }) => {
  const [currentFrame, setCurrentFrame] = useState<MovingFrameType>(
    useCallback(() => createMovingFrameBy(graphic.initial), [])
  );

  const frameUpdater = useCallback((currentStep, nextStep) => {
    setCurrentFrame((currentFrame) => {
      const nextFrame = lodash.cloneDeep(currentFrame);
      transitionFrame(nextFrame, graphic.transitions, currentStep, nextStep);
      return nextFrame;
    });
  }, []);

  return (
    <div className={styles.graphic}>
      <ControlPanel
        maxStep={graphic.transitions.length}
        frameUpdater={frameUpdater}
      />
      <p>time: {currentFrame.time}</p>
      <svg width="1000" height="1000" viewBox={getViewBox(currentFrame)}>
        {Array.from(currentFrame.UIDToElem)
          .map(([UID, elem]) => elem)
          .sort((x, y) =>
            x.elemType === "Camera" || y.elemType === "Camera" ? 0 : x.z - y.z
          )
          .map((elem) => (
            <MemoElem key={elem.UID} elem={elem} />
          ))}
      </svg>
    </div>
  );
};

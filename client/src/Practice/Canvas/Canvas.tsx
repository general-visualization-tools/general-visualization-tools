import React, {FC, useRef, useState} from "react";
import lodash from "lodash";
import { SeekBar } from "../ui/SeekBar";
import { Button } from "../ui/Button";

type CanvasProps = {};

type RectType = {
  shapeID: string;
  color: string;

  x: number;
  y: number;
  w: number;
  h: number;
  z: number;
  theta: number;
};

type CircleType = {
  shapeID: string;
  color: string;

  r: number;
  x: number;
  y: number;
  z: number;
  theta: number;
};

type ShapeType = RectType | CircleType;

type FrameType = {
  time: number;
  shapes: object;
};

type DiffType = ShapeType & {
  diffType: string;
};

type TransitionType = {
  time: number;
  next: Array<DiffType>;
  prev: Array<DiffType>;
};

type CanvasType = {
  initial: FrameType;
  final: FrameType;
  transitions: Array<TransitionType>;
};

const defaultFrame = { time: 0, shapes: [] };

const updateFrame = (currentFrame: FrameType, diff: DiffType) => {
  if (diff.diffType == "Create") {
    currentFrame.shapes[diff.shapeID] = {};
    for (const key in diff)
      if (diff.hasOwnProperty(key) && key !== "diffType")
        currentFrame.shapes[diff.shapeID][key] = diff[key];
  } else if (diff.diffType == "Update") {
    for (const key in diff)
      if (diff.hasOwnProperty(key) && key !== "diffType")
        currentFrame.shapes[diff.shapeID][key] = diff[key];
  } else if (diff.diffType == "Delete")
    delete currentFrame.shapes[diff.shapeID];
};

const transitionFrame = (
  frame: FrameType,
  transitions: Array<TransitionType>,
  currentStep: number,
  nextStep: number
) => {
  if (currentStep < nextStep)
    for (let i = currentStep; i < nextStep; ++i)
      for (const diff of transitions[i].next) updateFrame(frame, diff);
  else
    for (let i = currentStep; i > nextStep; --i)
      for (const diff of transitions[i].prev) updateFrame(frame, diff);
  frame.time = transitions[nextStep].time;
};

type ControlAreaProps = {
  intervalMS?: number;
  maxStep: number;
  frameUpdater: (currentStep: number, nextStep: number) => void;
};

const ControlArea: FC<ControlAreaProps> = ({
  intervalMS = 20,
  maxStep,
  frameUpdater,
}) => {
  const [intervalID, setIntervalID] = useState<number | null>(null);
  const [currentStep, setCurrentStep] = useState<number>(0);
  const [intervalSPS, setIntervalSPS] = useState<number>(1);
  const intervalSPSRef = useRef<number>(intervalSPS);
  const cumulativeDiffStep = useRef<number>(0);

  const duringTransition = intervalID !== null;
  const intervalStateChanger = () =>
    setIntervalID((currentIntervalID) => {
      if (!duringTransition) {
        return window.setInterval(() => {
          let nextCumulativeDiffStep =
            cumulativeDiffStep.current + (intervalSPSRef.current * intervalMS) / 1000;
          const diffStep =
            Math.sign(nextCumulativeDiffStep) *
            Math.floor(Math.abs(nextCumulativeDiffStep));
          nextCumulativeDiffStep -= diffStep;
          cumulativeDiffStep.current = nextCumulativeDiffStep;

          setCurrentStep(currentStep => {
            const nextStep = Math.max(0, Math.min(maxStep-1, currentStep + diffStep));
            frameUpdater(currentStep, nextStep);
            return nextStep;
          })
        }, intervalMS);
      } else {
        clearInterval(currentIntervalID);
        return null;
      }
    });

  if (duringTransition && currentStep == maxStep-1) intervalStateChanger();

  return (
    <>
      <div
        style={{
          width: 1200,
          height: 100,
          display: "flex",
          justifyContent: "space-around",
          alignItems: "center",
        }}
      >
        <Button msg={duringTransition ? "stop" : "start"} clickHandler={intervalStateChanger} />
        <div style={{ textAlign: "center" }}>
          <SeekBar
            value={intervalSPSRef.current}
            min={1}
            max={Math.max(1, Math.min(300, maxStep))}
            onChangeHandler={(event: React.ChangeEvent<HTMLInputElement>) => {
              intervalSPSRef.current = parseInt(event.target.value);
              setIntervalSPS(intervalSPSRef.current);
            }}
          />
          <SeekBar
            value={currentStep}
            max={maxStep}
            onChangeHandler={(event: React.ChangeEvent<HTMLInputElement>) => {
              setCurrentStep(currentStep => {
                const nextStep = Math.max(0, Math.min(maxStep-1, parseInt(event.target.value)));
                frameUpdater(currentStep, nextStep);
                return nextStep;
              })
            }}
          />
        </div>
        <div>
          <p style={{ width: 200, textAlign: "left" }}> step per second: {intervalSPS} </p>
          <p style={{ width: 200, textAlign: "left" }}> step: {currentStep} / {maxStep} th </p>
        </div>
      </div>
    </>
  );
};

export const Canvas: FC<CanvasProps> = () => {
  const [canvas, setCanvas] = useState<CanvasType>({
    initial: lodash.cloneDeep(defaultFrame),
    final: lodash.cloneDeep(defaultFrame),
    transitions: [],
  });
  const [currentFrame, setCurrentFrame] = useState<FrameType>(
    lodash.cloneDeep(defaultFrame)
  );

  const frameUpdater = (currentStep, nextStep) => {
    setCurrentFrame((currentFrame) => {
      const nextFrame = lodash.cloneDeep(currentFrame);
      transitionFrame(
        nextFrame,
        canvas.transitions,
        currentStep,
        nextStep
      );
      return nextFrame;
    });
  };

  return (
    <>
      <ControlArea maxStep={Math.max(0, canvas.transitions.length - 1)} frameUpdater={frameUpdater} />
      <p>
        {" "}
        now time is {currentFrame.time} / {canvas.final.time}{" "}
      </p>
      <input
        type="file"
        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
          e.target.files
            .item(0)
            .text()
            .then((original_data) => {
              const json = JSON.parse(original_data);
              console.log(json);
              const newCanvas: CanvasType = {
                initial: json.shapes.canvas0.initial,
                final: json.shapes.canvas0.final,
                transitions: json.shapes.canvas0.transitions,
              };
              const nextFrame = json.shapes.canvas0.initial.shapes.reduce(
                (result, x) => {
                  result.shapes[x.shapeID] = x;
                  return result;
                },
                { shapes: {} }
              );
              setCanvas(newCanvas);
              setCurrentFrame(nextFrame);
            });
        }}
      />
      <svg width="1000" height="1000" viewBox="0 0 10000 10000">
        {Object.values(currentFrame.shapes)
          .sort((x, y) => x.z - y.z)
          .map((shape) => {
            if (shape.shapeID[0] == "R") {
              return (
                <rect
                  x={shape.x}
                  y={shape.y}
                  width={shape.w}
                  height={shape.h}
                  fill={shape.color}
                />
              );
            } else if (shape.shapeID[0] == "P") {
              return (
                <circle
                  cx={shape.x}
                  cy={shape.y}
                  r={shape.r}
                  fill={shape.color}
                />
              );
            }
          })}
      </svg>
    </>
  );
};

import React, { FC, memo, useRef, useState } from "react";
import { Button } from "../../../components/atoms/Button";
import { SeekBar } from "../../../components/atoms/SeekBar";

const intervalMS = 20;

type Props = {
  maxStep: number;
  frameUpdater: (currentStep: number, nextStep: number) => void;
};

export const ControlPanel: FC<Props> = ({ maxStep, frameUpdater }) => {
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
            cumulativeDiffStep.current +
            intervalSPSRef.current * (intervalMS / 1000);
          const diffStep = Math.trunc(nextCumulativeDiffStep);
          nextCumulativeDiffStep -= diffStep;
          cumulativeDiffStep.current = nextCumulativeDiffStep;

          if (diffStep !== 0) {
            let nextStep;
            setCurrentStep((currentStep) => {
              nextStep = Math.max(
                0,
                Math.min(maxStep - 1, currentStep + diffStep)
              );
              return nextStep;
            });
            frameUpdater(currentStep, nextStep);
          }
        }, intervalMS);
      } else {
        clearInterval(currentIntervalID);
        return null;
      }
    });

  if (duringTransition && currentStep == maxStep - 1) intervalStateChanger();

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
        <Button
          msg={duringTransition ? "stop" : "start"}
          onClickHandler={intervalStateChanger}
        />
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
              const nextStep = Math.max(
                0,
                Math.min(maxStep - 1, parseInt(event.target.value))
              );
              setCurrentStep(nextStep);
              frameUpdater(currentStep, nextStep);
            }}
          />
        </div>
        <div>
          <p style={{ width: 200, textAlign: "left" }}>
            {" "}
            step per second: {intervalSPS}{" "}
          </p>
          <p style={{ width: 200, textAlign: "left" }}>
            {" "}
            step: {currentStep} / {maxStep - 1} th{" "}
          </p>
        </div>
      </div>
    </>
  );
};

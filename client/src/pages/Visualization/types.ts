export const DiffType = {
  create: "Create",
  update: "Update",
  delete: "Delete",
} as const;
export type DiffType = typeof DiffType[keyof typeof DiffType];

export type CameraType = {
  UID: string;
  elemType: "Camera";
  x: number;
  y: number;
  w: number;
  h: number;
};
export type CameraDiffType = {
  UID: string;
  elemType: "Camera";
  diffType: DiffType;
  x?: number;
  y?: number;
  w?: number;
  h?: number;
};

export type CircleType = {
  UID: string;
  elemType: "Circle";
  name: string;
  color: string;
  r: number;
  x: number;
  y: number;
  z: number;
  theta: number;
};
export type CircleDiffType = {
  UID: string;
  elemType: "Circle";
  diffType: DiffType;
  name?: string;
  color?: string;
  r?: number;
  x?: number;
  y?: number;
  z?: number;
  theta?: number;
};

export type RectType = {
  UID: string;
  elemType: "Rect";
  name: string;
  color: string;

  x: number;
  y: number;
  w: number;
  h: number;
  z: number;
  theta: number;
};
export type RectDiffType = {
  UID: string;
  elemType: "Rect";
  diffType: DiffType;
  name?: string;
  color?: string;

  x?: number;
  y?: number;
  w?: number;
  h?: number;
  z?: number;
  theta?: number;
};

export type PathType = {
  UID: string;
  elemType: "Path";
  name: string;
  color: string;
  points: string;
  z: number;
};
export type PathDiffType = {
  UID: string;
  elemType: "Path";
  diffType: DiffType;
  name?: string;
  color?: string;
  points?: string;
  z?: number;
};

export type ElemType = CameraType | CircleType | RectType | PathType;
export type ElemDiffType =
  | CameraDiffType
  | CircleDiffType
  | RectDiffType
  | PathDiffType;

export type FrameType = {
  time: number;
  elems: Array<ElemType>;
};
export type TransitionType = {
  time: number;
  next: Array<ElemDiffType>;
  prev: Array<ElemDiffType>;
};
export type GraphicType = {
  initial: FrameType;
  final: FrameType;
  transitions: Array<TransitionType>;
};

export type LineType = {
  name: string;
  color: string;
  data: Array<Array<number>> | Array<number>;
};
export type ChartType = {
  lines: Array<LineType>;
};

export type GroupedVisCompsType = {
  groupID: string;
  graphic?: GraphicType;
  chart?: ChartType;
  // log: Object;
};

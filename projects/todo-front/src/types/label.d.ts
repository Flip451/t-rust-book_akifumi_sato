import { v4 as uuidv4 } from "uuid";

export type Label = {
    id: LabelId
    name: LabelName
}

type LabelId = string;

type LabelName = string;

export type CreateLabelPayload = {
    name: LabelName
}

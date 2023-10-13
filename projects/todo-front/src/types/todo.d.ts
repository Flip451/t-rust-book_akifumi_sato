import { v4 as uuidv4 } from "uuid";
import { Label, LabelId } from "./label";

export type Todo = {
    id: TodoId
    text: TodoText
    completed: boolean
    labels: Label[]
}

type TodoId = string;

type TodoText = string;

export type CreateTodoPayload = {
    text: TodoText
    label_ids: LabelId[]
}
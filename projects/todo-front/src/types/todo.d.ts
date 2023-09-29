import { v4 as uuidv4 } from "uuid";

export type Todo = {
    id: TodoId
    text: TodoText
    completed: boolean
}

type TodoId = String;

type TodoText = {
    value: String
}

export type CreateTodoPayload = {
    text: TodoText
}
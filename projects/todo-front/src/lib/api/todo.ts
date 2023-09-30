import { CreateTodoPayload, Todo } from "../../types/todo";

export const createTodo = async (payload: CreateTodoPayload) => {
    const res = await fetch('http://localhost:3000/todos', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload),
    })
    if (!res.ok) {
        throw new Error("create todo request failed");

    }
    const json: Todo = await res.json()
    return json
}


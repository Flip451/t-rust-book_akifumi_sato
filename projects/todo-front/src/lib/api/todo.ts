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

export const getAllTodo = async () => {
    const res = await fetch('http://localhost:3000/todos')
    if (!res.ok) {
        throw new Error("get all todo request failed");
    }
    const json: Todo[] = await res.json()
    return json
}

export const updateTodo = async (todo: Todo) => {
    const { id, ...updateTodo } = todo
    const res = await fetch(`http://localhost:3000/todos/${id}`, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updateTodo),
    })
    if (!res.ok) {
        throw new Error("update todo request failed");
    }
    const json: Todo = await res.json()
    return json
} 
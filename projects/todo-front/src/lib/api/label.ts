import { CreateLabelPayload, Label, LabelId } from "../../types/label";

export const createLabel = async (payload: CreateLabelPayload) => {
    const res = await fetch('http://localhost:3000/labels', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload),
    })
    if (!res.ok) {
        throw new Error("create label request failed");

    }
    const json: Label = await res.json()
    return json
}

export const getAllLabel = async () => {
    const res = await fetch('http://localhost:3000/labels')
    if (!res.ok) {
        throw new Error("get all label request failed");
    }
    const json: Label[] = await res.json()
    return json
}

export const updateLabel = async (label: Label) => {
    const { id, ...updateLabel } = label
    const res = await fetch(`http://localhost:3000/labels/${id}`, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(updateLabel),
    })
    if (!res.ok) {
        throw new Error("update label request failed");
    }
    const json: Label = await res.json()
    return json
}

export const deleteLabel = async (label_id: LabelId) => {
    const res = await fetch(`http://localhost:3000/labels/${label_id}`, {
        method: 'DELETE',
    })
    if (!res.ok) {
        throw new Error("update label request failed");
    }
}
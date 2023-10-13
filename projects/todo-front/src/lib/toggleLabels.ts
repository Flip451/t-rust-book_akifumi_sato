import { Label } from "../types/label"

export const toggleLabels = (labels: Label[], target: Label) =>
    labels.find(({ id }) => id === target.id)
        ? labels.filter(({id}) => id !== target.id)
        : [...labels, target]

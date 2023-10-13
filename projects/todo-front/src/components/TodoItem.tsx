import { FC, useEffect, useState } from "react"
import { Todo, TodoId, UpdateTodoPayload } from "../types/todo"
import { Box, Button, Card, Checkbox, Chip, FormControl, FormControlLabel, Grid, Modal, Stack, TextField, Typography } from "@mui/material"
import { modalInnerStyle } from "../styles/modal"
import { Label } from "../types/label"
import { CheckBox } from "@mui/icons-material"
import { toggleLabels } from "../lib/toggleLabels"

type props = {
    todo: Todo
    labels: Label[]
    onUpdate: (todo: UpdateTodoPayload) => void
    onDelete: (id: TodoId) => void
}

const TodoItem: FC<props> = ({ todo, labels, onUpdate, onDelete }) => {
    const [editing, setEditing] = useState(false)
    const [editText, setEditText] = useState("")
    const [editLabels, setEditLabels] = useState<Label[]>([])

    // todo 変更時に初期化
    useEffect(() => {
        setEditText(todo.text)
        setEditLabels(todo.labels)
    }, [todo, editing])

    const handleCompletedCheckbox = (todo: Todo) => {
        onUpdate({
            ...todo,
            completed: !todo.completed,
        })
    }

    const handleDeleteButton = () => {
        onDelete(todo.id)
    }

    const handleEditButton = () => {
        setEditing(true)
    }

    const onCloseEditModal = () => {
        setEditing(false)
    }

    const handleSubmitButton = () => {
        if (!editText) {
            return
        }
        onUpdate({
            ...todo,
            text: editText,
            label_ids: editLabels.map((label) => label.id)
        })
        setEditing(false)
    }

    return (
        <Card key={todo.id} sx={{ p: 2 }}>
            <Grid sx={{ p: 1 }}>
                <Grid container spacing={2} alignItems="center">
                    <Grid item xs={1}>
                        <Checkbox
                            checked={todo.completed}
                            onChange={() => handleCompletedCheckbox(todo)}
                        />
                    </Grid>
                    <Grid item xs={8}>
                        <Stack spacing={1}>
                            <Typography variant="caption" fontSize={18}>
                                {todo.text}
                            </Typography>
                        </Stack>
                        <Stack direction="row" spacing={1} >
                            {todo.labels.map((label) => (
                                <Chip key={label.id} label={label.name} size="small" />
                            ))}
                        </Stack>
                    </Grid>
                    <Grid item xs={2}>
                        <Stack direction="row" spacing={1}>
                            <Button onClick={handleEditButton} color="info">
                                EDIT
                            </Button>
                            <Button onClick={handleDeleteButton} color="error">
                                DELETE
                            </Button>
                        </Stack>
                    </Grid>
                </Grid>
            </Grid>
            <Modal open={editing} onClose={onCloseEditModal}>
                <Box sx={modalInnerStyle}>
                    <Stack spacing={2}>
                        <TextField
                            size="small"
                            label="todo text"
                            defaultValue={todo.text}
                            onChange={(e) => setEditText(e.target.value)}
                        />
                        <Stack>
                            <Typography variant="subtitle1">Lables</Typography>
                            {labels.map((label) => (
                                <FormControlLabel
                                    key={label.id}
                                    control={
                                        <Checkbox
                                            defaultChecked={todo.labels.some((todoLabel) => todoLabel.id == label.id)}
                                        />
                                    }
                                    label={label.name}
                                    onChange={() => setEditLabels((prev) => toggleLabels(prev, label))}
                                />
                            ))}
                        </Stack>
                        <Button onClick={handleSubmitButton} color="info">
                            SUBMIT
                        </Button>
                    </Stack>
                </Box>
            </Modal>
        </Card>
    )
}

export default TodoItem;
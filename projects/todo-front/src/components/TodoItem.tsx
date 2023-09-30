import { FC, useState } from "react"
import { Todo, TodoId } from "../types/todo"
import { Box, Button, Card, Checkbox, Grid, Modal, Stack, TextField, Typography } from "@mui/material"
import { modalInnerStyle } from "../styles/modal"

type props = {
    todo: Todo
    onUpdate: (todo: Todo) => void
    onDelete: (id: TodoId) => void
}

const TodoItem: FC<props> = ({ todo, onUpdate, onDelete }) => {
    const [editing, setEditing] = useState(false)
    const [editText, setEditText] = useState("")

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
            text: {
                value: editText
            },
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
                            <Typography variant="caption" fontSize={16}>
                                {todo.text.value}
                            </Typography>
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
                            defaultValue={todo.text.value}
                            onChange={(e) => setEditText(e.target.value)}
                        />
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
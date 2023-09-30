import { FC } from "react"
import { Todo, TodoId } from "../types/todo"
import { Button, Card, Checkbox, Grid, Stack, Typography } from "@mui/material"

type props = {
    todo: Todo
    onUpdate: (todo: Todo) => void
    onDelete: (id: TodoId) => void
}

const TodoItem: FC<props> = ({ todo, onUpdate, onDelete }) => {
    const handleCompletedCheckbox = (todo: Todo) => {
        onUpdate({
            ...todo,
            completed: !todo.completed,
        })
    }

    const handleDeleteButton = () => {
        onDelete(todo.id)
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
                    <Grid item xs={9}>
                        <Stack spacing={1}>
                            <Typography variant="caption">
                                {todo.text.value}
                            </Typography>
                        </Stack>
                    </Grid>
                    <Grid item xs={1}>
                        <Button onClick={handleDeleteButton} color="error">
                            DELETE
                        </Button>
                    </Grid>
                </Grid>
            </Grid>
        </Card>
    )
}

export default TodoItem;
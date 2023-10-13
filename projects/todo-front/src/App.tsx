import { useState, FC, useEffect } from 'react'
import 'modern-css-reset'
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { CreateTodoPayload, Todo, TodoId, UpdateTodoPayload } from './types/todo'
import { Box, Stack, Typography } from '@mui/material'
import TodoForm from './components/TodoForm'
import TodoList from './components/TodoList'
import { createTodo, deleteTodo, getAllTodo, updateTodo } from './lib/api/todo'
import { CreateLabelPayload, Label, LabelId } from './types/label'
import { createLabel, deleteLabel, getAllLabel } from './lib/api/label'
import SideNav from './components/SideNav'

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([])
  const [labels, setLabels] = useState<Label[]>([])
  const [filterLabelId, setFilterLabelId] = useState<LabelId | null>(null)

  const onSubmit = async (payload: CreateTodoPayload) => {
    if (!payload.text) return
    await createTodo(payload)
    const todos = await getAllTodo()
    setTodos(todos)
  }

  const onUpdate = async (newTodo: UpdateTodoPayload) => {
    await updateTodo(newTodo)
    const todos = await getAllTodo();
    setTodos(todos)
  }

  const onDelete = async (id: TodoId) => {
    await deleteTodo(id)
    const todos = await getAllTodo();
    setTodos(todos)
  }

  const onSelectLabel = (label: Label | null) => {
    setFilterLabelId(label?.id ?? null)
  }

  const onSubmitCreateLabel = async (newLabel: CreateLabelPayload) => {
    if (labels.every((label) => label.name != newLabel.name)) {
      await createLabel(newLabel)
      const labels = await getAllLabel();
      setLabels(labels)
    }
  }

  const onDeleteLabel = async (id: LabelId) => {
    await deleteLabel(id)
    const labels = await getAllLabel();
    setLabels(labels)
  }

  const dispTodo = filterLabelId ? todos.filter((todo) => todo.labels.some((label) => label.id === filterLabelId)) : todos

  useEffect(() => {
    (async () => {
      const todos = await getAllTodo();
      setTodos(todos)
      const labels = await getAllLabel()
      setLabels(labels)
    })()
  }, [])

  return (
    <>
      <Box
        sx={{
          backgroundColor: 'white',
          borderBottom: '1px solid gray',
          display: 'flex',
          alignItems: 'center',
          position: 'fixed',
          top: 0,
          p: 2,
          width: '100%',
          height: '80',
          zIndex: 3,
        }}
      >
        <Typography variant="h1">Todo App</Typography>
      </Box>
      <Box sx={{
        backgroundColor: 'white',
        borderRight: '1px solid gray',
        position: 'fixed',
        height: 'calc(100% - 80px)',
        width: 200,
        zIndex: 2,
        left: 0,
      }}>
        <SideNav
          labels={labels}
          onSelectLabel={onSelectLabel}
          filterLabelId={filterLabelId}
          onSubmitCreateLabel={onSubmitCreateLabel}
          onDeleteLabel={onDeleteLabel}
        />
      </Box>
      <Box
        sx={{
          display: 'flex',
          justifyContent: 'center',
          p: 5,
          mt: 10
        }}
      >
        <Box maxWidth={700} width="100%">
          <Stack spacing={5}>
            <TodoForm onSubmit={onSubmit} labels={labels}/>
            <TodoList onUpdate={onUpdate} onDelete={onDelete} todos={dispTodo} labels={labels}/>
          </Stack>
        </Box>
      </Box>
    </>
  )
}

const theme = createTheme({
  typography: {
    h1: {
      fontSize: 30
    },
    h2: {
      fontSize: 20
    }
  }
})

const App: FC = () => {
  return (
    <ThemeProvider theme={theme}>
      <TodoApp />
    </ThemeProvider>
  )
}

export default App

import { useState, FC, useEffect } from 'react'
import 'modern-css-reset'
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { CreateTodoPayload, Todo, TodoId } from './types/todo'
import { Box, Stack, Typography } from '@mui/material'
import TodoForm from './components/TodoForm'
import TodoList from './components/TodoList'
import { createTodo, deleteTodo, getAllTodo, updateTodo } from './lib/api/todo'

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([])

  const onSubmit = async (payload: CreateTodoPayload) => {
    if (!payload.text.value) return
    await createTodo(payload)
    const todos = await getAllTodo()
    setTodos(todos)
  }

  const onUpdate = async (newTodo: Todo) => {
    await updateTodo(newTodo)
    const todos = await getAllTodo();
    setTodos(todos)
  }

  const onDelete = async (id: TodoId) => {
    await deleteTodo(id)
    const todos = await getAllTodo();
    setTodos(todos)
  }

  useEffect(() => {
    (async () => {
      const todos = await getAllTodo();
      setTodos(todos)
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
            <TodoForm onSubmit={onSubmit} />
            <TodoList onUpdate={onUpdate} onDelete={onDelete} todos={todos} />
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

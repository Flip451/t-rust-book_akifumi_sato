import { useState, FC } from 'react'
import 'modern-css-reset'
import {v4 as uuidv4} from 'uuid'
import { ThemeProvider, createTheme } from '@mui/material/styles'
import { CreateTodoPayload, Todo, TodoText } from './types/todo'
import { Box, Typography } from '@mui/material'
import TodoForm from './components/TodoForm'

const TodoApp: FC = () => {
  const [todos, setTodos] = useState<Todo[]>([])
  const createId = () => uuidv4()

  const onSubmit = async (payload: CreateTodoPayload) => {
    if(!payload.text.value) return
    setTodos((prev) => [
      {
        id: createId(),
        text: payload.text,
        completed: false
      },
      ...prev
    ])
  }

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
          <TodoForm onSubmit={onSubmit} />
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

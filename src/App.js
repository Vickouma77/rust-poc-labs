import React from 'react'
import { Container, Stack } from 'react-bootstrap'
import Header from './components/Header'
import Sidebar from './components/Sidebar'
import Posts from './components/Posts'

function App() {

  return (
    <Container fluid className='App'>
      <Header />
      <Container>
        <Stack direction='horizontal'>
          <Sidebar />
          <Container>
            <Posts />
          </Container>
        </Stack>
      </Container>
    </Container>
  )
}

export default App
import React from 'react'
import { Container } from 'react-bootstrap'
import Header from './components/Header'
import Posts from './components/Posts'
import Body from './components/Body'

function App() {

  return (
    <Container fluid className='App'>
      <Header />
      <Body sidebar>
        <Posts />
      </Body>
    </Container>
  )
}

export default App
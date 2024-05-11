import React from 'react'
import { Container, Stack } from 'react-bootstrap'
import Header from './components/Header'
import Sidebar from './components/Sidebar'

function App() {
  const posts = [
    {
      id: 1,
      text: 'Hello, Farmer!',
      timestap: 'a minute ago',
      author: {
        username: 'Madara',
      },
    },
    {
      id: 1,
      text: 'Farmers Save Lives',
      timestap: 'an hour ago',
      author: {
        username: 'Madara',
      },
    },

  ]

  return (
    <Container fluid className='App'>
      <Header />
      <Container>
        <Stack direction='horizontal'>
          <Sidebar />
          <Container>
            {posts.length === 0 ? 
              <p>Posts Not Available Yet</p>
            :
            posts.map(post => {
              return(
                <p key={post.id}>
                  <b>{post.author.username}</b> &mdash; {post.timestap}
                  <br />
                  {post.text}
                </p>
              )
            })
            }
          </Container>
        </Stack>
      </Container>
    </Container>
  )
}

export default App
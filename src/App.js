import React from 'react'

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
    <>
      <h1>AgriPulse</h1>
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
    </>
  )
}

export default App
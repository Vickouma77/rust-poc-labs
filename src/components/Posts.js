import { useState } from 'react';
import Spinner from 'react-bootstrap/Spinner';

function Posts() {
    const [posts, setPosts ] = useState();

  return (
    <>
      {posts === undefined ?
        <Spinner animation="border" />
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

export default Posts
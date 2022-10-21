import Button from '@suid/material/Button'
import { Component, createSignal } from 'solid-js'

export const HomePage: Component = () => {
  const [counter, setCounter] = createSignal(0)

  return (
    <>
      <h1>Hello world!</h1>
      <p>{counter()}</p>
      <p>
        <Button variant="contained" onClick={() => setCounter(counter() + 1)}>
          +
        </Button>
      </p>
    </>
  )
}

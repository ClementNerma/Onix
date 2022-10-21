import { Component, createSignal, onMount, Show } from 'solid-js'

export const HomePage: Component = () => {
  const [dockerVersion, setDockerVersion] = createSignal<string | null>(null)

  onMount(() => {
    setDockerVersion('coucou')
  })

  return (
    <Show when={dockerVersion() !== null} fallback={<h1>Loading...</h1>}>
      <h1>
        Docker version: <small>{dockerVersion()}</small>
      </h1>
    </Show>
  )
}

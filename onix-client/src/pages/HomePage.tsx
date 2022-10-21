import { useAppQuery } from '../graphql/generated'

export function HomePage() {
  const { data, loading, error } = useAppQuery()

  if (error) {
    return <h1>Failed: {error.message}</h1>
  }

  if (loading || !data) {
    return <h1>Loading...</h1>
  }

  return (
    <h1>
      Docker version: <small>{data.dockerVersion}</small>
    </h1>
  )
}

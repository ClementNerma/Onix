import { useAppQuery } from '../graphql/generated'
import { Box } from '@chakra-ui/react'

export function HomePage() {
  const { data, loading, error } = useAppQuery()

  if (error) {
    return <h1>Failed: {error.message}</h1>
  }

  if (loading || !data) {
    return <h1>Loading...</h1>
  }

  return <Box>{data.dockerVersion}</Box>
}

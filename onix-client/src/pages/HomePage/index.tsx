import { useHomePageQuery } from '../../graphql/generated'
import { Box } from '@chakra-ui/react'
import { AppsList } from './AppsList'

export function HomePage() {
  const { data, loading, error, refetch } = useHomePageQuery()

  if (error) {
    return <h2>Failed: {error.message}</h2>
  }

  if (loading || !data) {
    return <h2>Loading...</h2>
  }

  return (
    <Box>
      <AppsList apps={data.apps} onFinished={() => refetch()} />
    </Box>
  )
}

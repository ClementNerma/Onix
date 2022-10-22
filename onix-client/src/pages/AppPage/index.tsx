import { Box, Heading } from '@chakra-ui/react'
import { AppStatus } from '../../atoms/AppStatus'
import { BorderedBox } from '../../atoms/BorderedBox'
import { useAppPageQuery } from '../../graphql/generated'
import { AppActions } from '../../molecules/AppActions'
import { useParams } from '../../router'

export const AppPage = () => {
  const { appId } = useParams(['appId'])

  const { data, loading, error, refetch } = useAppPageQuery({ variables: { id: appId } })

  if (error) {
    return <Heading size="lg">Failed: {error.message}</Heading>
  }

  if (loading || !data) {
    return <Heading size="lg">Loading...</Heading>
  }

  const { app } = data

  return (
    <Box>
      <Heading size="lg">Application '{app.name}'</Heading>

      <BorderedBox>
        Status: <AppStatus status={app.fetchedStatus} />
      </BorderedBox>
      <BorderedBox>
        Actions: <AppActions size="sm" appId={app.id} status={app.fetchedStatus} onFinished={() => refetch()} />
      </BorderedBox>
    </Box>
  )
}

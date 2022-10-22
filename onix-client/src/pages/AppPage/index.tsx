import { Box, Heading, Table, Tbody, Td, Tr } from '@chakra-ui/react'
import { AppStatus } from '../../atoms/AppStatus'
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

      <Table>
        <Tbody>
          <Tr>
            <Td>Status</Td>
            <Td>
              <AppStatus status={app.fetchedStatus} />
            </Td>
          </Tr>
          <Tr>
            <Td>Actions</Td>
            <Td>
              <AppActions size="sm" appId={app.id} status={app.fetchedStatus} onFinished={() => refetch()} />
            </Td>
          </Tr>
        </Tbody>
      </Table>
    </Box>
  )
}

import { HStack, Table, TableContainer, Tbody, Td, Th, Thead, Tr } from '@chakra-ui/react'
import { ActionButtonState } from '../../atoms/ActionButton'
import { AppStatus } from '../../atoms/AppStatus'
import { HomePageQuery } from '../../graphql/generated'
import { AppActions } from '../../molecules/AppActions'
import { Link } from '../../atoms/Link'

export type AppsListProps = {
  apps: HomePageQuery['apps']
  onStateChange?: (state: ActionButtonState) => void
  onFinished?: (succeeded: boolean) => void
}

export const AppsList = ({ apps, onStateChange, onFinished }: AppsListProps) => {
  return (
    <TableContainer>
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>Application name</Th>
            <Th>Containers</Th>
            <Th>Status</Th>
            <Th>Actions</Th>
          </Tr>
        </Thead>
        <Tbody>
          {apps.map((app) => (
            <Tr key={app.id}>
              <Td>
                <Link to={`/apps/${app.id}`}> {app.name}</Link>
              </Td>
              <Td>{app.containers.length}</Td>
              <Td>
                <AppStatus size="lg" status={app.fetchedStatus} />
              </Td>
              <Td>
                <HStack>
                  <AppActions
                    appId={app.id}
                    status={app.fetchedStatus}
                    onStateChange={onStateChange}
                    onFinished={onFinished}
                  />
                </HStack>
              </Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  )
}

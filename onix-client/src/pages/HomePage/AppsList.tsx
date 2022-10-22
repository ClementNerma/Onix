import { HStack, Table, TableContainer, Tbody, Td, Th, Thead, Tr } from '@chakra-ui/react'
import { useEffect, useState } from 'react'
import { ActionButtonState } from '../../atoms/ActionButton'
import { AppStatus } from '../../atoms/AppStatus'
import { AppRunningStatus, HomePageQuery } from '../../graphql/generated'
import { AppActions } from '../../molecules/AppActions'

export type AppsListProps = {
  apps: HomePageQuery['apps']
  onStateChange: (state: ActionButtonState) => void
}

export const AppsList = ({ apps, onStateChange }: AppsListProps) => {
  const [appStatuses, setAppStatuses] = useState<Record<string, AppRunningStatus>>({})

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
              <Td>{app.name}</Td>
              <Td>{app.containers.length}</Td>
              <Td>
                <AppStatus status={app.fetchedStatus} />
              </Td>
              <Td>
                <HStack>
                  <AppActions appId={app.id} status={app.fetchedStatus} onStateChange={onStateChange} />
                </HStack>
              </Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  )
}

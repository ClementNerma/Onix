import { Table, TableContainer, Tbody, Td, Th, Thead, Tr } from '@chakra-ui/react'
import { AppFragment } from '../../graphql/generated'

export type AppsListProps = {
  apps: AppFragment[]
}

export const AppsList = ({ apps }: AppsListProps) => (
  <TableContainer>
    <Table variant="simple">
      <Thead>
        <Tr>
          <Th>Application name</Th>
          <Th>ID</Th>
          <Th>Containers</Th>
          <Th>Status</Th>
          <Th>Actions</Th>
        </Tr>
      </Thead>
      <Tbody>
        {apps.map((app) => (
          <Tr key={app.id}>
            <Td>{app.name}</Td>
            <Td>{app.id}</Td>
            <Td>{app.containers.length}</Td>
            <Td>-</Td>
            <Td>TODO</Td>
          </Tr>
        ))}
      </Tbody>
    </Table>
  </TableContainer>
)

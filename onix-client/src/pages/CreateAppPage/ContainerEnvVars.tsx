import { TableContainer, Table, Thead, Tr, Th, Tbody, Td, Input, IconButton, Button } from '@chakra-ui/react'
import { useCallback } from 'react'
import { MdAdd, MdDelete } from 'react-icons/md'
import { AppContainerTemplateInput, ContainerEnvironmentVarInput } from '../../graphql/generated'

export type ContainerEnvVarsProps = {
  state: AppContainerTemplateInput['envVars']
  onChange: (state: AppContainerTemplateInput['envVars']) => void
}

export const ContainerEnvVars = ({ state, onChange }: ContainerEnvVarsProps) => {
  const updateVar = useCallback(
    (content: ContainerEnvironmentVarInput, index: number) => {
      onChange([...state.slice(0, index), content, ...state.slice(index + 1)])
    },
    [state, onChange],
  )

  const removeVar = useCallback(
    (index: number) => {
      onChange([...state.slice(0, index), ...state.slice(index + 1)])
    },
    [state, onChange],
  )

  const addVar = useCallback(() => {
    onChange([...state, { name: '', value: '' }])
  }, [state, onChange])

  return (
    <TableContainer>
      <Table>
        <Thead>
          <Tr>
            <Th>Actions</Th>
            <Th>Name</Th>
            <Th>Value</Th>
          </Tr>
        </Thead>
        <Tbody>
          {state.map(({ name, value }, i) => (
            <Tr key={i}>
              <Td>
                <IconButton size="xs" as={MdDelete} onClick={() => removeVar(i)} aria-label="Remove this container" />
              </Td>
              <Td>
                <Input type="text" value={name} onChange={(e) => updateVar({ name: e.target.value, value }, i)} />
              </Td>
              <Td>
                <Input type="text" value={value} onChange={(e) => updateVar({ name, value: e.target.value }, i)} />
              </Td>
            </Tr>
          ))}
          <Tr>
            <Td colSpan={3}>
              <Button colorScheme="green" size="sm" leftIcon={<MdAdd />} onClick={addVar}>
                Add a variable
              </Button>
            </Td>
          </Tr>
        </Tbody>
      </Table>
    </TableContainer>
  )
}

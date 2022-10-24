import { Heading, HStack, IconButton } from '@chakra-ui/react'
import { MdDelete } from 'react-icons/md'
import { BoxedStack } from '../../atoms/BoxedStack'
import { AppContainerCreationInput } from '../../graphql/generated'
import { ValidableInput } from '../../molecules/ValidableInput/ValidableInput'
import { ContainerEnvVars } from './ContainerEnvVars'
import { ContainerPortBindings } from './ContainerPortBindings'

export type CreateAppContainerProps = {
  state: AppContainerCreationInput
  onChange: (state: AppContainerCreationInput) => void
  onRemove: () => void
}

export const CreateAppContainer = ({ state, onChange, onRemove }: CreateAppContainerProps) => {
  const { name, image, envVars, portBindings, volumes, dependsOn } = state

  return (
    <BoxedStack spacing={5}>
      <HStack>
        <IconButton size="xs" as={MdDelete} onClick={onRemove} aria-label="Remove this container" />
        <Heading size="md">Container &quot;{state.name}&quot;</Heading>
      </HStack>

      <ValidableInput
        type="text"
        label="Container name"
        value={name}
        onChange={(name) => onChange({ ...state, name })}
        helper="Must only be letters, digits, dashes and underscores."
      />

      <ValidableInput
        type="text"
        label="Docker image"
        value={image}
        onChange={(image) => onChange({ ...state, image })}
      />

      <Heading size="sm">Environment variables</Heading>

      <ContainerEnvVars state={envVars} onChange={(envVars) => onChange({ ...state, envVars })} />

      <Heading size="sm">Port bindings</Heading>

      <ContainerPortBindings state={portBindings} onChange={(portBindings) => onChange({ ...state, portBindings })} />
    </BoxedStack>
  )
}

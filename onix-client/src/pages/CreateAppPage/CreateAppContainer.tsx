import { Button, Heading, HStack, Icon, IconButton } from '@chakra-ui/react'
import { MdDelete } from 'react-icons/md'
import { BoxedStack } from '../../atoms/BoxedStack'
import { AppContainerTemplateInput } from '../../graphql/generated'
import { ValidableInput } from '../../molecules/ValidableInput/ValidableInput'
import { ContainerEnvVars } from './ContainerEnvVars'
import { ContainerPortBindings } from './ContainerPortBindings'
import { ContainerVolumes } from './ContainersVolumes'

export type CreateAppContainerProps = {
  state: AppContainerTemplateInput
  onChange: (state: AppContainerTemplateInput) => void
  onRemove: () => void
}

export const CreateAppContainer = ({ state, onChange, onRemove }: CreateAppContainerProps) => {
  const { name, image, envVars, portBindings, volumes, dependsOn } = state

  return (
    <BoxedStack spacing={5}>
      <Heading size="md">
        Container: {state.name}
        <Button onClick={onRemove} size="sm">
          <Icon as={MdDelete} /> Remove
        </Button>
      </Heading>

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

      <Heading size="sm">Volumes</Heading>

      <ContainerVolumes state={volumes} onChange={(volumes) => onChange({ ...state, volumes })} />
    </BoxedStack>
  )
}

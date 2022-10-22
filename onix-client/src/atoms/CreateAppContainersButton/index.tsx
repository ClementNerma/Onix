import { ButtonProps } from '@chakra-ui/react'
import { MdAddCircle } from 'react-icons/md'
import { useCreateAppContainerMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type CreateAppContainersButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
  onFinished?: (succeeded: boolean) => void
} & Omit<ButtonProps, 'onClick'>

export const CreateAppContainersButton = ({
  appId,
  onStateChange,
  onFinished,
  ...rest
}: CreateAppContainersButtonProps) => {
  const [createAppContainers, result] = useCreateAppContainerMutation()

  return (
    <ActionButton
      icon={<MdAddCircle />}
      colorScheme="blue"
      size="sm"
      onClick={() => createAppContainers({ variables: { id: appId } })}
      label="Create app containers"
      state={result}
      onStateChange={onStateChange}
      onFinished={onFinished}
      {...rest}
    />
  )
}

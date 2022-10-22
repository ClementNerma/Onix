import { MdAddCircle } from 'react-icons/md'
import { useCreateAppContainerMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type CreateAppContainersButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
}

export const CreateAppContainersButton = ({ appId, onStateChange }: CreateAppContainersButtonProps) => {
  const [createAppContainers, result] = useCreateAppContainerMutation()

  return (
    <ActionButton
      icon={<MdAddCircle />}
      colorScheme="blue"
      size="sm"
      onClick={() => createAppContainers({ variables: { id: appId } })}
      label="Create app containers"
      state={result}
    />
  )
}

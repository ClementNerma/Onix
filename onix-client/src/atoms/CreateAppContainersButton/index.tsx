import { useEffect } from 'react'
import { MdAddCircle } from 'react-icons/md'
import { useCreateAppContainerMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type CreateAppContainersButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
}

export const CreateAppContainersButton = ({ appId, onStateChange }: CreateAppContainersButtonProps) => {
  const [createAppContainers, result] = useCreateAppContainerMutation()

  useEffect(() => {
    if (result.loading) {
      onStateChange?.('loading')
    } else if (result.error) {
      onStateChange?.('failed')
    } else if (result.data) {
      onStateChange?.('done')
    }
  }, [result, onStateChange])

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

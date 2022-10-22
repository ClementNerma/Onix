import { MdPlayArrow } from 'react-icons/md'
import { useStartAppMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type StartAppButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
}

export const StartAppButton = ({ appId, onStateChange }: StartAppButtonProps) => {
  const [startApp, result] = useStartAppMutation()

  return (
    <ActionButton
      icon={<MdPlayArrow />}
      colorScheme="blue"
      size="sm"
      onClick={() => startApp({ variables: { id: appId } })}
      label="Start"
      state={result}
      onStateChange={onStateChange}
    />
  )
}

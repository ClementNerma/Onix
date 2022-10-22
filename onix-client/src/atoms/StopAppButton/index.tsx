import { ButtonProps } from '@chakra-ui/react'
import { MdStop } from 'react-icons/md'
import { useStopAppMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type StopAppButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
  onFinished?: (succeeded: boolean) => void
} & Omit<ButtonProps, 'onClick'>

export const StopAppButton = ({ appId, onStateChange, onFinished, ...rest }: StopAppButtonProps) => {
  const [stopApp, result] = useStopAppMutation()

  return (
    <ActionButton
      icon={<MdStop />}
      colorScheme="yellow"
      size="sm"
      onClick={() => stopApp({ variables: { id: appId } })}
      label="Stop"
      state={result}
      onStateChange={onStateChange}
      onFinished={onFinished}
      {...rest}
    />
  )
}

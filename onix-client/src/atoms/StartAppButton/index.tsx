import { ButtonProps } from '@chakra-ui/react'
import { MdPlayArrow } from 'react-icons/md'
import { useStartAppMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type StartAppButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
  onFinished?: (succeeded: boolean) => void
} & Omit<ButtonProps, 'onClick'>

export const StartAppButton = ({ appId, onStateChange, onFinished, ...rest }: StartAppButtonProps) => {
  const [startApp, result] = useStartAppMutation()

  return (
    <ActionButton
      icon={<MdPlayArrow />}
      colorScheme="blue"
      size="sm"
      onClick={() => startApp({ variables: { id: appId } })}
      label="Start"
      state={result}
      errorTitle="Failed to start the application"
      onStateChange={onStateChange}
      onFinished={onFinished}
      {...rest}
    />
  )
}

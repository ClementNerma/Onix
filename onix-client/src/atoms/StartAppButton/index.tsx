import { ButtonProps, useToast } from '@chakra-ui/react'
import { useEffect } from 'react'
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
  const toast = useToast()

  useEffect(() => {
    if (result.error) {
      toast({
        title: 'Failed to start application',
        description: result.error.message,
        status: 'error',
      })
    }
  }, [result.error])

  return (
    <ActionButton
      icon={<MdPlayArrow />}
      colorScheme="blue"
      size="sm"
      onClick={() => startApp({ variables: { id: appId } })}
      label="Start"
      state={result}
      onStateChange={onStateChange}
      onFinished={onFinished}
      {...rest}
    />
  )
}

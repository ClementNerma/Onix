import { useEffect } from 'react'
import { MdStop } from 'react-icons/md'
import { useStopAppMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type StopAppButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
}

export const StopAppButton = ({ appId, onStateChange }: StopAppButtonProps) => {
  const [stopApp, result] = useStopAppMutation()

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
      icon={<MdStop />}
      colorScheme="yellow"
      size="sm"
      onClick={() => stopApp({ variables: { id: appId } })}
      label="Stop"
      state={result}
    />
  )
}

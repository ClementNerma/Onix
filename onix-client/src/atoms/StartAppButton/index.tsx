import { useEffect } from 'react'
import { MdPlayArrow } from 'react-icons/md'
import { useStartAppMutation } from '../../graphql/generated'
import { ActionButton, ActionButtonState } from '../ActionButton'

export type StartAppButtonProps = {
  appId: string
  onStateChange?: (state: ActionButtonState) => void
}

export const StartAppButton = ({ appId, onStateChange }: StartAppButtonProps) => {
  const [startApp, result] = useStartAppMutation()

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
      icon={<MdPlayArrow />}
      colorScheme="blue"
      size="sm"
      onClick={() => startApp({ variables: { id: appId } })}
      label="Start"
      state={result}
    />
  )
}

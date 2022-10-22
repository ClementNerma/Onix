import { MutationResult } from '@apollo/client'
import { Button, ButtonProps, Spinner } from '@chakra-ui/react'
import React, { useEffect } from 'react'
import { MdError } from 'react-icons/md'

export type ActionButtonProps = {
  icon: React.ReactElement
  loadingIcon?: React.ReactElement
  errorIcon?: React.ReactElement
  label: string
  state: MutationResult<unknown>
  onClick: () => void
  onStateChange?: (state: ActionButtonState) => void
} & ButtonProps

export enum ActionButtonState {
  Loading,
  Failed,
  Done,
}

export const ActionButton = ({
  icon,
  loadingIcon,
  errorIcon,
  label,
  state,
  onClick,
  onStateChange,
  ...rest
}: ActionButtonProps) => {
  const dynIcon = state.error ? errorIcon ?? <MdError /> : loadingIcon ?? state.loading ? <Spinner /> : icon

  useEffect(() => {
    if (state.loading) {
      onStateChange?.(ActionButtonState.Loading)
    } else if (state.error) {
      onStateChange?.(ActionButtonState.Failed)
    } else if (state.data) {
      onStateChange?.(ActionButtonState.Done)
    }
  }, [state, onStateChange])

  return (
    <Button
      leftIcon={dynIcon}
      onClick={() => state !== 'loading' && onClick()}
      disabled={state === 'loading' || state === 'done'}
      {...rest}
    >
      {label}
    </Button>
  )
}

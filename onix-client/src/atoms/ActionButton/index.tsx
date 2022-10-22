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
  onFinished?: (succeeded: boolean) => void
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
  onFinished,
  ...rest
}: ActionButtonProps) => {
  errorIcon ??= <MdError />
  loadingIcon ??= <Spinner size="sm" />

  const dynIcon = state.error ? errorIcon : state.loading ? loadingIcon : icon

  useEffect(() => {
    if (state.loading) {
      onStateChange?.(ActionButtonState.Loading)
    } else if (state.error) {
      onStateChange?.(ActionButtonState.Failed)
      onFinished?.(false)
    } else if (state.data !== null) {
      onStateChange?.(ActionButtonState.Done)
      onFinished?.(true)
    }
  }, [state, onStateChange])

  const isActive = !state.loading && !Boolean(state.data)

  return (
    <Button {...rest} leftIcon={dynIcon} onClick={onClick} disabled={!isActive}>
      {label}
    </Button>
  )
}

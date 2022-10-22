import { MutationResult } from '@apollo/client'
import { Button, ButtonProps, Spinner } from '@chakra-ui/react'
import React from 'react'
import { MdError } from 'react-icons/md'

export type ActionButtonProps = {
  icon: React.ReactElement
  loadingIcon?: React.ReactElement
  errorIcon?: React.ReactElement
  label: string
  state: ActionButtonState | MutationResult<unknown>
  onClick: () => void
} & ButtonProps

export type ActionButtonState = 'actionable' | 'loading' | 'failed' | 'done'

export const ActionButton = ({ icon, loadingIcon, errorIcon, label, state, onClick, ...rest }: ActionButtonProps) => {
  if (typeof state !== 'string') {
    state = state.loading ? 'loading' : state.error ? 'failed' : state.data ? 'done' : 'actionable'
  }

  const dynIcon =
    state === 'actionable' || state === 'done'
      ? icon
      : state === 'loading'
      ? loadingIcon ?? <Spinner />
      : errorIcon ?? <MdError />

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

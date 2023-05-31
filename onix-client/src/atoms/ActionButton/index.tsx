import { MutationResult, QueryResult } from '@apollo/client'
import { Button, ButtonProps, Spinner, useToast } from '@chakra-ui/react'
import React, { useEffect } from 'react'
import { MdError } from 'react-icons/md'

export type ActionButtonProps = {
	icon: React.ReactElement
	loadingIcon?: React.ReactElement
	errorIcon?: React.ReactElement
	label: string
	state: Pick<MutationResult<unknown> | QueryResult<unknown, Record<string, never>>, 'loading' | 'error' | 'data'>
	errorTitle?: string
	redoable?: boolean
	onClick: () => void
	onStateChange?: (state: ActionButtonState) => void
	onFinished?: (succeeded: boolean) => void
} & ButtonProps

export enum ActionButtonState {
	Loading = 'Loading',
	Failed = 'Failed',
	Done = 'Done',
}

export const ActionButton = ({
	icon,
	loadingIcon,
	errorIcon,
	label,
	state,
	errorTitle,
	redoable,
	onClick,
	onStateChange,
	onFinished,
	...rest
}: ActionButtonProps) => {
	const toast = useToast()

	errorIcon ??= <MdError />
	loadingIcon ??= <Spinner size='sm' />

	const dynIcon = state.error ? errorIcon : state.loading ? loadingIcon : icon

	useEffect(() => {
		if (state.loading) {
			onStateChange?.(ActionButtonState.Loading)
		} else if (state.error) {
			onStateChange?.(ActionButtonState.Failed)
			onFinished?.(false)

			toast({
				title: errorTitle ?? 'Action failed',
				description: state.error.message,
				status: 'error',
			})
		} else if (state.data !== null) {
			onStateChange?.(ActionButtonState.Done)
			onFinished?.(true)
		}
	}, [state, onStateChange, onFinished, toast, errorTitle])

	const isDisabled = state.loading || (redoable !== true && Boolean(state.data))

	return (
		<Button {...rest} leftIcon={dynIcon} onClick={onClick} disabled={isDisabled}>
			{label}
		</Button>
	)
}

import {
	ButtonProps,
	Modal,
	ModalBody,
	ModalCloseButton,
	ModalContent,
	ModalHeader,
	ModalOverlay,
	Tag,
	useDisclosure,
} from '@chakra-ui/react'
import {
	AppRunningStatus,
	useCreateAppContainersMutation,
	useStartAppMutation,
	useStopAppMutation,
	useDestroyAppContainerMutation,
	useRemoveAppMutation,
	useGenerateAppTemplateLazyQuery,
} from '../../graphql/generated'
import { assertNever } from '../../utils'
import { ActionButton, ActionButtonState } from '../../atoms/ActionButton'
import { MdAddCircle, MdDelete, MdOutlineTextSnippet, MdPlayArrow, MdStop } from 'react-icons/md'
import { ConfirmModal } from '../../organisms/ConfirmModal'
import { useEffect } from 'react'
import styled from '@emotion/styled'

export type AppActionProps = {
	appId: string
	status: AppRunningStatus
	onStateChange?: (state: ActionButtonState) => void
	onFinished?: (succeeded: boolean) => void
} & ButtonProps

export const AppActions = (props: AppActionProps) => {
	const [generateTemplate, templateGeneration] = useGenerateAppTemplateLazyQuery()
	const { isOpen, onOpen, onClose } = useDisclosure()

	useEffect(() => {
		if (templateGeneration.data) {
			onOpen()
		}
	}, [templateGeneration, onOpen])

	return (
		<>
			<ActionButton
				size="sm"
				mr={2}
				icon={<MdOutlineTextSnippet />}
				label="Generate template"
				onClick={() => generateTemplate({ variables: { id: props.appId } })}
				redoable
				state={templateGeneration}
			/>

			<AppDynamicActions {...props} />

			<Modal isOpen={isOpen} onClose={onClose}>
				<ModalOverlay />
				<ModalContent>
					<ModalHeader>Application template</ModalHeader>
					<ModalCloseButton />

					<ModalBody>
						<CodeBlock>{templateGeneration.data?.app.generateTemplate}</CodeBlock>
					</ModalBody>
				</ModalContent>
			</Modal>
		</>
	)
}

export const AppDynamicActions = ({ appId, status, onStateChange, onFinished, ...rest }: AppActionProps) => {
	switch (status) {
		case AppRunningStatus.NotCreated:
			return (
				<>
					<CreateAppContainersButton
						mr={2}
						appId={appId}
						onStateChange={onStateChange}
						onFinished={onFinished}
						{...rest}
					/>
					<RemoveAppButton appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />
				</>
			)

		case AppRunningStatus.Stopped:
			return (
				<>
					<StartAppButton mr={2} appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />
					<DestroyAppContainersButton appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />
				</>
			)

		case AppRunningStatus.PartiallyCreated:
		case AppRunningStatus.Zombie:
		case AppRunningStatus.Intermediary:
		case AppRunningStatus.PartiallyRunning:
			// TODO
			return <Tag colorScheme="error">Unimplemented</Tag>

		case AppRunningStatus.FullyRunning:
			return <StopAppButton appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />

		default:
			return assertNever(status)
	}
}

type AppActionButtonProps = {
	appId: string
	onStateChange?: (state: ActionButtonState) => void
	onFinished?: (succeeded: boolean) => void
} & Omit<ButtonProps, 'onClick'>

const CreateAppContainersButton = ({ appId, onStateChange, onFinished, ...rest }: AppActionButtonProps) => {
	const [createAppContainers, result] = useCreateAppContainersMutation()

	return (
		<ActionButton
			icon={<MdAddCircle />}
			colorScheme="blue"
			size="sm"
			onClick={() => createAppContainers({ variables: { id: appId } })}
			label="Create containers"
			state={result}
			errorTitle="Failed to create application"
			onStateChange={onStateChange}
			onFinished={onFinished}
			{...rest}
		/>
	)
}

const StartAppButton = ({ appId, onStateChange, onFinished, ...rest }: AppActionButtonProps) => {
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

const StopAppButton = ({ appId, onStateChange, onFinished, ...rest }: AppActionButtonProps) => {
	const [stopApp, result] = useStopAppMutation()

	return (
		<ActionButton
			icon={<MdStop />}
			colorScheme="yellow"
			size="sm"
			onClick={() => stopApp({ variables: { id: appId } })}
			label="Stop"
			state={result}
			errorTitle="Failed to stop the application"
			onStateChange={onStateChange}
			onFinished={onFinished}
			{...rest}
		/>
	)
}

const DestroyAppContainersButton = ({ appId, onStateChange, onFinished, ...rest }: AppActionButtonProps) => {
	const [destroyAppContainers, result] = useDestroyAppContainerMutation()
	const { isOpen, onOpen, onClose } = useDisclosure()

	return (
		<>
			<ActionButton
				icon={<MdDelete />}
				colorScheme="red"
				size="sm"
				onClick={onOpen}
				label="Remove containers"
				state={result}
				errorTitle="Failed to destroy the application's containers"
				onStateChange={onStateChange}
				onFinished={onFinished}
				{...rest}
			/>

			<ConfirmModal
				isOpen={isOpen}
				onClose={onClose}
				onConfirm={() => destroyAppContainers({ variables: { id: appId } })}
			/>
		</>
	)
}

const RemoveAppButton = ({ appId, onStateChange, onFinished, ...rest }: AppActionButtonProps) => {
	const [removeApp, result] = useRemoveAppMutation()
	const { isOpen, onOpen, onClose } = useDisclosure()

	return (
		<>
			<ActionButton
				icon={<MdDelete />}
				colorScheme="red"
				size="sm"
				onClick={onOpen}
				label="Delete"
				state={result}
				errorTitle="Failed to remove the application"
				onStateChange={onStateChange}
				onFinished={onFinished}
				{...rest}
			/>

			<ConfirmModal isOpen={isOpen} onClose={onClose} onConfirm={() => removeApp({ variables: { id: appId } })} />
		</>
	)
}

const CodeBlock = styled('pre')`
  background-color: #f0f0f0;
  border-radius: 5px;
  padding: 0.5rem;
`

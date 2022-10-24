import { ButtonProps, Tag } from '@chakra-ui/react'
import {
  AppRunningStatus,
  useCreateAppContainerMutation,
  useStartAppMutation,
  useStopAppMutation,
} from '../../graphql/generated'
import { assertNever } from '../../utils'
import { ActionButton, ActionButtonState } from '../../atoms/ActionButton'
import { MdAddCircle, MdPlayArrow, MdStop } from 'react-icons/md'

export type AppActionProps = {
  appId: string
  status: AppRunningStatus
  onStateChange?: (state: ActionButtonState) => void
  onFinished?: (succeeded: boolean) => void
} & ButtonProps

export const AppActions = ({ appId, status, onStateChange, onFinished, ...rest }: AppActionProps) => {
  switch (status) {
    case AppRunningStatus.NotCreated:
      return <CreateAppContainersButton appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />

    case AppRunningStatus.Stopped:
      return <StartAppButton appId={appId} onStateChange={onStateChange} onFinished={onFinished} {...rest} />

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
  const [createAppContainers, result] = useCreateAppContainerMutation()

  return (
    <ActionButton
      icon={<MdAddCircle />}
      colorScheme="blue"
      size="sm"
      onClick={() => createAppContainers({ variables: { id: appId } })}
      label="Create app containers"
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

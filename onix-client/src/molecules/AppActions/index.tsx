import { Tag } from '@chakra-ui/react'
import { AppRunningStatus } from '../../graphql/generated'
import { assertNever } from '../../utils'
import { CreateAppContainersButton } from '../../atoms/CreateAppContainersButton'
import { StopAppButton } from '../../atoms/StopAppButton'
import { StartAppButton } from '../../atoms/StartAppButton'
import { ActionButtonState } from '../../atoms/ActionButton'

export type AppActionProps = {
  appId: string
  status: AppRunningStatus
  onStateChange?: (state: ActionButtonState) => void
}

export const AppActions = ({ appId, status, onStateChange }: AppActionProps) => {
  switch (status) {
    case AppRunningStatus.NotCreated:
      return <CreateAppContainersButton appId={appId} onStateChange={onStateChange} />

    case AppRunningStatus.Stopped:
      return <StartAppButton appId={appId} onStateChange={onStateChange} />

    case AppRunningStatus.PartiallyCreated:
    case AppRunningStatus.Zombie:
    case AppRunningStatus.Intermediary:
    case AppRunningStatus.PartiallyRunning:
      // TODO
      return <Tag colorScheme="error">Unimplemented</Tag>

    case AppRunningStatus.FullyRunning:
      return <StopAppButton appId={appId} onStateChange={onStateChange} />

    default:
      return assertNever(status)
  }
}

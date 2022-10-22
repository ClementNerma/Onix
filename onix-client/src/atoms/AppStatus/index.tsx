import { Tag } from '@chakra-ui/react'
import { AppRunningStatus } from '../../graphql/generated'
import { assertNever } from '../../utils'

export type AppStatusProps = {
  status: AppRunningStatus
}

export const AppStatus = ({ status }: AppStatusProps) => {
  switch (status) {
    case AppRunningStatus.NotCreated:
      return <Tag colorScheme="gray">Not created</Tag>

    case AppRunningStatus.PartiallyCreated:
      return <Tag colorScheme="red">Partially created</Tag>

    case AppRunningStatus.Zombie:
      return <Tag colorScheme="red">Zombie</Tag>

    case AppRunningStatus.Intermediary:
      return <Tag colorScheme="yellow">Intermediary</Tag>

    case AppRunningStatus.Stopped:
      return <Tag colorScheme="gray">Stopped</Tag>

    case AppRunningStatus.PartiallyRunning:
      return <Tag colorScheme="yellow">Partially running</Tag>

    case AppRunningStatus.FullyRunning:
      return <Tag colorScheme="green">Running</Tag>

    default:
      return assertNever(status)
  }
}

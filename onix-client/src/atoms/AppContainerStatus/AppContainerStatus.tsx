import { Tag, TagProps } from '@chakra-ui/react'
import { ExistingContainerStatus } from '../../graphql/generated'
import { assertNever } from '../../utils'

export type AppContainerStatusProps = {
  status: ExistingContainerStatus | null | undefined
} & TagProps

export const AppContainerStatus = ({ status, ...rest }: AppContainerStatusProps) => {
  switch (status) {
    case null:
    case undefined:
      return (
        <Tag colorScheme="gray" {...rest}>
          Not created
        </Tag>
      )

    case ExistingContainerStatus.Created:
      return (
        <Tag colorScheme="gray" {...rest}>
          Created
        </Tag>
      )

    case ExistingContainerStatus.Dead:
      return (
        <Tag colorScheme="red" {...rest}>
          Dead
        </Tag>
      )

    case ExistingContainerStatus.Paused:
      return (
        <Tag colorScheme="yellow" {...rest}>
          Paused
        </Tag>
      )

    case ExistingContainerStatus.Removing:
      return (
        <Tag colorScheme="yellow" {...rest}>
          Removing
        </Tag>
      )

    case ExistingContainerStatus.Restarting:
      return (
        <Tag colorScheme="yellow" {...rest}>
          Restarting
        </Tag>
      )

    case ExistingContainerStatus.Exited:
      return (
        <Tag colorScheme="gray" {...rest}>
          Stopped
        </Tag>
      )

    case ExistingContainerStatus.Running:
      return (
        <Tag colorScheme="green" {...rest}>
          Running
        </Tag>
      )

    default:
      return assertNever(status)
  }
}

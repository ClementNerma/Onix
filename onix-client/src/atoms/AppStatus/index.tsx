import { Tag, TagProps } from '@chakra-ui/react'
import { AppRunningStatus } from '../../graphql/generated'
import { assertNever } from '../../utils'

export type AppStatusProps = {
	status: AppRunningStatus
} & TagProps

export const AppStatus = ({ status, ...rest }: AppStatusProps) => {
	switch (status) {
		case AppRunningStatus.NotCreated:
			return (
				<Tag colorScheme="gray" {...rest}>
					Not created
				</Tag>
			)

		case AppRunningStatus.PartiallyCreated:
			return (
				<Tag colorScheme="red" {...rest}>
					Partially created
				</Tag>
			)

		case AppRunningStatus.Zombie:
			return (
				<Tag colorScheme="red" {...rest}>
					Zombie
				</Tag>
			)

		case AppRunningStatus.Intermediary:
			return (
				<Tag colorScheme="yellow" {...rest}>
					Intermediary
				</Tag>
			)

		case AppRunningStatus.Stopped:
			return (
				<Tag colorScheme="gray" {...rest}>
					Stopped
				</Tag>
			)

		case AppRunningStatus.PartiallyRunning:
			return (
				<Tag colorScheme="yellow" {...rest}>
					Partially running
				</Tag>
			)

		case AppRunningStatus.FullyRunning:
			return (
				<Tag colorScheme="green" {...rest}>
					Running
				</Tag>
			)

		default:
			return assertNever(status)
	}
}

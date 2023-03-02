import { Box, Heading, Table, Tbody, Td, Th, Thead, Tr } from '@chakra-ui/react'
import { AppContainerStatus } from '../../atoms/AppContainerStatus/AppContainerStatus'
import { AppStatus } from '../../atoms/AppStatus'
import { useAppPageQuery } from '../../graphql/generated'
import { AppActions } from '../../molecules/AppActions'
import { useParams } from '../../router'

export const AppPage = () => {
	const { appId } = useParams(['appId'])

	const { data, loading, error, refetch } = useAppPageQuery({ variables: { id: appId } })

	if (error) {
		return <Heading size="lg">Failed: {error.message}</Heading>
	}

	if (loading || !data) {
		return <Heading size="lg">Loading...</Heading>
	}

	const { app } = data

	return (
		<Box>
			<Heading size="lg">Application &apos;{app.name}&apos;</Heading>

			<Table variant="unstyled" display="inline-block">
				<Tbody>
					<Tr>
						<Td>Status</Td>
						<Td>
							<AppStatus status={app.fetchedStatus} />
						</Td>
					</Tr>
					<Tr>
						<Td>Actions</Td>
						<Td>
							<AppActions size="sm" appId={app.id} status={app.fetchedStatus} onFinished={() => refetch()} />
						</Td>
					</Tr>
				</Tbody>
			</Table>

			<Heading size="md">Containers</Heading>

			<Table>
				<Thead>
					<Tr>
						<Th>Container name</Th>
						<Th>Name in docker</Th>
						<Th>Image</Th>
						<Th>Status</Th>
						<Th>Volumes</Th>
						<Th>Actions</Th>
					</Tr>
				</Thead>
				<Tbody>
					{app.containers.map((container) => (
						<Tr key={container.id}>
							<Td>{container.name}</Td>
							<Td>{container.dockerContainer?.dockerContainerName ?? '-'}</Td>
							<Td>{container.image}</Td>
							<Td>
								<AppContainerStatus status={container.dockerContainer?.status} />
							</Td>
							<Td>{container.volumes.length}</Td>
						</Tr>
					))}
				</Tbody>
			</Table>
		</Box>
	)
}

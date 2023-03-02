import { Button, HStack, Table, TableContainer, Tbody, Td, Th, Thead, Tr } from '@chakra-ui/react'
import { ActionButtonState } from '../../atoms/ActionButton'
import { AppStatus } from '../../atoms/AppStatus'
import { HomePageQuery } from '../../graphql/generated'
import { AppActions } from '../../molecules/AppActions'
import { Link } from '../../atoms/Link'
import { MdAdd } from 'react-icons/md'
import { useNavigate } from '../../router'
import { TemplateImportButton } from '../../molecules/TemplateImportButton'

export type AppsListProps = {
	apps: HomePageQuery['apps']
	showCreateButton?: boolean
	onStateChange?: (state: ActionButtonState) => void
	onFinished?: (succeeded: boolean) => void
}

export const AppsList = ({ apps, showCreateButton, onStateChange, onFinished }: AppsListProps) => {
	const navigate = useNavigate()

	return (
		<TableContainer>
			<Table variant="simple">
				<Thead>
					<Tr>
						<Th>Application name</Th>
						<Th>Containers</Th>
						<Th>Status</Th>
						<Th>Actions</Th>
					</Tr>
				</Thead>
				<Tbody>
					{apps.map((app) => (
						<Tr key={app.id}>
							<Td>
								<Link to={`/apps/${app.id}`}> {app.name}</Link>
							</Td>
							<Td>{app.containers.length}</Td>
							<Td>
								<AppStatus size="lg" status={app.fetchedStatus} />
							</Td>
							<Td>
								<HStack>
									<AppActions
										appId={app.id}
										status={app.fetchedStatus}
										onStateChange={onStateChange}
										onFinished={onFinished}
									/>
								</HStack>
							</Td>
						</Tr>
					))}
					{showCreateButton === true && (
						<Tr>
							<Td>
								<Button colorScheme="green" leftIcon={<MdAdd />} onClick={() => navigate('/create')}>
									Create
								</Button>
							</Td>
							<Td>
								<TemplateImportButton />
							</Td>
							<Td colSpan={2}>&nbsp;</Td>
						</Tr>
					)}
				</Tbody>
			</Table>
		</TableContainer>
	)
}

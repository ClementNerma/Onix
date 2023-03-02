import { useHomePageQuery } from '../../graphql/generated'
import { Box } from '@chakra-ui/react'
import { AppsList } from './AppsList'

export function HomePage() {
	const { data, loading, error, refetch } = useHomePageQuery()

	if (error) {
		return <pre style={{ color: 'red' }}>Failed: {error.message}</pre>
	}

	if (loading || !data) {
		return <h2>Loading...</h2>
	}

	return (
		<Box>
			<AppsList apps={data.apps} onFinished={() => refetch()} showCreateButton />
		</Box>
	)
}

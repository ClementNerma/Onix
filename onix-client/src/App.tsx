import { ApolloProvider } from '@apollo/client'
import { ChakraProvider } from '@chakra-ui/react'
import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import { client } from './graphql/client'
import { getRoutes } from './router'
import { StrictMode } from 'react'

export const App = () => {
	const router = createBrowserRouter(getRoutes())

	return (
		<StrictMode>
			<ApolloProvider client={client}>
				<ChakraProvider>
					<RouterProvider router={router} />
				</ChakraProvider>
			</ApolloProvider>
		</StrictMode>
	)
}

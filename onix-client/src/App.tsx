import { ApolloProvider } from '@apollo/client'
import { ChakraProvider } from '@chakra-ui/react'
import { BrowserRouter, createBrowserRouter, Route, RouterProvider, Routes } from 'react-router-dom'
import { client } from './graphql/client'
import { HomePage } from './pages/HomePage'
import { getRoutes, validateRoute } from './routing'
import { Template } from './templates/Template'

export const App = () => {
  const router = createBrowserRouter(getRoutes())

  return (
    <ApolloProvider client={client}>
      <ChakraProvider>
        <RouterProvider router={router} />
      </ChakraProvider>
    </ApolloProvider>
  )
}

import { ApolloProvider } from '@apollo/client'
import { ChakraProvider } from '@chakra-ui/react'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { client } from './graphql/client'
import { HomePage } from './pages/HomePage'
import { validateRoute } from './routing'
import { Template } from './templates/Template'

export const App = () => {
  return (
    <ApolloProvider client={client}>
      <BrowserRouter>
        <ChakraProvider>
          <Routes>
            <Route path={validateRoute('/')} element={<Template />}>
              <Route index element={<HomePage />} />
            </Route>
          </Routes>
        </ChakraProvider>
      </BrowserRouter>
    </ApolloProvider>
  )
}

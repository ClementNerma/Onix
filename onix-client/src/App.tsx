import { ApolloProvider } from '@apollo/client'
import { client } from './graphql/client'
import { HomePage } from './pages/HomePage'
import { Template } from './templates/Template'

export const App = () => (
  <ApolloProvider client={client}>
    <Template>
      <HomePage />
    </Template>
  </ApolloProvider>
)

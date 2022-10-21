import { ApolloClient, InMemoryCache } from '@apollo/client/core'

export const API_SERVER_URL = `http://${location.hostname}:5871`

export default new ApolloClient({
  uri: API_SERVER_URL,
  cache: new InMemoryCache(),
})

import { ApolloProvider } from '@apollo/client'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { client } from './graphql/client'
import { HomePage } from './pages/HomePage'
import { validateRoute } from './routes'
import { Template } from './templates/Template'

export const App = () => {
  return (
    <ApolloProvider client={client}>
      <BrowserRouter>
        <Routes>
          <Route path={validateRoute('/')} element={<Template />}>
            <Route index element={<HomePage />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </ApolloProvider>
  )
}

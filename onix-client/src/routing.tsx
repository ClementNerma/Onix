import { RouteObject, useNavigate as useBaseNavigate } from 'react-router-dom'
import { HomePage } from './pages/HomePage'
import { Template } from './templates/Template'

export type Route = '/'

export const validateRoute = (route: Route) => route

export function useNavigate() {
  const navigate = useBaseNavigate()
  return (route: Route) => navigate(route)
}

export function getRoutes(): RouteObject[] {
  return [
    {
      path: validateRoute('/'),
      element: <Template />,
      children: [
        {
          index: true,
          element: <HomePage />,
        },
      ],
    },
  ]
}

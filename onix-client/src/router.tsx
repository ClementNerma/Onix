import { RouteObject, useNavigate as useBaseNavigate } from 'react-router-dom'
import { useParams as useBaseParams } from 'react-router-dom'
import { AppPage } from './pages/AppPage'
import { CreateAppPage } from './pages/CreateAppPage/CreateAppPage'
import { HomePage } from './pages/HomePage'
import { Template } from './templates/Template'

export type Route = '/' | `/apps/${string}` | `/create`

export const validateRoute = (route: Route) => route

export function useNavigate() {
  const navigate = useBaseNavigate()
  return (route: Route) => navigate(route)
}

export function useParams<S extends string>(names: S[]): Record<S, string> {
  const params = useBaseParams()
  const out: Record<string, string> = {}

  for (const name of names) {
    if (!Object.prototype.hasOwnProperty.call(params, name)) {
      throw new Error(`Route parameter "${name}" is missing!`)
    }

    const param = params[name]

    if (typeof param !== 'string') {
      throw new Error(`Route parameter "${name}" is missing or has the wrong type!`)
    }

    out[name] = param
  }

  return out as Record<S, string>
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
        {
          path: validateRoute('/apps/:appId'),
          element: <AppPage />,
        },
        {
          path: validateRoute('/create'),
          element: <CreateAppPage />,
        },
      ],
    },
  ]
}

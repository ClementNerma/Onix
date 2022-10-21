import { useNavigate as useBaseNavigate } from 'react-router-dom'

export type Route = '/'

export const validateRoute = (route: Route) => route

export function useNavigate() {
  const navigate = useBaseNavigate()
  return (route: Route) => navigate(route)
}

import { Route, Routes } from '@solidjs/router'
import { Component } from 'solid-js'
import { HomePage } from './pages/HomePage'
import { useNavigate as useBaseNavigate } from '@solidjs/router'

export type Route = '/'

export const validateRoute = (route: Route): Route => route

export const useNavigate = () => {
  const navigate = useBaseNavigate()
  return (route: Route) => navigate(route)
}

export const AppRoutes: Component = () => (
  <Routes>
    <Route path={validateRoute('/')} component={HomePage} />
  </Routes>
)

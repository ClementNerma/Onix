import { Router } from '@solidjs/router'
import type { Component } from 'solid-js'
import { AppRoutes } from './pages/Routes'

const App: Component = () => {
  return (
    <Router>
      <AppRoutes />
    </Router>
  )
}

export default App

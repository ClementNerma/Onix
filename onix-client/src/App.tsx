import { Router } from '@solidjs/router'
import type { Component } from 'solid-js'
import { Template } from './organisms/Template'
import { AppRoutes } from './Routes'

const App: Component = () => {
  return (
    <Router>
      <Template>
        <AppRoutes />
      </Template>
    </Router>
  )
}

export default App

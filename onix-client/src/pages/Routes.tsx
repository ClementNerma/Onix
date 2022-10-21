import { Route, Routes } from '@solidjs/router'
import { Component } from 'solid-js'
import { HomePage } from './HomePage'

export const AppRoutes: Component = () => (
  <Routes>
    <Route path="/" component={HomePage} />
  </Routes>
)

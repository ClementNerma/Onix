import React from 'react'
import { Outlet } from 'react-router-dom'

export type TemplateProps = React.PropsWithChildren<{}>

export const Template = () => <Outlet />

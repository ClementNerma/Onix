import { Link as ChakraLink } from '@chakra-ui/react'
import React from 'react'
import { Link as ReactRouterLink } from 'react-router-dom'
import { Route } from '../../router'

export type LinkProps = React.PropsWithChildren<{
  to: Route
}>

export const Link = ({ to, children }: LinkProps) => (
  <ChakraLink as={ReactRouterLink} to={to}>
    {children}
  </ChakraLink>
)

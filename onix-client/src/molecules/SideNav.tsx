import Box from '@suid/material/Box'
import List from '@suid/material/List'
import ListItem from '@suid/material/ListItem'
import ListItemButton from '@suid/material/ListItemButton'
import ListItemIcon from '@suid/material/ListItemIcon'
import { Component, For, JSX } from 'solid-js'
import { styled } from 'solid-styled-components'
import { Route, useNavigate } from '../Routes'

export type SideNavProps = {
  navList: SideNavItem[]
}

export type SideNavItem = {
  icon: JSX.Element
  text: string
  uri: Route
}

export const Container = styled(Box)`
  background: #081c4c;
  color: #f0f0f4;
`

export const SideNav: Component<SideNavProps> = (props: SideNavProps) => {
  const navigate = useNavigate()

  return (
    <Container>
      <List>
        <For each={props.navList}>
          {(item) => (
            <ListItem disablePadding>
              <ListItemButton onClick={() => navigate(item.uri)}>
                <ListItemIcon>{item.icon}</ListItemIcon>
                {item.text}
                {/* <ListItemText primary={item.text} /> */}
              </ListItemButton>
            </ListItem>
          )}
        </For>
      </List>
    </Container>
  )
}

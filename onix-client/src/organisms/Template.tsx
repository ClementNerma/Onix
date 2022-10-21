import Stack from '@suid/material/Stack'
import { children, ParentComponent, ParentProps } from 'solid-js'
import { SideNav, SideNavItem } from '../molecules/SideNav'
import { Home } from '@suid/icons-material'
import Box from '@suid/material/Box'
import { styled } from 'solid-styled-components'

const Container = styled(Stack)`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
`

const Slot = styled(Box)`
  padding: 1rem;
`

const navList: SideNavItem[] = [{ icon: <Home />, text: 'Home', uri: '/' }]

export const Template: ParentComponent = (props: ParentProps) => {
  const c = children(() => props.children)

  return (
    <Container direction="row">
      <SideNav navList={navList} />
      <Slot>{c()}</Slot>
    </Container>
  )
}

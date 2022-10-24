import { Box, Divider, Flex } from '@chakra-ui/react'
import styled from '@emotion/styled'
import { Outlet } from 'react-router-dom'
import { SideNav, SideNavItem } from '../organisms/SideNav'

const pages: SideNavItem[] = [
  {
    route: '/',
    text: 'Home',
  },
]

const FullPageContainer = styled(Flex)`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
`

const SideNavContainer = styled(Box)`
  height: 100%;
`

const OutletContainer = styled(Box)`
  padding: 1rem;
  width: 100%;
  overflow: auto;
`

const SubOutletContainer = styled(Box)`
  width: fit-content;
  height: fit-contnet;
`

export const Template = () => {
  return (
    <FullPageContainer direction="row">
      <SideNavContainer>
        <SideNav items={pages} />
      </SideNavContainer>
      <Divider orientation="vertical" />
      <OutletContainer>
        <SubOutletContainer>
          <Outlet />
        </SubOutletContainer>
      </OutletContainer>
    </FullPageContainer>
  )
}

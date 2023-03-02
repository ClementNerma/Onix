import { Button, List, ListItem } from '@chakra-ui/react'
import { Route, useNavigate } from '../../router'

export type SideNavProps = {
	items: SideNavItem[]
}

export type SideNavItem = {
	// icon: JSX.Element
	route: Route
	text: string
}

export const SideNav = (props: SideNavProps) => {
	const navigate = useNavigate()

	return (
		<List>
			{props.items.map((item, i) => (
				<ListItem key={i}>
					<Button variant="ghost" onClick={() => navigate(item.route)}>
						{item.text}
					</Button>
				</ListItem>
			))}
		</List>
	)
}

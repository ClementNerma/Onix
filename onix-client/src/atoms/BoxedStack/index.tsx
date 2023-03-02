import { StackProps, VStack } from '@chakra-ui/react'
import React from 'react'

export type BoxedStackProps = React.PropsWithChildren<{
	noBorder?: boolean
	style?: Pick<React.CSSProperties, 'borderColor' | 'borderWidth' | 'borderStyle' | 'borderRadius' | 'padding'>
}> &
	StackProps

export const BoxedStack = ({ noBorder, style, children, ...rest }: BoxedStackProps) => (
	<VStack
		style={
			noBorder === true
				? {}
				: {
						borderColor: style?.borderColor ?? 'lightgray',
						borderWidth: style?.borderWidth ?? '1px',
						borderStyle: style?.borderStyle ?? 'solid',
						borderRadius: style?.borderRadius ?? '10px',
						padding: style?.padding ?? '1rem',
				  }
		}
		alignItems="start"
		{...rest}
	>
		{children}
	</VStack>
)

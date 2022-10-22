import { Box } from '@chakra-ui/react'
import React from 'react'

export type BorderedBoxProps = React.PropsWithChildren<
  Pick<React.CSSProperties, 'borderColor' | 'borderWidth' | 'borderStyle' | 'padding'>
>

export const BorderedBox = ({ borderColor, borderWidth, borderStyle, padding, children }: BorderedBoxProps) => (
  <Box
    style={{
      borderColor: borderColor ?? 'lightgray',
      borderWidth: borderWidth ?? '1px',
      borderStyle: borderStyle ?? 'solid',
      padding: padding ?? '5px',
    }}
  >
    {children}
  </Box>
)

import React from 'react'

export type TemplateProps = React.PropsWithChildren<{}>

export const Template = (props: TemplateProps) => <>{props.children}</>

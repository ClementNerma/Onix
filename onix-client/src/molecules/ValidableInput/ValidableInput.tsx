import { FormControl, FormHelperText, FormLabel, Input, InputProps } from '@chakra-ui/react'
import React, { useState } from 'react'

export type ValidableInputProps = {
  label: React.ReactNode
  helper?: string
  isOptional?: boolean
  isValid?: (value: string) => boolean
  value: string
  onChange: (value: string) => void
} & Omit<InputProps, 'value' | 'onChange'>

export const ValidableInput = ({
  label,
  helper,
  isOptional,
  isValid,
  value,
  onChange,
  ...rest
}: ValidableInputProps) => {
  const [hasBeenFocused, setHasBeenFocused] = useState(false)

  return (
    <FormControl isRequired={isOptional !== true} isInvalid={isValid && hasBeenFocused && !isValid(value)}>
      <FormLabel>{label}</FormLabel>
      <Input
        value={value}
        onChange={(e) => onChange(e.target.value)}
        onFocus={() => setHasBeenFocused(true)}
        {...rest}
      />
      {helper !== undefined && <FormHelperText>{helper}</FormHelperText>}
    </FormControl>
  )
}

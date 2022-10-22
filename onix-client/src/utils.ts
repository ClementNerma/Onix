import React from 'react'

export function assertNever(_: never): never {
  throw new Error('Reached unreachable assertion!')
}

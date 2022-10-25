export function assertNever(_: never): never {
  throw new Error('Reached unreachable assertion!')
}

type Variants<T extends object> = {
  [Variant in keyof T]: {
    variantName: Variant
    value: NonNullable<T[Variant]>
  }
}

export type Variant<T extends object> = Required<NonNullable<Variants<T>[keyof Variants<T>]>>

export function variantUnion<T extends object>(obj: T): Variant<T> {
  const keys = Reflect.ownKeys(obj) as Array<keyof T>

  if (keys.length !== 1) {
    console.error({ obj })
    throw new Error('Object must have exactly 1 key to get a variant from')
  }

  const variantName = keys[0]
  const value = obj[variantName]

  // eslint-disable-next-line @typescript-eslint/no-unnecessary-condition
  if (value === null || value === undefined) {
    console.error({ obj })
    throw new Error('Object\'s only property must neither but "null" nor "undefined"')
  }

  return { variantName, value } as unknown as Variant<T>
}

export function coverProperties<T extends object>() {
  return <U>(obj: Required<{ [_Key in keyof T]: U }>) => obj
}

export function getPropUnsafely<T>(obj: unknown, propName: string): T | null {
  if (typeof obj !== 'object' || obj === null) {
    return null
  }

  if (!Object.prototype.hasOwnProperty.call(obj, propName)) {
    return null
  }

  // NOTE: UNSAFE
  return obj[propName as keyof typeof obj] as T
}

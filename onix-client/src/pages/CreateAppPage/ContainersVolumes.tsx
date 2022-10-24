import {
  TableContainer,
  Table,
  Thead,
  Tr,
  Th,
  Tbody,
  Td,
  Input,
  IconButton,
  Button,
  Switch,
  Select,
} from '@chakra-ui/react'
import { useCallback } from 'react'
import { MdAdd, MdDelete } from 'react-icons/md'
import { AppContainerTemplateInput, AppVolumeInput, AppVolumeTypeGraphQlInput } from '../../graphql/generated'
import { assertNever, coverProperties, variantUnion } from '../../utils'

export type ContainerVolumesProps = {
  state: AppContainerTemplateInput['volumes']
  onChange: (state: AppContainerTemplateInput['volumes']) => void
}

export const ContainerVolumes = ({ state, onChange }: ContainerVolumesProps) => {
  const updateVolume = useCallback(
    (content: AppVolumeInput, index: number) => {
      onChange([...state.slice(0, index), content, ...state.slice(index + 1)])
    },
    [state, onChange],
  )

  const removeVolume = useCallback(
    (index: number) => {
      onChange([...state.slice(0, index), ...state.slice(index + 1)])
    },
    [state, onChange],
  )

  const addVolume = useCallback(() => {
    onChange([
      ...state,
      {
        name: '',
        variant: getDefaultVolumeVariantValue('disposable'),
      },
    ])
  }, [state, onChange])

  return (
    <TableContainer>
      <Table>
        <Thead>
          <Tr>
            <Th>Actions</Th>
            <Th>Name</Th>
            <Th>Type</Th>
            <Th>Host path</Th>
            <Th>Path in container</Th>
            <Th>Read-only?</Th>
          </Tr>
        </Thead>
        <Tbody>
          {state.map(({ name, variant }, i) => (
            <Tr key={i}>
              <Td>
                <IconButton size="xs" as={MdDelete} onClick={() => removeVolume(i)} aria-label="Remove this volume" />
              </Td>
              <Td>
                <Input type="text" value={name} onChange={(e) => updateVolume({ name: e.target.value, variant }, i)} />
              </Td>
              <Td>
                <ContainerVolumeVariantSelector
                  variant={variantUnion(variant).variantName}
                  onChange={(variant) => updateVolume({ name, variant }, i)}
                />
              </Td>
              <ContainerVolumeVariant variant={variant} onChange={(variant) => updateVolume({ name, variant }, i)} />
            </Tr>
          ))}
          <Tr>
            <Td colSpan={3}>
              <Button colorScheme="green" size="sm" leftIcon={<MdAdd />} onClick={addVolume}>
                Add a volume
              </Button>
            </Td>
          </Tr>
        </Tbody>
      </Table>
    </TableContainer>
  )
}

type ContainerVolumeVariantSelectorProps = {
  variant: keyof AppVolumeTypeGraphQlInput
  onChange: (variant: AppVolumeTypeGraphQlInput) => void
}

const ContainerVolumeVariantSelector = ({ variant, onChange }: ContainerVolumeVariantSelectorProps) => {
  const options = coverProperties<AppVolumeTypeGraphQlInput>()({
    disposable: 'Disposable',
    internal: 'Internal',
    external: 'External',
    bindToPath: 'Bind to path',
  })

  return (
    <Select
      value={variant}
      onChange={(e) => onChange(getDefaultVolumeVariantValue(e.target.value as keyof AppVolumeTypeGraphQlInput))}
    >
      {Object.entries(options).map(([variant, label]) => (
        <option key={variant} value={variant}>
          {label}
        </option>
      ))}
    </Select>
  )
}

type ContainerVolumeVariantProps = {
  variant: AppVolumeTypeGraphQlInput
  onChange: (variant: AppVolumeTypeGraphQlInput) => void
}

const ContainerVolumeVariant = ({ variant, onChange }: ContainerVolumeVariantProps) => {
  const union = variantUnion(variant)

  const onUpdate = useCallback(<U extends typeof union>(union: U, variant: Partial<U['value']>) => {
    onChange({ [union.variantName]: { ...union.value, ...variant } })
  }, [])

  switch (union.variantName) {
    case 'disposable':
      return (
        <>
          <Td>
            <em>Disposable</em>
          </Td>
          <Td>
            <Input
              type="text"
              value={union.value.containerPath}
              onChange={(e) => onUpdate(union, { containerPath: e.target.value })}
            />
          </Td>
          <Td>
            <Switch isChecked isDisabled />
          </Td>
        </>
      )

    case 'internal':
      return (
        <>
          <Td>
            <em>Internal</em>
          </Td>
          <Input
            type="text"
            value={union.value.containerPath}
            onChange={(e) => onUpdate(union, { containerPath: e.target.value })}
          />
          <Td>
            <Switch isChecked isDisabled />
          </Td>
        </>
      )

    case 'external':
      return (
        <>
          <Td>
            <em>To be determined</em>
          </Td>
          <Td>
            <Input
              type="text"
              value={union.value.containerPath}
              onChange={(e) => onUpdate(union, { containerPath: e.target.value })}
            />
          </Td>
          <Td>
            <Switch
              isChecked={union.value.readonly}
              onChange={(e) => onUpdate(union, { readonly: !union.value.readonly })}
            />
          </Td>
        </>
      )

    case 'bindToPath':
      return (
        <>
          <Td>
            <Input
              type="text"
              value={union.value.hostPath}
              onChange={(e) => onUpdate(union, { hostPath: e.target.value })}
            />
          </Td>
          <Td>
            <Input
              type="text"
              value={union.value.containerPath}
              onChange={(e) => onUpdate(union, { containerPath: e.target.value })}
            />
          </Td>
          <Td>
            <Switch
              isChecked={union.value.readonly}
              onChange={(e) => onUpdate(union, { readonly: !union.value.readonly })}
            />
          </Td>
        </>
      )

    default:
      return assertNever(union)
  }
}

function getDefaultVolumeVariantValue(variantName: keyof AppVolumeTypeGraphQlInput): AppVolumeTypeGraphQlInput {
  switch (variantName) {
    case 'disposable':
      return { disposable: { containerPath: DEFAULT_MOUNT_PATH } }

    case 'internal':
      return { internal: { containerPath: DEFAULT_MOUNT_PATH } }

    case 'external':
      return { external: { containerPath: DEFAULT_MOUNT_PATH, readonly: false } }

    case 'bindToPath':
      return { bindToPath: { hostPath: '/home/you/some-folder', containerPath: DEFAULT_MOUNT_PATH, readonly: false } }

    default:
      return assertNever(variantName)
  }
}

const DEFAULT_MOUNT_PATH = '/mnt/my-volume'

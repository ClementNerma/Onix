import { Button, Heading } from '@chakra-ui/react'
import { useCallback, useEffect, useState } from 'react'
import { MdAdd } from 'react-icons/md'
import { IoIosRocket } from 'react-icons/io'
import { BoxedStack } from '../../atoms/BoxedStack'
import {
	AppContainerTemplateInput,
	AppTemplate,
	AppTemplateInput,
	AppVolumeInput,
	AppVolumeTypeGraphQl,
	AppVolumeTypeGraphQlInput,
	ContainerEnvironmentVarInput,
	ContainerPortBindingInput,
	useCreateAppMutation,
} from '../../graphql/generated'
import { ValidableInput } from '../../molecules/ValidableInput/ValidableInput'
import { CreateAppContainer } from './CreateAppContainer'
import { ActionButton } from '../../atoms/ActionButton'
import { useNavigate } from '../../router'
import { useLocation } from 'react-router-dom'
import { assertNever, getPropUnsafely } from '../../utils'

export const FROM_TEMPLATE_STATE_PROPNAME = 'fromTemplate'

export const CreateAppPage = () => {
	const location = useLocation()

	const fromTemplate = getPropUnsafely<AppTemplate>(location.state as unknown, FROM_TEMPLATE_STATE_PROPNAME)
	const fromTemplateInput = fromTemplate ? appTemplateToInput(fromTemplate) : null

	const [createApp, creatingApp] = useCreateAppMutation()

	const [appName, setAppName] = useState(fromTemplateInput?.name ?? '')
	const [appContainers, setAppContainers] = useState<AppContainerTemplateInput[]>(fromTemplateInput?.containers ?? [])

	const navigate = useNavigate()

	const submit = useCallback(() => {
		if (creatingApp.loading) {
			return
		}

		createApp({
			variables: {
				input: {
					name: appName,
					containers: appContainers,
				},
			},
		})
	}, [appName, appContainers, createApp, creatingApp])

	useEffect(() => {
		if (creatingApp.data) {
			navigate(`/apps/${creatingApp.data.createApp.id}`)
		}
	}, [creatingApp, navigate])

	const updateContainerInput = useCallback(
		(state: AppContainerTemplateInput, index: number) => {
			setAppContainers([...appContainers.slice(0, index), state, ...appContainers.slice(index + 1)])
		},
		[appContainers, setAppContainers],
	)

	const removeContainerInput = useCallback(
		(index: number) => {
			setAppContainers([...appContainers.slice(0, index), ...appContainers.slice(index + 1)])
		},
		[appContainers, setAppContainers],
	)

	const addContainer = useCallback(() => {
		setAppContainers([
			...appContainers,
			{
				name: '',
				image: '',
				envVars: [],
				portBindings: [],
				volumes: [],
				dependsOn: [],
			},
		])
	}, [appContainers, setAppContainers])

	return (
		<>
			<Heading>Create an application</Heading>

			<BoxedStack spacing={5}>
				<ValidableInput
					type='text'
					label='Application name'
					value={appName}
					onChange={setAppName}
					isValid={(name) => /^[a-zA-Z0_9_\-]+$/.test(name)}
					helper='Must only be letters, digits, dashes and underscores.'
				/>
			</BoxedStack>

			<Heading size='md'>Containers ({appContainers.length})</Heading>

			{appContainers.map((state, i) => (
				<CreateAppContainer
					key={state.name}
					state={state}
					onChange={(state) => updateContainerInput(state, i)}
					onRemove={() => removeContainerInput(i)}
				/>
			))}

			<Button colorScheme='green' size='sm' leftIcon={<MdAdd />} onClick={addContainer}>
				Add a container
			</Button>

			{appContainers.length > 0 && (
				<ActionButton
					colorScheme='blue'
					icon={<IoIosRocket />}
					label='Create the application'
					errorTitle='Failed to create the application'
					onClick={submit}
					state={creatingApp}
				/>
			)}
		</>
	)
}

export function appTemplateToInput(template: AppTemplate): AppTemplateInput {
	return {
		name: template.name,
		containers: template.containers.map<AppContainerTemplateInput>((container) => ({
			name: container.name,
			image: container.image,
			envVars: container.envVars.map<ContainerEnvironmentVarInput>((envVar) => ({
				name: envVar.name,
				value: envVar.value,
			})),
			portBindings: container.portBindings.map<ContainerPortBindingInput>((binding) => ({
				hostPort: {
					port: binding.hostPort.port,
					portType: binding.hostPort.portType,
				},
				containerPort: {
					port: binding.containerPort.port,
					portType: binding.containerPort.portType,
				},
			})),
			volumes: container.volumes.map<AppVolumeInput>((volume) => ({
				name: volume.name,
				variant: appTemplateVolumeTypeToInput(volume.variant),
			})),
			dependsOn: container.dependsOn,
		})),
	}
}

export function appTemplateVolumeTypeToInput(template: AppVolumeTypeGraphQl): AppVolumeTypeGraphQlInput {
	switch (template.__typename) {
		// TODO: find a fix for this
		case undefined: {
			const message = 'Missing typename in volume type'
			alert(message)
			throw new Error(message)
		}

		case 'AppVolumeTypeDisposableGraphQL':
			return { disposable: { containerPath: template.containerPath } }

		case 'AppVolumeTypeInternalGraphQL':
			return { internal: { containerPath: template.containerPath } }

		case 'AppVolumeTypeExternalGraphQL':
			return { external: { containerPath: template.containerPath, readonly: template.readonly } }

		case 'AppVolumeTypeBindToPathGraphQL':
			return {
				bindToPath: { containerPath: template.containerPath, hostPath: template.hostPath, readonly: template.readonly },
			}

		default:
			return assertNever(template)
	}
}

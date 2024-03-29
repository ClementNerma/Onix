import { TableContainer, Table, Thead, Tr, Th, Tbody, Td, Input, IconButton, Button, Select } from '@chakra-ui/react'
import { useCallback, useState } from 'react'
import { MdAdd, MdDelete } from 'react-icons/md'
import { AppContainerTemplateInput, ContainerPortBindingInput, PortInput, PortType } from '../../graphql/generated'

export type ContainerPortBindingsProps = {
	state: AppContainerTemplateInput['portBindings']
	onChange: (state: AppContainerTemplateInput['portBindings']) => void
}

export const ContainerPortBindings = ({ state, onChange }: ContainerPortBindingsProps) => {
	const updateVar = useCallback(
		(content: ContainerPortBindingInput, index: number) => {
			onChange([...state.slice(0, index), content, ...state.slice(index + 1)])
		},
		[state, onChange],
	)

	const removeVar = useCallback(
		(index: number) => {
			onChange([...state.slice(0, index), ...state.slice(index + 1)])
		},
		[state, onChange],
	)

	const addVar = useCallback(() => {
		onChange([
			...state,
			{
				hostPort: {
					port: 0,
					portType: PortType.TcpUdp,
				},
				containerPort: {
					port: 0,
					portType: PortType.TcpUdp,
				},
			},
		])
	}, [state, onChange])

	return (
		<TableContainer>
			<Table>
				<Thead>
					<Tr>
						<Th>Actions</Th>
						<Th>Host port type</Th>
						<Th>Host port</Th>
						<Th>Direction</Th>
						<Th>Container port</Th>
						<Th>Container port type</Th>
					</Tr>
				</Thead>
				<Tbody>
					{state.map(({ hostPort, containerPort }, i) => (
						<Tr key={`${hostPort.port}-${containerPort.port}`}>
							<Td>
								<IconButton size='xs' as={MdDelete} onClick={() => removeVar(i)} aria-label='Remove this binding' />
							</Td>
							<Port state={hostPort} onChange={(hostPort) => updateVar({ hostPort, containerPort }, i)} />
							<Td>&lt;=&gt;</Td>
							<Port
								state={containerPort}
								onChange={(containerPort) => updateVar({ hostPort, containerPort }, i)}
								reverse
							/>
						</Tr>
					))}
					<Tr>
						<Td colSpan={3}>
							<Button colorScheme='green' size='sm' leftIcon={<MdAdd />} onClick={addVar}>
								Add a port binding
							</Button>
						</Td>
					</Tr>
				</Tbody>
			</Table>
		</TableContainer>
	)
}

type PortProps = {
	state: PortInput
	onChange: (state: PortInput) => void
	reverse?: boolean
}

const Port = ({ state, onChange, reverse }: PortProps) => {
	const [isPortNumberInvalid, setIsPortNumberInvalid] = useState(false)

	const setPortNumber = useCallback(
		(input: string) => {
			if (!input.match(/^\d+$/)) {
				setIsPortNumberInvalid(true)
				return
			}

			onChange({ ...state, port: parseInt(input) })
		},
		[setIsPortNumberInvalid, onChange, state],
	)

	const left = (
		<Td>
			<Select value={state.portType} onChange={(e) => onChange({ ...state, portType: e.target.value as PortType })}>
				<option value={PortType.TcpUdp}>TCP & UDP</option>
				<option value={PortType.Tcp}>TCP only</option>
				<option value={PortType.Udp}>UDP only</option>
			</Select>
		</Td>
	)

	const right = (
		<Td>
			<Input
				type='number'
				isInvalid={isPortNumberInvalid}
				value={state.port}
				onChange={(e) => setPortNumber(e.target.value)}
			/>
		</Td>
	)

	return reverse !== true ? (
		<>
			{left}
			{right}
		</>
	) : (
		<>
			{right}
			{left}
		</>
	)
}

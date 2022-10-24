import { Button, Container, Heading, useToast } from '@chakra-ui/react'
import { useCallback, useEffect, useState } from 'react'
import { MdAdd } from 'react-icons/md'
import { IoIosRocket } from 'react-icons/io'
import { BoxedStack } from '../../atoms/BoxedStack'
import { AppContainerCreationInput, useCreateAppMutation } from '../../graphql/generated'
import { ValidableInput } from '../../molecules/ValidableInput/ValidableInput'
import { CreateAppContainer } from './CreateAppContainer'
import { ActionButton } from '../../atoms/ActionButton'
import { useNavigate } from '../../router'

export const CreateAppPage = () => {
  const [createApp, creatingApp] = useCreateAppMutation()

  const [appName, setAppName] = useState('')
  const [appContainers, setAppContainers] = useState<AppContainerCreationInput[]>([])

  const navigate = useNavigate()
  const toast = useToast()

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
    if (creatingApp.error) {
      toast({
        title: 'Application creation failed',
        description: creatingApp.error.message,
        status: 'error',
      })
    } else if (creatingApp.data) {
      navigate(`/apps/${creatingApp.data.createApp.id}`)
    }
  }, [creatingApp, toast, navigate])

  const updateContainerInput = useCallback(
    (state: AppContainerCreationInput, index: number) => {
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
          type="text"
          label="Application name"
          value={appName}
          onChange={setAppName}
          isValid={(name) => /^[a-zA-Z0_9_\-]+$/.test(name)}
          helper="Must only be letters, digits, dashes and underscores."
        />
      </BoxedStack>

      <Heading size="md">Containers ({appContainers.length})</Heading>

      {appContainers.map((state, i) => (
        <CreateAppContainer
          key={i}
          state={state}
          onChange={(state) => updateContainerInput(state, i)}
          onRemove={() => removeContainerInput(i)}
        />
      ))}

      <Button colorScheme="green" size="sm" leftIcon={<MdAdd />} onClick={addContainer}>
        Add a container
      </Button>

      {appContainers.length > 0 && (
        <ActionButton
          colorScheme="blue"
          icon={<IoIosRocket />}
          label="Create the application"
          onClick={submit}
          state={creatingApp}
        />
      )}
    </>
  )
}

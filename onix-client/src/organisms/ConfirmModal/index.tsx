import {
  Button,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  Text,
  useDisclosure,
} from '@chakra-ui/react'
import { useCallback } from 'react'

export type ConfirmModalProps = {
  isOpen: ReturnType<typeof useDisclosure>['isOpen']
  onClose: ReturnType<typeof useDisclosure>['onClose']
  title?: string
  message?: string
  confirmationLabel?: boolean
  cancellationLabel?: boolean
  onConfirm: () => void
}

export const ConfirmModal = ({
  isOpen,
  onClose,
  title,
  message,
  confirmationLabel,
  cancellationLabel,
  onConfirm,
}: ConfirmModalProps) => {
  const onClick = useCallback(() => {
    onClose()
    onConfirm()
  }, [onClose, onConfirm])

  return (
    <Modal isOpen={isOpen} onClose={onClose}>
      <ModalOverlay />
      <ModalContent>
        <ModalHeader>{title ?? 'Are you sure?'}</ModalHeader>
        <ModalCloseButton />

        <ModalBody>
          <Text>{message ?? 'Please confirm before proceeding.'}</Text>
        </ModalBody>

        <ModalFooter>
          <Button onClick={onClose} mr={3}>
            {cancellationLabel ?? 'Cancel'}
          </Button>
          <Button colorScheme="blue" onClick={onClick}>
            {confirmationLabel ?? 'Proceed'}
          </Button>
        </ModalFooter>
      </ModalContent>
    </Modal>
  )
}

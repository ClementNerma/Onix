import {
  Button,
  Modal,
  ModalBody,
  ModalCloseButton,
  ModalContent,
  ModalFooter,
  ModalHeader,
  ModalOverlay,
  useDisclosure,
} from '@chakra-ui/react'
import React, { useCallback } from 'react'

export type ConfirmModalProps = React.PropsWithChildren<{
  isOpen: ReturnType<typeof useDisclosure>['isOpen']
  onClose: ReturnType<typeof useDisclosure>['onClose']
  title?: string
  confirmationLabel?: string
  cancellationLabel?: string
  onConfirm: () => void
}>

export const ConfirmModal = ({
  isOpen,
  onClose,
  title,
  confirmationLabel,
  cancellationLabel,
  onConfirm,
  children,
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

        <ModalBody>{children}</ModalBody>

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

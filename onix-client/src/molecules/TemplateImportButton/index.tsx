import { ButtonProps, useDisclosure } from '@chakra-ui/react'
import styled from '@emotion/styled'
import { useCallback, useEffect, useState } from 'react'
import { MdUpload } from 'react-icons/md'
import { ActionButton } from '../../atoms/ActionButton'
import { useDecodeTemplateLazyQuery } from '../../graphql/generated'
import { ConfirmModal } from '../../organisms/ConfirmModal'
import { FROM_TEMPLATE_STATE_PROPNAME } from '../../pages/CreateAppPage/CreateAppPage'
import { useNavigate } from '../../router'

export type TemplateImportButton = ButtonProps

export const TemplateImportButton = () => {
  const { isOpen, onOpen, onClose } = useDisclosure()
  const [decodeTemplate, templateDecoding] = useDecodeTemplateLazyQuery()
  const [templateText, setTemplateText] = useState("# Paste the application's template here\n")
  const navigate = useNavigate()

  const onConfirm = useCallback(() => {
    if (!templateDecoding.loading) {
      decodeTemplate({ variables: { template: templateText } })
    }
  }, [decodeTemplate, templateDecoding.loading, templateText])

  useEffect(() => {
    if (templateDecoding.data) {
      navigate('/create', { [FROM_TEMPLATE_STATE_PROPNAME]: templateDecoding.data.decodeTemplate })
    }
  }, [navigate, templateDecoding.data])

  return (
    <>
      <ActionButton icon={<MdUpload />} label="Import" state={templateDecoding} onClick={onOpen} />

      <ConfirmModal
        isOpen={isOpen}
        onClose={onClose}
        title="Import a template"
        confirmationLabel="Import"
        onConfirm={onConfirm}
      >
        <TemplateTextarea rows={10} value={templateText} onChange={(e) => setTemplateText(e.target.value)} />
      </ConfirmModal>
    </>
  )
}

const TemplateTextarea = styled('textarea')`
  padding: 0.5rem;
  width: 100%;
  border: 1px solid lightgray;
`

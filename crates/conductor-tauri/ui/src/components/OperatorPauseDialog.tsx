import * as AlertDialog from '@radix-ui/react-alert-dialog'
import type { ReactNode } from 'react'
import type { ChecklistItem } from './OperatorChecklist'
import OperatorChecklistView from './OperatorChecklistView'
import './OperatorPauseDialog.css'

export default function OperatorPauseDialog({
  open,
  onOpenChange,
  title,
  body,
  checklist = [],
  onChecklistToggle,
  proceedLabel = 'Proceed',
  abortLabel = 'Abort',
  onProceed,
  onAbort,
  allowNoGo = true,
}: {
  open: boolean
  onOpenChange: (open: boolean) => void
  title: string
  body: ReactNode
  checklist?: ChecklistItem[]
  onChecklistToggle?: (id: string, checked: boolean) => void
  proceedLabel?: string
  abortLabel?: string
  onProceed: () => void
  onAbort: () => void
  allowNoGo?: boolean
}) {
  return (
    <AlertDialog.Root open={open} onOpenChange={onOpenChange}>
      <AlertDialog.Portal>
        <AlertDialog.Overlay className="dialog__overlay" />
        <AlertDialog.Content className="dialog__content">
          <AlertDialog.Title className="dialog__title type-heading">{title}</AlertDialog.Title>
          <AlertDialog.Description asChild>
            <div className="dialog__body type-body">{body}</div>
          </AlertDialog.Description>
          {/* Sibling of the Description, never inside it: Description is the aria-describedby
              target, and interactive rows nested there read as flat prose to a screen reader. */}
          {checklist.length > 0 && onChecklistToggle ? (
            <OperatorChecklistView items={checklist} onToggle={onChecklistToggle} />
          ) : null}
          <div className="dialog__actions">
            {allowNoGo ? (
              <AlertDialog.Cancel asChild>
                <button type="button" className="dialog__btn dialog__btn--abort" onClick={onAbort}>
                  {abortLabel}
                </button>
              </AlertDialog.Cancel>
            ) : null}
            <AlertDialog.Action asChild>
              <button type="button" className="dialog__btn dialog__btn--proceed" onClick={onProceed}>
                {proceedLabel}
              </button>
            </AlertDialog.Action>
          </div>
        </AlertDialog.Content>
      </AlertDialog.Portal>
    </AlertDialog.Root>
  )
}

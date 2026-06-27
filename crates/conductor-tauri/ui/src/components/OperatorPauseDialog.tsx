import * as AlertDialog from '@radix-ui/react-alert-dialog'
import type { ReactNode } from 'react'
import './OperatorPauseDialog.css'

export default function OperatorPauseDialog({
  open,
  onOpenChange,
  title,
  body,
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

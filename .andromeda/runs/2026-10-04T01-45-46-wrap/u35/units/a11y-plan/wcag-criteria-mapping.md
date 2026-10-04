### WCAG criteria mapping

- **Tier coverage:**
  - **Minimal (selected):** baseline **SC 2.1.1 Keyboard** + **SC 1.4.3 Contrast (Minimum)** + **SC 2.4.3 Focus Order**, augmented by the design-token-bound **SC 1.4.11 Non-text Contrast** (3:1 status lamps + focus ring), **SC 1.4.1 Use of Color** (six-state not-color-alone), **SC 2.3.3 Animation from Interactions** (reduced-motion drop — a WCAG 2.1 Level AAA criterion included ONLY as a design-token-driven binding that does NOT escalate the tier; not a baseline AA/AAA gate), plus pattern-inherent **SC 4.1.2 Name/Role/Value**, **SC 4.1.3 Status Messages**, **SC 2.1.2 No Keyboard Trap**, **SC 1.3.1 Info and Relationships**, **SC 2.4.7 Focus Visible** (AA — visible `--color-focus` ring on every interactive control, asserted by the focus harness), **SC 3.3.2 Labels or Instructions** (AA — scenario/seed `<label>` + `aria-describedby` association).
  - Standard: WCAG 2.1 AA full (~50 SCs) — not selected.
  - Comprehensive: WCAG 2.2 AAA + cognitive — not selected.
- **Compliance trigger override:** none — security plan excerpt's A11y Compliance Triggers = "No a11y compliance triggers" (no Section 508 / EU Accessibility Act / ADA mandate); security_tier=Minimal. No WCAG level mandate to bind beyond the Minimal baseline + token-driven SCs above.

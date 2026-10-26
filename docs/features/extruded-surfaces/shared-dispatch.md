# Resolved-feature object dispatch

The common object dispatcher reached 497 lines before the extruded-surface
increment. Nine existing resolved-feature cases now live in
`resolved_feature_objects.rs`. The main dispatcher delegates only those exact
class names; an unrecognized name returns to its original dispatch path without
reading any bytes. Parser calls, arguments, scopes and error results are unchanged.

All nine class arms match their original definitions. The remaining main
dispatcher tokens are unchanged apart from delegation. All 450 library tests
passed, with the same three corpus gates ignored by default. A fresh 906-case
native run preserved every outcome and failure cause, including all 532
accepted configurations. `shared-dispatch-verification.json` records the
comparisons and log hashes. The extraction is committed separately from
extruded-surface runtime behavior.

The humanizer skill is unavailable; its prose pass remains with the maintainer.

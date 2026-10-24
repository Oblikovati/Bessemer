# Native container fixtures

Copied without modification from `../inventor-interface/solidworks/fixtures/schema`
at commit `dd0537b3a142d8dda00ef846891efc736a6aac46`.
They were created through the SolidWorks Design Premium 2026 SP2.1 UI on
2026-10-24. Tests embed these files and need no sibling checkout or installation.

| Fixture | Contents | SHA-256 |
| --- | --- | --- |
| blank-part.SLDPRT | Default part, no added sketch or solid | b52e080ba2cc64d1d07ed28306f27281b07521aef550bc0c7fea8cd54be2b276 |
| blank-assembly.SLDASM | Empty assembly, no components or mates | a0ef60261152a4721bd2bf7bf04cf43cb9c6c37d8ae09b0eb5ec935ee44abcb0 |
| blank-drawing.SLDDRW | A4 ISO sheet, no sheet format or model view | 7cd605fb2663514ae5f8bec228bde280497bfa310efad8f27839d902c39304f6 |

`streams.tsv` is the Python reference reader's central-directory-order list of
filename, stream name, expanded length, and CRC-32 (hex). It covers 41 part streams,
43 assembly streams, and 29 drawing streams. Changes require a reviewed baseline
update with the reason and container behavior version. This first implementation
is container behavior version 1.

The Inspection drawings were copied from the same approved fixture directory.
`inspection-drawing.SLDDRW` starts an empty Inspection project; its SHA-256 is
`a5e94727fe2b0b8a4d4cc7b16f53fd5155f629eab1f8a65da74c4a675b97c82f`.
`inspection-job.SLDDRW` sets Job # to `SCHEMA-JOB-001`; its SHA-256 is
`ad057eb6da7b750e2d40ad42fb186b2746038cf4cf155d8981277427af8c97c9`.
Both are native 2026 SP2.1 drawings without extracted characteristics. Their cached
collections also differ; they are not an isolated byte mutation. The independent
ElementTree oracles in `inspection/` cover all elements and all 233 observed paths.
See the [Inspection evidence](../../docs/features/inspection-projects/README.md).

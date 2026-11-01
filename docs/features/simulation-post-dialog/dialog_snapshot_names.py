"""Keep partial Source snapshots distinct from complete replay evidence (#166)."""
import sys


def dialog_snapshot_filename() -> str:
    """Keep partial replay evidence separate from the mandatory full snapshot; e.g. dialog_snapshot_filename()."""
    modes = (('--legend-step-only', 'legend-step-source-snapshot.json'), ('--legend-yield-only', 'legend-yield-source-snapshot.json'), ('--legend-height-only', 'legend-height-source-snapshot.json'), ('--legend-palette-only', 'legend-palette-source-snapshot.json'), ('--legend-position-only', 'legend-position-source-snapshot.json'), ('--legend-format-only', 'legend-format-source-snapshot.json'), ('--preference-colors-only', 'preference-color-source-snapshot.json'), ('--mode3-rgb-only', 'mode3-rgb-source-snapshot.json'), ('--segment-color-only', 'segment-color-source-snapshot.json'), ('--plot54-only', 'plot54-source-snapshot.json'), ('--text-controls-only', 'text-control-source-snapshot.json'), ('--triangle-selectors-only', 'triangle-selector-source-snapshot.json'), ('--aux-triangle-only', 'aux-triangle-source-snapshot.json'), ('--annotations-only', 'annotation-source-snapshot.json'), ('--vector-style-only', 'vector-style-source-snapshot.json'), ('--raw-preferences-only', 'raw-preferences-source-snapshot.json'),
             ('--local-render-only', 'local-render-source-snapshot.json'),
             ('--resources-only', 'resource-source-snapshot.json'),
             ('--caps-only', 'cap-source-snapshot.json'),
             ('--reference-caps-only', 'reference-cap-source-snapshot.json'),
             ('--parent-views-only', 'parent-view-source-snapshot.json'),
             ('--postinfo-views-only', 'postinfo-view-source-snapshot.json'),
             ('--rgba-views-only', 'rgba-view-source-snapshot.json'),
             ('--colorref-import-only', 'colorref-import-source-snapshot.json'),
             ('--colorref-export-only', 'colorref-export-source-snapshot.json'),
             ('--rgba-comparison-only', 'rgba-comparison-source-snapshot.json'),
             ('--legend-comparison-only', 'legend-comparison-source-snapshot.json'),
             ('--render-policy-only', 'render-policy-source-snapshot.json'),
             ('--offset-fringe-only', 'offset-fringe-source-snapshot.json'),
             ('--vector-only', 'vector-source-snapshot.json'),
             ('--unit-only', 'unit-source-snapshot.json'))
    for flag, filename in modes:
        if flag in sys.argv: return filename
    return 'source-snapshot.json'

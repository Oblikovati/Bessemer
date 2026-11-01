"""Ordered Source reader profiles, independent of replay execution (#166)."""
from collections.abc import Callable, Mapping, Sequence

DialogReaderProvider = Callable[[], tuple[str, ...]]
DialogReaderSuffix = tuple[tuple[str, ...], tuple[str, ...]]
DIALOG_EARLY_READER_SUFFIXES: tuple[DialogReaderSuffix, ...] = (
    (('--legend-inverse-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_inverse_preference')),
    (('--legend-grouping-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout')),
    (('--legend-resets-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_read_resets')),
    (('--hidden-migration-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_legacy_hidden_policy')),
    (('--legend-width-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout')),
    (('--legend-end-cell-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_color_fields')),
    (('--legend-3d-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_colorref_import_view')),
    (('--legend-step-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_step_policy')),
    (('--legend-yield-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_color_fields', 'cosworks_legend_color_policy_fields')),
    (('--legend-height-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout')),
    (('--legend-palette-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_color_policy_fields')),
    (('--legend-position-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_position_fields')),
    (('--legend-format-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_legend_number_format')),
    (('--preference-colors-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_color_fields', 'cosworks_style_preference_colors', 'cosworks_colorref_import_view')),
    (('--mode3-rgb-only', '--segment-color-only', '--plot54-only'), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_color_fields')),
    (('--text-controls-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_post_style_text_view')),
    (('--aux-triangle-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_aux_triangle_gate')),
    (('--triangle-selectors-only', '--annotations-only'), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog')),
    (('--vector-style-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_style_field_catalog', 'cosworks_colorref_import_view')),
    (('--raw-preferences-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout')),
    (('--local-render-only',), ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_render_safety')),
)
DIALOG_DELEGATE_READER_FLAGS: tuple[str, ...] = ('--unit-only', '--vector-only', '--offset-fringe-only', '--render-policy-only')
DIALOG_FINAL_READER_RULES: tuple[tuple[str, tuple[str, ...], bool], ...] = (
    ('--legend-comparison-only', ('cosworks_legend_comparison', 'cosworks_color_comparison', 'cosworks_post_style_layout', 'cosworks_post_color_fields'), False),
    ('--rgba-comparison-only', ('cosworks_color_comparison',), False),
    ('--colorref-export-only', ('cosworks_colorref_view',), False),
    ('--colorref-import-only', ('cosworks_colorref_import_view', 'cosworks_colorref_view'), False),
    ('--rgba-views-only', ('cosworks_post_info_layout', 'cosworks_post_style_layout', 'cosworks_post_color_fields'), True),
    ('--postinfo-views-only', ('cosworks_post_info_fields', 'cosworks_post_contour_policy'), False),
    ('--parent-views-only', ('cosworks_legend_range_fields', 'cosworks_deform_scale_fields'), False),
)


def select_dialog_snapshot_readers(arguments: Sequence[str], base: tuple[str, ...], providers: Mapping[str, DialogReaderProvider]) -> tuple[str, ...]:
    """Preserve existing ordered mode priority without performing Source I/O; e.g. select_dialog_snapshot_readers(argv, base, providers)."""
    for flags, suffix in DIALOG_EARLY_READER_SUFFIXES:
        if any(flag in arguments for flag in flags): return base + suffix
    for flag in DIALOG_DELEGATE_READER_FLAGS:
        if flag in arguments: return providers[flag]()
    return select_final_dialog_readers(arguments, base, providers['default'])


def select_final_dialog_readers(arguments: Sequence[str], base: tuple[str, ...], fallback: DialogReaderProvider) -> tuple[str, ...]:
    """Keep final replacement/extension profiles and the existing default callback; e.g. select_final_dialog_readers(argv, base, fallback)."""
    for flag, modules, extend in DIALOG_FINAL_READER_RULES:
        if flag in arguments: return (base if extend else ()) + modules
    return fallback()

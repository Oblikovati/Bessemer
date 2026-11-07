"""Read-only PostInfo primitive/contour and RGBA component native field bindings (#166)."""
from typing import cast
import pefile
from native_carriers import digest
from dialog_bindings import dialog_images, dialog_export
from dialog_field_evidence import dialog_field_reports


def field_function_image(images: dict[str, pefile.PE], function: dict[str, str]) -> pefile.PE:
    """Normalize a field method's recorded module once; e.g. field_function_image(images, function)."""
    if function['address'] == '0x1814aabe0': return images['cosworks.dll']
    name = function.get('module', 'DGXPOST.dll').replace('\\', '/').rsplit('/', 1)[-1].lower()
    assert name in ('dgxpost.dll', 'dspogl.dll'), name
    return images[name]


def verify_field_function_spans(images: dict[str, pefile.PE], report: dict[str, object]) -> None:
    """Require every complete recorded field-method/decompilation byte binding; e.g. verify_field_function_spans(images, report)."""
    for function in cast(list[dict[str, str]], report.get('functions', [])):
        image = field_function_image(images, function)
        start, end = int(function['address'], 16), int(function['end'], 16)
        actual = digest(image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start))
        assert actual == function['raw_sha256'], (function['address'], actual)


def verify_field_leaf(image: pefile.PE, leaf: dict[str, str]) -> None:
    """Bind full leaf bytes, original digest and any exact exported name; e.g. verify_field_leaf(image, leaf)."""
    start, end = int(leaf['address'], 16), int(leaf['end'], 16)
    raw = image.get_data(start - image.OPTIONAL_HEADER.ImageBase, end - start)
    assert raw.hex() == leaf['raw_hex'] and digest(raw) == leaf['sha256'], leaf['address']
    if 'name' in leaf: dialog_export(image, leaf['name'].encode(), start)


def verify_field_literals(image: pefile.PE, report: dict[str, object]) -> None:
    """Keep named type constructors, direction tokens and the exact float sign mask bound; e.g. verify_field_literals(image, report)."""
    for exported in cast(list[dict[str, str]], report.get('type_exports', [])):
        dialog_export(image, exported['name'].encode(), int(exported['address'], 16))
    literals = cast(list[dict[str, str]], report.get('direction_literals', []))
    if 'sign_mask' in report: literals = literals + [cast(dict[str, str], report['sign_mask'])]
    for literal in literals:
        raw = bytes.fromhex(literal['raw_hex'])
        actual = image.get_data(int(literal['address'], 16) - image.OPTIONAL_HEADER.ImageBase, len(raw))
        assert actual == raw, literal['address']


def verify_dialog_field_bindings() -> None:
    """Check installed field accessors/conversions without native execution; e.g. verify_dialog_field_bindings()."""
    images = dialog_images()
    try:
        for report in dialog_field_reports()[:3]:
            verify_field_function_spans(images, report)
            image = images['dgxpost.dll']
            verify_field_literals(image, report)
            leaves = cast(list[dict[str, str]], report.get('leaves', [])) + cast(list[dict[str, str]], report.get('contour_methods', []))
            for leaf in leaves: verify_field_leaf(image, leaf)
            for leaf in cast(list[dict[str, str]], report.get('color_leaves', [])):
                verify_field_leaf(images['dspogl.dll'], leaf)
    finally:
        for image in images.values(): image.close()

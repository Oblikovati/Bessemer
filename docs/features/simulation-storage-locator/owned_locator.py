"""Independent literal UTF-16 prefix/suffix cases for the native lexical profile (#126)."""


def units(text: str) -> bytes:
    """Preserve unpaired code units instead of replacing them during reference transport."""
    return text.encode('utf-16-le', 'surrogatepass')


def valid_cases() -> list[tuple[bytes, bytes, bytes]]:
    """Construct expected components directly, with no last-dot parser in the builder."""
    prefixes = ('', 'x', 'x.y', 'a.b\\file', 'a/b/file', 'C:\\study', 'C:\\dir.name\\study',
                'relative.name', ' space ', '部品😀', '\ud800', '\udc00', '\uffff', '.hidden', '..', 'trailing.')
    suffixes = ('', 'GEN', 'MAS', 'ELF', 'OUT', 'EGR', 'gen ', 'MiXeD', 'stream\\leaf', 'stream/leaf',
                ' GÉN ', '名字😀', '\ud800', '\udc00', '\uffff', 'CWR')
    return [(units(prefix + '.' + suffix), units(prefix + '.CWR'), units(suffix))
            for prefix in prefixes for suffix in suffixes]


def invalid_cases() -> list[bytes]:
    """Keep both excluded shapes visible, including NUL before or after an otherwise valid delimiter."""
    return [units(value) for value in ('', 'study', 'a\\file', 'a/file', '😀', '\ud800', 'fullwidth．dot',
                                      '\0.GEN', 'study\0.GEN', 'study.GE\0N', 'study.GEN\0', '\0', '.\0')]

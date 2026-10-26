"""Independent profile word projection; kernel nodes retain their separate typed oracle."""
def reference_words(size: int, count: int, payload: bytes) -> list[list[int]]:
    if size <= 0 or size % 4 or count < 0 or len(payload) != size*count:
        raise ValueError(f'reference shape {(size,count,len(payload))}; expected aligned positive size and exact payload extent')
    return [[int.from_bytes(payload[row*size+column:row*size+column+4],'little')
             for column in range(0,size,4)] for row in range(count)]

def project(value: object, seen: set[int] | None = None) -> None:
    seen = set() if seen is None else seen
    if not isinstance(value,(dict,list,tuple)) or id(value) in seen:return
    seen.add(id(value))
    if not isinstance(value,dict):
        for child in value:project(child,seen)
        return
    if all(key in value for key in ['element_size','count','unknown_data']):
        value['words']=reference_words(value['element_size'],value['count'],value['unknown_data'])
        del value['unknown_data']
    if all(key in value for key in ['word_10','body','pairs','reference_data']):
        value['body']={'suppressed':value['body']['suppressed']}
        if 'extra_bodies' in value:
            value['extra_bodies']=[(index,{'suppressed':body['suppressed']}) for index,body in value['extra_bodies']]
    for child in value.values():project(child,seen)

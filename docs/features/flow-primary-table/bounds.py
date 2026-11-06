"""Count-field locations derived from independently reconstructed nested intervals."""
from __future__ import annotations
import packet


def element_counts(value: dict) -> list[int]:
    positions=[value['start']];offset=value['start']+4;width={1:4,2:8,3:12,4:16}[value['kind']]
    for row in value['rows']:
        positions.append(offset);offset+=4+width*len(row)
    positions.append(offset)
    return positions


def group_counts(value: dict, start: int) -> list[int]:
    offset=start+(1 if value['stored_mode_byte'] is not None else 0)+9;positions=[]
    if value['body'] is not None:
        body=value['body'];positions.extend((offset,offset+4+4*len(body['first_patterns'])))
        offset+=len(packet.body(body,offset)[0])
    if value['typed_elements'] is not None:
        positions.append(offset)
        for element in value['typed_elements']:
            positions.extend(element_counts(element))
    return positions


def positions(value: dict) -> list[int]:
    result=[4]
    for item in value['items']:
        offset=item['start']+2
        for group in item['groups']:
            result.extend(group_counts(group,offset));offset+=len(packet.group(group,offset)[0])
        assert offset==item['end']
    return result

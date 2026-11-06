"""Independent exact-real interpolation and branch oracle for accepted finite inputs."""
from __future__ import annotations
import bisect
import math
import struct
from fractions import Fraction


def bits(value: float) -> int:
    return struct.unpack('<Q',struct.pack('<d',value))[0]


def number(pattern: int) -> float:
    return struct.unpack('<d',struct.pack('<Q',pattern))[0]


def classification(value: float) -> int:
    if math.isnan(value):
        return 3
    if math.isinf(value):
        return 1 if value>0 else 2
    return 0


def branch(curve: dict, query: float) -> tuple[int, tuple[int,int] | None]:
    x=tuple(map(number,curve['first_patterns']))
    if not x:
        return 0,None
    if query<=x[0]:
        return 1,(0,0)
    if query>=x[-1]:
        return 2,(len(x)-1,len(x)-1)
    lower=bisect.bisect_right(x,query)-1
    return 3,(lower,lower+1)


def exact_value(curve: dict, query: float, indices: tuple[int,int] | None) -> Fraction:
    if indices is None:
        return Fraction.from_float(number(curve['scalar_pattern']))
    lower,upper=indices
    y=tuple(Fraction.from_float(number(value)) for value in curve['second_patterns'])
    if lower==upper:
        return y[lower]
    x=tuple(Fraction.from_float(number(value)) for value in curve['first_patterns'])
    position=Fraction.from_float(query)
    return ((x[upper]-position)*y[lower]+(position-x[lower])*y[upper])/(x[upper]-x[lower])


def error_bound(value: float, exact: Fraction) -> int | None:
    if not math.isfinite(value):
        return None
    error=abs(Fraction.from_float(value)-exact)
    try:
        rounded=float(error)
    except OverflowError:
        return bits(math.inf)
    if Fraction.from_float(rounded)<error:
        rounded=math.nextafter(rounded,math.inf)
    return bits(rounded)

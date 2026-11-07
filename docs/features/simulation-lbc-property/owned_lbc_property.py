"""Independent literal runtime-member orders for documented LBC profiles (#128)."""

BASE = (0x438, 0x43c, 0x440, 0x46c, 0x478, 0x47c, 0x480,
        0x494, 0x484, 0x488, 0x48c, 0x490, 0x508)
PREFIX = BASE + (0x4f8, 0x550, 0x498, 0x470, 0x474)
TEN = (0x444, 0x448, 0x44c, 0x450, 0x454, 0x458, 0x45c, 0x460, 0x464, 0x468)
VARIABLE = (0x49c, 0x4a0, 0x4a4, 0x4a8, 0x4ac, 0x4b0, 0x4b4, 0x4b8,
            0x4bc, 0x4c0, 0x4c4, 0x4c8, 0x4cc, 0x4d0, 0x4d4, 0x4d8, 0x4dc)
SIX = (0x4e0, 0x4e4, 0x4e8, 0x4ec, 0x4f0, 0x4f4)
FINAL = (0x520, 0x524, 0x528)
LAYOUTS: dict[int, tuple[int, ...]] = {
    52: BASE,
    56: BASE + (0x4f8,),
    60: BASE + (0x4f8, 0x550),
    64: BASE + (0x4f8, 0x550, 0x498),
    192: PREFIX + TEN + VARIABLE[:14] + SIX,
    204: PREFIX + TEN + VARIABLE[:14] + SIX + FINAL,
    208: PREFIX + TEN + VARIABLE[:15] + SIX + FINAL,
    212: PREFIX + TEN + VARIABLE[:16] + SIX + FINAL,
    216: PREFIX + TEN + VARIABLE + SIX + FINAL,
}
BOUNDARIES = ((0, 52), (800027, 52), (800028, 56), (800059, 56),
              (800060, 60), (820009, 60), (820010, 64), (840003, 64),
              (840004, 192), (840012, 192), (840013, 204), (890619, 204),
              (890620, 208), (890999, 208), (891000, 212), (891100, 212),
              (891101, 216), (0x7fffffff, 216))

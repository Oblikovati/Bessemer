# GDI+ checks pixel interpretation independently of Bessemer and the fixture packer.
param([string]$RepositoryRoot = (Resolve-Path "$PSScriptRoot/../../..").Path)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

function Read-IndexedWord([byte[]]$Packet, [ref]$Offset) {
    $word = [BitConverter]::ToUInt32($Packet, $Offset.Value)
    $Offset.Value += 4
    return $word
}

function Read-IndexedBlob([byte[]]$Packet, [ref]$Offset) {
    $length = Read-IndexedWord $Packet $Offset
    $blob = [byte[]]$Packet[$Offset.Value..($Offset.Value + $length - 1)]
    $Offset.Value += $length
    return ,$blob
}

function Convert-IndexedBmp([byte[]]$Packed) {
    $bits = [BitConverter]::ToUInt16($Packed, 18)
    $palette = [BitConverter]::ToUInt32($Packed, 36)
    if ($palette -eq 0) { $palette = 1 -shl $bits }
    $length = [BitConverter]::GetBytes([uint32]($Packed.Length + 10))
    $pixelOffset = [BitConverter]::GetBytes([uint32](54 + 4 * $palette))
    return ,([byte[]](0x42,0x4d) + $length + [byte[]](0,0,0,0) + $pixelOffset + $Packed[4..($Packed.Length-1)])
}

function Assert-NativeIndexedPixels([Drawing.Bitmap]$Bitmap, [byte[]]$Indices) {
    $rgb = [Collections.Generic.List[byte]]::new()
    for ($position = 0; $position -lt $Indices.Length; $position++) {
        $color = $Bitmap.GetPixel($position % $Bitmap.Width, [int][Math]::Floor($position / $Bitmap.Width))
        $index = $Indices[$position]
        if ($color.B -ne $index -or $color.G -ne 255 - $index -or $color.R -ne ($index -bxor 0x5a)) {
            throw "Pixel $position has RGB $($color.R)/$($color.G)/$($color.B); expected palette index $index"
        }
        $rgb.AddRange([byte[]]($color.B,$color.G,$color.R))
    }
    return ,$rgb.ToArray()
}

function Read-NativeIndexedRgb([byte[]]$Packed, [byte[]]$Indices) {
    $bitmapStream = [IO.MemoryStream]::new([byte[]](Convert-IndexedBmp $Packed))
    $bitmap = $null
    try {
        $bitmap = [Drawing.Bitmap]::new($bitmapStream)
        return ,([byte[]](Assert-NativeIndexedPixels $bitmap $Indices))
    } finally {
        if ($null -ne $bitmap) { $bitmap.Dispose() }
        $bitmapStream.Dispose()
    }
}

$fixturePath = Join-Path $RepositoryRoot 'tests/fixtures/preview-indexed-rgb/cases.bin'
$packet = [IO.File]::ReadAllBytes($fixturePath)
if ([Text.Encoding]::ASCII.GetString($packet,0,4) -ne 'BIX1') { throw 'Fixture packet has an invalid magic; expected BIX1' }
$offset = 4
$count = Read-IndexedWord $packet ([ref]$offset)
if ($count -ne 864) { throw "Fixture count $count; expected 864" }
$nativeDigest = [Security.Cryptography.IncrementalHash]::CreateHash([Security.Cryptography.HashAlgorithmName]::SHA256)
$images = 0
$pixels = 0
for ($caseIndex = 0; $caseIndex -lt $count; $caseIndex++) {
    $original = Read-IndexedBlob $packet ([ref]$offset)
    $edited = Read-IndexedBlob $packet ([ref]$offset)
    $indices = Read-IndexedBlob $packet ([ref]$offset)
    $editedIndices = Read-IndexedBlob $packet ([ref]$offset)
    if ($caseIndex -eq 0) { $firstImage = $original; $firstIndices = $indices }
    $nativeDigest.AppendData([byte[]](Read-NativeIndexedRgb $original $indices))
    $nativeDigest.AppendData([byte[]](Read-NativeIndexedRgb $edited $editedIndices))
    $images += 2
    $pixels += $indices.Length + $editedIndices.Length
}
if ($offset -ne $packet.Length) { throw "Consumed $offset bytes; expected exact packet length $($packet.Length)" }
$minimalHashes = @{}
foreach ($bits in @(1,4,8)) {
    $minimal = [IO.File]::ReadAllBytes((Join-Path (Split-Path $fixturePath) "one-entry-$bits.bin"))
    $nativeDigest.AppendData([byte[]](Read-NativeIndexedRgb $minimal ([byte[]](0))))
    $minimalHashes["one-entry-$bits.bin"] = [Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($minimal)).ToLowerInvariant()
    $images++
    $pixels++
}
$wrongIndices = [byte[]]$firstIndices.Clone()
$wrongIndices[0] = $wrongIndices[0] -bxor 1
$negativeRejected = $false
try { Read-NativeIndexedRgb $firstImage $wrongIndices | Out-Null } catch {
    if ($_.Exception.Message -notlike 'Pixel *') { throw }
    $negativeRejected = $true
}
if (-not $negativeRejected) { throw 'Wrong index was accepted; expected native pixel mismatch' }
$manifest = Get-Content (Join-Path $RepositoryRoot 'tests/fixtures/preview-indexed-rgb/manifest.json') -Raw | ConvertFrom-Json
if ($images -ne $manifest.images + 3 -or $pixels -ne $manifest.pixels + 3) { throw 'Native image/pixel totals differ from the matrix and three minimal cases' }
$result = @{images=$images; pixels=$pixels; cases=$count; one_entry_sha256=$minimalHashes; negative_oracle_rejected=$negativeRejected; decoder=[Drawing.Bitmap].Assembly.FullName; rgb_sha256=[Convert]::ToHexString($nativeDigest.GetHashAndReset()).ToLowerInvariant(); packet_sha256=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($packet)).ToLowerInvariant()}
$nativeDigest.Dispose()
$result | ConvertTo-Json

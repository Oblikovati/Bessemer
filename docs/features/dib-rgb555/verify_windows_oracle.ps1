# A native decoder checks all raw RGB555 words independently of the Rust implementation.
param([string]$RepositoryRoot = (Resolve-Path "$PSScriptRoot/../../..").Path)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing

function Convert-Rgb555Bmp([byte[]]$Packed) {
    $palette = [BitConverter]::ToUInt32($Packed, 36)
    $length = [BitConverter]::GetBytes([uint32]($Packed.Length + 10))
    $offset = [BitConverter]::GetBytes([uint32](54 + 4 * $palette))
    return ,([byte[]](0x42,0x4d) + $length + [byte[]](0,0,0,0) + $offset + $Packed[4..($Packed.Length-1)])
}

function Assert-Rgb555Pixels([Drawing.Bitmap]$Bitmap, [byte[]]$Words) {
    $rgb = [Collections.Generic.List[byte]]::new()
    for ($position = 0; $position -lt $Words.Length / 2; $position++) {
        $word = [BitConverter]::ToUInt16($Words, $position * 2)
        $color = $Bitmap.GetPixel($position % $Bitmap.Width, [int][Math]::Floor($position / $Bitmap.Width))
        $expected = [byte[]]@(foreach ($shift in @(0,5,10)) {
            $level = ($word -shr $shift) -band 31
            ($level -shl 3) -bor ($level -shr 2)
        })
        $actual = [byte[]]($color.B,$color.G,$color.R)
        if ([Convert]::ToHexString($actual) -ne [Convert]::ToHexString($expected)) {
            throw "Pixel $position has BGR $([Convert]::ToHexString($actual)); expected RGB555 word $word"
        }
        $rgb.AddRange($actual)
    }
    return ,$rgb.ToArray()
}

function Read-NativeRgb555([byte[]]$Packed, [byte[]]$Words) {
    $bitmapStream = [IO.MemoryStream]::new([byte[]](Convert-Rgb555Bmp $Packed))
    $bitmap = $null
    try {
        $bitmap = [Drawing.Bitmap]::new($bitmapStream)
        if ($bitmap.Width * $bitmap.Height * 2 -ne $Words.Length) { throw "Raster bytes $($Words.Length); expected native dimensions $($bitmap.Width)x$($bitmap.Height) times two" }
        return ,([byte[]](Assert-Rgb555Pixels $bitmap $Words))
    } finally {
        if ($null -ne $bitmap) { $bitmap.Dispose() }
        $bitmapStream.Dispose()
    }
}

$fixturePath = Join-Path $RepositoryRoot 'tests/fixtures/preview-rgb555/cases.bin'
$packet = [IO.File]::ReadAllBytes($fixturePath)
$reader = [IO.BinaryReader]::new([IO.MemoryStream]::new($packet))
if ([Text.Encoding]::ASCII.GetString($reader.ReadBytes(4)) -ne 'B165') { throw 'Invalid fixture magic; expected B165' }
$count = $reader.ReadUInt32()
if ($count -ne 136) { throw "Fixture count $count; expected 136" }
$digest = [Security.Cryptography.IncrementalHash]::CreateHash([Security.Cryptography.HashAlgorithmName]::SHA256)
$images = 0
$pixels = 0
for ($caseIndex = 0; $caseIndex -lt $count; $caseIndex++) {
    $original = $reader.ReadBytes($reader.ReadInt32())
    $edited = $reader.ReadBytes($reader.ReadInt32())
    $words = $reader.ReadBytes($reader.ReadInt32())
    $editedWords = $reader.ReadBytes($reader.ReadInt32())
    if ($caseIndex -eq 0) { $firstImage = $original; $firstWords = $words }
    $digest.AppendData([byte[]](Read-NativeRgb555 $original $words))
    $digest.AppendData([byte[]](Read-NativeRgb555 $edited $editedWords))
    $images += 2
    $pixels += ($words.Length + $editedWords.Length) / 2
}
if ($reader.BaseStream.Position -ne $reader.BaseStream.Length) { throw 'Fixture parser did not reach exact EOF' }
$reader.Dispose()
$wrong = [byte[]]$firstWords.Clone()
$wrong[0] = $wrong[0] -bxor 1
$negativeRejected = $false
try { Read-NativeRgb555 $firstImage $wrong | Out-Null } catch {
    if ($_.Exception.Message -notlike 'Pixel *') { throw }
    $negativeRejected = $true
}
if (-not $negativeRejected) { throw 'Wrong RGB555 word was accepted; expected native mismatch' }
$unused = [byte[]]$firstWords.Clone()
for ($index = 1; $index -lt $unused.Length; $index += 2) { $unused[$index] = $unused[$index] -bxor 0x80 }
$highBitIgnored = [byte[]](Read-NativeRgb555 $firstImage $unused)
$originalRgb = [byte[]](Read-NativeRgb555 $firstImage $firstWords)
if ([Convert]::ToHexString($highBitIgnored) -ne [Convert]::ToHexString($originalRgb)) { throw 'Unused high bit changed RGB; expected identical native colors' }
$manifest = Get-Content (Join-Path (Split-Path $fixturePath) 'manifest.json') -Raw | ConvertFrom-Json
if ($images -ne $manifest.images -or $pixels -ne $manifest.pixels) { throw 'Native image/pixel totals differ from the literal manifest' }
$result = @{cases=$count; images=$images; pixels=$pixels; decoder=[Drawing.Bitmap].Assembly.FullName; negative_oracle_rejected=$negativeRejected; unused_high_bit_ignored=$true; rgb_sha256=[Convert]::ToHexString($digest.GetHashAndReset()).ToLowerInvariant(); packet_sha256=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($packet)).ToLowerInvariant()}
$digest.Dispose()
$result | ConvertTo-Json

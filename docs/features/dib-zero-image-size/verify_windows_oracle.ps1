# Native GDI+ supplies an independent RGB oracle; BGRX fourth bytes stay raw in Rust.
param([string]$RepositoryRoot = (Resolve-Path "$PSScriptRoot/../../..").Path)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName System.Drawing
$fixtureRoot = Join-Path $RepositoryRoot 'tests/fixtures/preview-zero-size'
$nativeResults = @()
foreach ($fixtureCase in (Get-Content "$fixtureRoot/manifest.json" -Raw | ConvertFrom-Json)) {
    $packed = [IO.File]::ReadAllBytes("$fixtureRoot/$($fixtureCase.name).bin")
    $expected = [IO.File]::ReadAllBytes("$fixtureRoot/$($fixtureCase.name).bgrx")
    $bmpSize = $packed.Length + 10
    $pixelOffset = 54 + 4 * $fixtureCase.palette_count
    $bmpHeader = [byte[]](0x42,0x4d) + [BitConverter]::GetBytes([uint32]$bmpSize) + [byte[]](0,0,0,0) + [BitConverter]::GetBytes([uint32]$pixelOffset)
    $bitmapStream = [IO.MemoryStream]::new([byte[]]($bmpHeader + $packed[4..($packed.Length-1)]))
    $nativeBitmap = $null
    try {
        $nativeBitmap = [Drawing.Bitmap]::new($bitmapStream)
        $nativeRgb = [Collections.Generic.List[byte]]::new()
        for ($pixelIndex = 0; $pixelIndex -lt $expected.Length / 4; $pixelIndex++) {
            $color = $nativeBitmap.GetPixel($pixelIndex % $fixtureCase.width, [int][Math]::Floor($pixelIndex / $fixtureCase.width))
            $actualRgb = [byte[]]($color.B,$color.G,$color.R)
            $expectedRgb = $expected[($pixelIndex*4)..($pixelIndex*4+2)]
            if ([Convert]::ToHexString($actualRgb) -ne [Convert]::ToHexString($expectedRgb)) { throw "Pixel $pixelIndex of $($fixtureCase.name) did not match literal BGR" }
            $nativeRgb.AddRange($actualRgb)
        }
        $nativeResults += @{name=$fixtureCase.name; pixels=$expected.Length/4; rgb_sha256=[Convert]::ToHexString([Security.Cryptography.SHA256]::HashData($nativeRgb.ToArray())).ToLowerInvariant()}
    } finally {
        if ($null -ne $nativeBitmap) { $nativeBitmap.Dispose() }
        $bitmapStream.Dispose()
    }
}
@{decoder=[Drawing.Bitmap].Assembly.FullName; cases=$nativeResults; total_pixels=($nativeResults | Measure-Object pixels -Sum).Sum} | ConvertTo-Json -Depth 5

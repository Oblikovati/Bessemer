param([string]$Workspace = (Get-Location).Path, [string]$SourcePdf)
$ErrorActionPreference = 'Stop'

function Test-ChecksumBytes([byte[]]$Actual, [byte[]]$Expected) {
    if ($Actual.Length -ne 16 -or $Expected.Length -ne 16) { return $false }
    for ($index = 0; $index -lt 16; $index++) {
        if ($Actual[$index] -ne $Expected[$index]) { return $false }
    }
    return $true
}

function Get-CallerMd5Hex([byte[]]$Bytes, [Security.Cryptography.HashAlgorithm]$Algorithm) {
    $digest = $Algorithm.ComputeHash($Bytes)
    return [BitConverter]::ToString($digest).Replace('-', '').ToLowerInvariant()
}

function Get-NativePdfOracle([string]$Path, [Security.Cryptography.HashAlgorithm]$Algorithm) {
    $original = [IO.File]::ReadAllBytes($Path)
    $digest = Get-CallerMd5Hex $original $Algorithm
    if ($original.Length -ne 164616 -or $digest -ne '32c2e42f4813cfedb8edc1d7a5b39b8e') { throw "PDF $Path length $($original.Length) MD5 $digest; expected original tutorial identity" }
    $sha256 = (Get-FileHash -LiteralPath $Path -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($sha256 -ne 'e7769d7679d8e845849e733af21e36013bd88922afae77a7193840dc38263591') { throw "PDF $Path SHA256 $sha256; expected pinned tutorial bytes" }
    $modified = [byte[]]$original.Clone()
    $modified[$modified.Length - 1] = $modified[$modified.Length - 1] -bxor 1
    $truncated = New-Object byte[] ($original.Length - 1)
    [Array]::Copy($original, $truncated, $truncated.Length)
    return @{ path=$Path; bytes=$original.Length; sha256=$sha256; md5=$digest; modified_md5=(Get-CallerMd5Hex $modified $Algorithm); truncated_md5=(Get-CallerMd5Hex $truncated $Algorithm) }
}

$packetPath = Join-Path $Workspace 'tests/fixtures/inspection-source-identity/vectors.bin'
$reader = [IO.BinaryReader]::new([IO.File]::OpenRead($packetPath))
$algorithm = [Security.Cryptography.MD5]::Create()
$utf8 = [Text.UTF8Encoding]::new($false, $true)
$verified = 0
try {
    if ([Text.Encoding]::ASCII.GetString($reader.ReadBytes(4)) -ne 'ISMD') { throw 'Packet magic; expected ISMD' }
    $count = $reader.ReadUInt32()
    if ($count -ne 172) { throw "Case count $count; expected 172" }
    for ($case = 0; $case -lt $count; $case++) {
        $labelLength = $reader.ReadUInt32(); $inputLength = $reader.ReadUInt32()
        $expected = $reader.ReadBytes(16)
        $label = $utf8.GetString($reader.ReadBytes($labelLength))
        $inputBytes = $reader.ReadBytes($inputLength)
        if ($inputBytes.Length -ne $inputLength) { throw "Case $label bytes $($inputBytes.Length); expected $inputLength" }
        if (!(Test-ChecksumBytes ($algorithm.ComputeHash($inputBytes)) $expected)) { throw "Case $label checksum; expected frozen Python MD5 bytes" }
        $verified++
    }
    if ($reader.BaseStream.Position -ne $reader.BaseStream.Length) { throw 'Packet suffix; expected exact EOF' }
    $report = @{ issue=195; verified_cases=$verified; provider=$algorithm.GetType().FullName; framework=[Environment]::Version.ToString(); packet_bytes=$reader.BaseStream.Length; packet_sha256=(Get-FileHash -LiteralPath $packetPath -Algorithm SHA256).Hash.ToLowerInvariant(); scope='Independent installed .NET provider checked every frozen Python expectation; no Bessemer implementation exercised' }
    if ($SourcePdf) { $report.native_source_pdf = Get-NativePdfOracle $SourcePdf $algorithm }
    $destination = Join-Path $Workspace 'docs/features/inspection-source-identity/dotnet-reference-verification.json'
    [IO.File]::WriteAllText($destination, ($report | ConvertTo-Json) + "`n", [Text.UTF8Encoding]::new($false))
    Write-Output "Verified $verified independent .NET checksum cases through EOF."
}
finally {
    $reader.Dispose()
    $algorithm.Dispose()
}

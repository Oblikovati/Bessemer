param([string]$Workspace = (Get-Location).Path)
$ErrorActionPreference = 'Stop'

function Get-Digest([byte[]]$Bytes) {
    $algorithm = [Security.Cryptography.SHA256]::Create()
    try { return [BitConverter]::ToString($algorithm.ComputeHash($Bytes)).Replace('-', '').ToLowerInvariant() }
    finally { $algorithm.Dispose() }
}

function Convert-Envelope([byte[]]$Bytes, [bool]$Historical, [bool]$Encrypt) {
    $algorithm = if ($Historical) { New-Object Security.Cryptography.RijndaelManaged } else { [Security.Cryptography.Aes]::Create() }
    $key = if ($Historical) { [byte[]](1,2,2,3,3,5,5,4,3) } else { [byte[]](1,2,2,3,3,5,5,5,4,4,5,5,3,3,2,1) }
    $iv = New-Object byte[] 16
    $transform = if ($Encrypt) { $algorithm.CreateEncryptor($key, $iv) } else { $algorithm.CreateDecryptor($key, $iv) }
    try { return ,$transform.TransformFinalBlock($Bytes, 0, $Bytes.Length) }
    finally { $transform.Dispose(); $algorithm.Dispose() }
}

function Save-Case([string]$Stem, [byte[]]$Stored, [byte[]]$Plain, [bool]$Historical) {
    $rebuilt = Convert-Envelope $Plain $Historical $true
    if ((Get-Digest $rebuilt) -ne (Get-Digest $Stored)) { throw "Envelope $Stem; expected byte-identical Framework reconstruction" }
    $suffix = [Text.Encoding]::UTF8.GetBytes("`n<!-- Bessemer #92 -->`n")
    $edited = New-Object byte[] ($Plain.Length + $suffix.Length)
    [Array]::Copy($Plain, $edited, $Plain.Length)
    [Array]::Copy($suffix, 0, $edited, $Plain.Length, $suffix.Length)
    [IO.File]::WriteAllBytes((Join-Path $fixtureRoot "$Stem.bin"), $Stored)
    [IO.File]::WriteAllBytes((Join-Path $fixtureRoot "$Stem.xml"), $Plain)
    [IO.File]::WriteAllBytes((Join-Path $fixtureRoot "$Stem-edited.xml"), $edited)
    [IO.File]::WriteAllBytes((Join-Path $fixtureRoot "$Stem-edited.bin"), (Convert-Envelope $edited $Historical $true))
    return @{ stem=$Stem; historical=$Historical; stored_bytes=$Stored.Length; xml_bytes=$Plain.Length; stored_sha256=(Get-Digest $Stored); xml_sha256=(Get-Digest $Plain) }
}

$reference = [IO.Path]::GetFullPath((Join-Path $Workspace '../inventor-interface/solidworks/research'))
$fixtureRoot = [IO.Path]::GetFullPath((Join-Path $Workspace 'tests/fixtures/standalone-inspection'))
[void][IO.Directory]::CreateDirectory($fixtureRoot)
$templates = 'C:/ProgramData/SOLIDWORKS/SOLIDWORKS Inspection 2026 Standalone/Templates'
Add-Type -AssemblyName System.Web.Extensions
# Framework PowerShell's ConvertFrom-Json rejects the evidence's valid empty-name path keys.
$json = New-Object Web.Script.Serialization.JavaScriptSerializer
$json.MaxJsonLength = [int]::MaxValue
$corpus = $json.DeserializeObject([IO.File]::ReadAllText((Join-Path $reference 'inspection-template-corpus.json')))
$legacy = $json.DeserializeObject([IO.File]::ReadAllText((Join-Path $reference 'inspection-legacy-evidence.json')))
$orderedDigests = [string[]]@($corpus.unique_framework_decodes.Keys | Sort-Object)
$seen = @{}; $cases = @(); $files = @()
if ([IO.Directory]::GetFiles($templates, '*.ixpdot').Length -ne 42) { throw 'Installed template inventory; expected 42 files' }
foreach ($fixture in $corpus.files) {
    $stored = [IO.File]::ReadAllBytes((Join-Path $templates $fixture.name))
    $digest = Get-Digest $stored
    if ($digest -ne $fixture.sha256) { throw "Template $($fixture.name) hash $digest; expected $($fixture.sha256)" }
    $plain = Convert-Envelope $stored $false $false
    if ((Get-Digest $plain) -ne $fixture.decoded_sha256) { throw "Template $($fixture.name); expected independent plaintext $($fixture.decoded_sha256)" }
    if (-not $seen.ContainsKey($digest)) {
        $stem = 'current-' + [Array]::IndexOf($orderedDigests, $digest)
        $cases += Save-Case $stem $stored $plain $false
        $seen[$digest] = $stem
    }
    $files += @{ name=$fixture.name; stem=$seen[$digest]; stored_sha256=$digest; xml_sha256=(Get-Digest $plain) }
}
if ($files.Count -ne 42 -or $seen.Count -ne 2) { throw "Template files $($files.Count), variants $($seen.Count); expected 42 and 2" }
$tutorial = 'C:/Program Files/SOLIDWORKS Corp/SOLIDWORKS Inspection/PDF/Tutorial/cmm data import.ixprj'
$stored = [IO.File]::ReadAllBytes($tutorial)
$plain = Convert-Envelope $stored $true $false
if ((Get-Digest $stored) -ne $legacy.fixture.stored_sha256 -or (Get-Digest $plain) -ne $legacy.fixture.framework.sha256) { throw 'Historical tutorial hashes; expected retained native evidence' }
$cases += Save-Case 'historical' $stored $plain $true
$rows = @($files | ForEach-Object { "$($_.name)`t$($_.stem)`t$($_.stored_sha256)`t$($_.xml_sha256)" })
[IO.File]::WriteAllText((Join-Path $fixtureRoot 'templates.tsv'), (($rows -join "`n") + "`n"), (New-Object Text.UTF8Encoding $false))
$report = @{ framework=[Environment]::Version.ToString(); cases=$cases; files=$files; source_hashes=@{} }
foreach ($name in @('inspection-template-corpus.json','inspection-legacy-evidence.json','inspection-legacy-vectors.json')) {
    $report.source_hashes[$name] = Get-Digest ([IO.File]::ReadAllBytes((Join-Path $reference $name)))
}
$destination = Join-Path $Workspace 'docs/features/standalone-inspection/native-reference.json'
[IO.File]::WriteAllText($destination, (($report | ConvertTo-Json -Depth 12) + "`n"), (New-Object Text.UTF8Encoding $false))
Write-Output "Framework $($report.framework): $($files.Count) current templates, $($cases.Count) distinct envelope fixtures"

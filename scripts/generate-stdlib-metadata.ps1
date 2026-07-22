param(
  [switch]$Check
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$stdlibDir = Join-Path $repoRoot "src\stdlib"
$outputPath = Join-Path $stdlibDir "metadata\registered.rs"

function Convert-RustTypeToRhaiType {
  param([string]$RustType)

  $type = $RustType.Trim()
  switch -Regex ($type) {
    '^(i64|u64|i32|u32|usize|isize)$' { return "int" }
    '^bool$' { return "bool" }
    '^(&str|String)$' { return "string" }
    '^rhai::Blob$' { return "blob" }
    '^(Array|Vec<.*>)$' { return "array" }
    default { return $type }
  }
}

function Convert-ParamListToSignature {
  param([string]$ParamText)

  if ([string]::IsNullOrWhiteSpace($ParamText)) {
    return ""
  }

  $params = @()
  foreach ($match in [regex]::Matches($ParamText, '([A-Za-z_][A-Za-z0-9_]*)\s*:\s*([^,\)]+)')) {
    $name = $match.Groups[1].Value
    if ($name -eq "context") {
      continue
    }

    $type = Convert-RustTypeToRhaiType $match.Groups[2].Value
    $params += "${name}: ${type}"
  }

  return ($params -join ", ")
}

function New-Registration {
  param(
    [string]$Module,
    [string]$Name,
    [string]$Params
  )

  $signatureParams = Convert-ParamListToSignature $Params
  $signature = "${Module}::${Name}(${signatureParams})"
  [pscustomobject]@{
    Module = $Module
    Name = $Name
    Signature = $signature
  }
}

$registrations = @()
$sourceFiles = Get-ChildItem -Path $stdlibDir -Recurse -Filter "*.rs" |
  Where-Object {
    $_.BaseName -notin @("mod", "registration", "util") -and
    $_.FullName -notmatch "[\\/]metadata[\\/]" -and
    $_.FullName -notmatch "[\\/]traits[\\/]"
  }

foreach ($file in $sourceFiles) {
  $moduleName = $file.BaseName
  $source = Get-Content -Raw -Encoding UTF8 -Path $file.FullName

  foreach ($match in [regex]::Matches($source, 'register_stdlib_fn_\d!\s*\(\s*module\s*,\s*stdlib(?:\.clone\(\))?\s*,\s*"([^"]+)"\s*,\s*[A-Za-z_][A-Za-z0-9_]*\s*(?:,\s*(.*?))?\);', [System.Text.RegularExpressions.RegexOptions]::Singleline)) {
    $registrations += New-Registration $moduleName $match.Groups[1].Value $match.Groups[2].Value
  }

  foreach ($match in [regex]::Matches($source, 'module\.set_native_fn\s*\(\s*"([^"]+)"\s*,\s*(?:move\s*)?\|([^|]*)\|', [System.Text.RegularExpressions.RegexOptions]::Singleline)) {
    $registrations += New-Registration $moduleName $match.Groups[1].Value $match.Groups[2].Value
  }
}

$registrations = $registrations |
  Sort-Object Module, Name -Unique

$lines = @()
$lines += "use super::StdlibFunctionRegistration;"
$lines += ""
$lines += "pub const FUNCTIONS: &[StdlibFunctionRegistration] = &["
foreach ($registration in $registrations) {
  $lines += "    StdlibFunctionRegistration::new(`"$($registration.Module)`", `"$($registration.Name)`", `"$($registration.Signature)`"),"
}
$lines += "];"
$content = ($lines -join "`n") + "`n"

if ($Check) {
  $current = Get-Content -Raw -Encoding UTF8 -Path $outputPath
  if ($current -cne $content) {
    throw "stdlib metadata is out of date; run scripts\generate-stdlib-metadata.ps1"
  }
  Write-Host "stdlib metadata is up to date ($($registrations.Count) functions)"
  exit 0
}

Set-Content -Encoding UTF8 -NoNewline -Path $outputPath -Value $content
Write-Host "generated $outputPath ($($registrations.Count) functions)"

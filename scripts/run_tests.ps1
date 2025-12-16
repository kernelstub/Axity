param([switch]$Release,[switch]$Verbose)
$Repo = Join-Path (Split-Path -Parent $PSCommandPath) ".."
Set-Location $Repo
$env:RUST_BACKTRACE = "1"
if ($Release) {
  if ($Verbose) { cargo test --release } else { cargo test --release -q }
} else {
  if ($Verbose) { cargo test } else { cargo test -q }
}

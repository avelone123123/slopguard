$ErrorActionPreference = 'Stop'
$repo = 'https://github.com/avelone123123/slopguard.git'
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
if ($cargo) {
    Write-Host 'Installing SlopGuard via Cargo Git...' -ForegroundColor Cyan
    & cargo install --git $repo --bin slopguard
    exit $LASTEXITCODE
}
$binDir = Join-Path $env:USERPROFILE '.cargo\bin'
if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir -Force | Out-Null }
$dest = Join-Path $binDir 'slopguard.exe'
$url = 'https://github.com/avelone123123/slopguard/releases/latest/download/slopguard-windows-x64.exe'
Write-Host 'Downloading SlopGuard binary from GitHub Releases...' -ForegroundColor Cyan
Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
$userPath = [Environment]::GetEnvironmentVariable('Path', [EnvironmentVariableTarget]::User)
if ($userPath -notlike "*$binDir*") {
    [Environment]::SetEnvironmentVariable('Path', "$userPath;$binDir", [EnvironmentVariableTarget]::User)
}
Write-Host "SlopGuard installed successfully into $dest" -ForegroundColor Green
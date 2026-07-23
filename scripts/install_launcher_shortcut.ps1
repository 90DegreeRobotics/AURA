param(
    [string]$Configuration = "release"
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$DistRoot = Join-Path $RepoRoot "dist"
$TargetExe = Join-Path $RepoRoot "target\$Configuration\aura_launcher.exe"
$DistExe = Join-Path $DistRoot "aura_launcher.exe"

Push-Location $RepoRoot
try {
    if ($Configuration -eq "release") {
        cargo build -p aura_launcher --release
    } else {
        cargo build -p aura_launcher
    }

    New-Item -ItemType Directory -Force -Path $DistRoot | Out-Null
    Copy-Item -LiteralPath $TargetExe -Destination $DistExe -Force

    $shell = New-Object -ComObject WScript.Shell
    $desktop = [Environment]::GetFolderPath("DesktopDirectory")
    $programs = [Environment]::GetFolderPath("Programs")
    $startDir = Join-Path $programs "NeuroCognica"
    New-Item -ItemType Directory -Force -Path $startDir | Out-Null

    $shortcuts = @(
        (Join-Path $desktop "AURA.lnk"),
        (Join-Path $startDir "AURA.lnk")
    )

    foreach ($path in $shortcuts) {
        $shortcut = $shell.CreateShortcut($path)
        $shortcut.TargetPath = $DistExe
        $shortcut.WorkingDirectory = $DistRoot
        $shortcut.Description = "AURA Sentinel-first desktop launcher"
        $shortcut.Save()
    }

    Write-Host "AURA launcher installed:"
    Write-Host "  $DistExe"
    Write-Host "Shortcuts:"
    foreach ($path in $shortcuts) {
        Write-Host "  $path"
    }
}
finally {
    Pop-Location
}

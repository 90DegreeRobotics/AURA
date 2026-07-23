param(
    [string]$Configuration = "release"
)

$ErrorActionPreference = "Stop"

$RepoRoot = Split-Path -Parent $PSScriptRoot
$DistRoot = Join-Path $RepoRoot "dist"
$TargetExe = Join-Path $RepoRoot "target\$Configuration\aura_launcher.exe"
$DistExe = Join-Path $DistRoot "aura_launcher.exe"
$IconSource = Join-Path $RepoRoot "assets\icon\aura.ico"
$DistIcon = Join-Path $DistRoot "aura.ico"

Push-Location $RepoRoot
try {
    if ($Configuration -eq "release") {
        cargo build -p aura_launcher --release
    } else {
        cargo build -p aura_launcher
    }

    New-Item -ItemType Directory -Force -Path $DistRoot | Out-Null
    Copy-Item -LiteralPath $TargetExe -Destination $DistExe -Force
    Copy-Item -LiteralPath $IconSource -Destination $DistIcon -Force

    $shell = New-Object -ComObject WScript.Shell
    $desktopCandidates = @(
        [Environment]::GetFolderPath("DesktopDirectory"),
        (Join-Path $env:USERPROFILE "Desktop"),
        (Join-Path $env:PUBLIC "Desktop")
    ) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -Unique
    $programs = [Environment]::GetFolderPath("Programs")
    $startDir = Join-Path $programs "NeuroCognica"
    New-Item -ItemType Directory -Force -Path $startDir | Out-Null

    $shortcuts = @()
    foreach ($desktop in $desktopCandidates) {
        New-Item -ItemType Directory -Force -Path $desktop | Out-Null
        $shortcuts += (Join-Path $desktop "AURA.lnk")
    }
    $shortcuts += (Join-Path $startDir "AURA.lnk")
    $shortcuts = $shortcuts | Select-Object -Unique

    $createdShortcuts = @()
    foreach ($path in $shortcuts) {
        try {
            $shortcut = $shell.CreateShortcut($path)
            $shortcut.TargetPath = $DistExe
            $shortcut.WorkingDirectory = $DistRoot
            $shortcut.IconLocation = "$DistIcon,0"
            $shortcut.Description = "AURA Sentinel-first desktop launcher"
            $shortcut.Save()
            $createdShortcuts += $path
        }
        catch {
            Write-Warning "Could not save shortcut ${path}: $($_.Exception.Message)"
        }
    }

    Write-Host "AURA launcher installed:"
    Write-Host "  $DistExe"
    Write-Host "Icon:"
    Write-Host "  $DistIcon"
    Write-Host "Shortcuts:"
    foreach ($path in $createdShortcuts) {
        Write-Host "  $path"
    }
}
finally {
    Pop-Location
}

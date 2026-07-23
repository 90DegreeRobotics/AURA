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
$BrandSource = Join-Path $RepoRoot "assets\brand"
$DistAssetsRoot = Join-Path $DistRoot "assets"
$DistBrand = Join-Path $DistAssetsRoot "brand"

Push-Location $RepoRoot
try {
    if ($Configuration -eq "release") {
        cargo build -p aura_launcher --release
    } else {
        cargo build -p aura_launcher
    }

    New-Item -ItemType Directory -Force -Path $DistRoot | Out-Null
    New-Item -ItemType Directory -Force -Path $DistAssetsRoot | Out-Null
    Copy-Item -LiteralPath $TargetExe -Destination $DistExe -Force
    Copy-Item -LiteralPath $IconSource -Destination $DistIcon -Force
    Copy-Item -LiteralPath $BrandSource -Destination $DistAssetsRoot -Recurse -Force

    $shell = New-Object -ComObject WScript.Shell
    $desktopCandidates = @(
        $env:USERPROFILE,
        [Environment]::GetFolderPath("DesktopDirectory"),
        (Join-Path $env:USERPROFILE "Desktop"),
        (Join-Path $env:PUBLIC "Desktop")
    ) | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Select-Object -Unique
    $programs = [Environment]::GetFolderPath("Programs")
    $startDir = Join-Path $programs "NeuroCognica"
    New-Item -ItemType Directory -Force -Path $startDir | Out-Null
    $requiredShortcuts = @(
        (Join-Path $env:USERPROFILE "AURA.lnk"),
        (Join-Path ([Environment]::GetFolderPath("DesktopDirectory")) "AURA.lnk"),
        (Join-Path $startDir "AURA.lnk")
    ) | Select-Object -Unique

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
            $shortcut.Arguments = ""
            $shortcut.WorkingDirectory = $DistRoot
            $shortcut.IconLocation = "$DistIcon,0"
            $shortcut.Description = "AURA desktop launcher"
            $shortcut.Save()

            $saved = $shell.CreateShortcut($path)
            if ($saved.TargetPath -ne $DistExe) {
                throw "shortcut target stayed stale: $($saved.TargetPath)"
            }
            if (-not [string]::IsNullOrWhiteSpace($saved.Arguments)) {
                throw "shortcut arguments stayed stale: $($saved.Arguments)"
            }
            if ($saved.WorkingDirectory -ne $DistRoot) {
                throw "shortcut working directory stayed stale: $($saved.WorkingDirectory)"
            }
            if ($saved.IconLocation -ne "$DistIcon,0") {
                throw "shortcut icon stayed stale: $($saved.IconLocation)"
            }

            $createdShortcuts += $path
        }
        catch {
            Write-Warning "Could not save shortcut ${path}: $($_.Exception.Message)"
        }
    }

    foreach ($path in $requiredShortcuts) {
        if ($createdShortcuts -notcontains $path) {
            throw "required AURA shortcut was not created correctly: $path"
        }
    }

    Write-Host "AURA launcher installed:"
    Write-Host "  $DistExe"
    Write-Host "Icon:"
    Write-Host "  $DistIcon"
    Write-Host "Brand assets:"
    Write-Host "  $DistBrand"
    Write-Host "Shortcuts:"
    foreach ($path in $createdShortcuts) {
        Write-Host "  $path"
    }
}
finally {
    Pop-Location
}

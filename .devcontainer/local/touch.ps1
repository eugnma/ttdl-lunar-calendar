# $args: https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_automatic_variables#args
foreach ($path in $args) {
    if (Test-Path $path) {
        (Get-ChildItem $path).LastWriteTime = Get-Date
    } else {
        New-Item $path | Out-Null
    }
}

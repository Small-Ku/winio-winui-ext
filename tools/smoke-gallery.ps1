$ErrorActionPreference = 'Stop'

Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$gallery = Join-Path $repoRoot 'target\debug\winio-winui-ext-gallery.exe'
if (-not (Test-Path -LiteralPath $gallery -PathType Leaf)) {
    throw "Gallery executable not found: $gallery (run mise run build first)"
}

$process = $null
try {
    $process = Start-Process -FilePath $gallery -PassThru
    $deadline = [DateTime]::UtcNow.AddSeconds(30)
    $window = $null
    $button = $null
    $processCondition = [System.Windows.Automation.PropertyCondition]::new(
        [System.Windows.Automation.AutomationElement]::ProcessIdProperty,
        $process.Id)
    $buttonCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
            [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            'Native WinUI ToggleButton'))
    $initialTitle = 'Winio WinUI Extensions — state: indeterminate'

    while ([DateTime]::UtcNow -lt $deadline -and $null -eq $button) {
        if ($process.HasExited) {
            throw "Gallery exited before UI Automation found its button (exit code $($process.ExitCode))."
        }
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window) {
            $button = $window.FindFirst(
                [System.Windows.Automation.TreeScope]::Descendants,
                $buttonCondition)
        }
        if ($null -eq $button) { Start-Sleep -Milliseconds 150 }
    }
    if ($null -eq $button) { throw 'Timed out waiting for the native ToggleButton in the gallery.' }

    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    while ([DateTime]::UtcNow -lt $deadline -and $window.Current.Name -ne $initialTitle) {
        Start-Sleep -Milliseconds 100
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
    }
    if ($window.Current.Name -ne $initialTitle) {
        throw "Nullable checked-state round-trip failed; expected initial title '$initialTitle', got '$($window.Current.Name)'."
    }

    $toggle = $button.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $toggle.Toggle()

    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    $validTitles = @(
        'Winio WinUI Extensions — state: checked',
        'Winio WinUI Extensions — state: unchecked'
    )
    while ([DateTime]::UtcNow -lt $deadline) {
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window -and $validTitles -contains $window.Current.Name) {
            Write-Output "PASS: nullable state round-trip succeeded; UIA toggled '$($button.Current.Name)' and Winio observed '$($window.Current.Name)'."
            exit 0
        }
        if ($process.HasExited) {
            throw "Gallery exited before the Winio-visible event result (exit code $($process.ExitCode))."
        }
        Start-Sleep -Milliseconds 100
    }
    throw "Timed out waiting for a checked/unchecked Winio-visible title after toggling."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}
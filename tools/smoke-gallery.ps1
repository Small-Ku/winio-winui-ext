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
    $detailsItem = $null
    $infoBarMessage = $null
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
    $infoBarCondition = [System.Windows.Automation.PropertyCondition]::new(
        [System.Windows.Automation.AutomationElement]::NameProperty,
        'Supplemental InfoBar canary')
    $detailsCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
            [System.Windows.Automation.ControlType]::ListItem),
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::NameProperty,
            'Details'))
    $initialTitle = 'Winio WinUI Extensions — state: indeterminate'

    while ([DateTime]::UtcNow -lt $deadline -and ($null -eq $button -or $null -eq $infoBarMessage -or $null -eq $detailsItem)) {
        if ($process.HasExited) {
            throw "Gallery exited before UI Automation found its button (exit code $($process.ExitCode))."
        }
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window) {
            if ($null -eq $button) {
                $button = $window.FindFirst(
                    [System.Windows.Automation.TreeScope]::Descendants,
                    $buttonCondition)
            }
            if ($null -eq $infoBarMessage) {
                $infoBarMessage = $window.FindFirst(
                    [System.Windows.Automation.TreeScope]::Descendants,
                    $infoBarCondition)
            }
            if ($null -eq $detailsItem) {
                $detailsItem = $window.FindFirst(
                    [System.Windows.Automation.TreeScope]::Descendants,
                    $detailsCondition)
            }
        }
        if ($null -eq $button -or $null -eq $infoBarMessage -or $null -eq $detailsItem) { Start-Sleep -Milliseconds 150 }
    }
    if ($null -eq $button) { throw 'Timed out waiting for the native ToggleButton in the gallery.' }
    if ($null -eq $infoBarMessage) {
        $tree = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants,
            [System.Windows.Automation.Condition]::TrueCondition)
        $observed = @(
            for ($index = 0; $index -lt $tree.Count; $index++) {
                $element = $tree.Item($index)
                "Name='$($element.Current.Name)' ControlType='$($element.Current.ControlType.ProgrammaticName)' Class='$($element.Current.ClassName)'"
            }
        ) -join "`n"
        throw "Timed out waiting for the InfoBar message in the UIA tree. Observed descendants:`n$observed"
    }
    if ($null -eq $detailsItem) { throw 'Timed out waiting for the native NavigationView Details item in the UIA tree.' }

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
            if ($process.HasExited) {
                throw "Gallery exited after InfoBar UIA discovery (exit code $($process.ExitCode))."
            }
            break
        }
        if ($process.HasExited) {
            throw "Gallery exited before the Winio-visible event result (exit code $($process.ExitCode))."
        }
        Start-Sleep -Milliseconds 100
    }
    if ($validTitles -notcontains $window.Current.Name) {
        throw 'Timed out waiting for a checked/unchecked Winio-visible title after toggling.'
    }
    $toggleTitle = $window.Current.Name

    $selection = $detailsItem.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)
    $selection.Select()
    $navigationTitle = 'Winio WinUI Extensions — navigation: details'
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    while ([DateTime]::UtcNow -lt $deadline) {
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window -and $window.Current.Name -eq $navigationTitle) {
            if ($process.HasExited) {
                throw "Gallery exited after NavigationView selection (exit code $($process.ExitCode))."
            }
            Write-Output "PASS: UIA found '$($infoBarMessage.Current.Name)', toggled '$($button.Current.Name)' and observed '$toggleTitle', then selected '$($detailsItem.Current.Name)' and observed '$navigationTitle'."
            exit 0
        }
        if ($process.HasExited) {
            throw "Gallery exited before the Winio-visible NavigationView result (exit code $($process.ExitCode))."
        }
        Start-Sleep -Milliseconds 100
    }
    throw "Timed out waiting for NavigationView selection title '$navigationTitle'."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}

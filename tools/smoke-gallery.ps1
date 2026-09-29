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
    $numberBox = $null

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
    $numberCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::ControlTypeProperty,
            [System.Windows.Automation.ControlType]::Spinner),
        [System.Windows.Automation.PropertyCondition]::new(
            [System.Windows.Automation.AutomationElement]::ClassNameProperty,
            'Microsoft.UI.Xaml.Controls.NumberBox'))

    while ([DateTime]::UtcNow -lt $deadline -and
        ($null -eq $button -or $null -eq $infoBarMessage -or $null -eq $detailsItem -or $null -eq $numberBox)) {
        if ($process.HasExited) {
            throw "Gallery exited before UI Automation found all canaries (exit code $($process.ExitCode))."
        }
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window) {
            if ($null -eq $button) {
                $button = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $buttonCondition)
            }
            if ($null -eq $infoBarMessage) {
                $infoBarMessage = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $infoBarCondition)
            }
            if ($null -eq $detailsItem) {
                $detailsItem = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $detailsCondition)
            }
            if ($null -eq $numberBox) {
                $numberBox = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $numberCondition)
            }
        }
        if ($null -eq $button -or $null -eq $infoBarMessage -or $null -eq $detailsItem -or $null -eq $numberBox) {
            Start-Sleep -Milliseconds 150
        }
    }

    if ($null -eq $button) { throw 'Timed out waiting for the native ToggleButton in the gallery.' }
    if ($null -eq $detailsItem) { throw 'Timed out waiting for the native NavigationView Details item in the UIA tree.' }
    if ($null -eq $numberBox) { throw 'Timed out waiting for the native NumberBox Spinner in the UIA tree.' }
    if ($null -eq $infoBarMessage) {
        $tree = $window.FindAll(
            [System.Windows.Automation.TreeScope]::Descendants,
            [System.Windows.Automation.Condition]::TrueCondition)
        $observed = @(
            for ($index = 0; $index -lt $tree.Count; $index++) {
                $element = $tree.Item($index)
                "Name='$($element.Current.Name)' ControlType='$($element.Current.ControlType.ProgrammaticName)' Class='$($element.Current.ClassName)'"
            }
        ) -join "`n"
        throw "Timed out waiting for the InfoBar message in the UIA tree. Observed descendants:`n$observed"
    }

    $range = [System.Windows.Automation.RangeValuePattern]$numberBox.GetCurrentPattern(
        [System.Windows.Automation.RangeValuePattern]::Pattern)
    if ($range.Current.IsReadOnly) { throw 'NumberBox unexpectedly reports RangeValue as read-only.' }
    if ($range.Current.Value -ne 10 -or $range.Current.Minimum -ne 0 -or
        $range.Current.Maximum -ne 100 -or $range.Current.SmallChange -ne 1) {
        throw "NumberBox RangeValue metadata mismatch: value=$($range.Current.Value) min=$($range.Current.Minimum) max=$($range.Current.Maximum) small=$($range.Current.SmallChange)."
    }

    $initialTitle = 'Winio WinUI Extensions — number: 10'
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    while ([DateTime]::UtcNow -lt $deadline -and $window.Current.Name -ne $initialTitle) {
        Start-Sleep -Milliseconds 100
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
    }
    if ($window.Current.Name -ne $initialTitle) {
        throw "Initial NumberBox event path failed; expected '$initialTitle', got '$($window.Current.Name)'."
    }

    $toggle = $button.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $toggle.Toggle()
    $validToggleTitles = @(
        'Winio WinUI Extensions — state: checked',
        'Winio WinUI Extensions — state: unchecked'
    )
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window -and $validToggleTitles -contains $window.Current.Name) { break }
        if ($process.HasExited) { throw "Gallery exited before the ToggleButton result (exit code $($process.ExitCode))." }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($validToggleTitles -notcontains $window.Current.Name) {
        throw 'Timed out waiting for a checked/unchecked Winio-visible title after toggling.'
    }
    $toggleTitle = $window.Current.Name

    $selection = $detailsItem.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)
    $selection.Select()
    $navigationTitle = 'Winio WinUI Extensions — navigation: details'
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window -and $window.Current.Name -eq $navigationTitle) { break }
        if ($process.HasExited) { throw "Gallery exited before the NavigationView result (exit code $($process.ExitCode))." }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($window.Current.Name -ne $navigationTitle) {
        throw "Timed out waiting for NavigationView selection title '$navigationTitle'."
    }

    $range.SetValue(42.0)
    $numberTitle = 'Winio WinUI Extensions — number: 42'
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst(
            [System.Windows.Automation.TreeScope]::Children,
            $processCondition)
        if ($null -ne $window -and $window.Current.Name -eq $numberTitle) { break }
        if ($process.HasExited) { throw "Gallery exited before the NumberBox result (exit code $($process.ExitCode))." }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($window.Current.Name -ne $numberTitle) {
        throw "Timed out waiting for NumberBox value title '$numberTitle'."
    }

    Write-Output "PASS: UIA found '$($infoBarMessage.Current.Name)', verified NumberBox range metadata, toggled '$($button.Current.Name)' and observed '$toggleTitle', selected '$($detailsItem.Current.Name)' and observed '$navigationTitle', then set NumberBox to 42 and observed '$numberTitle'."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}
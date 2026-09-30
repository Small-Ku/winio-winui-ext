$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

$automation = [System.Windows.Automation.AutomationElement]
$root = $automation::RootElement
$descendants = [System.Windows.Automation.TreeScope]::Descendants
$children = [System.Windows.Automation.TreeScope]::Children

function Condition($Type, [string]$Name) {
    return [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, $Type),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, $Name))
}
function TypeCondition($Type) {
    return [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, $Type)
}
function Find-MainWindow($ProcessCondition) {
    $windows = $root.FindAll($children, $ProcessCondition)
    for ($i = 0; $i -lt $windows.Count; $i++) {
        $candidate = $windows.Item($i)
        if ($candidate.Current.Name -like 'Winio WinUI Extensions*') { return $candidate }
    }
    return $null
}
function Wait-MainTitle($Process, $ProcessCondition, [string]$Expected) {
    $deadline = [DateTime]::UtcNow.AddSeconds(12)
    do {
        if ($Process.HasExited) { throw "Gallery exited while waiting for '$Expected' (exit code $($Process.ExitCode))." }
        $window = Find-MainWindow $ProcessCondition
        if ($null -ne $window -and $window.Current.Name -eq $Expected) { return $window }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    $actual = if ($null -eq $window) { '<missing>' } else { $window.Current.Name }
    throw "Timed out waiting for '$Expected'; got '$actual'."
}
function Wait-MainWindow($Process, $ProcessCondition) {
    $deadline = [DateTime]::UtcNow.AddSeconds(12)
    do {
        if ($Process.HasExited) { throw "Gallery exited during UIA discovery (exit code $($Process.ExitCode))." }
        $window = Find-MainWindow $ProcessCondition
        if ($null -ne $window) { return $window }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw 'Timed out discovering the Gallery main window.'
}
function Wait-Element($Window, $Condition, [string]$Description) {
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $element = $Window.FindFirst($descendants, $Condition)
        if ($null -ne $element) { return $element }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out finding $Description."
}
function Select-NavigationSample($Window, [string]$Section, [string]$Sample) {
    $sectionElement = Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::ListItem) $Section) "NavigationView section '$Section'"
    $expand = [System.Windows.Automation.ExpandCollapsePattern]$sectionElement.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($expand.Current.ExpandCollapseState -eq [System.Windows.Automation.ExpandCollapseState]::Collapsed) { $expand.Expand() }
    $item = Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::ListItem) $Sample) "NavigationView sample '$Sample'"
    ([System.Windows.Automation.SelectionItemPattern]$item.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$gallery = Join-Path $repoRoot 'target\debug\winio-winui-ext-gallery.exe'
if (-not (Test-Path -LiteralPath $gallery -PathType Leaf)) { throw "Gallery executable not found: $gallery" }
$process = $null
try {
    $process = Start-Process -FilePath $gallery -PassThru
    $pc = [System.Windows.Automation.PropertyCondition]::new($automation::ProcessIdProperty, $process.Id)
    $window = Wait-MainWindow $process $pc
    $titleBar = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::TitleBar),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.TitleBar'))) 'integrated native TitleBar'
    $captionClose = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'Close'))) 'system caption Close button'
    $section = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::ListItem) 'Basic input') 'NavigationView section hierarchy'
    $search = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'AutoSuggestBox')) 'catalog AutoSuggestBox'

    $titleRect = $titleBar.Current.BoundingRectangle
    $searchRect = $search.Current.BoundingRectangle
    if ($searchRect.Top -lt ($titleRect.Top - 4) -or $searchRect.Bottom -gt ($titleRect.Bottom + 4)) {
        throw "AutoSuggestBox escaped the integrated TitleBar band: title=$titleRect search=$searchRect."
    }
    $legacyPaneToggle = $window.FindFirst($descendants, [System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'TogglePaneButton'))
    if ($null -ne $legacyPaneToggle -and -not $legacyPaneToggle.Current.IsOffscreen) {
        throw 'NavigationView exposed its own pane toggle instead of delegating shell chrome to TitleBar.'
    }
    $titlePaneToggle = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'PART_PaneToggleButton')) 'TitleBar pane toggle'
    $pane = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'PaneRoot')) 'NavigationView pane root'
    $expandedPaneWidth = $pane.Current.BoundingRectangle.Width
    $homeTexts = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Home'))
    $expandedPageHome = $null
    for ($i = 0; $i -lt $homeTexts.Count; $i++) {
        $candidate = $homeTexts.Item($i)
        if (-not $candidate.Current.IsOffscreen -and ($null -eq $expandedPageHome -or $candidate.Current.BoundingRectangle.Left -gt $expandedPageHome.Current.BoundingRectangle.Left)) {
            $expandedPageHome = $candidate
        }
    }
    if ($null -eq $expandedPageHome) { throw 'Expanded Gallery page header Home was not found.' }
    $expandedPageHomeLeft = $expandedPageHome.Current.BoundingRectangle.Left
    ([System.Windows.Automation.InvokePattern]$titlePaneToggle.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        Start-Sleep -Milliseconds 50
        $pane = $window.FindFirst($descendants, [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot'))
        $compactPaneWidth = if ($null -eq $pane) { 0 } else { $pane.Current.BoundingRectangle.Width }
        if ($compactPaneWidth -lt ($expandedPaneWidth * 0.5)) { break }
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($compactPaneWidth -ge ($expandedPaneWidth * 0.5)) { throw "TitleBar pane toggle did not compact NavigationView: expanded=$expandedPaneWidth compact=$compactPaneWidth." }
    $homeTexts = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Home'))
    $compactPageHome = $null
    for ($i = 0; $i -lt $homeTexts.Count; $i++) {
        $candidate = $homeTexts.Item($i)
        if (-not $candidate.Current.IsOffscreen -and ($null -eq $compactPageHome -or $candidate.Current.BoundingRectangle.Left -gt $compactPageHome.Current.BoundingRectangle.Left)) {
            $compactPageHome = $candidate
        }
    }
    if ($null -eq $compactPageHome) { throw 'Compact Gallery page header Home was not found.' }
    $compactPageHomeLeft = $compactPageHome.Current.BoundingRectangle.Left
    if ($compactPageHomeLeft -ge ($expandedPageHomeLeft - 100)) {
        throw "Gallery content did not reclaim compact-pane width: expandedHomeX=$expandedPageHomeLeft compactHomeX=$compactPageHomeLeft."
    }
    ([System.Windows.Automation.InvokePattern]$titlePaneToggle.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        Start-Sleep -Milliseconds 50
        $pane = $window.FindFirst($descendants, [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot'))
        $reopenedPaneWidth = if ($null -eq $pane) { 0 } else { $pane.Current.BoundingRectangle.Width }
        if ($reopenedPaneWidth -ge ($expandedPaneWidth * 0.9)) { break }
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($reopenedPaneWidth -lt ($expandedPaneWidth * 0.9)) { throw "TitleBar pane toggle did not reopen NavigationView: expanded=$expandedPaneWidth reopened=$reopenedPaneWidth." }
    $homeTexts = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Home'))
    $reopenedPageHome = $null
    for ($i = 0; $i -lt $homeTexts.Count; $i++) {
        $candidate = $homeTexts.Item($i)
        if (-not $candidate.Current.IsOffscreen -and ($null -eq $reopenedPageHome -or $candidate.Current.BoundingRectangle.Left -gt $reopenedPageHome.Current.BoundingRectangle.Left)) {
            $reopenedPageHome = $candidate
        }
    }
    if ($null -eq $reopenedPageHome) { throw 'Reopened Gallery page header Home was not found.' }
    if ([Math]::Abs($reopenedPageHome.Current.BoundingRectangle.Left - $expandedPageHomeLeft) -gt 12) {
        throw "Gallery content did not restore after pane reopen: expandedHomeX=$expandedPageHomeLeft reopenedHomeX=$($reopenedPageHome.Current.BoundingRectangle.Left)."
    }

    Select-NavigationSample $window 'Basic input' 'ToggleButton'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ToggleButton'
    $toggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Native WinUI ToggleButton') 'routed ToggleButton'
    ([System.Windows.Automation.TogglePattern]$toggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $window = Find-MainWindow $pc
        if ($null -ne $window -and $window.Current.Name -match '^Winio WinUI Extensions — sample toggle-button: (checked|unchecked|indeterminate)$') { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $window -or $window.Current.Name -notmatch '^Winio WinUI Extensions — sample toggle-button: (checked|unchecked|indeterminate)$') { throw 'ToggleButton sample output title was not observed.' }
    $toggleOutput = $window.Current.Name

    $search = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'AutoSuggestBox')) 'AutoSuggestBox after sample route'
    $edit = Wait-Element $search (TypeCondition ([System.Windows.Automation.ControlType]::Edit)) 'AutoSuggestBox edit'
    ([System.Windows.Automation.ValuePattern]$edit.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)).SetValue('NumberBox')
    ([System.Windows.Automation.InvokePattern]$search.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — NumberBox'
    $staleToggle = $window.FindFirst($descendants, (Condition ([System.Windows.Automation.ControlType]::Button) 'Native WinUI ToggleButton'))
    if ($null -ne $staleToggle) { throw 'ToggleButton remained in the main UIA subtree after replacing it with NumberBox.' }
    $number = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Spinner),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.NumberBox'))) 'routed NumberBox'
    $range = [System.Windows.Automation.RangeValuePattern]$number.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
    if ($range.Current.IsReadOnly -or $range.Current.Value -ne 10 -or $range.Current.Minimum -ne 0 -or $range.Current.Maximum -ne 100 -or $range.Current.SmallChange -ne 1) {
        throw "NumberBox range mismatch: value=$($range.Current.Value) min=$($range.Current.Minimum) max=$($range.Current.Maximum) small=$($range.Current.SmallChange)."
    }
    $range.SetValue(42)
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample number-box: 42'

    Select-NavigationSample $window 'Status and info' 'InfoBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — InfoBar'
    $info = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Text) 'Supplemental InfoBar canary') 'InfoBar canary message'

    Select-NavigationSample $window 'Collections' 'TreeView'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — TreeView'
    $treeItemCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::TreeItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.TreeViewItem'))
    $treeRoot = Wait-Element $window $treeItemCondition 'TreeView root item'
    ([System.Windows.Automation.ExpandCollapsePattern]$treeRoot.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)).Expand()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — tree expanded: root'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $treeItems = $window.FindAll($descendants, $treeItemCondition)
        if ($treeItems.Count -ge 2) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($treeItems.Count -lt 2) { throw "Expected root and leaf TreeView peers; found $($treeItems.Count)." }
    ([System.Windows.Automation.InvokePattern]$treeItems.Item(1).GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — tree invoked: leaf'

    Select-NavigationSample $window 'Dialogs and flyouts' 'TeachingTip'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — TeachingTip'
    $target = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Open TeachingTip') 'TeachingTip target'
    ([System.Windows.Automation.TogglePattern]$target.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $tipPopupCondition = [System.Windows.Automation.AndCondition]::new($pc,[System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Window),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Native TeachingTip canary')))
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $tipPopup = $root.FindFirst($descendants, $tipPopupCondition)
        if ($null -ne $tipPopup) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $tipPopup) { throw 'TeachingTip native popup was not found.' }
    $tipCloseCondition = [System.Windows.Automation.AndCondition]::new($pc,[System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'CloseButton')))
    $tipClose = $root.FindFirst($descendants, $tipCloseCondition)
    if ($null -eq $tipClose) { throw 'TeachingTip native CloseButton was not found.' }
    # Opening the tip is an inner-child message of a boxed routed sample. Give the routed TeachingTip a bounded settle window to finish its native opening transition and re-arm the nested event listener before closing it.
    Start-Sleep -Milliseconds 750
    ([System.Windows.Automation.InvokePattern]$tipClose.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — teaching tip closed CloseButton'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $tipAfter = $root.FindFirst($descendants, $tipPopupCondition)
        if ($null -eq $tipAfter) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -ne $tipAfter) { throw 'TeachingTip popup remained after CloseButton invoke.' }

    Write-Output "PASS: integrated TitleBar search/pane chrome and caption Close; catalog NavigationView hierarchy; NavigationView ToggleButton route/output '$toggleOutput'; AutoSuggestBox ValuePattern + InvokePattern NumberBox route; old ToggleButton UIA peer removed; NumberBox RangeValue 10→42; InfoBar canary visible; TreeView root expansion/leaf invoke outputs; TeachingTip target, native popup CloseButton, popup removal, and close-reason output."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}

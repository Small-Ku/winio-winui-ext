$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
Add-Type -TypeDefinition @'
using System;
using System.Runtime.InteropServices;
public static class GalleryWindow {
    [DllImport("user32.dll", SetLastError = true)]
    public static extern bool SetWindowPos(IntPtr hwnd, IntPtr insertAfter, int x, int y, int width, int height, uint flags);
}
'@

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
function Wait-Text($Window, [string]$Name) {
    return Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::Text) $Name) "visible text '$Name'"
}
function Wait-ShellTitle($Window, [string]$Name) {
    $pane = Wait-Element $Window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot')) 'NavigationView pane boundary'
    $texts = $Window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) $Name))
    $chosen = $null
    for ($i = 0; $i -lt $texts.Count; $i++) {
        $candidate = $texts.Item($i)
        if (-not $candidate.Current.IsOffscreen -and $candidate.Current.BoundingRectangle.Left -ge ($pane.Current.BoundingRectangle.Right - 4) -and ($null -eq $chosen -or $candidate.Current.BoundingRectangle.Top -lt $chosen.Current.BoundingRectangle.Top)) {
            $chosen = $candidate
        }
    }
    if ($null -eq $chosen) { throw "Could not find fixed shell title '$Name'." }
    return $chosen
}
function Wait-ScrollViewer($Window) {
    $condition = [System.Windows.Automation.AndCondition]::new(
        (TypeCondition ([System.Windows.Automation.ControlType]::Pane)),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'ScrollViewer'))
    $viewers = $Window.FindAll($descendants, $condition)
    $chosen = $null
    for ($i = 0; $i -lt $viewers.Count; $i++) {
        $candidate = $viewers.Item($i)
        if (-not $candidate.Current.IsOffscreen -and ($null -eq $chosen -or $candidate.Current.BoundingRectangle.Top -gt $chosen.Current.BoundingRectangle.Top)) {
            $chosen = $candidate
        }
    }
    if ($null -eq $chosen) { throw 'Timed out finding routed sample ScrollViewer.' }
    return $chosen
}
function Find-SourceMarker($Window, [string]$Marker) {
    $texts = $Window.FindAll($descendants, (TypeCondition ([System.Windows.Automation.ControlType]::Text)))
    for ($i = 0; $i -lt $texts.Count; $i++) {
        $element = $texts.Item($i)
        if ($element.Current.Name.Contains($Marker)) { return $element }
        try {
            $textPattern = [System.Windows.Automation.TextPattern]$element.GetCurrentPattern([System.Windows.Automation.TextPattern]::Pattern)
            if ($textPattern.DocumentRange.GetText(20000).Contains($Marker)) { return $element }
        } catch { }
    }
    return $null
}
function Select-NavigationSample($Window, [string]$Section, [string]$Sample) {
    $sectionElement = Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::ListItem) $Section) "NavigationView section '$Section'"
    $expand = [System.Windows.Automation.ExpandCollapsePattern]$sectionElement.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($expand.Current.ExpandCollapseState -eq [System.Windows.Automation.ExpandCollapseState]::Collapsed) { $expand.Expand() }
    $item = Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::ListItem) $Sample) "NavigationView sample '$Sample'"
    ([System.Windows.Automation.SelectionItemPattern]$item.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$targetDirectory = if (Test-Path Env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { Join-Path $repoRoot 'target' }
$gallery = Join-Path $targetDirectory 'debug\winio-winui-ext-gallery.exe'
if (-not (Test-Path -LiteralPath $gallery -PathType Leaf)) { throw "Gallery executable not found: $gallery" }
$process = $null
try {
    $process = Start-Process -FilePath $gallery -PassThru
    $pc = [System.Windows.Automation.PropertyCondition]::new($automation::ProcessIdProperty, $process.Id)
    $window = Wait-MainWindow $process $pc
    $fixedHomeTitle = Wait-ShellTitle $window 'Home'
    $homeScroll = Wait-ScrollViewer $window
    if ($homeScroll.Current.BoundingRectangle.Top -le $fixedHomeTitle.Current.BoundingRectangle.Bottom) {
        throw 'The Home sample viewport overlaps the fixed shell page title.'
    }
    $homeTitleRect = $fixedHomeTitle.Current.BoundingRectangle
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
    $toggleOutput = Wait-Text $window 'indeterminate'
    $sourceToggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Source code') 'collapsed Source code toggle'
    $sourcePattern = [System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    if ($sourcePattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'Source code did not start collapsed.' }
    $threeStateOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state: On') 'live ToggleButton option'
    ([System.Windows.Automation.TogglePattern]$toggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $window = Find-MainWindow $pc
        if ($null -ne $window -and $window.Current.Name -match '^Winio WinUI Extensions — sample toggle-button: (checked|unchecked|indeterminate)$') { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $window -or $window.Current.Name -notmatch '^Winio WinUI Extensions — sample toggle-button: (checked|unchecked|indeterminate)$') { throw 'ToggleButton sample output title was not observed.' }
    $toggleState = ($window.Current.Name -split ': ')[1]
    $toggleOutput = Wait-Text $window $toggleState
    if ($toggleOutput.Current.Name -ne $toggleState) { throw 'ToggleButton card output did not follow the sample state event.' }
    ([System.Windows.Automation.TogglePattern]$threeStateOption.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $threeStateOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state: Off') 'disabled three-state option'
    ([System.Windows.Automation.TogglePattern]$toggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample toggle-button: checked'
    $toggleOutput = Wait-Text $window 'checked'

    $sourcePattern.Toggle()
    $sourceToggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Source code') 'expanded Source code toggle'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $sourceMarker = Find-SourceMarker $window 'skip_initial_event'
        if ($null -ne $sourceMarker) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $sourceMarker) { throw 'Expanded Rust source did not expose its skip_initial_event marker.' }
    $sampleScroll = Wait-ScrollViewer $window
    $scrollPattern = [System.Windows.Automation.ScrollPattern]$sampleScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
    if ($scrollPattern.Current.HorizontallyScrollable) { throw 'The routed sample ScrollViewer enabled horizontal scrolling.' }
    if ($scrollPattern.Current.VerticalScrollPercent -lt 0 -or $scrollPattern.Current.VerticalViewSize -ge 100) {
        throw "Expanded source did not increase the vertical ScrollViewer extent: percent=$($scrollPattern.Current.VerticalScrollPercent), view=$($scrollPattern.Current.VerticalViewSize)."
    }
    $fixedTitleBeforeScroll = (Wait-ShellTitle $window 'ToggleButton').Current.BoundingRectangle
    $paneBeforeScroll = (Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot')) 'fixed navigation pane').Current.BoundingRectangle
    $scrollPercentBefore = $scrollPattern.Current.VerticalScrollPercent
    try {
        $scrollPattern.SetScrollPercent(-1, 50)
    } catch {
        throw "Expanded source exposed an unusable scroll range (verticalScrollable=$($scrollPattern.Current.VerticallyScrollable), percent=$($scrollPattern.Current.VerticalScrollPercent), view=$($scrollPattern.Current.VerticalViewSize), rect=$($sampleScroll.Current.BoundingRectangle)): $($_.Exception.Message)"
    }
    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        if ($scrollPattern.Current.VerticalScrollPercent -ge 45 -and $scrollPattern.Current.VerticalScrollPercent -le 55) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ([Math]::Abs($scrollPattern.Current.VerticalScrollPercent - $scrollPercentBefore) -lt 10) {
        throw "The routed ScrollViewer did not move to the requested position: before=$scrollPercentBefore after=$($scrollPattern.Current.VerticalScrollPercent)."
    }
    Start-Sleep -Milliseconds 250
    $fixedTitleAfterScroll = (Wait-ShellTitle $window 'ToggleButton').Current.BoundingRectangle
    $paneAfterScroll = (Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot')) 'fixed navigation pane after scrolling').Current.BoundingRectangle
    if ([Math]::Abs($fixedTitleAfterScroll.Top - $fixedTitleBeforeScroll.Top) -gt 2 -or [Math]::Abs($paneAfterScroll.Top - $paneBeforeScroll.Top) -gt 2) {
        throw 'Scrolling source moved the fixed page title or navigation pane.'
    }
    $scrollPattern.SetScrollPercent(-1, 0)
    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        if ($scrollPattern.Current.VerticalScrollPercent -le 2) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($scrollPattern.Current.VerticalScrollPercent -gt 2) { throw "The routed page did not return to its start: $($scrollPattern.Current.VerticalScrollPercent)." }
    $sourceToggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Source code') 'Source code toggle after scrolling'
    ([System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    if (([System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) {
        throw 'Source code did not collapse after the scroll check.'
    }

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
    $numberOutput = Wait-Text $window '42'
    $stepOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Step by 1') 'live NumberBox step option'
    ([System.Windows.Automation.TogglePattern]$stepOption.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $stepOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Step by 10') 'NumberBox step 10 option'
    if ($range.Current.SmallChange -ne 10) { throw "NumberBox step option did not change SmallChange: $($range.Current.SmallChange)." }
    ([System.Windows.Automation.TogglePattern]$stepOption.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $stepOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Step by 1') 'restored NumberBox step 1 option'
    if ($range.Current.SmallChange -ne 1) { throw "NumberBox step option did not restore SmallChange: $($range.Current.SmallChange)." }

    # Compact the pane, then narrow the window enough to exercise the stacked card layout.
    $titlePaneToggle = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PART_PaneToggleButton')) 'TitleBar pane toggle for narrow viewport'
    ([System.Windows.Automation.InvokePattern]$titlePaneToggle.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    Start-Sleep -Milliseconds 350
    $hwnd = [IntPtr]$window.Current.NativeWindowHandle
    if (-not [GalleryWindow]::SetWindowPos($hwnd, [IntPtr]::Zero, 0, 0, 1080, 900, 0x0006)) {
        throw "Could not resize Gallery window for narrow layout (Win32 $([Runtime.InteropServices.Marshal]::GetLastWin32Error()))."
    }
    Start-Sleep -Milliseconds 500
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample number-box: 42'
    $narrowScroll = Wait-ScrollViewer $window
    $narrowRange = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Spinner),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.NumberBox'))) 'narrow NumberBox'
    $narrowOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Step by 1') 'narrow NumberBox option'
    $narrowControlRect = $narrowRange.Current.BoundingRectangle
    $narrowOptionRect = $narrowOption.Current.BoundingRectangle
    $narrowViewportRect = $narrowScroll.Current.BoundingRectangle
    if ($narrowControlRect.Right -gt ($narrowViewportRect.Right + 4) -or $narrowControlRect.Left -lt ($narrowViewportRect.Left - 4) -or $narrowOptionRect.Top -lt $narrowControlRect.Bottom) {
        throw "Narrow card geometry overlaps or clips the NumberBox/options regions: viewport=$narrowViewportRect number=$narrowControlRect option=$narrowOptionRect."
    }

    Select-NavigationSample $window 'Status and info' 'InfoBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — InfoBar'
    $info = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Text) 'Supplemental InfoBar canary') 'InfoBar canary message'
    $infoOutput = Wait-Text $window 'Open · informational status'

    Select-NavigationSample $window 'Collections' 'TreeView'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — TreeView'
    $treeItemCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::TreeItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.TreeViewItem'))
    $treeRoot = Wait-Element $window $treeItemCondition 'TreeView root item'
    if ($treeRoot.Current.Name -ne 'Root node') { throw "TreeView root text was not preserved: '$($treeRoot.Current.Name)'." }
    ([System.Windows.Automation.ExpandCollapsePattern]$treeRoot.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)).Expand()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — tree expanded: root'
    $treeOutput = Wait-Text $window 'expanded: root'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $treeItems = $window.FindAll($descendants, $treeItemCondition)
        if ($treeItems.Count -ge 2) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($treeItems.Count -lt 2) { throw "Expected root and leaf TreeView peers; found $($treeItems.Count)." }
    if ($treeItems.Item(1).Current.Name -ne 'Leaf node') { throw "TreeView leaf text was not preserved: '$($treeItems.Item(1).Current.Name)'." }
    $treeScroll = Wait-ScrollViewer $window
    $treeViewportRect = $treeScroll.Current.BoundingRectangle
    if ($treeItems.Item(0).Current.BoundingRectangle.Right -gt ($treeViewportRect.Right + 4) -or $treeItems.Item(1).Current.BoundingRectangle.Right -gt ($treeViewportRect.Right + 4)) {
        throw 'TreeView node text exceeded the routed viewport at the normal window size.'
    }
    ([System.Windows.Automation.InvokePattern]$treeItems.Item(1).GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — tree invoked: leaf'
    $treeOutput = Wait-Text $window 'invoked: leaf'

    Select-NavigationSample $window 'Dialogs and flyouts' 'TeachingTip'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — TeachingTip'
    $target = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Open TeachingTip') 'TeachingTip target'
    ([System.Windows.Automation.TogglePattern]$target.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — teaching tip opened'
    $tipOutput = Wait-Text $window 'opened'
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
    $tipOutput = Wait-Text $window 'closed:CloseButton'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $tipAfter = $root.FindFirst($descendants, $tipPopupCondition)
        if ($null -eq $tipAfter) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -ne $tipAfter) { throw 'TeachingTip popup remained after CloseButton invoke.' }

    Write-Output "PASS: fixed shell and ScrollViewer viewport; integrated TitleBar/search/pane and caption Close; catalog navigation; live ToggleButton output/options; collapsed, expanded, scrollable, and recollapsed Rust source; AutoSuggestBox route and stale-peer removal; NumberBox RangeValue 10→42/output and live SmallChange option; narrow stacked geometry; InfoBar; TreeView; native TeachingTip popup lifecycle."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}

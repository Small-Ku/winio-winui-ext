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
    try {
        $windows = $root.FindAll($children, $ProcessCondition)
    } catch {
        # UI Automation can transiently time out while WinUI rebuilds its peer tree.
        return $null
    }
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
function Get-PageScrollViewers($Window) {
    $condition = [System.Windows.Automation.AndCondition]::new(
        (TypeCondition ([System.Windows.Automation.ControlType]::Pane)),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'ScrollViewer'))
    $all = $Window.FindAll($descendants, $condition)
    $pane = $Window.FindFirst($descendants, [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'PaneRoot'))
    $pageLeft = if ($null -eq $pane) { 0 } else { $pane.Current.BoundingRectangle.Right - 4 }
    $candidates = [System.Collections.Generic.List[System.Windows.Automation.AutomationElement]]::new()
    $top = [double]::PositiveInfinity
    for ($i = 0; $i -lt $all.Count; $i++) {
        $candidate = $all.Item($i)
        $rect = $candidate.Current.BoundingRectangle
        if (-not $candidate.Current.IsOffscreen -and $rect.Left -ge $pageLeft -and $rect.Top -gt 100 -and $rect.Height -gt 150) {
            $candidates.Add($candidate)
            $top = [Math]::Min($top, $rect.Top)
        }
    }
    if ($candidates.Count -eq 0) { return @() }
    $matches = [System.Collections.Generic.List[System.Windows.Automation.AutomationElement]]::new()
    foreach ($candidate in $candidates) {
        if ([Math]::Abs($candidate.Current.BoundingRectangle.Top - $top) -le 4) { $matches.Add($candidate) }
    }
    return ,$matches.ToArray()
}
function Wait-ScrollViewer($Window) {
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $viewers = Get-PageScrollViewers $Window
        if ($viewers.Count -gt 0) { return $viewers[0] }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw 'Timed out finding routed page ScrollViewer.'
}
function Wait-CardSourceToggle($Window, [string]$CardName) {
    return Wait-Element $Window (Condition ([System.Windows.Automation.ControlType]::Button) "Source code — $CardName") "Source code toggle for $CardName"
}
function Wait-SourceViewer($Window, [string]$CardName) {
    $condition = Condition ([System.Windows.Automation.ControlType]::Edit) "Source code viewer — $CardName"
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $viewer = $Window.FindFirst($descendants, $condition)
        if ($null -ne $viewer) { return $viewer }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "Timed out finding source viewer for $CardName."
}
function Wait-SourceViewerVisible($Window, [string]$CardName) {
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $viewer = Wait-SourceViewer $Window $CardName
        if (-not $viewer.Current.IsOffscreen -and $viewer.Current.BoundingRectangle.Height -gt 1) { return $viewer }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    throw "Source viewer for $CardName did not enter the visible viewport."
}
function Get-ScrollContentTop($Element, $ScrollViewer) {
    $scroll = [System.Windows.Automation.ScrollPattern]$ScrollViewer.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
    $viewport = $ScrollViewer.Current.BoundingRectangle
    $viewSize = $scroll.Current.VerticalViewSize
    if ($viewSize -le 0) { throw 'ScrollViewer exposed an invalid vertical view size.' }
    $extent = $viewport.Height * 100 / $viewSize
    $percent = [Math]::Max(0, $scroll.Current.VerticalScrollPercent)
    $offset = $percent / 100 * [Math]::Max(0, $extent - $viewport.Height)
    return $Element.Current.BoundingRectangle.Top - $viewport.Top + $offset
}
function Assert-SourceViewerContract($SourceViewer, [string]$Marker) {
    $rect = $SourceViewer.Current.BoundingRectangle
    if ($rect.Height -lt 250 -or $rect.Height -gt 500) {
        throw "Visible source viewer is outside the bounded code-area height range: $rect."
    }
    $value = [System.Windows.Automation.ValuePattern]$SourceViewer.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    if (-not $value.Current.IsReadOnly) { throw 'Source viewer is editable instead of read-only.' }
    $setSucceeded = $false
    try {
        $value.SetValue('should-not-be-editable')
        $setSucceeded = $true
    } catch { }
    if ($setSucceeded) { throw 'Read-only source viewer accepted a UIA SetValue mutation.' }

    $text = [System.Windows.Automation.TextPattern]$SourceViewer.GetCurrentPattern([System.Windows.Automation.TextPattern]::Pattern)
    $documentText = $text.DocumentRange.GetText(-1)
    if (-not $documentText.Contains($Marker)) { throw "Source viewer text does not contain marker '$Marker'." }
    $selectionRange = $text.DocumentRange.Clone()
    $selectionRange.MoveEndpointByRange(
        [System.Windows.Automation.Text.TextPatternRangeEndpoint]::End,
        $selectionRange,
        [System.Windows.Automation.Text.TextPatternRangeEndpoint]::Start)
    [void]$selectionRange.MoveEndpointByUnit(
        [System.Windows.Automation.Text.TextPatternRangeEndpoint]::End,
        [System.Windows.Automation.Text.TextUnit]::Character,
        12)
    $selectionRange.Select()
    $selection = $text.GetSelection()
    if ($selection.Length -lt 1 -or [string]::IsNullOrEmpty($selection[0].GetText(-1))) {
        throw 'Source viewer text could not be selected.'
    }


}

function Select-NavigationSample($Window, [string]$Section, [string]$Sample) {
    $sectionCondition = Condition ([System.Windows.Automation.ControlType]::ListItem) $Section
    $sectionElement = Wait-Element $Window $sectionCondition "NavigationView section '$Section'"

    # In compact/closed NavigationView mode only the section glyph is realized.
    # Open the pane before asking UIA for one of that section's child items.
    if ($sectionElement.Current.BoundingRectangle.Width -lt 120) {
        $paneToggle = Wait-Element $Window ([System.Windows.Automation.PropertyCondition]::new(
            $automation::AutomationIdProperty, 'PART_PaneToggleButton')) 'TitleBar pane toggle for navigation'
        ([System.Windows.Automation.InvokePattern]$paneToggle.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
        Start-Sleep -Milliseconds 250
        $sectionElement = Wait-Element $Window $sectionCondition "expanded NavigationView section '$Section'"
    }

    try {
        ([System.Windows.Automation.ScrollItemPattern]$sectionElement.GetCurrentPattern([System.Windows.Automation.ScrollItemPattern]::Pattern)).ScrollIntoView()
    } catch {}

    $expand = [System.Windows.Automation.ExpandCollapsePattern]$sectionElement.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($expand.Current.ExpandCollapseState -eq [System.Windows.Automation.ExpandCollapseState]::Collapsed) {
        $expand.Expand()
        Start-Sleep -Milliseconds 120
    }

    $condition = Condition ([System.Windows.Automation.ControlType]::ListItem) $Sample
    $item = $Window.FindFirst($descendants, $condition)
    if ($null -eq $item) {
        # Re-anchor and reopen the group once for virtualized peers near the
        # bottom of a long navigation pane.
        $expand.Collapse()
        Start-Sleep -Milliseconds 50
        try {
            ([System.Windows.Automation.ScrollItemPattern]$sectionElement.GetCurrentPattern([System.Windows.Automation.ScrollItemPattern]::Pattern)).ScrollIntoView()
        } catch {}
        $expand.Expand()
        Start-Sleep -Milliseconds 180
        $item = Wait-Element $Window $condition "NavigationView sample '$Sample'"
    }
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

    $hwnd = [IntPtr]$window.Current.NativeWindowHandle
    if (-not [GalleryWindow]::SetWindowPos($hwnd, [IntPtr]::Zero, 0, 0, 1280, 760, 0x0006)) { throw "Could not resize Gallery window for multi-card layout evidence (Win32 $([Runtime.InteropServices.Marshal]::GetLastWin32Error()))." }
    Start-Sleep -Milliseconds 400
    $window = Wait-MainWindow $process $pc
    Select-NavigationSample $window 'Basic input' 'ToggleButton'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ToggleButton'
    $scrollViewers = Get-PageScrollViewers $window
    if ($scrollViewers.Count -ne 1) { throw "ToggleButton route must expose exactly one page ScrollViewer; found $($scrollViewers.Count)." }
    $toggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state sample') 'three-state ToggleButton example'
    $basicToggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample') 'basic ToggleButton example'
    if ($basicToggle.Current.BoundingRectangle.Top -le $toggle.Current.BoundingRectangle.Bottom) { throw 'ToggleButton example cards overlap instead of stacking vertically.' }
    $toggleOutput = Wait-Text $window 'indeterminate'
    $basicOutput = Wait-Text $window 'unchecked'
    $sourceToggle = Wait-CardSourceToggle $window 'ToggleButton — three-state'
    $basicSourceToggle = Wait-CardSourceToggle $window 'ToggleButton — basic'
    if ($sourceToggle.Current.Name -eq $basicSourceToggle.Current.Name) { throw 'ToggleButton cards do not have distinct accessible source toggle names.' }
    $toggleButtons = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state sample'))
    $basicButtons = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample'))
    if ($toggleButtons.Count -ne 1 -or $basicButtons.Count -ne 1) { throw "Expected exactly two distinct example controls; three-state=$($toggleButtons.Count), basic=$($basicButtons.Count)." }
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

    ([System.Windows.Automation.TogglePattern]$basicToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample toggle-button: basic checked'
    $basicOutput = Wait-Text $window 'checked'
    if ($basicOutput.Current.Name -ne 'checked') { throw 'Basic ToggleButton output did not follow its independent state.' }
    ([System.Windows.Automation.TogglePattern]$basicToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample toggle-button: basic unchecked'
    $basicOutput = Wait-Text $window 'unchecked'

    $collapsedScroll = Wait-ScrollViewer $window
    $collapsedScrollPattern = [System.Windows.Automation.ScrollPattern]$collapsedScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
    if ($collapsedScrollPattern.Current.VerticallyScrollable) { $collapsedScrollPattern.SetScrollPercent(-1, 100) }
    $basicBeforeElement = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample') 'basic ToggleButton before Card A source expansion'
    $basicContentTopBefore = Get-ScrollContentTop $basicBeforeElement $collapsedScroll
    if ($collapsedScrollPattern.Current.VerticallyScrollable) { $collapsedScrollPattern.SetScrollPercent(-1, 0) }
    $sourcePattern.Toggle()
    $sourceToggle = Wait-CardSourceToggle $window 'ToggleButton — three-state'
    $sourceViewer = Wait-SourceViewer $window 'ToggleButton — three-state'
    $sourceValue = [System.Windows.Automation.ValuePattern]$sourceViewer.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    if (-not $sourceValue.Current.IsReadOnly) { throw 'Expanded source viewer is editable instead of read-only.' }
    $sampleScroll = Wait-ScrollViewer $window
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'Source expansion created a second page-level ScrollViewer.' }
    $scrollPattern = [System.Windows.Automation.ScrollPattern]$sampleScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
    if ($scrollPattern.Current.VerticallyScrollable) { $scrollPattern.SetScrollPercent(-1, 45) }
    $sourceViewer = Wait-SourceViewerVisible $window 'ToggleButton — three-state'
    Assert-SourceViewerContract $sourceViewer 'skip_initial_event'
    if ($scrollPattern.Current.VerticallyScrollable) { $scrollPattern.SetScrollPercent(-1, 0) }
    $basicHeading = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample') 'basic ToggleButton after Card A source expansion'
    $scrollPattern = [System.Windows.Automation.ScrollPattern]$sampleScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
    if ($scrollPattern.Current.HorizontallyScrollable) { throw 'The routed sample ScrollViewer enabled horizontal scrolling.' }
    if ($scrollPattern.Current.VerticalScrollPercent -lt 0 -or $scrollPattern.Current.VerticalViewSize -ge 100) {
        throw "Expanded source did not increase the vertical ScrollViewer extent: percent=$($scrollPattern.Current.VerticalScrollPercent), view=$($scrollPattern.Current.VerticalViewSize)."
    }
    $scrollPattern.SetScrollPercent(-1, 100)
    $basicContentTopAfter = Get-ScrollContentTop $basicHeading $sampleScroll
    if ($basicContentTopAfter -le $basicContentTopBefore) { throw "Expanding Card A source did not push Card B downward: contentY before=$basicContentTopBefore after=$basicContentTopAfter." }
    $sourceExtentAfterA = $scrollPattern.Current.VerticalViewSize
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
    $sourceToggle = Wait-CardSourceToggle $window 'ToggleButton — three-state'
    ([System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    $collapsedExtent = $null
    do {
        $sourceToggle = Wait-CardSourceToggle $window 'ToggleButton — three-state'
        $sourceState = ([System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState
        $sampleScroll = Wait-ScrollViewer $window
        $collapsedPattern = [System.Windows.Automation.ScrollPattern]$sampleScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
        if ($sourceState -eq [System.Windows.Automation.ToggleState]::Off -and $collapsedPattern.Current.VerticalViewSize -gt ($sourceExtentAfterA + 0.01)) {
            $collapsedExtent = $collapsedPattern.Current.VerticalViewSize
            break
        }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $collapsedExtent) {
        throw "Card A source collapsed visually but page extent did not shrink: expanded view=$sourceExtentAfterA current view=$($collapsedPattern.Current.VerticalViewSize)."
    }
    $basicSourceToggle = Wait-CardSourceToggle $window 'ToggleButton — basic'
    if (([System.Windows.Automation.TogglePattern]$basicSourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'Card B source expanded when Card A was toggled.' }
    ([System.Windows.Automation.TogglePattern]$basicSourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    $expandedBasicExtent = $null
    do {
        $basicSourceToggle = Wait-CardSourceToggle $window 'ToggleButton — basic'
        $basicSourceState = ([System.Windows.Automation.TogglePattern]$basicSourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState
        $sampleScroll = Wait-ScrollViewer $window
        $scrollPattern = [System.Windows.Automation.ScrollPattern]$sampleScroll.GetCurrentPattern([System.Windows.Automation.ScrollPattern]::Pattern)
        if ($basicSourceState -eq [System.Windows.Automation.ToggleState]::On -and $scrollPattern.Current.VerticalViewSize -lt ($collapsedExtent - 0.01)) {
            $expandedBasicExtent = $scrollPattern.Current.VerticalViewSize
            break
        }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -eq $expandedBasicExtent) { throw "Card B source expansion did not independently change page scroll extent: collapsed view=$collapsedExtent current view=$($scrollPattern.Current.VerticalViewSize)." }
    if (([System.Windows.Automation.TogglePattern]$sourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'Card B source toggle changed Card A state.' }
    $basicSourceViewer = Wait-SourceViewer $window 'ToggleButton — basic'
    $basicValue = [System.Windows.Automation.ValuePattern]$basicSourceViewer.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    $basicText = [System.Windows.Automation.TextPattern]$basicSourceViewer.GetCurrentPattern([System.Windows.Automation.TextPattern]::Pattern)
    if (-not $basicValue.Current.IsReadOnly -or -not $basicText.DocumentRange.GetText(-1).Contains('skip_initial_event')) { throw 'Card B source viewer did not expose the read-only compiled Rust source.' }
    ([System.Windows.Automation.TogglePattern]$basicSourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $basicSourceToggle = Wait-CardSourceToggle $window 'ToggleButton — basic'
        $basicSourceState = ([System.Windows.Automation.TogglePattern]$basicSourceToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Current.ToggleState
        if ($basicSourceState -eq [System.Windows.Automation.ToggleState]::Off) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($basicSourceState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'Card B source did not collapse independently.' }

    Select-NavigationSample $window 'Basic input' 'ToggleSwitch'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ToggleSwitch'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'ToggleSwitch route did not keep one page-level ScrollViewer.' }
    $switchCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'ToggleSwitch'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Wi-Fi'))
    $toggleSwitch = Wait-Element $window $switchCondition 'native ToggleSwitch'
    $toggleSwitchPattern = [System.Windows.Automation.TogglePattern]$toggleSwitch.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    if ($toggleSwitchPattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'ToggleSwitch did not start off.' }
    $toggleSwitchPattern.Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample toggle-switch: on'
    $toggleSwitchOutput = Wait-Text $window 'on'
    $toggleSwitch = Wait-Element $window $switchCondition 'native ToggleSwitch after on state'
    $toggleSwitchPattern = [System.Windows.Automation.TogglePattern]$toggleSwitch.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    if ($toggleSwitchPattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::On) { throw 'ToggleSwitch UIA state did not become on.' }
    $toggleSwitchPattern.Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample toggle-switch: off'
    $toggleSwitchOutput = Wait-Text $window 'off'
    Select-NavigationSample $window 'Basic input' 'RatingControl'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — RatingControl'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'RatingControl route did not keep one page-level ScrollViewer.' }
    $rating = Wait-Element $window (TypeCondition ([System.Windows.Automation.ControlType]::Slider)) 'native RatingControl'
    $ratingRange = [System.Windows.Automation.RangeValuePattern]$rating.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
    if ($ratingRange.Current.IsReadOnly -or [Math]::Abs($ratingRange.Current.Value - 3) -gt 0.01 -or [Math]::Abs($ratingRange.Current.Maximum - 5) -gt 0.01) {
        throw "RatingControl RangeValue contract mismatch: value=$($ratingRange.Current.Value) max=$($ratingRange.Current.Maximum) readonly=$($ratingRange.Current.IsReadOnly)."
    }
    $ratingRange.SetValue(4)
    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        $rating = Wait-Element $window (TypeCondition ([System.Windows.Automation.ControlType]::Slider)) 'native RatingControl after RangeValue set'
        $ratingRange = [System.Windows.Automation.RangeValuePattern]$rating.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
        if ([Math]::Abs($ratingRange.Current.Value - 4) -le 0.01) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ([Math]::Abs($ratingRange.Current.Value - 4) -gt 0.01) { throw 'RatingControl RangeValue did not round-trip 3 to 4.' }
    Select-NavigationSample $window 'Layout' 'Expander'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — Expander'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'Expander route did not keep one page-level ScrollViewer.' }
    $expanderCondition = [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.Expander')
    $expander = Wait-Element $window $expanderCondition 'native Expander'
    $expanderPattern = [System.Windows.Automation.ExpandCollapsePattern]$expander.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($expanderPattern.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Collapsed) { throw 'Expander did not start collapsed.' }
    $nested = $window.FindFirst($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Nested Winio content'))
    if ($null -ne $nested -and -not $nested.Current.IsOffscreen) { throw 'Nested Winio content was visible while Expander was collapsed.' }
    $expanderPattern.Expand()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample expander: expanded'
    $deadline = [DateTime]::UtcNow.AddSeconds(3)
    do {
        $nested = Wait-Text $window 'Nested Winio content'
        if (-not $nested.Current.IsOffscreen) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($nested.Current.IsOffscreen) { throw 'Nested Winio content did not become visible after expanding.' }
    $expander = Wait-Element $window $expanderCondition 'native Expander after expansion'
    $expanderPattern = [System.Windows.Automation.ExpandCollapsePattern]$expander.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($expanderPattern.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Expanded) { throw 'Expander UIA state did not become expanded.' }
    $expanderPattern.Collapse()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample expander: collapsed'
    $nested = $window.FindFirst($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Nested Winio content'))
    if ($null -ne $nested -and -not $nested.Current.IsOffscreen) { throw 'Nested Winio content remained visible after collapsing.' }
    $search = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'AutoSuggestBox')) 'AutoSuggestBox after sample route'
    $edit = Wait-Element $search (TypeCondition ([System.Windows.Automation.ControlType]::Edit)) 'AutoSuggestBox edit'
    ([System.Windows.Automation.ValuePattern]$edit.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)).SetValue('NumberBox')
    ([System.Windows.Automation.InvokePattern]$search.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — NumberBox'
    $staleThreeToggle = $window.FindFirst($descendants, (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state sample'))
    $staleBasicToggle = $window.FindFirst($descendants, (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample'))
    if ($null -ne $staleThreeToggle -or $null -ne $staleBasicToggle) { throw 'A ToggleButton example remained in the main UIA subtree after replacing it with NumberBox.' }
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
    if (-not [GalleryWindow]::SetWindowPos($hwnd, [IntPtr]::Zero, 0, 0, 900, 900, 0x0006)) {
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

    Select-NavigationSample $window 'Basic input' 'ToggleButton'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ToggleButton'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'Narrow ToggleButton route did not keep one page ScrollViewer.' }
    $narrowThree = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state sample') 'narrow three-state example'
    $narrowBasic = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Two-state sample') 'narrow basic example'
    $narrowOption = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Three-state: On') 'narrow ToggleButton option'
    $outputHeadings = $window.FindAll($descendants, (Condition ([System.Windows.Automation.ControlType]::Text) 'Output'))
    $optionsHeading = Wait-Text $window 'Options'
    if ($outputHeadings.Count -ne 2 -or $outputHeadings.Item(0).Current.BoundingRectangle.Top -le $narrowThree.Current.BoundingRectangle.Bottom -or $optionsHeading.Current.BoundingRectangle.Top -le $outputHeadings.Item(0).Current.BoundingRectangle.Bottom -or $narrowOption.Current.BoundingRectangle.Top -le $optionsHeading.Current.BoundingRectangle.Bottom -or $narrowBasic.Current.BoundingRectangle.Top -le $narrowOption.Current.BoundingRectangle.Bottom) {
        throw 'Narrow ToggleButton cards, Output, or Options regions overlap instead of stacking.'
    }

    Select-NavigationSample $window 'Status and info' 'ProgressRing'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ProgressRing'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'ProgressRing route did not keep one page-level ScrollViewer.' }
    $progressRing = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::ProgressBar),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.ProgressRing'))) 'native ProgressRing'
    $progressRange = [System.Windows.Automation.RangeValuePattern]$progressRing.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
    if ([Math]::Abs($progressRange.Current.Value - 65.0) -gt 0.001) { throw "ProgressRing RangeValue was not 65: $($progressRange.Current.Value)." }
    $progressToggle = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Indeterminate'))) 'ProgressRing indeterminate option'
    $progressTogglePattern = [System.Windows.Automation.TogglePattern]$progressToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $progressTogglePattern.Toggle()
    $progressOutput = Wait-Text $window 'indeterminate'
    if ($progressTogglePattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::On) { throw 'ProgressRing indeterminate option did not toggle on.' }
    $progressTogglePattern.Toggle()
    $progressOutput = Wait-Text $window 'value:65 · determinate'
    if ($progressTogglePattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'ProgressRing indeterminate option did not toggle off.' }
    Select-NavigationSample $window 'Status and info' 'ProgressBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ProgressBar'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'ProgressBar route did not keep one page-level ScrollViewer.' }
    $progressBar = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::ProgressBar),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.ProgressBar'))) 'native ProgressBar'
    $progressBarRange = [System.Windows.Automation.RangeValuePattern]$progressBar.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
    if ([Math]::Abs($progressBarRange.Current.Value - 65.0) -gt 0.001) { throw "ProgressBar RangeValue was not 65: $($progressBarRange.Current.Value)." }
    $progressBarToggle = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Indeterminate') 'ProgressBar indeterminate option'
    $progressBarTogglePattern = [System.Windows.Automation.TogglePattern]$progressBarToggle.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $progressBarTogglePattern.Toggle()
    $null = Wait-Text $window 'indeterminate'
    if ($progressBarTogglePattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::On) { throw 'ProgressBar indeterminate option did not toggle on.' }
    $progressBarTogglePattern.Toggle()
    $null = Wait-Text $window 'value:65 · determinate'
    if ($progressBarTogglePattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::Off) { throw 'ProgressBar indeterminate option did not toggle off.' }
    Select-NavigationSample $window 'Status and info' 'PersonPicture'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — PersonPicture'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'PersonPicture route did not keep one page-level ScrollViewer.' }
    $personOutput = Wait-Text $window 'Ada Lovelace · initials AL · badge 3'
    Select-NavigationSample $window 'Status and info' 'InfoBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — InfoBar'
    $info = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Text) 'Supplemental InfoBar canary') 'InfoBar canary message'
    $infoOutput = Wait-Text $window 'Open · informational status'

    Select-NavigationSample $window 'Collections' 'PipsPager'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — PipsPager'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'PipsPager route did not keep one page-level ScrollViewer.' }
    $pipsPager = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Menu),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.PipsPager'))) 'native PipsPager'
    $pipsSelection = [System.Windows.Automation.SelectionPattern]$pipsPager.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    if ($pipsSelection.Current.GetSelection().Count -ne 1) { throw 'PipsPager did not expose a single selected pip before Next.' }
    $pipsNext = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'NextPageButton')) 'PipsPager NextPageButton'
    ([System.Windows.Automation.InvokePattern]$pipsNext.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample pips-pager: page:2'
    $pipsOutput = Wait-Text $window 'page:2'
    $pipsPager = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Menu),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.PipsPager'))) 'native PipsPager after Next'
    $pipsSelection = [System.Windows.Automation.SelectionPattern]$pipsPager.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    if ($pipsSelection.Current.GetSelection().Count -ne 1) { throw 'PipsPager did not retain a single selected pip after Next.' }
    $pipsPrevious = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'PreviousPageButton')) 'PipsPager PreviousPageButton'
    ([System.Windows.Automation.InvokePattern]$pipsPrevious.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample pips-pager: page:1'
    $pipsOutput = Wait-Text $window 'page:1'
    Select-NavigationSample $window 'Collections' 'FlipView'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — FlipView'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'FlipView route did not keep one page-level ScrollViewer.' }
    $flip = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::ClassNameProperty, 'FlipView')) 'native FlipView'
    $flipSelection = [System.Windows.Automation.SelectionPattern]$flip.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    $flipSelected = $flipSelection.Current.GetSelection()
    if ($flipSelected.Count -ne 1 -or $flipSelected[0].Current.Name -ne 'First page') { throw 'FlipView did not start on First page.' }
    $flipItems = [System.Windows.Automation.ItemContainerPattern]$flip.GetCurrentPattern([System.Windows.Automation.ItemContainerPattern]::Pattern)
    $secondPage = $flipItems.FindItemByProperty($null, $automation::NameProperty, 'Second page')
    if ($null -eq $secondPage) { throw 'FlipView ItemContainerPattern did not find Second page.' }
    try {
        ([System.Windows.Automation.VirtualizedItemPattern]$secondPage.GetCurrentPattern([System.Windows.Automation.VirtualizedItemPattern]::Pattern)).Realize()
    } catch {}
    ([System.Windows.Automation.SelectionItemPattern]$secondPage.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample flip-view: page:second'
    $flipOutput = Wait-Text $window 'page:second'
    $flip = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::ClassNameProperty, 'FlipView')) 'native FlipView after selection'
    $flipSelection = [System.Windows.Automation.SelectionPattern]$flip.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    $flipSelected = $flipSelection.Current.GetSelection()
    if ($flipSelected.Count -ne 1 -or $flipSelected[0].Current.Name -ne 'Second page') { throw 'FlipView selection did not round-trip to Second page.' }
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

    Select-NavigationSample $window 'Basic input' 'DropDownButton'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — DropDownButton'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'DropDownButton route did not keep one page-level ScrollViewer.' }
    $dropDownCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.DropDownButton'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Choose format'))
    $dropDown = Wait-Element $window $dropDownCondition 'native DropDownButton'
    $dropDownExpand = [System.Windows.Automation.ExpandCollapsePattern]$dropDown.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($dropDownExpand.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Collapsed) { throw 'DropDownButton flyout did not start collapsed.' }
    $dropDownExpand.Expand()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample drop-down-button: flyout opened'
    $dropDownOutput = Wait-Text $window 'flyout opened'
    $dropDownChoice = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        (Condition ([System.Windows.Automation.ControlType]::Text) 'PDF document'))) 'DropDownButton flyout Winio content'
    if ($dropDownChoice.Current.IsOffscreen) { throw 'DropDownButton flyout content remained offscreen after expansion.' }
    $dropDown = Wait-Element $window $dropDownCondition 'native DropDownButton after flyout open'
    $dropDownExpand = [System.Windows.Automation.ExpandCollapsePattern]$dropDown.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($dropDownExpand.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Expanded) { throw 'DropDownButton UIA state did not become expanded.' }
    $dropDownExpand.Collapse()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample drop-down-button: flyout closed'
    $dropDownOutput = Wait-Text $window 'flyout closed'
    Select-NavigationSample $window 'Basic input' 'SplitButton'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — SplitButton'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'SplitButton route did not keep one page-level ScrollViewer.' }
    $splitCondition = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::SplitButton),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.SplitButton'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Save'))
    $split = Wait-Element $window $splitCondition 'native SplitButton'
    ([System.Windows.Automation.InvokePattern]$split.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample split-button: primary click'
    $splitOutput = Wait-Text $window 'primary click'
    $split = Wait-Element $window $splitCondition 'native SplitButton after primary click'
    $splitExpand = [System.Windows.Automation.ExpandCollapsePattern]$split.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($splitExpand.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Collapsed) { throw 'SplitButton flyout did not start collapsed.' }
    $splitExpand.Expand()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample split-button: flyout opened'
    $splitOutput = Wait-Text $window 'flyout opened'
    $secondary = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        (Condition ([System.Windows.Automation.ControlType]::Text) 'Secondary action'))) 'SplitButton flyout Winio content'
    if ($secondary.Current.IsOffscreen) { throw 'SplitButton flyout content remained offscreen after expansion.' }
    $split = Wait-Element $window $splitCondition 'native SplitButton after flyout open'
    $splitExpand = [System.Windows.Automation.ExpandCollapsePattern]$split.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($splitExpand.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Expanded) { throw 'SplitButton UIA state did not become expanded.' }
    $splitExpand.Collapse()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample split-button: flyout closed'
    $splitOutput = Wait-Text $window 'flyout closed'
    Select-NavigationSample $window 'Basic input' 'ColorPicker'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ColorPicker'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'ColorPicker route did not keep one page-level ScrollViewer.' }
    $hexInput = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'HexTextBox')) 'ColorPicker HexTextBox'
    if ($hexInput.Current.IsOffscreen) { throw 'ColorPicker HexTextBox was offscreen in compact sample.' }
    $hexValue = [System.Windows.Automation.ValuePattern]$hexInput.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    if ($hexValue.Current.Value -ne '#3366CC') { throw "ColorPicker initial hex value was '$($hexValue.Current.Value)'." }
    $hexValue.SetValue('#112233')
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample color-picker: #FF112233'
    $colorOutput = Wait-Text $window '#FF112233'
    $hexInput = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'HexTextBox')) 'ColorPicker HexTextBox after edit'
    $hexValue = [System.Windows.Automation.ValuePattern]$hexInput.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    if ($hexValue.Current.Value -ne '#112233') { throw "ColorPicker edited hex value was '$($hexValue.Current.Value)'." }
    Select-NavigationSample $window 'Basic input' 'CalendarDatePicker'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — CalendarDatePicker'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'CalendarDatePicker route did not keep one page-level ScrollViewer.' }
    $calendarDatePicker = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'CalendarDatePicker'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Calendar date'))) 'native CalendarDatePicker'
    ([System.Windows.Automation.InvokePattern]$calendarDatePicker.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $calendarView = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'CalendarView'))) 'CalendarDatePicker CalendarView'
    $calendarSelection = [System.Windows.Automation.SelectionPattern]$calendarView.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    $calendarCurrent = $calendarSelection.Current.GetSelection() | Select-Object -First 1
    if ($null -eq $calendarCurrent) { throw 'CalendarDatePicker CalendarView had no selected day.' }
    $calendarWalker = [System.Windows.Automation.TreeWalker]::ControlViewWalker
    $calendarNext = $calendarWalker.GetNextSibling($calendarCurrent)
    if ($null -eq $calendarNext) { throw 'CalendarDatePicker selected day had no next sibling.' }
    ([System.Windows.Automation.SelectionItemPattern]$calendarNext.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample calendar-date-picker: 2026-10-02'
    $calendarDateOutput = Wait-Text $window '2026-10-02'
    $calendarDatePicker = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'CalendarDatePicker'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Calendar date'))) 'CalendarDatePicker after selection'
    $calendarDateValue = [System.Windows.Automation.ValuePattern]$calendarDatePicker.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    if ($calendarDateValue.Current.Value -notmatch '2') { throw "CalendarDatePicker ValuePattern did not reflect the next day: '$($calendarDateValue.Current.Value)'." }
    Select-NavigationSample $window 'Basic input' 'DatePicker'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — DatePicker'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'DatePicker route did not keep one page-level ScrollViewer.' }
    $dateFlyoutButton = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'FlyoutButton')) 'DatePicker FlyoutButton'
    ([System.Windows.Automation.InvokePattern]$dateFlyoutButton.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()

    $dateWalker = [System.Windows.Automation.TreeWalker]::ControlViewWalker
    $monthSelector = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'MonthLoopingSelector'))) 'DatePicker month selector'
    $monthSelection = [System.Windows.Automation.SelectionPattern]$monthSelector.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    $currentMonth = $monthSelection.Current.GetSelection() | Select-Object -First 1
    if ($null -eq $currentMonth) { throw 'DatePicker month selector had no selected item.' }
    $nextMonth = $dateWalker.GetNextSibling($currentMonth)
    if ($null -eq $nextMonth) { throw 'DatePicker month selector had no next item.' }
    ([System.Windows.Automation.SelectionItemPattern]$nextMonth.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()

    $daySelector = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'DayLoopingSelector'))) 'DatePicker day selector'
    $daySelection = [System.Windows.Automation.SelectionPattern]$daySelector.GetCurrentPattern([System.Windows.Automation.SelectionPattern]::Pattern)
    $currentDay = $daySelection.Current.GetSelection() | Select-Object -First 1
    if ($null -eq $currentDay) { throw 'DatePicker day selector had no selected item.' }
    $nextDay = $dateWalker.GetNextSibling($currentDay)
    if ($null -eq $nextDay) { throw 'DatePicker day selector had no next item.' }
    ([System.Windows.Automation.SelectionItemPattern]$nextDay.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()

    $dateAccept = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'AcceptButton'))) 'DatePicker AcceptButton'
    ([System.Windows.Automation.InvokePattern]$dateAccept.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample date-picker: 2026-11-02'
    $dateOutput = Wait-Text $window '2026-11-02'
    Select-NavigationSample $window 'Basic input' 'PasswordBox'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — PasswordBox'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'PasswordBox route did not keep one page-level ScrollViewer.' }
    $passwordBox = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Edit),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'PasswordBox'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Password'))) 'native PasswordBox'
    if (-not $passwordBox.Current.IsPassword) { throw 'PasswordBox exposed the password field without IsPassword.' }
    $passwordValue = [System.Windows.Automation.ValuePattern]$passwordBox.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)
    $passwordValue.SetValue('secret42')
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample password-box: password length:8'
    $passwordOutput = Wait-Text $window 'password length:8'
    Select-NavigationSample $window 'Basic input' 'TimePicker'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — TimePicker'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'TimePicker route did not keep one page-level ScrollViewer.' }
    $timeFlyoutButton = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'FlyoutButton')) 'TimePicker FlyoutButton'
    ([System.Windows.Automation.InvokePattern]$timeFlyoutButton.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()

    $hourSelector = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'HourLoopingSelector'))) 'TimePicker hour selector'
    $minuteSelector = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'MinuteLoopingSelector'))) 'TimePicker minute selector'

    $hour11 = Wait-Element $hourSelector ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, '11'))) 'TimePicker hour 11'
    ([System.Windows.Automation.SelectionItemPattern]$hour11.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()

    $minute45 = Wait-Element $minuteSelector ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, '45'))) 'TimePicker minute 45'
    ([System.Windows.Automation.SelectionItemPattern]$minute45.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()

    $deadline = [DateTime]::UtcNow.AddSeconds(4)
    do {
        $hour11Selected = [System.Windows.Automation.SelectionItemPattern]$hour11.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)
        $minute45Selected = [System.Windows.Automation.SelectionItemPattern]$minute45.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)
        if ($hour11Selected.Current.IsSelected -and $minute45Selected.Current.IsSelected) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if (-not $hour11Selected.Current.IsSelected) { throw 'TimePicker hour selector did not select 11.' }
    if (-not $minute45Selected.Current.IsSelected) { throw 'TimePicker minute selector did not select 45.' }

    $timeAccept = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'AcceptButton'))) 'TimePicker AcceptButton'
    ([System.Windows.Automation.InvokePattern]$timeAccept.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample time-picker: 11:45'
    $timeOutput = Wait-Text $window '11:45'
    Select-NavigationSample $window 'Menus and toolbars' 'CommandBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — CommandBar'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'CommandBar route did not keep one page-level ScrollViewer.' }
    $commandSave = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'AppBarButton'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Save'))) 'CommandBar primary Save action'
    ([System.Windows.Automation.InvokePattern]$commandSave.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample command-bar: action:save'
    $commandOutput = Wait-Text $window 'action:save'
    $moreButton = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new(
        $automation::AutomationIdProperty, 'MoreButton')) 'CommandBar MoreButton'
    ([System.Windows.Automation.InvokePattern]$moreButton.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $commandSettings = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.AndCondition]::new(
            [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
            [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'AppBarButton'),
            [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Settings')))) 'CommandBar secondary Settings action'
    if ($commandSettings.Current.IsOffscreen) { throw 'CommandBar secondary Settings action remained offscreen after opening overflow.' }
    ([System.Windows.Automation.InvokePattern]$commandSettings.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample command-bar: action:settings'
    $commandOutput = Wait-Text $window 'action:settings'
    Select-NavigationSample $window 'Menus and toolbars' 'MenuBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — MenuBar'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'MenuBar route did not keep one page-level ScrollViewer.' }
    $fileMenu = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::MenuItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.MenuBarItem'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'File'))) 'MenuBar File menu'
    $fileExpand = [System.Windows.Automation.ExpandCollapsePattern]$fileMenu.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($fileExpand.Current.ExpandCollapseState -ne [System.Windows.Automation.ExpandCollapseState]::Collapsed) { throw 'MenuBar File menu did not start collapsed.' }
    $fileExpand.Expand()
    $newAction = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.AndCondition]::new(
            [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::MenuItem),
            [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'MenuFlyoutItem'),
            [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'New')))) 'MenuBar File/New action'
    if ($newAction.Current.IsOffscreen) { throw 'MenuBar File/New action remained offscreen after expanding File.' }
    ([System.Windows.Automation.InvokePattern]$newAction.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample menu-bar: action:new'
    $menuOutput = Wait-Text $window 'action:new'
    $helpMenu = Wait-Element $window ([System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::MenuItem),
        [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.MenuBarItem'),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Help'))) 'MenuBar Help menu'
    ([System.Windows.Automation.ExpandCollapsePattern]$helpMenu.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)).Expand()
    $aboutAction = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.AndCondition]::new(
            [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::MenuItem),
            [System.Windows.Automation.PropertyCondition]::new($automation::ClassNameProperty, 'MenuFlyoutItem'),
            [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'About')))) 'MenuBar Help/About action'
    ([System.Windows.Automation.InvokePattern]$aboutAction.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample menu-bar: action:about'
    $menuOutput = Wait-Text $window 'action:about'
    Select-NavigationSample $window 'Dialogs and flyouts' 'ContentDialog'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — ContentDialog'
    if ((Get-PageScrollViewers $window).Count -ne 1) { throw 'ContentDialog route did not keep one page-level ScrollViewer.' }
    $dialogTarget = Wait-Element $window (Condition ([System.Windows.Automation.ControlType]::Button) 'Open ContentDialog') 'ContentDialog target'
    ([System.Windows.Automation.TogglePattern]$dialogTarget.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample content-dialog: opened'
    $dialogOutput = Wait-Text $window 'opened'
    $dialogPopupCondition = [System.Windows.Automation.AndCondition]::new($pc,[System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new($automation::ControlTypeProperty, [System.Windows.Automation.ControlType]::Window),
        [System.Windows.Automation.PropertyCondition]::new($automation::NameProperty, 'Save changes?')))
    $dialogPopup = Wait-Element $root $dialogPopupCondition 'native ContentDialog popup'
    $dialogContent = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        (Condition ([System.Windows.Automation.ControlType]::Text) 'This content is owned by Winio inside the native ContentDialog.'))) 'Winio-owned ContentDialog content'
    if ($dialogContent.Current.IsOffscreen) { throw 'ContentDialog Winio-owned content remained offscreen.' }
    $dialogSave = Wait-Element $root ([System.Windows.Automation.AndCondition]::new(
        $pc,
        (Condition ([System.Windows.Automation.ControlType]::Button) 'Save'))) 'ContentDialog primary button'
    ([System.Windows.Automation.InvokePattern]$dialogSave.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample content-dialog: closing:Primary'
    $dialogOutput = Wait-Text $window 'closing:Primary'
    $deadline = [DateTime]::UtcNow.AddSeconds(8)
    do {
        $dialogAfter = $root.FindFirst($descendants, $dialogPopupCondition)
        if ($null -eq $dialogAfter) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -ne $dialogAfter) { throw 'ContentDialog popup remained after primary-button close.' }
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

    Select-NavigationSample $window 'Status and info' 'InfoBadge'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — InfoBadge'
    $badgeRoundTrip = Wait-Text $window 'value:7'

    Select-NavigationSample $window 'Basic input' 'RadioButtons'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample radio-buttons: choice:standard'
    $radio = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'radio-compact')) 'RadioButtons option by AutomationId'
    ([System.Windows.Automation.SelectionItemPattern]$radio.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample radio-buttons: choice:compact'

    Select-NavigationSample $window 'Collections' 'SelectorBar'
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample selector-bar: section:overview'
    $selectorItem = Wait-Element $window ([System.Windows.Automation.PropertyCondition]::new($automation::AutomationIdProperty, 'selector-details')) 'SelectorBar item by AutomationId'
    ([System.Windows.Automation.SelectionItemPattern]$selectorItem.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
    $window = Wait-MainTitle $process $pc 'Winio WinUI Extensions — sample selector-bar: section:details'

    Write-Output "PASS: fixed shell and one page-level routed ScrollViewer; integrated TitleBar/search/pane and caption Close; catalog navigation; independent two-card ToggleButton examples, outputs, options, source toggles, stack movement and extent; bounded read-only/selectable Rust source viewers and recollapse; AutoSuggestBox route and both stale-peer removals; NumberBox RangeValue 10→42/output and live SmallChange option; narrow stacked geometry; native ToggleSwitch on/off event path; RatingControl RangeValue 3→4; ProgressRing RangeValue 65 + indeterminate option round-trip; ColorPicker HexTextBox #3366CC→#112233 with typed RGBA event; CalendarDatePicker CalendarView next-day selection 2026-10-01→2026-10-02 with nullable time::Date event; DatePicker native month/day next-selection 2026-10-01→2026-11-02 with typed time::Date event; PasswordBox IsPassword + ValuePattern change without secret echo; TimePicker native hour/minute selection 10:30→11:45 with typed Duration event; native Expander nested-content ownership and expand/collapse event path; DropDownButton native expand/collapse flyout lifecycle; SplitButton primary invoke and owned flyout lifecycle; CommandBar primary/overflow stable action events; MenuBar top-level expand and stable flyout action events; ContentDialog XamlRoot/content ownership and Primary close result; PersonPicture native display-name/initials/badge property round-trip; InfoBadge value getter round-trip; RadioButtons SelectionItemPattern stable ID; SelectorBar SelectionItemPattern stable ID; InfoBar; PipsPager Next/Previous stable selected-index events; FlipView ItemContainer/Selection first→second with typed selection event; TreeView; native TeachingTip popup lifecycle."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}

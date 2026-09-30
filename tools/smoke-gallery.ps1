$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes

function Find-MainWindow($ProcessCondition) {
    $windows = [System.Windows.Automation.AutomationElement]::RootElement.FindAll(
        [System.Windows.Automation.TreeScope]::Children, $ProcessCondition)
    for ($i = 0; $i -lt $windows.Count; $i++) {
        $candidate = $windows.Item($i)
        if ($candidate.Current.Name -like 'Winio WinUI Extensions*') { return $candidate }
    }
    return $null
}

function Wait-MainTitle($Process, $ProcessCondition, [string]$Expected) {
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        if ($Process.HasExited) { throw "Gallery exited while waiting for '$Expected' (exit code $($Process.ExitCode))." }
        $window = Find-MainWindow $ProcessCondition
        if ($null -ne $window -and $window.Current.Name -eq $Expected) { return $window }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    $actual = if ($null -eq $window) { '<missing>' } else { $window.Current.Name }
    throw "Timed out waiting for '$Expected'; got '$actual'."
}

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$gallery = Join-Path $repoRoot 'target\debug\winio-winui-ext-gallery.exe'
if (-not (Test-Path -LiteralPath $gallery -PathType Leaf)) { throw "Gallery executable not found: $gallery" }

$process = $null
try {
    $process = Start-Process -FilePath $gallery -PassThru
    $pc = [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $process.Id)
    $buttonCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty, 'Native WinUI ToggleButton'))
    $infoCond = [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty, 'Supplemental InfoBar canary')
    $detailsCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty, 'Details'))
    $controlsCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::ListItem),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty, 'Controls'))
    $searchCond = [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ClassNameProperty, 'AutoSuggestBox')
    $numberCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Spinner),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.NumberBox'))
    $treeCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::TreeItem),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.TreeViewItem'))
    $titleBarCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::TitleBar),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ClassNameProperty, 'Microsoft.UI.Xaml.Controls.TitleBar'))
    $captionCloseCond = [System.Windows.Automation.AndCondition]::new(
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::AutomationIdProperty, 'Close'))
    $tipPopupCond = [System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.AndCondition]::new(
            [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Window),
            [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::NameProperty, 'Native TeachingTip canary')))
    $tipCloseCond = [System.Windows.Automation.AndCondition]::new(
        $pc,
        [System.Windows.Automation.AndCondition]::new(
            [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Button),
            [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::AutomationIdProperty, 'CloseButton')))

    $deadline = [DateTime]::UtcNow.AddSeconds(30)
    do {
        if ($process.HasExited) { throw "Gallery exited before UIA discovery (exit code $($process.ExitCode))." }
        $window = Find-MainWindow $pc
        if ($null -ne $window) {
            $button = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $buttonCond)
            $info = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $infoCond)
            $details = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $detailsCond)
            $controls = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $controlsCond)
            $search = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $searchCond)
            $number = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $numberCond)
            $treeRoot = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $treeCond)
            $titleBar = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $titleBarCond)
            $captionClose = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $captionCloseCond)
        }
        $tipPopup = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $tipPopupCond)
        $tipClose = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $tipCloseCond)
        if ($null -ne $button -and $null -ne $info -and $null -ne $controls -and $null -ne $search -and $null -ne $number -and
            $null -ne $treeRoot -and $null -ne $titleBar -and $null -ne $captionClose -and
            $null -ne $tipPopup -and $null -ne $tipClose) { break }
        Start-Sleep -Milliseconds 150
    } while ([DateTime]::UtcNow -lt $deadline)

    if ($null -eq $button) { throw 'ToggleButton not found.' }
    if ($null -eq $info) { throw 'InfoBar message not found.' }
    if ($null -eq $controls) { throw 'NavigationView nested Controls item not found.' }
    if ($null -eq $search) { throw 'Native AutoSuggestBox not found.' }

    $controlsExpand = [System.Windows.Automation.ExpandCollapsePattern]$controls.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)
    if ($controlsExpand.Current.ExpandCollapseState -eq [System.Windows.Automation.ExpandCollapseState]::Collapsed) {
        $controlsExpand.Expand()
    }
    $details = $window.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $detailsCond)
    if ($null -eq $details) { throw 'Nested NavigationView Details item was not exposed after expanding Controls.' }
    if ($null -eq $number) { throw 'NumberBox not found.' }
    if ($null -eq $treeRoot) { throw 'TreeView root not found.' }
    if ($null -eq $titleBar) { throw 'Integrated native TitleBar not found.' }
    if ($null -eq $captionClose) { throw 'System caption Close button not found beside integrated TitleBar.' }
    if ($null -eq $tipPopup -or $null -eq $tipClose) { throw 'TeachingTip popup/close button not found.' }

    $range = [System.Windows.Automation.RangeValuePattern]$number.GetCurrentPattern([System.Windows.Automation.RangeValuePattern]::Pattern)
    if ($range.Current.IsReadOnly -or $range.Current.Value -ne 10 -or $range.Current.Minimum -ne 0 -or
        $range.Current.Maximum -ne 100 -or $range.Current.SmallChange -ne 1) {
        throw "NumberBox range mismatch: value=$($range.Current.Value) min=$($range.Current.Minimum) max=$($range.Current.Maximum) small=$($range.Current.SmallChange)."
    }

    ([System.Windows.Automation.InvokePattern]$tipClose.GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $tipTitle = 'Winio WinUI Extensions — teaching tip closed: CloseButton'
    $window = Wait-MainTitle $process $pc $tipTitle
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $tipAfter = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Descendants, $tipPopupCond)
        if ($null -eq $tipAfter) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($null -ne $tipAfter) { throw 'TeachingTip popup remained after close.' }

    ([System.Windows.Automation.TogglePattern]$button.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)).Toggle()
    $deadline = [DateTime]::UtcNow.AddSeconds(10)
    do {
        $window = Find-MainWindow $pc
        if ($null -ne $window -and $window.Current.Name -in @('Winio WinUI Extensions — state: checked','Winio WinUI Extensions — state: unchecked')) { break }
        Start-Sleep -Milliseconds 100
    } while ([DateTime]::UtcNow -lt $deadline)
    if ($window.Current.Name -notlike 'Winio WinUI Extensions — state:*') { throw 'ToggleButton event path failed.' }
    $toggleTitle = $window.Current.Name

    ([System.Windows.Automation.SelectionItemPattern]$details.GetCurrentPattern([System.Windows.Automation.SelectionItemPattern]::Pattern)).Select()
    $navTitle = 'Winio WinUI Extensions — navigation: details'
    $window = Wait-MainTitle $process $pc $navTitle

    $searchInput = $search.FindFirst([System.Windows.Automation.TreeScope]::Descendants,
        [System.Windows.Automation.PropertyCondition]::new([System.Windows.Automation.AutomationElement]::ControlTypeProperty, [System.Windows.Automation.ControlType]::Edit))
    if ($null -eq $searchInput) { throw 'AutoSuggestBox edit field not found.' }
    ([System.Windows.Automation.ValuePattern]$searchInput.GetCurrentPattern([System.Windows.Automation.ValuePattern]::Pattern)).SetValue('TreeView')
    $searchInvoke = [System.Windows.Automation.InvokePattern]$search.GetCurrentPattern(
        [System.Windows.Automation.InvokePattern]::Pattern)
    $searchInvoke.Invoke()
    $searchTitle = 'Winio WinUI Extensions — search submitted: TreeView'
    $window = Wait-MainTitle $process $pc $searchTitle

    $range.SetValue(42)
    $numberTitle = 'Winio WinUI Extensions — number: 42'
    $window = Wait-MainTitle $process $pc $numberTitle

    ([System.Windows.Automation.ExpandCollapsePattern]$treeRoot.GetCurrentPattern([System.Windows.Automation.ExpandCollapsePattern]::Pattern)).Expand()
    $treeExpandedTitle = 'Winio WinUI Extensions — tree expanded: root'
    $window = Wait-MainTitle $process $pc $treeExpandedTitle
    $treeItems = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants, $treeCond)
    if ($treeItems.Count -lt 2) { throw "Expected 2 TreeViewItem peers, found $($treeItems.Count)." }
    ([System.Windows.Automation.InvokePattern]$treeItems.Item(1).GetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern)).Invoke()
    $treeInvokedTitle = 'Winio WinUI Extensions — tree invoked: leaf'
    $window = Wait-MainTitle $process $pc $treeInvokedTitle

    Write-Output "PASS: integrated native TitleBar '$($titleBar.Current.Name)' coexists with system caption controls; TeachingTip closed through native popup with '$tipTitle'; InfoBar visible; NumberBox metadata and value event passed; ToggleButton observed '$toggleTitle'; nested NavigationView selected child and observed '$navTitle'; AutoSuggestBox QuerySubmitted observed '$searchTitle'; TreeView observed '$treeExpandedTitle' then '$treeInvokedTitle'."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        $process.WaitForExit(5000) | Out-Null
    }
}

# flip-test.ps1 -- a live flip of Windows' "Choose your mode" for apps while Rookparcel runs
# with Theme = System and a dialog open (tools/win32-shots). For a test machine or VM: it
# writes HKCU\...\Themes\Personalize\AppsUseLightTheme and broadcasts WM_SETTINGCHANGE
# "ImmersiveColorSet" as the Settings app does, then puts the value back.
#
# Windows PowerShell 5.1:
#   powershell.exe -NoProfile -ExecutionPolicy Bypass -File flip-test.ps1 `
#       -SourceExe <rookparcel.exe> -WorkDir <folder for the portable copy> -OutDir <PNG folder>
#
# Copies the exe into WorkDir beside an empty rookparcel.ini (Theme = System), starts it with
# --demo <DemoCount> through run_hidden.ps1 -NoWait, opens Preferences > UI Settings, pictures the
# main window and the dialog, flips Windows to dark, pictures both again, flips back to light,
# pictures again, closes the dialog, stops the program by its own PID and prints the ini's
# ui.theme line (there must be none: the preference stays System). Helpers are capture.ps1's.
param(
    [Parameter(Mandatory = $true)][string]$SourceExe,
    [Parameter(Mandatory = $true)][string]$WorkDir,
    [Parameter(Mandatory = $true)][string]$OutDir,
    [int]$DemoCount = 40,
    [string]$Desktop = 'AgentHidden',
    [string]$RunHidden = 'L:\dev\tools\run_hidden\run_hidden.ps1'
)

$ErrorActionPreference = 'Stop'
(Get-Process -Id $PID).PriorityClass = 'BelowNormal'

if (-not (Test-Path $SourceExe)) { throw "exe not found: $SourceExe" }
New-Item -ItemType Directory -Force $WorkDir | Out-Null
New-Item -ItemType Directory -Force $OutDir | Out-Null
Get-ChildItem $OutDir -File | Where-Object { $_.Extension -eq '.png' -or $_.Name -in 'manifest.txt', 'flip-log.txt' } | Remove-Item -Force
$ExePath = Join-Path $WorkDir 'rookparcel.exe'
Copy-Item $SourceExe $ExePath -Force
$ini = Join-Path $WorkDir 'rookparcel.ini'
Set-Content -Path $ini -Value '' -NoNewline

Add-Type -ReferencedAssemblies System.Drawing -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading;

public static class Shot {
    [DllImport("user32.dll", SetLastError = true)] static extern IntPtr OpenDesktop(string name, uint flags, bool inherit, uint access);
    [DllImport("user32.dll")] static extern bool SetThreadDesktop(IntPtr h);
    delegate bool EnumProc(IntPtr hwnd, IntPtr lp);
    [DllImport("user32.dll")] static extern bool EnumDesktopWindows(IntPtr desk, EnumProc cb, IntPtr lp);
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr parent, EnumProc cb, IntPtr lp);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hwnd, out uint pid);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr hwnd);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr hwnd, out RECT r);
    [DllImport("user32.dll")] static extern bool PrintWindow(IntPtr hwnd, IntPtr hdc, uint flags);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern bool PostMessageW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr SendMessageTimeoutW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern IntPtr SendMessageTimeoutW(IntPtr hwnd, uint msg, IntPtr wp, string lp, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr dlg, int id);
    [DllImport("user32.dll")] public static extern int GetDlgCtrlID(IntPtr hwnd);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr ctx);
    [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr hwnd);

    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }

    public static void DpiAware() { try { SetProcessDpiAwarenessContext(new IntPtr(-4)); } catch {} }

    // what the Settings app sends after a change of the mode: WM_SETTINGCHANGE "ImmersiveColorSet" to every top-level window
    public static long BroadcastImmersive() {
        IntPtr r;
        return SendMessageTimeoutW(new IntPtr(0xFFFF), 0x1A, IntPtr.Zero, "ImmersiveColorSet", 2, 2000, out r).ToInt64();
    }
    // the same to one window, on the hidden desktop (HWND_BROADCAST only reaches the caller's desktop)
    public static long SendImmersive(IntPtr desk, IntPtr hwnd) {
        long result = -1;
        var t = new Thread(delegate() {
            if (!SetThreadDesktop(desk)) { result = -2; return; }
            IntPtr r;
            result = SendMessageTimeoutW(hwnd, 0x1A, IntPtr.Zero, "ImmersiveColorSet", 2, 3000, out r).ToInt64();
        });
        t.Start(); t.Join();
        return result;
    }

    public static IntPtr Open(string name) {
        IntPtr h = OpenDesktop(name, 0, false, 0x10000000 /* GENERIC_ALL */);
        if (h == IntPtr.Zero) throw new Exception("OpenDesktop failed: " + Marshal.GetLastWin32Error());
        return h;
    }

    public class Win { public IntPtr H; public string Cls; public string Title; public bool Vis; public int W; public int Hgt; public uint Pid; }

    static Win Describe(IntPtr h) {
        var w = new Win(); w.H = h;
        uint p; GetWindowThreadProcessId(h, out p); w.Pid = p;
        var sb = new StringBuilder(256); GetClassName(h, sb, 256); w.Cls = sb.ToString();
        sb = new StringBuilder(512); GetWindowText(h, sb, 512); w.Title = sb.ToString();
        w.Vis = IsWindowVisible(h);
        RECT r; GetWindowRect(h, out r); w.W = r.R - r.L; w.Hgt = r.B - r.T;
        return w;
    }

    public static List<Win> Windows(IntPtr desk, uint pid) {
        var list = new List<Win>();
        EnumDesktopWindows(desk, delegate(IntPtr h, IntPtr lp) {
            uint p; GetWindowThreadProcessId(h, out p);
            if (pid != 0 && p != pid) return true;
            list.Add(Describe(h)); return true;
        }, IntPtr.Zero);
        return list;
    }

    public static List<Win> Descendants(IntPtr parent, int id) {
        var list = new List<Win>();
        EnumChildWindows(parent, delegate(IntPtr h, IntPtr lp) {
            if (GetDlgCtrlID(h) == id) list.Add(Describe(h));
            return true;
        }, IntPtr.Zero);
        return list;
    }

    public static string Capture(IntPtr desk, IntPtr hwnd, string path, uint flags) {
        string result = null;
        var t = new Thread(delegate() {
            try {
                if (!SetThreadDesktop(desk)) { result = "SetThreadDesktop failed: " + Marshal.GetLastWin32Error(); return; }
                RECT r; if (!GetWindowRect(hwnd, out r)) { result = "GetWindowRect failed"; return; }
                int w = r.R - r.L, h = r.B - r.T;
                if (w <= 0 || h <= 0) { result = "empty rect"; return; }
                using (var bmp = new Bitmap(w, h, PixelFormat.Format32bppArgb))
                using (var g = Graphics.FromImage(bmp)) {
                    g.Clear(Color.Magenta);
                    IntPtr dc = g.GetHdc();
                    bool ok = PrintWindow(hwnd, dc, flags);
                    g.ReleaseHdc(dc);
                    if (!ok) { result = "PrintWindow failed: " + Marshal.GetLastWin32Error(); return; }
                    bmp.Save(path, ImageFormat.Png);
                    result = w + "x" + h;
                }
            } catch (Exception e) { result = "error: " + e.Message; }
        });
        t.SetApartmentState(ApartmentState.STA);
        t.Start(); t.Join();
        return result;
    }
}
'@

[Shot]::DpiAware()

$WM_CLOSE = 0x10; $WM_KEYDOWN = 0x100; $WM_KEYUP = 0x101; $WM_COMMAND = 0x111
$VK_HOME = 0x24
$LVM_GETNEXTITEM = 0x100C; $LVNI_SELECTED = 2
$TVM_GETNEXTITEM = 0x110A; $TVM_SELECTITEM = 0x110B; $TVGN_ROOT = 0; $TVGN_NEXT = 1; $TVGN_CARET = 9
$CB_GETCOUNT = 0x146; $CB_GETCURSEL = 0x147
$IDC_LIST = 12; $ID_PREF_TREE = 900; $ID_THEME_COMBO = 1110
$IDCANCEL = 2
$personalize = 'HKCU:\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize'

$manifest = New-Object System.Collections.Generic.List[string]
$manifest.Add("# Windows mode flip test, $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
$manifest.Add("# exe: $ExePath (copy of $SourceExe), empty ini (Theme = System)")
$manifest.Add("# format: png | class | title | size | note")
$log = New-Object System.Collections.Generic.List[string]
function Log($s) { $script:log.Add($s); Write-Host $s }

function Post($h, $msg, $wp, $lp) { [Shot]::PostMessageW($h, [uint32]$msg, [IntPtr][int64]$wp, [IntPtr][int64]$lp) | Out-Null }
function Send($h, $msg, $wp, $lp) {
    $r = [IntPtr]::Zero
    [Shot]::SendMessageTimeoutW($h, [uint32]$msg, [IntPtr][int64]$wp, [IntPtr][int64]$lp, 2, 3000, [ref]$r) | Out-Null
    return $r.ToInt64()
}
function Key($h, $vk) { Post $h $WM_KEYDOWN $vk 0; Start-Sleep -Milliseconds 40; Post $h $WM_KEYUP $vk 0 }
function Wins() { [Shot]::Windows($desk, 0) }
function TopWins() { Wins | Where-Object { $_.Vis -and $_.W -gt 0 } }
function Dialogs() { TopWins | Where-Object { $_.Cls -notlike 'RookparcelMainWnd*' -and $_.Cls -ne '#32768' -and $_.Cls -ne 'tooltips_class32' -and $_.Cls -ne 'IME' -and $_.Cls -ne 'MSCTFIME UI' -and $_.Cls -ne 'SysShadow' -and $_.Cls -notlike 'UAC*' } }
function Shot($name, $hwnd, $note, $flags = 2) {
    $path = Join-Path $OutDir $name
    $size = [Shot]::Capture($desk, $hwnd, $path, [uint32]$flags)
    $ws = Wins | Where-Object { $_.H -eq $hwnd } | Select-Object -First 1
    $cls = if ($ws) { $ws.Cls } else { '?' }
    $title = if ($ws) { $ws.Title } else { '?' }
    $script:manifest.Add("$name | $cls | $title | $size | $note")
    Log "  shot $name  [$cls] '$title' $size"
}
function WaitFor([scriptblock]$cond, [int]$ms = 5000) {
    $t = [Diagnostics.Stopwatch]::StartNew()
    while ($t.ElapsedMilliseconds -lt $ms) { $r = & $cond; if ($r) { return $r }; Start-Sleep -Milliseconds 150 }
    return $null
}
function Gone($h) { WaitFor { if (-not [Shot]::IsWindow($h) -or -not [Shot]::IsWindowVisible($h)) { $true } } 3000 }
function CloseWin($h, $id = 2) {
    Post $h $WM_COMMAND $id 0
    $gone = Gone $h
    if (-not $gone) { Post $h $WM_CLOSE 0 0; $gone = Gone $h }
    return [bool]$gone
}
function OpenByCommand($id) {
    $before = @(Dialogs | ForEach-Object { $_.H })
    Post $hMain $WM_COMMAND $id 0
    $w = WaitFor { Dialogs | Where-Object { $before -notcontains $_.H } | Select-Object -First 1 } 4000
    if ($w) { Start-Sleep -Milliseconds 900 }
    return $w
}
function OpenUiSettings() {
    $prefs = OpenByCommand 104
    if (-not $prefs) { Log "Preferences (104) did not open"; return $null }
    $tree = [Shot]::GetDlgItem($prefs.H, $ID_PREF_TREE)
    if ($tree -ne [IntPtr]::Zero) {
        $root = Send $tree $TVM_GETNEXTITEM $TVGN_ROOT 0
        $second = Send $tree $TVM_GETNEXTITEM $TVGN_NEXT $root
        Send $tree $TVM_SELECTITEM $TVGN_CARET $second | Out-Null
        Start-Sleep -Milliseconds 800
        $combo = [Shot]::Descendants($prefs.H, $ID_THEME_COMBO) | Where-Object { $_.Vis } | Select-Object -First 1
        if ($combo) { Log "  theme combo 1110: $(Send $combo.H $CB_GETCOUNT 0 0) items, current $(Send $combo.H $CB_GETCURSEL 0 0) (0 = System)" }
    }
    return $prefs
}
# Windows' mode for apps: 1 = light, 0 = dark; then the broadcast the Settings app makes
function SetWindowsMode($light, $label) {
    Set-ItemProperty -Path $personalize -Name AppsUseLightTheme -Value ([int]$light) -Type DWord
    $r1 = [Shot]::BroadcastImmersive()
    $r2 = [Shot]::SendImmersive($desk, $hMain)
    Log "  AppsUseLightTheme=$([int]$light) ($label); broadcast=$r1 main=$r2"
    Start-Sleep -Seconds 3
}

$original = (Get-ItemProperty -Path $personalize -Name AppsUseLightTheme -ErrorAction SilentlyContinue).AppsUseLightTheme
if ($null -eq $original) { $original = 1 }
Log "AppsUseLightTheme before the test: $original"
if ($original -ne 1) { Log "the test expects a light Windows at the start; setting 1"; Set-ItemProperty -Path $personalize -Name AppsUseLightTheme -Value 1 -Type DWord; [Shot]::BroadcastImmersive() | Out-Null; Start-Sleep -Seconds 2 }

Log "starting $ExePath --demo $DemoCount on $Desktop"
$out = & $RunHidden -NoWait -Desktop $Desktop $ExePath --demo $DemoCount 2>&1
$appPid = [int](($out | ForEach-Object { "$_" } | Where-Object { $_ -match '^\d+$' } | Select-Object -Last 1))
if (-not $appPid) { throw "run_hidden gave no PID: $out" }
Log "pid $appPid"
$desk = [IntPtr]::Zero
for ($i = 0; $i -lt 20 -and $desk -eq [IntPtr]::Zero; $i++) {
    try { $desk = [Shot]::Open($Desktop) } catch { Start-Sleep -Milliseconds 250 }
}
if ($desk -eq [IntPtr]::Zero) { & taskkill /T /F /PID $appPid | Out-Null; throw "could not open desktop $Desktop" }

try {
    $first = WaitFor { TopWins | Where-Object { $_.Cls -like 'RookparcelMainWnd*' -or $_.Cls -eq '#32770' } | Select-Object -First 1 } 20000
    if (-not $first) { throw "no window appeared; windows: $((Wins | ForEach-Object { "[$($_.Cls)] '$($_.Title)' vis=$($_.Vis)" }) -join '; ')" }
    $guide = WaitFor { Dialogs | Where-Object { $_.Cls -eq '#32770' } | Select-Object -First 1 } 4000
    if ($guide) { Log "startup dialog [$($guide.Cls)] '$($guide.Title)' -> closing"; CloseWin $guide.H | Out-Null }
    $main = WaitFor { TopWins | Where-Object { $_.Cls -like 'RookparcelMainWnd*' } | Select-Object -First 1 } 20000
    if (-not $main) { throw "main window not found" }
    $hMain = $main.H
    Log "main [$($main.Cls)] '$($main.Title)' $($main.W)x$($main.Hgt) dpi $([Shot]::GetDpiForWindow($hMain))"
    Start-Sleep -Seconds 4
    $list = [Shot]::GetDlgItem($hMain, $IDC_LIST)
    Key $list $VK_HOME
    Start-Sleep -Milliseconds 700
    Shot '01-system-light-main.png' $hMain 'Theme = System, Windows light: main window at start'

    $prefs = OpenUiSettings
    if ($prefs) { Shot '02-system-light-prefs.png' $prefs.H 'Preferences > UI Settings open before the flip' }

    SetWindowsMode $false 'flip to dark with Preferences open'
    Shot '03-flip-dark-main.png' $hMain 'main window after Windows went dark (Theme = System)'
    if ($prefs -and [Shot]::IsWindow($prefs.H)) { Shot '04-flip-dark-prefs.png' $prefs.H 'the open Preferences after Windows went dark' }

    SetWindowsMode $true 'flip back to light'
    Shot '05-flip-light-main.png' $hMain 'main window after Windows went light again'
    if ($prefs -and [Shot]::IsWindow($prefs.H)) { Shot '06-flip-light-prefs.png' $prefs.H 'the open Preferences after Windows went light again'; CloseWin $prefs.H | Out-Null }

    # a second round with the Add URL prompt open, ending dark, to see a dialog opened after a flip
    SetWindowsMode $false 'flip to dark, no dialog open'
    $d = OpenByCommand 101
    if ($d) { Shot '07-dark-dialog-opened-after-flip.png' $d.H 'Add URL prompt opened after the flip to dark'; SetWindowsMode $true 'flip to light with the prompt open'; Shot '08-flip-light-dialog.png' $d.H 'the open Add URL prompt after Windows went light'; Shot '09-flip-light-main.png' $hMain 'main window, light again'; CloseWin $d.H | Out-Null } else { Log "Add URL (101) did not open"; SetWindowsMode $true 'back to light' }

    foreach ($w in TopWins) { Log "window at end: pid $($w.Pid) [$($w.Cls)] '$($w.Title)' $($w.W)x$($w.Hgt)" }
}
finally {
    Log "stopping pid $appPid"
    & taskkill /T /F /PID $appPid 2>&1 | ForEach-Object { Log "  $_" }
    Set-ItemProperty -Path $personalize -Name AppsUseLightTheme -Value ([int]$original) -Type DWord
    [Shot]::BroadcastImmersive() | Out-Null
    Log "AppsUseLightTheme restored to $original"
    Start-Sleep -Milliseconds 800
    $theme = Get-Content $ini | Where-Object { $_ -match '^\s*ui\.theme\s*=' }
    Log "ini ui.theme line(s) (none expected): $($theme -join ' | ')"
    $manifest.Add("# ini after the run: $($theme -join ' | ')")
    $manifest | Set-Content -Encoding UTF8 (Join-Path $OutDir 'manifest.txt')
    $log | Set-Content -Encoding UTF8 (Join-Path $OutDir 'flip-log.txt')
}

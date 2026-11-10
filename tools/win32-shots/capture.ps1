# capture.ps1 -- screenshots of the Rookparcel Win32 GUI on the hidden desktop (tools/win32-shots).
#
# Run with Windows PowerShell 5.1 (System.Drawing inside Add-Type works there):
#   powershell.exe -NoProfile -ExecutionPolicy Bypass -File capture.ps1 `
#       -ExePath <portable rookparcel.exe> -OutDir <folder for PNGs>
#
# The exe must sit beside a rookparcel.ini (portable mode). The program is started with
# run_hidden.ps1 -NoWait on the AgentHidden desktop with "--demo <DemoCount>", driven by
# posted window messages, pictured with PrintWindow(hwnd, dc, PW_RENDERFULLCONTENT) from a
# thread attached to the hidden desktop, and stopped by its own PID at the end.
param(
    [Parameter(Mandatory = $true)][string]$ExePath,
    [Parameter(Mandatory = $true)][string]$OutDir,
    [int]$DemoCount = 40,
    [string]$Desktop = 'AgentHidden',
    [string]$RunHidden = 'L:\dev\tools\run_hidden\run_hidden.ps1',
    [switch]$SkipMenu,
    [switch]$ResetIni,  # truncate rookparcel.ini first, so the run starts from a fresh portable profile
    [string]$Language,  # with -ResetIni: the fresh ini says ui.language=<code>
    [switch]$Audit,     # list the cut-off texts of every pictured window in audit.txt
    [switch]$NoShots    # no PNGs (an audit run over every language)
)

$ErrorActionPreference = 'Stop'
(Get-Process -Id $PID).PriorityClass = 'BelowNormal'

if (-not (Test-Path $ExePath)) { throw "exe not found: $ExePath" }
$ini = Join-Path (Split-Path $ExePath) 'rookparcel.ini'
if (-not (Test-Path $ini)) { throw "portable ini not found beside the exe: $ini" }
if ($ResetIni) { Set-Content -Path $ini -Value $(if ($Language) { "ui.language=$Language`n" } else { '' }) -NoNewline }
New-Item -ItemType Directory -Force $OutDir | Out-Null
Get-ChildItem $OutDir -File | Where-Object { $_.Extension -eq '.png' -or $_.Name -in 'manifest.txt', 'capture-log.txt' } | Remove-Item -Force

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
    [DllImport("user32.dll")] static extern bool CloseDesktop(IntPtr h);
    [DllImport("user32.dll")] static extern bool SetThreadDesktop(IntPtr h);
    delegate bool EnumProc(IntPtr hwnd, IntPtr lp);
    [DllImport("user32.dll")] static extern bool EnumDesktopWindows(IntPtr desk, EnumProc cb, IntPtr lp);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hwnd, out uint pid);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern bool IsWindow(IntPtr hwnd);
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr hwnd, out RECT r);
    [DllImport("user32.dll")] static extern bool PrintWindow(IntPtr hwnd, IntPtr hdc, uint flags);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern bool PostMessageW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr SendMessageW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr SendMessageTimeoutW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr dlg, int id);
    [DllImport("user32.dll")] public static extern IntPtr GetWindow(IntPtr hwnd, uint cmd);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr ctx);

    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }

    public static void DpiAware() { try { SetProcessDpiAwarenessContext(new IntPtr(-4)); } catch {} }

    public static IntPtr Open(string name) {
        IntPtr h = OpenDesktop(name, 0, false, 0x10000000 /* GENERIC_ALL */);
        if (h == IntPtr.Zero) throw new Exception("OpenDesktop failed: " + Marshal.GetLastWin32Error());
        return h;
    }

    public class Win { public IntPtr H; public string Cls; public string Title; public bool Vis; public int W; public int Hgt; public uint Pid; }

    public static List<Win> Windows(IntPtr desk, uint pid) {
        var list = new List<Win>();
        EnumDesktopWindows(desk, delegate(IntPtr h, IntPtr lp) {
            uint p; GetWindowThreadProcessId(h, out p);
            if (pid != 0 && p != pid) return true;
            var w = new Win(); w.H = h; w.Pid = p;
            var sb = new StringBuilder(256); GetClassName(h, sb, 256); w.Cls = sb.ToString();
            sb = new StringBuilder(512); GetWindowText(h, sb, 512); w.Title = sb.ToString();
            w.Vis = IsWindowVisible(h);
            RECT r; GetWindowRect(h, out r); w.W = r.R - r.L; w.Hgt = r.B - r.T;
            list.Add(w); return true;
        }, IntPtr.Zero);
        return list;
    }

    // ---- the audit of cut-off texts (-Audit) ----
    [DllImport("user32.dll")] static extern bool EnumChildWindows(IntPtr parent, EnumProc cb, IntPtr lp);
    [DllImport("user32.dll", EntryPoint = "GetWindowLongW")] static extern int GetWindowLong(IntPtr h, int idx);
    [DllImport("user32.dll")] static extern uint GetDpiForWindow(IntPtr h);
    [DllImport("user32.dll", CharSet = CharSet.Unicode, EntryPoint = "SendMessageTimeoutW")] static extern IntPtr SendGetText(IntPtr hwnd, uint msg, IntPtr wp, StringBuilder lp, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int DrawTextW(IntPtr dc, string s, int n, ref RECT r, uint flags);
    [DllImport("gdi32.dll")] static extern IntPtr CreateCompatibleDC(IntPtr dc);
    [DllImport("gdi32.dll")] static extern bool DeleteDC(IntPtr dc);
    [DllImport("gdi32.dll")] static extern IntPtr SelectObject(IntPtr dc, IntPtr o);
    [DllImport("gdi32.dll")] static extern bool DeleteObject(IntPtr o);
    [DllImport("gdi32.dll", CharSet = CharSet.Unicode)] static extern IntPtr CreateFontW(int h, int w, int esc, int ori, int weight, uint italic, uint underline, uint strike, uint charset, uint outp, uint clip, uint quality, uint pitch, string face);

    // Every visible label, check box, radio button, push button and group box under `top` whose
    // text needs more room than the control has: "id | class | need WxH, has WxH | text".
    // Measured with Segoe UI 9 pt at the window's DPI (the message font the dialogs use), so a
    // control in another font (bold) is measured a little small. Values with an ellipsis style
    // are skipped: they are meant to be cut.
    public static List<string> Audit(IntPtr top) {
        var found = new List<string>();
        int dpi = (int)GetDpiForWindow(top); if (dpi <= 0) dpi = 96;
        IntPtr dc = CreateCompatibleDC(IntPtr.Zero);
        IntPtr font = CreateFontW(-(9 * dpi + 36) / 72, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, "Segoe UI");
        IntPtr old = SelectObject(dc, font);
        const uint CALC = 0x400, WORDBREAK = 0x10, SINGLE = 0x20, NOPREFIX = 0x800;
        EnumChildWindows(top, delegate(IntPtr h, IntPtr lp) {
            if (!IsWindowVisible(h)) return true;
            var sb = new StringBuilder(256); GetClassName(h, sb, 256); string cls = sb.ToString();
            bool isStatic = String.Equals(cls, "Static", StringComparison.OrdinalIgnoreCase);
            bool isButton = String.Equals(cls, "Button", StringComparison.OrdinalIgnoreCase);
            if (!isStatic && !isButton) return true;
            var tb = new StringBuilder(2048); IntPtr res;
            SendGetText(h, 0x000D, new IntPtr(2048), tb, 2, 2000, out res);
            string text = tb.ToString();
            if (text.Trim().Length == 0) return true;
            RECT wr; GetWindowRect(h, out wr); int w = wr.R - wr.L, hgt = wr.B - wr.T;
            uint style = (uint)GetWindowLong(h, -16);
            int needW = 0, needH = 0; bool cut = false;
            int line = dpi * 15 / 96;
            if (isStatic) {
                uint t = style & 0x1F;
                if ((style & 0xC000) != 0 || t > 2 && t != 0xB && t != 0xC) return true;
                // SS_EDITCONTROL wraps inside a long word (a path) too: DT_EDITCONTROL
                uint f = CALC | ((style & 0x80) != 0 ? NOPREFIX : 0) | (style & 0x2000);
                var r = new RECT(); r.R = w;
                bool wraps = t <= 2 && hgt >= line * 7 / 4;
                needH = DrawTextW(dc, text, -1, ref r, f | (wraps ? WORDBREAK : SINGLE));
                needW = r.R - r.L;
                cut = needW > w || (wraps && needH > hgt + 1);
            } else {
                uint t = style & 0xF;
                int extra;
                if (t == 0 || t == 1) extra = dpi * 8 / 96;                 // push
                else if (t == 7) extra = dpi * 14 / 96;                    // group box title
                else if (t == 2 || t == 3 || t == 4 || t == 5 || t == 6 || t == 9) extra = dpi * 17 / 96;  // check, radio
                else return true;
                bool multi = (style & 0x2000) != 0 && t != 7;
                var r = new RECT(); r.R = Math.Max(1, w - extra);
                needH = DrawTextW(dc, text, -1, ref r, CALC | (multi ? WORDBREAK : SINGLE));
                needW = r.R - r.L + extra;
                cut = needW > w || (multi && needH > hgt + 1);
            }
            if (cut) found.Add(GetWindowLong(h, -12) + " | " + cls + " | need " + needW + "x" + needH + ", has " + w + "x" + hgt + " | " + text.Replace("\r", "").Replace("\n", "\\n"));
            return true;
        }, IntPtr.Zero);
        SelectObject(dc, old); DeleteObject(font); DeleteDC(dc);
        return found;
    }

    public static string SendText(IntPtr hwnd) {
        var sb = new StringBuilder(512); GetWindowText(hwnd, sb, 512); return sb.ToString();
    }

    // PrintWindow from a thread attached to the hidden desktop; returns "WxH" or an error text.
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

# ---- constants -------------------------------------------------------------------------
$WM_CLOSE = 0x10; $WM_KEYDOWN = 0x100; $WM_KEYUP = 0x101; $WM_COMMAND = 0x111; $WM_SYSCOMMAND = 0x112
$WM_CANCELMODE = 0x1F; $SC_KEYMENU = 0xF100
$VK_HOME = 0x24; $VK_LEFT = 0x25; $VK_RIGHT = 0x27
$LVM_GETNEXTITEM = 0x100C; $LVNI_SELECTED = 2
$TCM_GETITEMCOUNT = 0x1304; $TCM_GETCURSEL = 0x130B
$TVM_GETNEXTITEM = 0x110A; $TVM_SELECTITEM = 0x110B; $TVGN_ROOT = 0; $TVGN_NEXT = 1; $TVGN_CHILD = 4; $TVGN_CARET = 9
$IDC_LIST = 12; $IDC_TABS = 13; $ID_PREF_TREE = 900
$IDCANCEL = 2

$manifest = New-Object System.Collections.Generic.List[string]
$manifest.Add("# light-mode baseline, $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')")
$manifest.Add("# exe: $ExePath")
$manifest.Add("# format: png | class | title | size | note")
$log = New-Object System.Collections.Generic.List[string]
$cut = New-Object System.Collections.Generic.List[string]
function Log($s) { $script:log.Add($s); Write-Host $s }

function Post($h, $msg, $wp, $lp) { [Shot]::PostMessageW($h, [uint32]$msg, [IntPtr][int64]$wp, [IntPtr][int64]$lp) | Out-Null }
function Send($h, $msg, $wp, $lp) {
    $r = [IntPtr]::Zero
    [Shot]::SendMessageTimeoutW($h, [uint32]$msg, [IntPtr][int64]$wp, [IntPtr][int64]$lp, 2, 3000, [ref]$r) | Out-Null  # SMTO_ABORTIFHUNG
    return $r.ToInt64()
}
function Key($h, $vk) { Post $h $WM_KEYDOWN $vk 0; Start-Sleep -Milliseconds 40; Post $h $WM_KEYUP $vk 0 }

# every window on the hidden desktop: only this program (and the children it spawns, e.g. the
# remote window of WM_COMMAND 141, a second rookparcel.exe --remote process) lives there
function Wins() { [Shot]::Windows($desk, 0) }
function Shot($name, $hwnd, $note, $flags = 2) {
    $path = Join-Path $OutDir $name
    if ($Audit) { foreach ($l in [Shot]::Audit($hwnd)) { $script:cut.Add("$Language | $name | $l") } }
    $size = if ($NoShots) { 'not pictured' } else { [Shot]::Capture($desk, $hwnd, $path, [uint32]$flags) }
    $ws = Wins | Where-Object { $_.H -eq $hwnd } | Select-Object -First 1
    $cls = if ($ws) { $ws.Cls } else { '?' }
    $title = if ($ws) { $ws.Title } else { '?' }
    $pidNote = if ($ws -and $ws.Pid -ne $appPid) { " (pid $($ws.Pid), child process)" } else { '' }
    $script:manifest.Add("$name | $cls | $title | $size | $note$pidNote")
    Log "  shot $name  [$cls] '$title' $size"
}
function WaitFor([scriptblock]$cond, [int]$ms = 5000) {
    $t = [Diagnostics.Stopwatch]::StartNew()
    while ($t.ElapsedMilliseconds -lt $ms) { $r = & $cond; if ($r) { return $r }; Start-Sleep -Milliseconds 150 }
    return $null
}
function TopWins() { Wins | Where-Object { $_.Vis -and $_.W -gt 0 } }
function Dialogs() { TopWins | Where-Object { $_.Cls -notlike 'RookparcelMainWnd*' -and $_.Cls -ne '#32768' -and $_.Cls -ne 'tooltips_class32' -and $_.Cls -ne 'IME' -and $_.Cls -ne 'MSCTFIME UI' -and $_.Cls -ne 'SysShadow' -and $_.Cls -notlike 'UAC*' } }
function CloseWin($h) {
    Post $h $WM_COMMAND $IDCANCEL 0
    $gone = WaitFor { if (-not [Shot]::IsWindow($h) -or -not [Shot]::IsWindowVisible($h)) { $true } } 2500
    if (-not $gone) { Post $h $WM_CLOSE 0 0; $gone = WaitFor { if (-not [Shot]::IsWindow($h) -or -not [Shot]::IsWindowVisible($h)) { $true } } 2500 }
    return [bool]$gone
}

# ---- start -----------------------------------------------------------------------------
Log "starting $ExePath --demo $DemoCount on $Desktop"
$out = & $RunHidden -NoWait -Desktop $Desktop $ExePath --demo $DemoCount 2>&1
$appPid = [int](($out | ForEach-Object { "$_" } | Where-Object { $_ -match '^\d+$' } | Select-Object -Last 1))
if (-not $appPid) { throw "run_hidden gave no PID: $out" }
Log "pid $appPid"
# the hidden desktop exists only while something runs on it: open it after the start
$desk = [IntPtr]::Zero
for ($i = 0; $i -lt 20 -and $desk -eq [IntPtr]::Zero; $i++) {
    try { $desk = [Shot]::Open($Desktop) } catch { Start-Sleep -Milliseconds 250 }
}
if ($desk -eq [IntPtr]::Zero) { & taskkill /T /F /PID $appPid | Out-Null; throw "could not open desktop $Desktop" }

try {
    # fresh ini: the Setup Guide may come first. Close it with Skip (IDCANCEL).
    $first = WaitFor { TopWins | Where-Object { $_.Cls -like 'RookparcelMainWnd*' -or $_.Cls -eq '#32770' } | Select-Object -First 1 } 20000
    if (-not $first) { throw "no window appeared; windows: $((Wins | ForEach-Object { "[$($_.Cls)] '$($_.Title)' vis=$($_.Vis)" }) -join '; ')" }
    $guide = WaitFor { Dialogs | Where-Object { $_.Cls -eq '#32770' } | Select-Object -First 1 } 4000
    if ($guide) {
        Log "startup dialog [$($guide.Cls)] '$($guide.Title)' -> closing"
        Shot 'startup-setup-guide.png' $guide.H 'shown automatically on a fresh ini; closed with IDCANCEL (Skip)'
        if (-not (CloseWin $guide.H)) { Log "  startup dialog did not close" }
    } else { Log "no startup dialog appeared" }

    $main = WaitFor { TopWins | Where-Object { $_.Cls -like 'RookparcelMainWnd*' } | Select-Object -First 1 } 20000
    if (-not $main) { throw "main window not found; windows: $((Wins | ForEach-Object { "[$($_.Cls)] '$($_.Title)' vis=$($_.Vis)" }) -join '; ')" }
    $hMain = $main.H
    Log "main [$($main.Cls)] '$($main.Title)' $($main.W)x$($main.Hgt)"
    Start-Sleep -Seconds 4   # let the demo torrents land and the first status refresh paint

    # select the first row so the detail tabs have content
    $list = [Shot]::GetDlgItem($hMain, $IDC_LIST)
    $selNote = 'no selection'
    if ($list -ne [IntPtr]::Zero) {
        Key $list $VK_HOME
        Start-Sleep -Milliseconds 700
        $sel = Send $list $LVM_GETNEXTITEM -1 $LVNI_SELECTED
        $selNote = if ($sel -ge 0) { "row $sel selected (VK_HOME to list id 12)" } else { 'VK_HOME posted but no row reported selected' }
    } else { $selNote = 'list control id 12 not found' }
    Log "selection: $selNote"
    Shot 'main.png' $hMain "main window with demo torrents; $selNote"

    # detail tabs: VK_RIGHT on the tab control (id 13) walks them
    $tabs = [Shot]::GetDlgItem($hMain, $IDC_TABS)
    if ($tabs -ne [IntPtr]::Zero) {
        $n = Send $tabs $TCM_GETITEMCOUNT 0 0
        $cur = Send $tabs $TCM_GETCURSEL 0 0
        Log "tabs: $n, current $cur"
        $manifest.Add("main.png | (detail tab index $cur)")
        for ($i = 1; $i -lt $n; $i++) {
            Key $tabs $VK_RIGHT
            Start-Sleep -Milliseconds 900
            $now = Send $tabs $TCM_GETCURSEL 0 0
            Shot "main-tab-$now.png" $hMain "detail tab index $now (VK_RIGHT to tab control id 13)"
        }
        for ($i = 1; $i -lt $n; $i++) { Key $tabs $VK_LEFT; Start-Sleep -Milliseconds 200 }
        Start-Sleep -Milliseconds 600
    } else { Log "tab control id 13 not found" }

    # dialogs by WM_COMMAND id
    $cmds = [ordered]@{
        100 = 'add'; 101 = 'add-url'; 102 = 'create'; 133 = 'properties'; 134 = 'rss';
        141 = 'connect-remote'; 142 = 'notices'; 146 = 'search-sources'; 151 = 'new-profile'; 154 = 'setup-guide'; 156 = 'donate'
    }
    foreach ($e in $cmds.GetEnumerator()) {
        $id = [int]$e.Key; $name = $e.Value
        $before = @(Dialogs | ForEach-Object { $_.H })
        Post $hMain $WM_COMMAND $id 0
        $new = WaitFor { Dialogs | Where-Object { $before -notcontains $_.H } } 4000
        if (-not $new) {
            Log "cmd $id ($name): nothing opened"
            $manifest.Add("(none) | | | | WM_COMMAND $id ($name) opened no window")
            continue
        }
        Start-Sleep -Milliseconds 900   # let it finish painting / loading
        $all = @(Dialogs | Where-Object { $before -notcontains $_.H })
        $k = 0
        foreach ($w in $all) {
            $suffix = if ($k -eq 0) { '' } else { "-$k" }
            Shot "dlg-$name$suffix.png" $w.H "WM_COMMAND $id"
            $k++
        }
        # close newest first (a message box on top of a dialog), then the rest
        [array]::Reverse($all)
        foreach ($w in $all) {
            if ([Shot]::IsWindow($w.H) -and [Shot]::IsWindowVisible($w.H)) {
                if (-not (CloseWin $w.H)) { Log "  '$($w.Title)' did not close on IDCANCEL/WM_CLOSE" }
            }
        }
        Start-Sleep -Milliseconds 500
        # anything that remained (e.g. a confirmation box) -- try once more, then log
        $left = @(Dialogs | Where-Object { $before -notcontains $_.H })
        foreach ($w in $left) { Log "  leftover window [$($w.Cls)] '$($w.Title)'"; CloseWin $w.H | Out-Null }
    }

    # preferences pages
    $before = @(Dialogs | ForEach-Object { $_.H })
    Post $hMain $WM_COMMAND 104 0
    $prefs = WaitFor { Dialogs | Where-Object { $before -notcontains $_.H } | Select-Object -First 1 } 4000
    if ($prefs) {
        Start-Sleep -Milliseconds 800
        $tree = [Shot]::GetDlgItem($prefs.H, $ID_PREF_TREE)
        if ($tree -eq [IntPtr]::Zero) {
            Log "prefs tree id 900 not found"
            Shot 'prefs-0.png' $prefs.H 'Preferences as opened (tree not found)'
        } else {
            $items = New-Object System.Collections.Generic.List[object]
            $root = Send $tree $TVM_GETNEXTITEM $TVGN_ROOT 0
            $it = $root
            while ($it -ne 0) {
                $items.Add(@{ H = $it; Depth = 0 })
                $child = Send $tree $TVM_GETNEXTITEM $TVGN_CHILD $it
                while ($child -ne 0) {
                    $items.Add(@{ H = $child; Depth = 1 })
                    $child = Send $tree $TVM_GETNEXTITEM $TVGN_NEXT $child
                }
                $it = Send $tree $TVM_GETNEXTITEM $TVGN_NEXT $it
            }
            Log "prefs tree: $($items.Count) items"
            $n = 0
            foreach ($item in $items) {
                $n++
                Send $tree $TVM_SELECTITEM $TVGN_CARET $item.H | Out-Null
                Start-Sleep -Milliseconds 700
                Shot ("prefs-{0:d2}.png" -f $n) $prefs.H "Preferences page $n (tree item 0x$('{0:X}' -f $item.H), depth $($item.Depth))"
            }
        }
        if (-not (CloseWin $prefs.H)) { Log "  Preferences did not close" }
    } else { Log "Preferences (104) did not open" }

    # File menu via SC_KEYMENU 'F'
    if (-not $SkipMenu) {
        Post $hMain $WM_SYSCOMMAND $SC_KEYMENU ([int][char]'F')
        $menu = WaitFor { TopWins | Where-Object { $_.Cls -eq '#32768' } | Select-Object -First 1 } 2500
        if ($menu) {
            Start-Sleep -Milliseconds 400
            Shot 'menu-file.png' $menu.H 'File menu (WM_SYSCOMMAND SC_KEYMENU F)' 0
            Shot 'menu-file-main.png' $hMain 'main window while the File menu is open' 2
        } else { Log "no popup menu appeared" ; $manifest.Add("(none) | | | | SC_KEYMENU F opened no #32768 window") }
        Send $hMain $WM_CANCELMODE 0 0 | Out-Null
        Start-Sleep -Milliseconds 300
    }

    # final state of all top-level windows
    foreach ($w in TopWins) { Log "window at end: pid $($w.Pid) [$($w.Cls)] '$($w.Title)' $($w.W)x$($w.Hgt)" }
}
finally {
    Log "stopping pid $appPid"
    & taskkill /T /F /PID $appPid 2>&1 | ForEach-Object { Log "  $_" }
    $manifest | Set-Content -Encoding UTF8 (Join-Path $OutDir 'manifest.txt')
    $log | Set-Content -Encoding UTF8 (Join-Path $OutDir 'capture-log.txt')
    if ($Audit) { $cut | Select-Object -Unique | Set-Content -Encoding UTF8 (Join-Path $OutDir 'audit.txt') }
}

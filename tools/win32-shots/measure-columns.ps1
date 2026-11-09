# measure-columns.ps1 -- torrent list column widths and header geometry on the hidden desktop.
#
# Windows PowerShell 5.1:
#   powershell.exe -NoProfile -ExecutionPolicy Bypass -File measure-columns.ps1 `
#       -SourceExe <rookparcel.exe> -WorkDir <folder> -IniText '' -Label light
# Copies the exe into WorkDir beside a rookparcel.ini holding IniText, starts it with --demo
# <DemoCount> through run_hidden.ps1 -NoWait, waits for the main window, then reads (twice,
# 3 s apart): LVM_GETCOLUMNWIDTH for columns 0..12 of list id 12, LVM_GETHEADER + HDM_GETITEMCOUNT,
# GetWindowRect of list and header, GetDpiForWindow(main), GetSystemMetrics(SM_CXVSCROLL).
# Stops the program by its own PID tree.
param(
    [Parameter(Mandatory = $true)][string]$SourceExe,
    [Parameter(Mandatory = $true)][string]$WorkDir,
    [string]$IniText = '',
    [string]$Label = 'run',
    [int]$DemoCount = 40,
    [int]$FirstReadAfterSec = 3,
    [int]$SecondReadAfterSec = 3,
    [string]$Desktop = 'AgentHidden',
    [string]$RunHidden = 'L:\dev\tools\run_hidden\run_hidden.ps1'
)
$ErrorActionPreference = 'Stop'
(Get-Process -Id $PID).PriorityClass = 'BelowNormal'
if (-not (Test-Path $SourceExe)) { throw "exe not found: $SourceExe" }
New-Item -ItemType Directory -Force $WorkDir | Out-Null
$ExePath = Join-Path $WorkDir 'rookparcel.exe'
Copy-Item $SourceExe $ExePath -Force
[IO.File]::WriteAllText((Join-Path $WorkDir 'rookparcel.ini'), $IniText, [Text.Encoding]::ASCII)

Add-Type -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public static class Meas {
    [DllImport("user32.dll", SetLastError = true)] static extern IntPtr OpenDesktop(string name, uint flags, bool inherit, uint access);
    delegate bool EnumProc(IntPtr hwnd, IntPtr lp);
    [DllImport("user32.dll")] static extern bool EnumDesktopWindows(IntPtr desk, EnumProc cb, IntPtr lp);
    [DllImport("user32.dll")] static extern uint GetWindowThreadProcessId(IntPtr hwnd, out uint pid);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetClassName(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr hwnd, StringBuilder sb, int n);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr hwnd, out RECT r);
    [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr hwnd, out RECT r);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern bool PostMessageW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp);
    [DllImport("user32.dll", CharSet = CharSet.Unicode)] public static extern IntPtr SendMessageTimeoutW(IntPtr hwnd, uint msg, IntPtr wp, IntPtr lp, uint flags, uint timeout, out IntPtr result);
    [DllImport("user32.dll")] public static extern IntPtr GetDlgItem(IntPtr dlg, int id);
    [DllImport("user32.dll")] public static extern uint GetDpiForWindow(IntPtr hwnd);
    [DllImport("user32.dll")] public static extern int GetSystemMetrics(int n);
    [DllImport("user32.dll")] public static extern int GetSystemMetricsForDpi(int n, uint dpi);
    [DllImport("user32.dll")] static extern bool SetProcessDpiAwarenessContext(IntPtr ctx);
    [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
    public static void DpiAware() { try { SetProcessDpiAwarenessContext(new IntPtr(-4)); } catch {} }
    public static IntPtr Open(string name) {
        IntPtr h = OpenDesktop(name, 0, false, 0x10000000);
        if (h == IntPtr.Zero) throw new Exception("OpenDesktop failed: " + Marshal.GetLastWin32Error());
        return h;
    }
    public class Win { public IntPtr H; public string Cls; public string Title; public bool Vis; public uint Pid; }
    public static List<Win> Windows(IntPtr desk) {
        var list = new List<Win>();
        EnumDesktopWindows(desk, delegate(IntPtr h, IntPtr lp) {
            var w = new Win(); w.H = h; GetWindowThreadProcessId(h, out w.Pid);
            var sb = new StringBuilder(256); GetClassName(h, sb, 256); w.Cls = sb.ToString();
            sb = new StringBuilder(512); GetWindowText(h, sb, 512); w.Title = sb.ToString();
            w.Vis = IsWindowVisible(h); list.Add(w); return true;
        }, IntPtr.Zero);
        return list;
    }
}
'@
[Meas]::DpiAware()

function Send($h, $msg, $wp, $lp) {
    $r = [IntPtr]::Zero
    $ok = [Meas]::SendMessageTimeoutW($h, [uint32]$msg, [IntPtr][int64]$wp, [IntPtr][int64]$lp, 2, 3000, [ref]$r)
    if ($ok -eq [IntPtr]::Zero) { return $null }
    return $r.ToInt64()
}
function Rect($h) { $r = New-Object Meas+RECT; [Meas]::GetWindowRect($h, [ref]$r) | Out-Null; "L=$($r.L) T=$($r.T) R=$($r.R) B=$($r.B) ($($r.R-$r.L)x$($r.B-$r.T))" }
function CRect($h) { $r = New-Object Meas+RECT; [Meas]::GetClientRect($h, [ref]$r) | Out-Null; "$($r.R)x$($r.B)" }

$LVM_GETHEADER = 0x101F; $LVM_GETCOLUMNWIDTH = 0x101D; $LVM_GETITEMCOUNT = 0x1004; $LVM_GETCOUNTPERPAGE = 0x1028
$HDM_GETITEMCOUNT = 0x1200; $SM_CXVSCROLL = 2
$WM_GETFONT = 0x31

Write-Host "=== $Label : ini = '$($IniText -replace "`n",'\n')'"
$out = & $RunHidden -NoWait -Desktop $Desktop $ExePath --demo $DemoCount 2>&1
$appPid = [int](($out | ForEach-Object { "$_" } | Where-Object { $_ -match '^\d+$' } | Select-Object -Last 1))
if (-not $appPid) { throw "run_hidden gave no PID: $out" }
Write-Host "pid $appPid"
$desk = [IntPtr]::Zero
for ($i = 0; $i -lt 20 -and $desk -eq [IntPtr]::Zero; $i++) { try { $desk = [Meas]::Open($Desktop) } catch { Start-Sleep -Milliseconds 250 } }
if ($desk -eq [IntPtr]::Zero) { & taskkill /T /F /PID $appPid | Out-Null; throw "could not open desktop" }
try {
    $sw = [Diagnostics.Stopwatch]::StartNew(); $main = $null
    while ($sw.ElapsedMilliseconds -lt 20000 -and -not $main) {
        $main = [Meas]::Windows($desk) | Where-Object { $_.Pid -eq $appPid -and $_.Vis -and $_.Cls -like 'RookparcelMainWnd*' } | Select-Object -First 1
        if (-not $main) { Start-Sleep -Milliseconds 150 }
    }
    if (-not $main) { throw "main window not found" }
    Write-Host "main [$($main.Cls)] '$($main.Title)' appeared after $($sw.ElapsedMilliseconds) ms"
    $hMain = $main.H
    $list = [Meas]::GetDlgItem($hMain, 12)
    if ($list -eq [IntPtr]::Zero) { throw "list id 12 not found" }
    Write-Host ("GetDpiForWindow(main) = {0}; GetSystemMetrics(SM_CXVSCROLL) = {1} (ForDpi: {2})" -f [Meas]::GetDpiForWindow($hMain), [Meas]::GetSystemMetrics($SM_CXVSCROLL), [Meas]::GetSystemMetricsForDpi($SM_CXVSCROLL, [Meas]::GetDpiForWindow($hMain)))
    Write-Host "main rect: $(Rect $hMain)  client $(CRect $hMain)"
    foreach ($pass in 1, 2) {
        Start-Sleep -Seconds $(if ($pass -eq 1) { $FirstReadAfterSec } else { $SecondReadAfterSec })
        $hdr = [IntPtr](Send $list $LVM_GETHEADER 0 0)
        $cols = @()
        for ($c = 0; $c -le 12; $c++) {
            $w = Send $list $LVM_GETCOLUMNWIDTH $c 0
            if ($null -eq $w -or $w -le 0) { break }
            $cols += $w
        }
        $hcount = if ($hdr -ne [IntPtr]::Zero) { Send $hdr $HDM_GETITEMCOUNT 0 0 } else { '?' }
        Write-Host ("--- read {0} at +{1:n1} s" -f $pass, $sw.Elapsed.TotalSeconds)
        Write-Host "  column widths (0..): $($cols -join ', ')   [count $($cols.Count), sum $(($cols | Measure-Object -Sum).Sum)]"
        Write-Host "  header hwnd 0x$('{0:X}' -f $hdr.ToInt64()), HDM_GETITEMCOUNT = $hcount"
        Write-Host "  list rect:   $(Rect $list)  client $(CRect $list)"
        if ($hdr -ne [IntPtr]::Zero) { Write-Host "  header rect: $(Rect $hdr)  client $(CRect $hdr)" }
        Write-Host "  items $(Send $list $LVM_GETITEMCOUNT 0 0), per page $(Send $list $LVM_GETCOUNTPERPAGE 0 0); list WM_GETFONT 0x$('{0:X}' -f (Send $list $WM_GETFONT 0 0)), header WM_GETFONT 0x$('{0:X}' -f (Send $hdr $WM_GETFONT 0 0))"
    }
}
finally {
    & taskkill /T /F /PID $appPid 2>&1 | ForEach-Object { Write-Host "  $_" }
}

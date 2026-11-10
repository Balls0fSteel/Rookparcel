# Win32 screenshots on the hidden desktop

`capture.ps1` pictures the Windows program without touching the user's screen: it starts a
portable copy with `--demo 40` on the hidden desktop (`L:\dev\tools\run_hidden\run_hidden.ps1`,
desktop `AgentHidden`), drives it with posted window messages and saves one PNG per view through
`PrintWindow`. Written for the dark-mode work (D64), where every slice's light output had to
equal the light output before it, pixel for pixel.

```powershell
powershell.exe -NoProfile -ExecutionPolicy Bypass -File tools\win32-shots\capture.ps1 `
    -ExePath C:\scratch\light\rookparcel.exe -OutDir C:\scratch\shots\light -ResetIni
```

Windows PowerShell 5.1 (the `System.Drawing` type lives inside `Add-Type`). The exe must sit
beside a `rookparcel.ini` (portable mode); `-ResetIni` truncates it so every run starts from the
same fresh profile, or write `ui.theme=dark` into it for a dark run. Parameters: `-DemoCount`
(40), `-Desktop` (`AgentHidden`), `-RunHidden` (the tool's path), `-SkipMenu`.

What it saves: `main.png` (row 0 selected, the General tab), `main-tab-1..6.png` (the other
detail tabs), `dlg-*.png` (every dialog the main window opens by `WM_COMMAND`: Add's file picker,
Add URL, Create, Properties, RSS, Connect to Remote, Notices, Search Sources, New Profile, Setup
Guide, Donate, Statistics), `prefs-01..13.png` (every Preferences page through the tree), `menu-file.png`, and
`manifest.txt` (file, class, title, size, note) plus `capture-log.txt`.

**Cut-off texts:** `-Audit` lists, in `audit.txt`, every visible label, check box, radio button,
push button and group box title of each pictured window whose text needs more room than the
control has (`language | view | control id | class | need, has | text`); `-Language <code>` with
`-ResetIni` starts the program in that language, `-NoShots` skips the PNGs. One run per language
(about 50 s each; a second desktop name lets it run beside other captures):

```powershell
foreach ($l in (Get-ChildItem crates\frontend\lang\*.lng).BaseName) {
    powershell.exe -NoProfile -ExecutionPolicy Bypass -File tools\win32-shots\capture.ps1 `
        -ExePath C:\scratch\app\rookparcel.exe -OutDir C:\scratch\audit\$l `
        -ResetIni -Language $l -Audit -NoShots -SkipMenu -DemoCount 5 -Desktop AgentAudit
}
```

It measures with Segoe UI 9 pt at the window's DPI, sees only what the script opens (texts that
depend on state, such as the General page's note about another program, show only when that
state exists), and always reports the system file picker's `Files of &type:` (a hidden label of
Windows' own dialog).

Comparing two runs (Pillow in the Python environment):

```python
from PIL import Image, ImageChops
d = ImageChops.difference(Image.open(a).convert("RGB"), Image.open(b).convert("RGB"))
print(d.getbbox())  # None: identical
```

Expected differences between runs of the same look: the caption's version text (x 123-202,
y 11-20 at 100 %), the Logger's lines, the RSS Downloader's feed list, a blinking caret, and a
help pane or detail panel caught mid-paint; anything else is a change. Window corners are black
in the PNGs (DWM's rounding lies outside the surface), the same in every run. Connect to Remote
starts a second `rookparcel.exe --remote`; the script enumerates every window on the hidden
desktop and stops the whole process tree at the end.

Beside it: `switch-test.ps1` starts a dark build, switches Preferences > UI Settings > Theme
(control 1110: System 0, Light 1, Dark 2) to Light and back by posted messages and pictures the
main window and a dialog after each switch; `measure-columns.ps1` reads the torrent list's column
widths, header, DPI and rectangles twice, for a run that looks different without an obvious
cause; `compare.py` prints the differing pixel count and the clustered bounding boxes of two PNGs;
`flip-test.ps1` is for a test machine or VM only: with Theme = System it writes Windows' own
`AppsUseLightTheme` value (dark, light, dark, light), broadcasts `WM_SETTINGCHANGE
"ImmersiveColorSet"` as the Settings app does and pictures the main window, the open Preferences
and the Add URL prompt after each flip, then puts the value back. On the Windows 11 VM the
scripts run in the console session through `tools/windows-clean/desktop/in-session.ps1` with
`-RunHidden` pointing at a copy of run_hidden.ps1; 150 % and 200 % need a screen mode bigger than
1024x768 first (`SPI_SETLOGICALDPIOVERRIDE` offers 125 % at most there).

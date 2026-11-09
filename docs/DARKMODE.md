# Win32 dark mode: design

Status: design only (D56, step H; theme pass 2026-11-09). No code of it has landed. The Web
UI's themes (D63, [WEBUI.md](WEBUI.md) "Themes"), GTK's and macOS's dark handling and the
shared `frontend::palette` are in place; this document is what the Win32 GUI still needs.

## 1. Goal and non-goals

- A **Theme** preference on Preferences > UI Settings: **System** (Windows' *Choose your mode*
  for apps), **Light**, **Dark**; applied live, including to a change of the Windows setting.
- Dark: the title bar, menus, toolbar, status bar, the torrent list and every list and tree
  view, the detail tabs, the category tree, splitters, every dialog (Preferences included) and
  its controls, the drawn parts (bars, piece map, speed graph, scheduler grid) and the dark
  icon set that already exists.
- **High Contrast** (`SPI_GETHIGHCONTRAST`) wins over the preference: the GUI stays with the
  system colours it uses today, as it should.
- Non-goals: colour themes beyond light and dark on Win32 (icon skins and a colour file for the
  drawn parts stay in [BACKLOG](BACKLOG.md#themes-researched-2026-11-02_110639_1793613999)),
  Windows 7/8 looks, anything that needs a new dependency (D4).

## 2. What Windows provides

| Need | Facility | Notes |
|---|---|---|
| Dark title bar | `DwmSetWindowAttribute(hwnd, DWMWA_USE_IMMERSIVE_DARK_MODE, &TRUE)` | Attribute 20 from Windows 10 20H1 (19041), 19 on 1809–1909. Needs the `Win32_Graphics_Dwm` feature of `windows-sys` (`crates/win32/Cargo.toml`). Documented. |
| Read the setting | Registry `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`, `AppsUseLightTheme` (DWORD 0 = dark) | Documented enough; `WM_SETTINGCHANGE` with `lParam` `"ImmersiveColorSet"` announces a change. `uxtheme` ordinal 132 `ShouldAppsUseDarkMode` says the same. |
| Dark common controls | `SetWindowTheme(h, L"DarkMode_Explorer", NULL)` on list views, tree views, scroll bars, edit boxes, buttons, tooltips; `DarkMode_CFD` on combo boxes; `DarkMode_ItemsView` for header controls in some builds | Undocumented theme class names; stable since 1809 and used by Explorer, Notepad++, WinMerge, Chromium. |
| Dark popup menus and context menus | `uxtheme` ordinals 135 `SetPreferredAppMode(AllowDark \| ForceDark)` (133 `AllowDarkModeForApp` before 1903) **before any window is created**, 133 `AllowDarkModeForWindow(hwnd, TRUE)` per top-level window, 136 `FlushMenuThemes`, 104 `RefreshImmersiveColorPolicyState` | Undocumented ordinals; load `uxtheme.dll` by hand, feature-test each export, and guard on the build number (17763 or later); fall back to light when any is missing. |
| Menu bar | `WM_UAHDRAWMENU` (0x91) and `WM_UAHDRAWMENUITEM` (0x92), owner-drawing the bar over `GetMenuBarInfo` rectangles | Undocumented messages; the popups come from `SetPreferredAppMode`. |
| Dialog surfaces | `WM_CTLCOLORDLG`, `WM_CTLCOLORSTATIC`, `WM_CTLCOLOREDIT`, `WM_CTLCOLORBTN`, `WM_CTLCOLORLISTBOX` returning a dark brush and `SetTextColor`/`SetBkColor` | Documented. Group boxes and check-box/radio texts need `NM_CUSTOMDRAW` or owner draw for their captions; tabs need owner draw (`TCS_OWNERDRAWFIXED`). |
| Theme change at run time | `WM_SETTINGCHANGE("ImmersiveColorSet")`, `WM_THEMECHANGED`, `WM_SYSCOLORCHANGE` | Re-read the setting, call `AllowDarkModeForWindow` again, `SetWindowTheme` again, `RefreshImmersiveColorPolicyState`, `FlushMenuThemes`, invalidate everything. |

## 3. Palette

- **Drawn parts** take `frontend::palette::DrawPalette::for_dark(dark)` (`DARK` is already
  defined and used by GTK): the Done column bar (`app.rs` `list_custom_draw`), the detail
  panels (`details.rs`: piece bars, piece map, block bars, speed graph), the scheduler grid
  (`sched_grid.rs`), all of which read `palette::LIGHT` today.
- **System colours** get a Win32 `SysPalette { window, window_text, gray_text, highlight,
  highlight_text, button_face, button_text, shadow, header }` resolved once per theme: the
  `GetSysColor` values when light, fixed dark values when dark (window `#202020`, text
  `#e0e0e0`, grey `#9a9a9a`, button face `#2b2b2b`, highlight `#2d4f7a`, shadow `#3a3d42`,
  header `#262626`; aligned with the Web UI's dark tokens). Every `GetSysColor(...)` site goes
  through it: `controls.rs` `row_background` (the 15/16 darkening becomes a lightening on dark),
  `selection_colours`, `details.rs` panel background/text/frames, `sched_grid.rs`,
  `setup_guide.rs`, `search_helper.rs`, the Files tab statics in `app.rs`.
- **Icons**: `crates/win32/src/icons.rs` passes `dark` into `Icon::png(size, dark)` and
  `themed_png(size, dark)`; `toolbar_bgra(icon, size, gray)` gains the flag; the menu bitmap
  cache key gains it. The dark PNG sets exist (`assets/ui-icon-explorations/png-dark`). The
  tray icon stays the coloured Postal Rook.

## 4. Control by control

| Control | Technique |
|---|---|
| Main window, dialogs | `WM_ERASEBKGND`/class brush from `SysPalette.window`; `WM_CTLCOLOR*` for children; dark title bar by DWM. |
| List views (main list, detail tabs, Add, RSS, search, Advanced) | `SetWindowTheme(DarkMode_Explorer)` for the scroll bars and the frame; `NM_CUSTOMDRAW` already paints rows (`controls.rs`): colours from `SysPalette`; the header control through `SetWindowTheme(DarkMode_ItemsView)` plus `NM_CUSTOMDRAW` on the header for text and dividers. |
| Tree views (categories, Preferences tree, Files tab tree) | `SetWindowTheme(DarkMode_Explorer)`, `TVM_SETBKCOLOR`, `TVM_SETTEXTCOLOR`, `TVM_SETLINECOLOR`. |
| Toolbar | `NM_CUSTOMDRAW` with `TBCDRF_*` flags for text and hot colours, `TBSTYLE_CUSTOMERASE` for the background; dark toolbar images. |
| Status bar | `SBT_OWNERDRAW` parts drawn over a `SysPalette` background (today a stock control). |
| Menus | `SetPreferredAppMode` for the popups; `WM_UAHDRAWMENU`/`WM_UAHDRAWMENUITEM` for the bar; menu bitmaps from the dark set. |
| Tabs (detail tabs, Properties) | `TCS_OWNERDRAWFIXED` and `WM_DRAWITEM`, or hide the tab control's frame and draw the strip in the panel. |
| Buttons, check boxes, radios | `SetWindowTheme(DarkMode_Explorer)` gives dark push buttons; check and radio captions need `WM_CTLCOLORSTATIC`-style text (set `BS_OWNERDRAW` only where the theme leaves them black). |
| Edit boxes, combo boxes, spin buttons | `SetWindowTheme(DarkMode_CFD)` on combos, `DarkMode_Explorer` on edits; `WM_CTLCOLOREDIT`. |
| Group boxes | Owner-drawn caption and frame (`WM_PAINT` subclass): the themed group box ignores text colour. |
| Progress bars (Add, Setup Guide) | `PBS_SMOOTH` with `PBM_SETBARCOLOR`/`PBM_SETBKCOLOR`, which only work with the theme off (`SetWindowTheme(h, L"", L"")`). |
| Scroll bars elsewhere | Come with `DarkMode_Explorer` on their parent. |
| Tooltips | `SetWindowTheme(DarkMode_Explorer)`. |
| Splitters, panels | Already our own painting; `SysPalette`. |

## 5. Preference and plumbing

- `ui.theme` (`system` \| `light` \| `dark`) in `frontend::options::GuiOptions` (the
  `ui.language` pattern in `options.rs` load/store), a `Choice` on the shared UI Settings page
  in `frontend/src/pref_pages.rs` (`language_choices` is the model) gated by a `Features` flag
  so GTK and macOS, which follow the desktop, do not show it; the Win32 dialog needs a combo
  `Field` kind in `win32/src/preferences.rs` (`ID_LANG` is the template) and one more row in
  page 1's Display group.
- Apply path: `app.rs` `apply_options_from` resolves the effective mode (preference, Windows
  setting, High Contrast), swaps `SysPalette`, re-themes every window (section 2, last row) and
  reloads the icon caches; the same on `WM_SETTINGCHANGE("ImmersiveColorSet")` when the
  preference is System.
- Help text `PrefTheme` exists for the Web UI's picker; Win32 reuses the key with a Win32 note,
  or adds `PrefThemeDesktop` if the wording must differ.

## 6. Risks and mitigations

- **Undocumented ordinals** (133/135/136/104, `DarkMode_*` class names, `WM_UAH*`) can change
  with a Windows build: load them by hand, check every export, guard on the build number, and
  fall back to light with no crash; keep the feature behind the preference so a broken build
  can be told to stay light.
- `SetPreferredAppMode` must run before the first window; on a later preference change the
  popups follow only after `FlushMenuThemes`.
- GDI text on dark backgrounds: ClearType looks heavier; use `SetBkMode(TRANSPARENT)` over an
  opaque fill, never alpha-blended text.
- A switch while dialogs are open repaints in pieces; invalidate with `RDW_ALLCHILDREN` and
  accept one flash.
- High Contrast must bypass all of it (system colours, no DWM attribute).
- Store builds (D46) run the same code; nothing here needs a capability.

## 7. Slices

1. Preference, `SysPalette`, dark title bar, `WM_SETTINGCHANGE`; everything else still light
   (the window reads mixed on purpose for one slice).
2. Main list, header, category tree, splitters, status bar.
3. Toolbar, menus (popups then the bar), dark icons.
4. Detail tabs: tab strip, panels, the drawn parts through `DrawPalette::DARK`.
5. Dialogs: Add, Properties, Create, RSS, Setup Guide, search; the `WM_CTLCOLOR*` family.
6. Preferences: tree, every page, the scheduler grid.
7. Live switching and High Contrast passes; hidden-desktop screenshots at 100/150/200 %.
8. Hand check on a real Windows 11 and Windows 10 1809 (the oldest build with the ordinals).

## 8. Acceptance

Every window and dialog readable in dark at 100–200 % DPI with no white flashes larger than a
control; the Windows setting followed within a second; High Contrast unchanged from today;
light mode pixel-identical to 1.2.21's screenshots; no new dependency; the `uxtheme` exports
missing (simulated) leaves a light, working program.

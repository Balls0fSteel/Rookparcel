//! The colours of the parts the GUIs draw themselves: the General tab's piece bars, the Pieces
//! tab's map and Blocks cells, the Speed graph and the scheduler's table. The toolkit's theme
//! colours (text, frames, hints) stay the toolkit's; these are the few fixed colours drawn on
//! top of them, one set per light and per dark surface, so a dark theme gets no white boxes.

use crate::scheduler::Mode;

/// Colours of the parts the GUIs draw themselves (piece bars, piece map, block bars, the speed
/// graph, the scheduler grid), in `0xRRGGBB`, one set per light and dark surface.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DrawPalette {
    /// The page behind the drawn widgets (the scheduler's Turn off cell).
    pub surface: u32,
    /// The empty part of a bar, the piece map's frame fill, the Blocks cell's free blocks.
    pub bar_bg: u32,
    /// A bar's one-pixel frame where the toolkit gives no frame colour.
    pub bar_frame: u32,
    /// Text over a bar (the torrent list's Done column).
    pub bar_fg: u32,
    /// Pieces on disk: the Downloaded bar, the Done bar while downloading, finished blocks.
    pub bar_blue: u32,
    /// The Availability bar, the Done bar once complete, blocks being written.
    pub bar_green: u32,
    /// Blocks requested and pieces being downloaded.
    pub block_requested: u32,
    /// The speed graph's download line.
    pub graph_down: u32,
    /// The speed graph's upload line.
    pub graph_up: u32,
    /// The speed graph's horizontal grid lines.
    pub grid_line: u32,
    /// The piece map's shades for pieces 0, 1, 2–3 and 4 or more peers have.
    pub availability: [u32; 4],
}

/// The colours on a light surface: what every client drew before there was a palette.
pub const LIGHT: DrawPalette = DrawPalette {
    surface: 0xffffff,
    bar_bg: 0xffffff,
    bar_frame: 0xa0a0a0,
    bar_fg: 0x000000,
    bar_blue: 0x3c78d8,
    bar_green: 0x4bb543,
    block_requested: 0xe8a000,
    graph_down: 0x2aa52a,
    graph_up: 0xd23c3c,
    grid_line: 0xe0e0e0,
    availability: [0xffffff, 0xd8e8d0, 0xa8d29a, 0x6fb85c],
};

/// The colours on a dark surface: the same hues, lifted so they read on a dark page, and white
/// text over the bars (3:1 or better over each bar colour, see the tests).
pub const DARK: DrawPalette = DrawPalette {
    surface: 0x1e1f22,
    bar_bg: 0x2b2d31,
    bar_frame: 0x5a5e66,
    bar_fg: 0xffffff,
    bar_blue: 0x4b8fe0,
    bar_green: 0x3f9e55,
    block_requested: 0xd99a1e,
    graph_down: 0x4fc04f,
    graph_up: 0xe06060,
    grid_line: 0x3a3d42,
    availability: [0x2b2d31, 0x2f4a33, 0x3b7a44, 0x4caf64],
};

impl DrawPalette {
    /// The palette for a dark (`true`) or light surface.
    pub fn for_dark(dark: bool) -> &'static DrawPalette {
        if dark { &DARK } else { &LIGHT }
    }

    /// The scheduler cell colour for `m`: µTorrent's table ([`Mode::rgb`]) for Full speed,
    /// Limited and Seeding only; the surface colour for Turn off (never literal white).
    pub fn sched_cell(&self, m: Mode) -> u32 {
        match m {
            Mode::Off => self.surface,
            _ => m.rgb(),
        }
    }

    /// `color` at strength `v` (0..=1) over [`bar_bg`](Self::bar_bg): the piece bars' partial
    /// steps (what the GUIs blended toward white).
    pub fn blend(&self, color: u32, v: f32) -> u32 {
        let v = v.clamp(0.0, 1.0);
        let mix = |shift: u32| {
            let bg = ((self.bar_bg >> shift) & 0xff) as f32;
            let ch = ((color >> shift) & 0xff) as f32;
            ((bg + (ch - bg) * v) as u32) << shift
        };
        mix(16) | mix(8) | mix(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WCAG 2 relative luminance of an `0xRRGGBB` colour.
    fn luminance(c: u32) -> f64 {
        let lin = |shift: u32| {
            let x = f64::from((c >> shift) & 0xff) / 255.0;
            if x <= 0.03928 { x / 12.92 } else { ((x + 0.055) / 1.055).powf(2.4) }
        };
        0.2126 * lin(16) + 0.7152 * lin(8) + 0.0722 * lin(0)
    }

    /// WCAG 2 contrast ratio between two colours (1 to 21).
    fn contrast(a: u32, b: u32) -> f64 {
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }

    #[test]
    fn every_field_differs_between_light_and_dark() {
        // nothing is intentionally equal: a dark page shares no colour with a light one
        assert_ne!(LIGHT.surface, DARK.surface);
        assert_ne!(LIGHT.bar_bg, DARK.bar_bg);
        assert_ne!(LIGHT.bar_frame, DARK.bar_frame);
        assert_ne!(LIGHT.bar_fg, DARK.bar_fg);
        assert_ne!(LIGHT.bar_blue, DARK.bar_blue);
        assert_ne!(LIGHT.bar_green, DARK.bar_green);
        assert_ne!(LIGHT.block_requested, DARK.block_requested);
        assert_ne!(LIGHT.graph_down, DARK.graph_down);
        assert_ne!(LIGHT.graph_up, DARK.graph_up);
        assert_ne!(LIGHT.grid_line, DARK.grid_line);
        for i in 0..4 {
            assert_ne!(LIGHT.availability[i], DARK.availability[i], "availability[{i}]");
        }
    }

    #[test]
    fn for_dark_picks_the_set() {
        assert_eq!(*DrawPalette::for_dark(false), LIGHT);
        assert_eq!(*DrawPalette::for_dark(true), DARK);
    }

    #[test]
    fn the_off_cell_is_the_surface_and_the_others_are_utorrents() {
        for pal in [&LIGHT, &DARK] {
            assert_eq!(pal.sched_cell(Mode::Off), pal.surface);
            for m in [Mode::Full, Mode::Limited, Mode::SeedOnly] {
                assert_eq!(pal.sched_cell(m), m.rgb());
            }
        }
        // the light surface is white, so the Win32 client's pixels do not change
        assert_eq!(LIGHT.sched_cell(Mode::Off), 0xffffff);
        assert_ne!(DARK.sched_cell(Mode::Off), 0xffffff);
    }

    #[test]
    fn blend_runs_from_the_background_to_the_colour() {
        for pal in [&LIGHT, &DARK] {
            for c in [0x3c78d8, 0x4bb543, 0x000000, 0xffffff, 0x123456] {
                assert_eq!(pal.blend(c, 1.0), c, "{c:06x} at 1");
                assert_eq!(pal.blend(c, 0.0), pal.bar_bg, "{c:06x} at 0");
                // beyond the range: clamped, not wrapped
                assert_eq!(pal.blend(c, 2.0), c);
                assert_eq!(pal.blend(c, -1.0), pal.bar_bg);
            }
        }
        // over white, the old formula: 255 - (255 - channel) * v, truncated
        let old = |c: u32, v: f32| {
            let mix = |shift: u32| {
                let ch = ((c >> shift) & 0xff) as f32;
                ((255.0 - (255.0 - ch) * v) as u32) << shift
            };
            mix(16) | mix(8) | mix(0)
        };
        for v in [0.1, 0.25, 0.333, 0.5, 0.75, 0.9] {
            assert_eq!(LIGHT.blend(0x3c78d8, v), old(0x3c78d8, v), "v = {v}");
            assert_eq!(LIGHT.blend(0x4bb543, v), old(0x4bb543, v), "v = {v}");
        }
        // halfway toward a dark background sits between the two
        let half = DARK.blend(0xffffff, 0.5);
        assert!(half > DARK.bar_bg && half < 0xffffff);
    }

    #[test]
    fn bar_text_reads_over_the_bars() {
        // 3:1 is WCAG's floor for large text and graphics; the Done column's percentage is small
        // but short and bold enough that the bars' colours keep this margin
        for (name, over) in [
            ("bar_bg", LIGHT.bar_bg),
            ("bar_blue", LIGHT.bar_blue),
            ("bar_green", LIGHT.bar_green),
            ("grey", 0xa8a8a8),
        ] {
            let c = contrast(LIGHT.bar_fg, over);
            assert!(c >= 3.0, "light bar_fg over {name}: {c:.2}");
        }
        for (name, over) in
            [("bar_bg", DARK.bar_bg), ("bar_blue", DARK.bar_blue), ("bar_green", DARK.bar_green)]
        {
            let c = contrast(DARK.bar_fg, over);
            assert!(c >= 3.0, "dark bar_fg over {name}: {c:.2}");
        }
    }
}

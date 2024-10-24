//!
//! Implements a dark theme.
//!
use crate::Scheme;
use rat_widget::button::ButtonStyle;
use rat_widget::choice::ChoiceStyle;
use rat_widget::file_dialog::FileDialogStyle;
use rat_widget::line_number::LineNumberStyle;
use rat_widget::list::ListStyle;
use rat_widget::menu::MenuStyle;
use rat_widget::msgdialog::MsgDialogStyle;
use rat_widget::paragraph::ParagraphStyle;
use rat_widget::popup::PopupStyle;
use rat_widget::scrolled::ScrollStyle;
use rat_widget::shadow::{ShadowDirection, ShadowStyle};
use rat_widget::splitter::SplitStyle;
use rat_widget::tabbed::TabbedStyle;
use rat_widget::table::TableStyle;
use rat_widget::text::TextStyle;
use rat_widget::view::{ClipperStyle, PagerStyle, ViewStyle};
use ratatui::prelude::{Style, Stylize};
use ratatui::widgets::Block;

/// One sample theme which prefers dark colors from the color-scheme
/// and generates styles for widgets.
///
/// The widget set fits for the widgets provided by
/// [rat-widget](https://www.docs.rs/rat-widget), for other needs
/// take it as an idea for your own implementation.
///
#[derive(Debug, Clone)]
pub struct DarkTheme {
    s: Scheme,
    name: String,
}

impl DarkTheme {
    pub fn new(name: String, s: Scheme) -> Self {
        Self { s, name }
    }
}

impl DarkTheme {
    /// Some display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Hint at dark.
    pub fn dark_theme(&self) -> bool {
        true
    }

    /// The underlying scheme.
    pub fn scheme(&self) -> &Scheme {
        &self.s
    }

    /// Create a style from the given white shade.
    /// n is `0..=3`
    pub fn white(&self, n: usize) -> Style {
        self.s.style(self.s.white[n])
    }

    /// Create a style from the given black shade.
    /// n is `0..=3`
    pub fn black(&self, n: usize) -> Style {
        self.s.style(self.s.black[n])
    }

    /// Create a style from the given gray shade.
    /// n is `0..=3`
    pub fn gray(&self, n: usize) -> Style {
        self.s.style(self.s.gray[n])
    }

    /// Create a style from the given red shade.
    /// n is `0..=3`
    pub fn red(&self, n: usize) -> Style {
        self.s.style(self.s.red[n])
    }

    /// Create a style from the given orange shade.
    /// n is `0..=3`
    pub fn orange(&self, n: usize) -> Style {
        self.s.style(self.s.orange[n])
    }

    /// Create a style from the given yellow shade.
    /// n is `0..=3`
    pub fn yellow(&self, n: usize) -> Style {
        self.s.style(self.s.yellow[n])
    }

    /// Create a style from the given limegreen shade.
    /// n is `0..=3`
    pub fn limegreen(&self, n: usize) -> Style {
        self.s.style(self.s.limegreen[n])
    }

    /// Create a style from the given green shade.
    /// n is `0..=3`
    pub fn green(&self, n: usize) -> Style {
        self.s.style(self.s.green[n])
    }

    /// Create a style from the given bluegreen shade.
    /// n is `0..=3`
    pub fn bluegreen(&self, n: usize) -> Style {
        self.s.style(self.s.bluegreen[n])
    }

    /// Create a style from the given cyan shade.
    /// n is `0..=3`
    pub fn cyan(&self, n: usize) -> Style {
        self.s.style(self.s.cyan[n])
    }

    /// Create a style from the given blue shade.
    /// n is `0..=3`
    pub fn blue(&self, n: usize) -> Style {
        self.s.style(self.s.blue[n])
    }

    /// Create a style from the given deepblue shade.
    /// n is `0..=3`
    pub fn deepblue(&self, n: usize) -> Style {
        self.s.style(self.s.deepblue[n])
    }

    /// Create a style from the given purple shade.
    /// n is `0..=3`
    pub fn purple(&self, n: usize) -> Style {
        self.s.style(self.s.purple[n])
    }

    /// Create a style from the given magenta shade.
    /// n is `0..=3`
    pub fn magenta(&self, n: usize) -> Style {
        self.s.style(self.s.magenta[n])
    }

    /// Create a style from the given redpink shade.
    /// n is `0..=3`
    pub fn redpink(&self, n: usize) -> Style {
        self.s.style(self.s.redpink[n])
    }

    /// Create a style from the given primary shade.
    /// n is `0..=3`
    pub fn primary(&self, n: usize) -> Style {
        self.s.style(self.s.primary[n])
    }

    /// Create a style from the given secondary shade.
    /// n is `0..=3`
    pub fn secondary(&self, n: usize) -> Style {
        self.s.style(self.s.secondary[n])
    }

    /// Focus style
    pub fn focus(&self) -> Style {
        let bg = self.s.primary[2];
        Style::default().fg(self.s.text_color(bg)).bg(bg)
    }

    /// Selection style
    pub fn select(&self) -> Style {
        let bg = self.s.secondary[1];
        Style::default().fg(self.s.text_color(bg)).bg(bg)
    }

    /// Text field style.
    pub fn text_input(&self) -> Style {
        Style::default().fg(self.s.black[0]).bg(self.s.gray[3])
    }

    /// Focused text field style.
    pub fn text_focus(&self) -> Style {
        let bg = self.s.primary[0];
        Style::default().fg(self.s.text_color(bg)).bg(bg)
    }

    /// Text selection style.
    pub fn text_select(&self) -> Style {
        let bg = self.s.secondary[0];
        Style::default().fg(self.s.text_color(bg)).bg(bg)
    }

    /// Container base
    pub fn container(&self) -> Style {
        Style::default().fg(self.s.gray[0]).bg(self.s.black[1])
    }

    /// Data display style. Used for lists, tables, ...
    pub fn data(&self) -> Style {
        Style::default().fg(self.s.white[0]).bg(self.s.black[1])
    }

    /// Background for dialogs.
    pub fn dialog_style(&self) -> Style {
        Style::default().fg(self.s.white[2]).bg(self.s.gray[1])
    }

    /// Style for the status line.
    pub fn status_style(&self) -> Style {
        Style::default().fg(self.s.white[0]).bg(self.s.black[2])
    }

    /// Style for shadows.
    pub fn shadow_style(&self) -> ShadowStyle {
        ShadowStyle {
            style: Style::new().bg(self.s.black[0]),
            dir: ShadowDirection::BottomRight,
            ..Default::default()
        }
    }

    /// Style for LineNumbers.
    pub fn line_nr_style(&self) -> LineNumberStyle {
        LineNumberStyle {
            style: self.data().fg(self.s.gray[0]),
            cursor: Some(self.text_select()),
            ..LineNumberStyle::default()
        }
    }

    /// Complete TextAreaStyle
    pub fn textarea_style(&self) -> TextStyle {
        TextStyle {
            style: self.data(),
            focus: Some(self.focus()),
            select: Some(self.text_select()),
            scroll: Some(self.scroll_style()),
            ..TextStyle::default()
        }
    }

    /// Complete TextInputStyle
    pub fn input_style(&self) -> TextStyle {
        TextStyle {
            style: self.text_input(),
            focus: Some(self.text_focus()),
            select: Some(self.text_select()),
            invalid: Some(Style::default().bg(self.s.red[3])),
            ..TextStyle::default()
        }
    }

    /// Complete MenuStyle
    pub fn menu_style(&self) -> MenuStyle {
        let menu = Style::default().fg(self.s.white[3]).bg(self.s.black[2]);
        MenuStyle {
            style: menu,
            title: Some(Style::default().fg(self.s.black[0]).bg(self.s.yellow[2])),
            select: Some(self.select()),
            focus: Some(self.focus()),
            right: Some(Style::default().fg(self.s.bluegreen[0])),
            disabled: Some(Style::default().fg(self.s.gray[0])),
            highlight: Some(Style::default().underlined()),
            popup: PopupStyle {
                style: menu,
                block: Some(Block::bordered()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Paragraph style
    pub fn paragraph_style(&self) -> ParagraphStyle {
        ParagraphStyle {
            style: self.data(),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// Table style
    pub fn table_style(&self) -> TableStyle {
        TableStyle {
            style: self.data(),
            select_row: Some(self.select()),
            show_row_focus: true,
            focus_style: Some(self.focus()),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// List style
    pub fn list_style(&self) -> ListStyle {
        ListStyle {
            style: self.data(),
            select: Some(self.select()),
            focus: Some(self.focus()),
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// Button style
    pub fn button_style(&self) -> ButtonStyle {
        ButtonStyle {
            style: Style::default()
                .fg(self.s.text_color(self.s.primary[0]))
                .bg(self.s.primary[0]),
            focus: Some(
                Style::default()
                    .fg(self.s.text_color(self.s.primary[3]))
                    .bg(self.s.primary[3]),
            ),
            armed: Some(Style::default().fg(self.s.black[0]).bg(self.s.secondary[0])),
            ..Default::default()
        }
    }

    /// Scroll style
    pub fn scroll_style(&self) -> ScrollStyle {
        let style = self.container();
        let arrow_style = Style::default().fg(self.s.secondary[0]).bg(self.s.black[1]);
        ScrollStyle {
            thumb_style: Some(style),
            track_style: Some(style),
            min_style: Some(style),
            begin_style: Some(arrow_style),
            end_style: Some(arrow_style),
            ..Default::default()
        }
    }

    /// Split style
    pub fn split_style(&self) -> SplitStyle {
        let style = self.container();
        let arrow_style = Style::default().fg(self.s.secondary[0]).bg(self.s.black[1]);
        SplitStyle {
            style,
            arrow_style: Some(arrow_style),
            drag_style: Some(self.focus()),
            ..Default::default()
        }
    }

    /// View style
    pub fn view_style(&self) -> ViewStyle {
        ViewStyle {
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }

    /// Tabbed style
    pub fn tabbed_style(&self) -> TabbedStyle {
        let style = self.container();
        TabbedStyle {
            style,
            tab: Some(self.gray(1)),
            select: Some(self.gray(3)),
            focus: Some(self.focus()),
            ..Default::default()
        }
    }

    /// StatusLineStyle for a StatusLine with 3 indicator fields.
    /// This is what I need for the
    /// [minimal](https://github.com/thscharler/rat-salsa/blob/master/examples/minimal.rs)
    /// example, which shows timings for Render/Event/Action.
    pub fn statusline_style(&self) -> Vec<Style> {
        let s = &self.s;
        vec![
            self.status_style(),
            Style::default().fg(s.text_color(s.white[0])).bg(s.blue[3]),
            Style::default().fg(s.text_color(s.white[0])).bg(s.blue[2]),
            Style::default().fg(s.text_color(s.white[0])).bg(s.blue[1]),
        ]
    }

    /// MsgDialog style.
    pub fn msg_dialog_style(&self) -> MsgDialogStyle {
        MsgDialogStyle {
            style: self.dialog_style(),
            button: self.button_style(),
            ..Default::default()
        }
    }

    /// FileDialog style.
    pub fn file_dialog_style(&self) -> FileDialogStyle {
        FileDialogStyle {
            style: self.dialog_style(),
            list: Some(self.data()),
            path: Some(self.text_input()),
            name: Some(self.text_input()),
            new: Some(self.text_input()),
            invalid: Some(Style::new().fg(self.s.red[3]).bg(self.s.gray[2])),
            select: Some(self.select()),
            focus: Some(self.focus()),
            button: Some(self.button_style()),
            ..Default::default()
        }
    }

    /// Choice style.
    pub fn choice_style(&self) -> ChoiceStyle {
        ChoiceStyle {
            style: self.text_input(),
            button: Some(self.text_input()),
            select: Some(self.select()),
            focus: Some(self.focus()),
            popup: PopupStyle {
                style: self.text_input(),
                block: Some(Block::bordered()),
                scroll: Some(self.scroll_style()),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Pager style.
    pub fn pager_style(&self) -> PagerStyle {
        PagerStyle {
            style: self.container(),
            nav: Some(self.select()),
            divider: Some(self.container()),
            ..Default::default()
        }
    }

    /// Clipper style.
    pub fn clipper_style(&self) -> ClipperStyle {
        ClipperStyle {
            scroll: Some(self.scroll_style()),
            ..Default::default()
        }
    }
}

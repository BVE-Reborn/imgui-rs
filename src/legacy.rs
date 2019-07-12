#![allow(non_upper_case_globals)]
use bitflags::bitflags;
use std::os::raw::c_int;

use crate::string::ImStr;
use crate::widget::progress_bar::ProgressBar;
use crate::window::{Window, WindowFlags};
use crate::Ui;

bitflags!(
    /// Color edit flags
    #[repr(C)]
    pub struct ImGuiColorEditFlags: c_int {
        /// ColorEdit, ColorPicker, ColorButton: ignore Alpha component (read 3 components from the
        /// input pointer).
        const NoAlpha = 1;
        /// ColorEdit: disable picker when clicking on colored square.
        const NoPicker = 1 << 2;
        /// ColorEdit: disable toggling options menu when right-clicking on inputs/small preview.
        const NoOptions = 1 << 3;
        /// ColorEdit, ColorPicker: disable colored square preview next to the inputs. (e.g. to
        /// show only the inputs)
        const NoSmallPreview = 1 << 4;
        /// ColorEdit, ColorPicker: disable inputs sliders/text widgets (e.g. to show only the
        /// small preview colored square).
        const NoInputs = 1 << 5;
        /// ColorEdit, ColorPicker, ColorButton: disable tooltip when hovering the preview.
        const NoTooltip = 1 << 6;
        /// ColorEdit, ColorPicker: disable display of inline text label (the label is still
        /// forwarded to the tooltip and picker).
        const NoLabel = 1 << 7;
        /// ColorPicker: disable bigger color preview on right side of the picker, use small
        /// colored square preview instead.
        const NoSidePreview = 1 << 8;
        /// ColorEdit: disable drag and drop target. ColorButton: disable drag and drop source.
        const NoDragDrop = 1 << 9;

        /// ColorEdit, ColorPicker: show vertical alpha bar/gradient in picker.
        const AlphaBar = 1 << 16;
        /// ColorEdit, ColorPicker, ColorButton: display preview as a transparent color over a
        /// checkerboard, instead of opaque.
        const AlphaPreview = 1 << 17;
        /// ColorEdit, ColorPicker, ColorButton: display half opaque / half checkerboard, instead
        /// of opaque.
        const AlphaPreviewHalf= 1 << 18;
        /// (WIP) ColorEdit: Currently only disable 0.0f..1.0f limits in RGBA edition (note: you
        /// probably want to use ImGuiColorEditFlags::Float flag as well).
        const HDR = 1 << 19;
        /// ColorEdit: choose one among RGB/HSV/HEX. ColorPicker: choose any combination using
        /// RGB/HSV/HEX.
        const RGB = 1 << 20;
        const HSV = 1 << 21;
        const HEX = 1 << 22;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0..255.
        const Uint8 = 1 << 23;
        /// ColorEdit, ColorPicker, ColorButton: _display_ values formatted as 0.0f..1.0f floats
        /// instead of 0..255 integers. No round-trip of value via integers.
        const Float = 1 << 24;
        /// ColorPicker: bar for Hue, rectangle for Sat/Value.
        const PickerHueBar = 1 << 25;
        /// ColorPicker: wheel for Hue, triangle for Sat/Value.
        const PickerHueWheel = 1 << 26;
    }
);

bitflags!(
    /// Flags for combo boxes
    #[repr(C)]
    pub struct ImGuiComboFlags: c_int {
        /// Align the popup toward the left by default
        const PopupAlignLeft = 1;
        /// Max ~4 items visible.
        const HeightSmall = 1 << 1;
        /// Max ~8 items visible (default)
        const HeightRegular = 1 << 2;
        /// Max ~20 items visible
        const HeightLarge = 1 << 3;
        /// As many fitting items as possible
        const HeightLargest = 1 << 4;
        /// Display on the preview box without the square arrow button
        const NoArrowButton = 1 << 5;
        /// Display only a square arrow button
        const NoPreview = 1 << 6;

        const HeightMask     = ImGuiComboFlags::HeightSmall.bits
            | ImGuiComboFlags::HeightRegular.bits
            | ImGuiComboFlags::HeightLarge.bits
            | ImGuiComboFlags::HeightLargest.bits;
    }
);

bitflags!(
    /// Flags for igBeginDragDropSource(), igAcceptDragDropPayload()
    #[repr(C)]
    pub struct ImGuiDragDropFlags: c_int {
        /// By default, a successful call to igBeginDragDropSource opens a tooltip so you can
        /// display a preview or description of the source contents. This flag disable this
        /// behavior.
        const SourceNoPreviewTooltip = 1;
        /// By default, when dragging we clear data so that igIsItemHovered() will return false, to
        /// avoid subsequent user code submitting tooltips. This flag disable this behavior so you
        /// can still call igIsItemHovered() on the source item.
        const SourceNoDisableHover = 1 << 1;
        /// Disable the behavior that allows to open tree nodes and collapsing header by holding
        /// over them while dragging a source item.
        const SourceNoHoldToOpenOthers = 1 << 2;
        /// Allow items such as igText(), igImage() that have no unique identifier to be used as
        /// drag source, by manufacturing a temporary identifier based on their window-relative
        /// position. This is extremely unusual within the dear imgui ecosystem and so we made it
        /// explicit.
        const SourceAllowNullID = 1 << 3;
        /// External source (from outside of imgui), won't attempt to read current item/window
        /// info. Will always return true. Only one Extern source can be active simultaneously.
        const SourceExtern = 1 << 4;
        /// Automatically expire the payload if the source cease to be submitted (otherwise
        /// payloads are persisting while being dragged)
        const SourceAutoExpirePayload = 1 << 5;
        /// igAcceptDragDropPayload() will returns true even before the mouse button is released.
        /// You can then call igIsDelivery() to test if the payload needs to be delivered.
        const AcceptBeforeDelivery = 1 << 10;
        /// Do not draw the default highlight rectangle when hovering over target.
        const AcceptNoDrawDefaultRect = 1 << 11;
        /// Request hiding the igBeginDragDropSource tooltip from the igBeginDragDropTarget site.
        const AcceptNoPreviewTooltip = 1 << 12;
        /// For peeking ahead and inspecting the payload before delivery.
        const AcceptPeekOnly = ImGuiDragDropFlags::AcceptBeforeDelivery.bits
            | ImGuiDragDropFlags::AcceptNoDrawDefaultRect.bits;
    }
);

bitflags!(
    /// Flags for indictating which corner of a rectangle should be rounded
    #[repr(C)]
    pub struct ImDrawCornerFlags: c_int {
        const TopLeft = 1;
        const TopRight = 1 << 1;
        const BotLeft = 1 << 2;
        const BotRight = 1 << 3;
        const Top = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::TopRight.bits;
        const Bot = ImDrawCornerFlags::BotLeft.bits
            | ImDrawCornerFlags::BotRight.bits;
        const Left = ImDrawCornerFlags::TopLeft.bits
            | ImDrawCornerFlags::BotLeft.bits;
        const Right = ImDrawCornerFlags::TopRight.bits
            | ImDrawCornerFlags::BotRight.bits;
        const All = 0xF;
    }
);

bitflags!(
    /// Draw list flags
    #[repr(C)]
    pub struct ImDrawListFlags: c_int {
        const AntiAliasedLines = 1;
        const AntiAliasedFill = 1 << 1;
    }
);

bitflags!(
    /// Flags for window focus checks
    #[repr(C)]
    pub struct ImGuiFocusedFlags: c_int {
        /// Return true if any children of the window is focused
        const ChildWindows = 1;
        /// Test from root window (top most parent of the current hierarchy)
        const RootWindow = 1 << 1;
        /// Return true if any window is focused
        const AnyWindow = 1 << 2;

        const RootAndChildWindows =
            ImGuiFocusedFlags::RootWindow.bits | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for hover checks
    #[repr(C)]
    pub struct ImGuiHoveredFlags: c_int {
        /// Window hover checks only: Return true if any children of the window is hovered
        const ChildWindows = 1;
        /// Window hover checks only: Test from root window (top most parent of the current hierarchy)
        const RootWindow = 1 << 1;
        /// Window hover checks only: Return true if any window is hovered
        const AnyWindow = 1 << 2;
        /// Return true even if a popup window is normally blocking access to this item/window
        const AllowWhenBlockedByPopup = 1 << 3;
        /// Return true even if an active item is blocking access to this item/window. Useful for
        /// Drag and Drop patterns.
        const AllowWhenBlockedByActiveItem = 1 << 5;
        /// Return true even if the position is overlapped by another window
        const AllowWhenOverlapped = 1 << 6;
        /// Return true even if the item is disabled
        const AllowWhenDisabled = 1 << 7;

        const RectOnly = ImGuiHoveredFlags::AllowWhenBlockedByPopup.bits
            | ImGuiHoveredFlags::AllowWhenBlockedByActiveItem.bits
            | ImGuiHoveredFlags::AllowWhenOverlapped.bits;
        const RootAndChildWindows = ImGuiFocusedFlags::RootWindow.bits
            | ImGuiFocusedFlags::ChildWindows.bits;
    }
);

bitflags!(
    /// Flags for text inputs
    #[repr(C)]
    pub struct ImGuiInputTextFlags: c_int {
        /// Allow 0123456789.+-*/
        const CharsDecimal = 1;
        /// Allow 0123456789ABCDEFabcdef
        const CharsHexadecimal = 1 << 1;
        /// Turn a..z into A..Z
        const CharsUppercase = 1 << 2;
        /// Filter out spaces, tabs
        const CharsNoBlank = 1 << 3;
        /// Select entire text when first taking mouse focus
        const AutoSelectAll = 1 << 4;
        /// Return 'true' when Enter is pressed (as opposed to when the value was modified)
        const EnterReturnsTrue = 1 << 5;
        /// Call user function on pressing TAB (for completion handling)
        const CallbackCompletion = 1 << 6;
        /// Call user function on pressing Up/Down arrows (for history handling)
        const CallbackHistory = 1 << 7;
        /// Call user function every time. User code may query cursor position, modify text buffer.
        const CallbackAlways = 1 << 8;
        /// Call user function to filter character.
        const CallbackCharFilter = 1 << 9;
        /// Pressing TAB input a '\t' character into the text field
        const AllowTabInput = 1 << 10;
        /// In multi-line mode, unfocus with Enter, add new line with Ctrl+Enter (default is
        /// opposite: unfocus with Ctrl+Enter, add line with Enter).
        const CtrlEnterForNewLine = 1 << 11;
        /// Disable following the cursor horizontally
        const NoHorizontalScroll = 1 << 12;
        /// Insert mode
        const AlwaysInsertMode = 1 << 13;
        /// Read-only mode
        const ReadOnly = 1 << 14;
        /// Password mode, display all characters as '*'
        const Password = 1 << 15;
        /// Disable undo/redo.
        const NoUndoRedo = 1 << 16;
        /// Allow 0123456789.+-*/eE (Scientific notation input)
        const CharsScientific = 1 << 17;
        /// Allow buffer capacity resize + notify when the string wants to be resized
        const CallbackResize = 1 << 18;
    }
);

bitflags!(
    /// Flags for selectables
    #[repr(C)]
    pub struct ImGuiSelectableFlags: c_int {
        /// Clicking this don't close parent popup window
        const DontClosePopups = 1;
        /// Selectable frame can span all columns (text will still fit in current column)
        const SpanAllColumns = 1 << 1;
        /// Generate press events on double clicks too
        const AllowDoubleClick = 1 << 2;
        /// Cannot be selected, display greyed out text
        const Disabled = 1 << 3;
    }
);

bitflags!(
    /// Flags for trees and collapsing headers
    #[repr(C)]
    pub struct ImGuiTreeNodeFlags: c_int {
        /// Draw as selected
        const Selected = 1;
        /// Full colored frame (e.g. for collapsing header)
        const Framed = 1 << 1;
        /// Hit testing to allow subsequent widgets to overlap this one
        const AllowItemOverlap = 1 << 2;
        /// Don't do a tree push when open (e.g. for collapsing header) = no extra indent nor
        /// pushing on ID stack
        const NoTreePushOnOpen = 1 << 3;
        /// Don't automatically and temporarily open node when Logging is active (by default
        /// logging will automatically open tree nodes)
        const NoAutoOpenOnLog = 1 << 4;
        /// Default node to be open
        const DefaultOpen = 1 << 5;
        /// Need double-click to open node
        const OpenOnDoubleClick = 1 << 6;
        /// Only open when clicking on the arrow part. If OpenOnDoubleClick is also set,
        /// single-click arrow or double-click all box to open.
        const OpenOnArrow = 1 << 7;
        /// No collapsing, no arrow (use as a convenience for leaf nodes).
        const Leaf = 1 << 8;
        /// Display a bullet instead of arrow
        const Bullet = 1 << 9;
        /// Use FramePadding (even for an unframed text node) to vertically align text baseline to
        /// regular widget height.
        const FramePadding = 1 << 10;
        const NavLeftJumpsBackHere = 1 << 13;

        const CollapsingHeader  =
            ImGuiTreeNodeFlags::Framed.bits | ImGuiTreeNodeFlags::NoTreePushOnOpen.bits |
            ImGuiTreeNodeFlags::NoAutoOpenOnLog.bits;
    }
);

#[deprecated(since = "0.2.0", note = "use WindowFlags instead")]
pub type ImGuiWindowFlags = WindowFlags;

impl<'ui> Ui<'ui> {
    #[deprecated(since = "0.2.0", note = "use imgui::Window::new(...) instead")]
    pub fn window<'p>(&self, name: &'p ImStr) -> Window<'p> {
        Window::new(name)
    }
    #[deprecated(since = "0.2.0", note = "use Ui::window_size instead")]
    pub fn get_window_size(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowSize_nonUDT2() };
        size.into()
    }
    #[deprecated(since = "0.2.0", note = "use Ui::window_pos instead")]
    pub fn get_window_pos(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowPos_nonUDT2() };
        size.into()
    }
    #[deprecated(since = "0.2.0", note = "use Ui::content_region_max instead")]
    pub fn get_content_region_max(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetContentRegionMax_nonUDT2() };
        size.into()
    }
    #[deprecated(since = "0.2.0", note = "use Ui::content_region_avail instead")]
    pub fn get_content_region_avail(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetContentRegionAvail_nonUDT2() };
        size.into()
    }
    #[deprecated(since = "0.2.0", note = "use Ui::window_content_region_min instead")]
    pub fn get_window_content_region_min(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowContentRegionMin_nonUDT2() };
        size.into()
    }
    #[deprecated(since = "0.2.0", note = "use Ui::window_content_region_max instead")]
    pub fn get_window_content_region_max(&self) -> [f32; 2] {
        let size = unsafe { sys::igGetWindowContentRegionMax_nonUDT2() };
        size.into()
    }
    #[deprecated(
        since = "0.2.0",
        note = "use Ui::is_window_focused(WindowFlags::RootWindow) instead"
    )]
    pub fn is_root_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(ImGuiFocusedFlags::RootWindow.bits()) }
    }
    #[deprecated(
        since = "0.2.0",
        note = "use Ui::is_window_focused(WindowFlags::ChildWindows) instead"
    )]
    pub fn is_child_window_focused(&self) -> bool {
        unsafe { sys::igIsWindowFocused(ImGuiFocusedFlags::ChildWindows.bits()) }
    }
}

impl<'ui> Ui<'ui> {
    #[deprecated(since = "0.2.0", note = "use imgui::ProgressBar::new(...) instead")]
    pub fn progress_bar<'p>(&self, fraction: f32) -> ProgressBar<'p> {
        ProgressBar::new(fraction)
    }
}

// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Input` X11 extension.

#![allow(clippy::too_many_arguments)]

#[allow(unused_imports)]
use std::borrow::Cow;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::{Request, RequestHeader, Serialize, TryParse, TryParseFd};
use std::io::IoSlice;
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::connection::Connection as X11Connection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::ConnectionError;
#[allow(unused_imports)]
use crate::errors::ReplyOrIdError;
#[allow(unused_imports)]
use super::xfixes;
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::xinput::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_get_extension_version<'c, Conn>(req: GetExtensionVersionRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetExtensionVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_extension_version<'c, 'input, Conn>(conn: &'c Conn, name: &'input [u8]) -> Result<Cookie<'c, Conn, GetExtensionVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetExtensionVersionRequest {
        name: Cow::Borrowed(name),
    };
    send_get_extension_version(request0, conn)
}

fn send_list_input_devices<'c, Conn>(req: ListInputDevicesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ListInputDevicesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn list_input_devices<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListInputDevicesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListInputDevicesRequest;
    send_list_input_devices(request0, conn)
}

fn send_open_device<'c, Conn>(req: OpenDeviceRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, OpenDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn open_device<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, OpenDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = OpenDeviceRequest {
        device_id,
    };
    send_open_device(request0, conn)
}

fn send_close_device<'c, Conn>(req: CloseDeviceRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn close_device<Conn>(conn: &Conn, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CloseDeviceRequest {
        device_id,
    };
    send_close_device(request0, conn)
}

fn send_set_device_mode<'c, Conn>(req: SetDeviceModeRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, SetDeviceModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn set_device_mode<Conn>(conn: &Conn, device_id: u8, mode: ValuatorMode) -> Result<Cookie<'_, Conn, SetDeviceModeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceModeRequest {
        device_id,
        mode,
    };
    send_set_device_mode(request0, conn)
}

fn send_select_extension_event<'c, Conn>(req: SelectExtensionEventRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn select_extension_event<'c, 'input, Conn>(conn: &'c Conn, window: xproto::Window, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SelectExtensionEventRequest {
        window,
        classes: Cow::Borrowed(classes),
    };
    send_select_extension_event(request0, conn)
}

fn send_get_selected_extension_events<'c, Conn>(req: GetSelectedExtensionEventsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSelectedExtensionEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_selected_extension_events<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetSelectedExtensionEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectedExtensionEventsRequest {
        window,
    };
    send_get_selected_extension_events(request0, conn)
}

fn send_change_device_dont_propagate_list<'c, Conn>(req: ChangeDeviceDontPropagateListRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_device_dont_propagate_list<'c, 'input, Conn>(conn: &'c Conn, window: xproto::Window, mode: PropagateMode, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDeviceDontPropagateListRequest {
        window,
        mode,
        classes: Cow::Borrowed(classes),
    };
    send_change_device_dont_propagate_list(request0, conn)
}

fn send_get_device_dont_propagate_list<'c, Conn>(req: GetDeviceDontPropagateListRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceDontPropagateListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_dont_propagate_list<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetDeviceDontPropagateListReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceDontPropagateListRequest {
        window,
    };
    send_get_device_dont_propagate_list(request0, conn)
}

fn send_get_device_motion_events<'c, Conn>(req: GetDeviceMotionEventsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceMotionEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_motion_events<Conn, A>(conn: &Conn, start: xproto::Timestamp, stop: A, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceMotionEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let stop: xproto::Timestamp = stop.into();
    let request0 = GetDeviceMotionEventsRequest {
        start,
        stop,
        device_id,
    };
    send_get_device_motion_events(request0, conn)
}

fn send_change_keyboard_device<'c, Conn>(req: ChangeKeyboardDeviceRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ChangeKeyboardDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn change_keyboard_device<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, ChangeKeyboardDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeKeyboardDeviceRequest {
        device_id,
    };
    send_change_keyboard_device(request0, conn)
}

fn send_change_pointer_device<'c, Conn>(req: ChangePointerDeviceRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ChangePointerDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn change_pointer_device<Conn>(conn: &Conn, x_axis: u8, y_axis: u8, device_id: u8) -> Result<Cookie<'_, Conn, ChangePointerDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangePointerDeviceRequest {
        x_axis,
        y_axis,
        device_id,
    };
    send_change_pointer_device(request0, conn)
}

fn send_grab_device<'c, Conn>(req: GrabDeviceRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, GrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn grab_device<'c, 'input, Conn, A>(conn: &'c Conn, grab_window: xproto::Window, time: A, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, owner_events: bool, device_id: u8, classes: &'input [EventClass]) -> Result<Cookie<'c, Conn, GrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = GrabDeviceRequest {
        grab_window,
        time,
        this_device_mode,
        other_device_mode,
        owner_events,
        device_id,
        classes: Cow::Borrowed(classes),
    };
    send_grab_device(request0, conn)
}

fn send_ungrab_device<'c, Conn>(req: UngrabDeviceRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn ungrab_device<Conn, A>(conn: &Conn, time: A, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = UngrabDeviceRequest {
        time,
        device_id,
    };
    send_ungrab_device(request0, conn)
}

fn send_grab_device_key<'c, Conn>(req: GrabDeviceKeyRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn grab_device_key<'c, 'input, Conn, A, B, C>(conn: &'c Conn, grab_window: xproto::Window, modifiers: A, modifier_device: B, grabbed_device: u8, key: C, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, owner_events: bool, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let key: u8 = key.into();
    let request0 = GrabDeviceKeyRequest {
        grab_window,
        modifiers,
        modifier_device,
        grabbed_device,
        key,
        this_device_mode,
        other_device_mode,
        owner_events,
        classes: Cow::Borrowed(classes),
    };
    send_grab_device_key(request0, conn)
}

fn send_ungrab_device_key<'c, Conn>(req: UngrabDeviceKeyRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn ungrab_device_key<Conn, A, B, C>(conn: &Conn, grab_window: xproto::Window, modifiers: A, modifier_device: B, key: C, grabbed_device: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let key: u8 = key.into();
    let request0 = UngrabDeviceKeyRequest {
        grab_window,
        modifiers,
        modifier_device,
        key,
        grabbed_device,
    };
    send_ungrab_device_key(request0, conn)
}

fn send_grab_device_button<'c, Conn>(req: GrabDeviceButtonRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn grab_device_button<'c, 'input, Conn, A, B, C>(conn: &'c Conn, grab_window: xproto::Window, grabbed_device: u8, modifier_device: A, modifiers: B, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, button: C, owner_events: bool, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u8>,
    B: Into<u16>,
    C: Into<u8>,
{
    let modifier_device: u8 = modifier_device.into();
    let modifiers: u16 = modifiers.into();
    let button: u8 = button.into();
    let request0 = GrabDeviceButtonRequest {
        grab_window,
        grabbed_device,
        modifier_device,
        modifiers,
        this_device_mode,
        other_device_mode,
        button,
        owner_events,
        classes: Cow::Borrowed(classes),
    };
    send_grab_device_button(request0, conn)
}

fn send_ungrab_device_button<'c, Conn>(req: UngrabDeviceButtonRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn ungrab_device_button<Conn, A, B, C>(conn: &Conn, grab_window: xproto::Window, modifiers: A, modifier_device: B, button: C, grabbed_device: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u16>,
    B: Into<u8>,
    C: Into<u8>,
{
    let modifiers: u16 = modifiers.into();
    let modifier_device: u8 = modifier_device.into();
    let button: u8 = button.into();
    let request0 = UngrabDeviceButtonRequest {
        grab_window,
        modifiers,
        modifier_device,
        button,
        grabbed_device,
    };
    send_ungrab_device_button(request0, conn)
}

fn send_allow_device_events<'c, Conn>(req: AllowDeviceEventsRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn allow_device_events<Conn, A>(conn: &Conn, time: A, mode: DeviceInputMode, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
{
    let time: xproto::Timestamp = time.into();
    let request0 = AllowDeviceEventsRequest {
        time,
        mode,
        device_id,
    };
    send_allow_device_events(request0, conn)
}

fn send_get_device_focus<'c, Conn>(req: GetDeviceFocusRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceFocusReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_focus<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceFocusReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceFocusRequest {
        device_id,
    };
    send_get_device_focus(request0, conn)
}

fn send_set_device_focus<'c, Conn>(req: SetDeviceFocusRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_device_focus<Conn, A, B>(conn: &Conn, focus: A, time: B, revert_to: xproto::InputFocus, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Window>,
    B: Into<xproto::Timestamp>,
{
    let focus: xproto::Window = focus.into();
    let time: xproto::Timestamp = time.into();
    let request0 = SetDeviceFocusRequest {
        focus,
        time,
        revert_to,
        device_id,
    };
    send_set_device_focus(request0, conn)
}

fn send_get_feedback_control<'c, Conn>(req: GetFeedbackControlRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetFeedbackControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_feedback_control<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetFeedbackControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetFeedbackControlRequest {
        device_id,
    };
    send_get_feedback_control(request0, conn)
}

fn send_change_feedback_control<'c, Conn>(req: ChangeFeedbackControlRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_feedback_control<Conn, A>(conn: &Conn, mask: A, device_id: u8, feedback_id: u8, feedback: FeedbackCtl) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let mask: u32 = mask.into();
    let request0 = ChangeFeedbackControlRequest {
        mask,
        device_id,
        feedback_id,
        feedback,
    };
    send_change_feedback_control(request0, conn)
}

fn send_get_device_key_mapping<'c, Conn>(req: GetDeviceKeyMappingRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceKeyMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_key_mapping<Conn>(conn: &Conn, device_id: u8, first_keycode: KeyCode, count: u8) -> Result<Cookie<'_, Conn, GetDeviceKeyMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceKeyMappingRequest {
        device_id,
        first_keycode,
        count,
    };
    send_get_device_key_mapping(request0, conn)
}

fn send_change_device_key_mapping<'c, Conn>(req: ChangeDeviceKeyMappingRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_device_key_mapping<'c, 'input, Conn>(conn: &'c Conn, device_id: u8, first_keycode: KeyCode, keysyms_per_keycode: u8, keycode_count: u8, keysyms: &'input [xproto::Keysym]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDeviceKeyMappingRequest {
        device_id,
        first_keycode,
        keysyms_per_keycode,
        keycode_count,
        keysyms: Cow::Borrowed(keysyms),
    };
    send_change_device_key_mapping(request0, conn)
}

fn send_get_device_modifier_mapping<'c, Conn>(req: GetDeviceModifierMappingRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceModifierMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_modifier_mapping<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceModifierMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceModifierMappingRequest {
        device_id,
    };
    send_get_device_modifier_mapping(request0, conn)
}

fn send_set_device_modifier_mapping<'c, Conn>(req: SetDeviceModifierMappingRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, SetDeviceModifierMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn set_device_modifier_mapping<'c, 'input, Conn>(conn: &'c Conn, device_id: u8, keymaps: &'input [u8]) -> Result<Cookie<'c, Conn, SetDeviceModifierMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceModifierMappingRequest {
        device_id,
        keymaps: Cow::Borrowed(keymaps),
    };
    send_set_device_modifier_mapping(request0, conn)
}

fn send_get_device_button_mapping<'c, Conn>(req: GetDeviceButtonMappingRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceButtonMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_button_mapping<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceButtonMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceButtonMappingRequest {
        device_id,
    };
    send_get_device_button_mapping(request0, conn)
}

fn send_set_device_button_mapping<'c, Conn>(req: SetDeviceButtonMappingRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, SetDeviceButtonMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn set_device_button_mapping<'c, 'input, Conn>(conn: &'c Conn, device_id: u8, map: &'input [u8]) -> Result<Cookie<'c, Conn, SetDeviceButtonMappingReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceButtonMappingRequest {
        device_id,
        map: Cow::Borrowed(map),
    };
    send_set_device_button_mapping(request0, conn)
}

fn send_query_device_state<'c, Conn>(req: QueryDeviceStateRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryDeviceStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_device_state<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, QueryDeviceStateReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryDeviceStateRequest {
        device_id,
    };
    send_query_device_state(request0, conn)
}

fn send_device_bell<'c, Conn>(req: DeviceBellRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn device_bell<Conn>(conn: &Conn, device_id: u8, feedback_id: u8, feedback_class: u8, percent: i8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeviceBellRequest {
        device_id,
        feedback_id,
        feedback_class,
        percent,
    };
    send_device_bell(request0, conn)
}

fn send_set_device_valuators<'c, Conn>(req: SetDeviceValuatorsRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, SetDeviceValuatorsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn set_device_valuators<'c, 'input, Conn>(conn: &'c Conn, device_id: u8, first_valuator: u8, valuators: &'input [i32]) -> Result<Cookie<'c, Conn, SetDeviceValuatorsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceValuatorsRequest {
        device_id,
        first_valuator,
        valuators: Cow::Borrowed(valuators),
    };
    send_set_device_valuators(request0, conn)
}

fn send_get_device_control<'c, Conn>(req: GetDeviceControlRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_control<Conn>(conn: &Conn, control_id: DeviceControl, device_id: u8) -> Result<Cookie<'_, Conn, GetDeviceControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceControlRequest {
        control_id,
        device_id,
    };
    send_get_device_control(request0, conn)
}

fn send_change_device_control<'c, Conn>(req: ChangeDeviceControlRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ChangeDeviceControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn change_device_control<Conn>(conn: &Conn, control_id: DeviceControl, device_id: u8, control: DeviceCtl) -> Result<Cookie<'_, Conn, ChangeDeviceControlReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDeviceControlRequest {
        control_id,
        device_id,
        control,
    };
    send_change_device_control(request0, conn)
}

fn send_list_device_properties<'c, Conn>(req: ListDevicePropertiesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ListDevicePropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn list_device_properties<Conn>(conn: &Conn, device_id: u8) -> Result<Cookie<'_, Conn, ListDevicePropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListDevicePropertiesRequest {
        device_id,
    };
    send_list_device_properties(request0, conn)
}

fn send_change_device_property<'c, Conn>(req: ChangeDevicePropertyRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn change_device_property<'c, 'input, Conn>(conn: &'c Conn, property: xproto::Atom, type_: xproto::Atom, device_id: u8, mode: xproto::PropMode, num_items: u32, items: &'input ChangeDevicePropertyAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeDevicePropertyRequest {
        property,
        type_,
        device_id,
        mode,
        num_items,
        items: Cow::Borrowed(items),
    };
    send_change_device_property(request0, conn)
}

fn send_delete_device_property<'c, Conn>(req: DeleteDevicePropertyRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_device_property<Conn>(conn: &Conn, property: xproto::Atom, device_id: u8) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DeleteDevicePropertyRequest {
        property,
        device_id,
    };
    send_delete_device_property(request0, conn)
}

fn send_get_device_property<'c, Conn>(req: GetDevicePropertyRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDevicePropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_property<Conn>(conn: &Conn, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32, device_id: u8, delete: bool) -> Result<Cookie<'_, Conn, GetDevicePropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDevicePropertyRequest {
        property,
        type_,
        offset,
        len,
        device_id,
        delete,
    };
    send_get_device_property(request0, conn)
}

fn send_xi_query_pointer<'c, Conn>(req: XIQueryPointerRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIQueryPointerReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_query_pointer<Conn, A>(conn: &Conn, window: xproto::Window, deviceid: A) -> Result<Cookie<'_, Conn, XIQueryPointerReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIQueryPointerRequest {
        window,
        deviceid,
    };
    send_xi_query_pointer(request0, conn)
}

fn send_xi_warp_pointer<'c, Conn>(req: XIWarpPointerRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_warp_pointer<Conn, A>(conn: &Conn, src_win: xproto::Window, dst_win: xproto::Window, src_x: Fp1616, src_y: Fp1616, src_width: u16, src_height: u16, dst_x: Fp1616, dst_y: Fp1616, deviceid: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIWarpPointerRequest {
        src_win,
        dst_win,
        src_x,
        src_y,
        src_width,
        src_height,
        dst_x,
        dst_y,
        deviceid,
    };
    send_xi_warp_pointer(request0, conn)
}

fn send_xi_change_cursor<'c, Conn>(req: XIChangeCursorRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_change_cursor<Conn, A>(conn: &Conn, window: xproto::Window, cursor: xproto::Cursor, deviceid: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIChangeCursorRequest {
        window,
        cursor,
        deviceid,
    };
    send_xi_change_cursor(request0, conn)
}

fn send_xi_change_hierarchy<'c, Conn>(req: XIChangeHierarchyRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_change_hierarchy<'c, 'input, Conn>(conn: &'c Conn, changes: &'input [HierarchyChange]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XIChangeHierarchyRequest {
        changes: Cow::Borrowed(changes),
    };
    send_xi_change_hierarchy(request0, conn)
}

fn send_xi_set_client_pointer<'c, Conn>(req: XISetClientPointerRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_set_client_pointer<Conn, A>(conn: &Conn, window: xproto::Window, deviceid: A) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XISetClientPointerRequest {
        window,
        deviceid,
    };
    send_xi_set_client_pointer(request0, conn)
}

fn send_xi_get_client_pointer<'c, Conn>(req: XIGetClientPointerRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIGetClientPointerReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_get_client_pointer<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, XIGetClientPointerReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XIGetClientPointerRequest {
        window,
    };
    send_xi_get_client_pointer(request0, conn)
}

fn send_xi_select_events<'c, Conn>(req: XISelectEventsRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_select_events<'c, 'input, Conn>(conn: &'c Conn, window: xproto::Window, masks: &'input [EventMask]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XISelectEventsRequest {
        window,
        masks: Cow::Borrowed(masks),
    };
    send_xi_select_events(request0, conn)
}

fn send_xi_query_version<'c, Conn>(req: XIQueryVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIQueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_query_version<Conn>(conn: &Conn, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Conn, XIQueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XIQueryVersionRequest {
        major_version,
        minor_version,
    };
    send_xi_query_version(request0, conn)
}

fn send_xi_query_device<'c, Conn>(req: XIQueryDeviceRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIQueryDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_query_device<Conn, A>(conn: &Conn, deviceid: A) -> Result<Cookie<'_, Conn, XIQueryDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIQueryDeviceRequest {
        deviceid,
    };
    send_xi_query_device(request0, conn)
}

fn send_xi_set_focus<'c, Conn>(req: XISetFocusRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_set_focus<Conn, A, B>(conn: &Conn, window: xproto::Window, time: A, deviceid: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XISetFocusRequest {
        window,
        time,
        deviceid,
    };
    send_xi_set_focus(request0, conn)
}

fn send_xi_get_focus<'c, Conn>(req: XIGetFocusRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIGetFocusReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_get_focus<Conn, A>(conn: &Conn, deviceid: A) -> Result<Cookie<'_, Conn, XIGetFocusReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGetFocusRequest {
        deviceid,
    };
    send_xi_get_focus(request0, conn)
}

fn send_xi_grab_device<'c, Conn>(req: XIGrabDeviceRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIGrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_grab_device<'c, 'input, Conn, A, B>(conn: &'c Conn, window: xproto::Window, time: A, cursor: xproto::Cursor, deviceid: B, mode: xproto::GrabMode, paired_device_mode: xproto::GrabMode, owner_events: GrabOwner, mask: &'input [u32]) -> Result<Cookie<'c, Conn, XIGrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGrabDeviceRequest {
        window,
        time,
        cursor,
        deviceid,
        mode,
        paired_device_mode,
        owner_events,
        mask: Cow::Borrowed(mask),
    };
    send_xi_grab_device(request0, conn)
}

fn send_xi_ungrab_device<'c, Conn>(req: XIUngrabDeviceRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_ungrab_device<Conn, A, B>(conn: &Conn, time: A, deviceid: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIUngrabDeviceRequest {
        time,
        deviceid,
    };
    send_xi_ungrab_device(request0, conn)
}

fn send_xi_allow_events<'c, Conn>(req: XIAllowEventsRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_allow_events<Conn, A, B>(conn: &Conn, time: A, deviceid: B, event_mode: EventMode, touchid: u32, grab_window: xproto::Window) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIAllowEventsRequest {
        time,
        deviceid,
        event_mode,
        touchid,
        grab_window,
    };
    send_xi_allow_events(request0, conn)
}

fn send_xi_passive_grab_device<'c, Conn>(req: XIPassiveGrabDeviceRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIPassiveGrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_passive_grab_device<'c, 'input, Conn, A, B>(conn: &'c Conn, time: A, grab_window: xproto::Window, cursor: xproto::Cursor, detail: u32, deviceid: B, grab_type: GrabType, grab_mode: GrabMode22, paired_device_mode: xproto::GrabMode, owner_events: GrabOwner, mask: &'input [u32], modifiers: &'input [u32]) -> Result<Cookie<'c, Conn, XIPassiveGrabDeviceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xproto::Timestamp>,
    B: Into<DeviceId>,
{
    let time: xproto::Timestamp = time.into();
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIPassiveGrabDeviceRequest {
        time,
        grab_window,
        cursor,
        detail,
        deviceid,
        grab_type,
        grab_mode,
        paired_device_mode,
        owner_events,
        mask: Cow::Borrowed(mask),
        modifiers: Cow::Borrowed(modifiers),
    };
    send_xi_passive_grab_device(request0, conn)
}

fn send_xi_passive_ungrab_device<'c, Conn>(req: XIPassiveUngrabDeviceRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_passive_ungrab_device<'c, 'input, Conn, A>(conn: &'c Conn, grab_window: xproto::Window, detail: u32, deviceid: A, grab_type: GrabType, modifiers: &'input [u32]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIPassiveUngrabDeviceRequest {
        grab_window,
        detail,
        deviceid,
        grab_type,
        modifiers: Cow::Borrowed(modifiers),
    };
    send_xi_passive_ungrab_device(request0, conn)
}

fn send_xi_list_properties<'c, Conn>(req: XIListPropertiesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_list_properties<Conn, A>(conn: &Conn, deviceid: A) -> Result<Cookie<'_, Conn, XIListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIListPropertiesRequest {
        deviceid,
    };
    send_xi_list_properties(request0, conn)
}

fn send_xi_change_property<'c, Conn>(req: XIChangePropertyRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_change_property<'c, 'input, Conn, A>(conn: &'c Conn, deviceid: A, mode: xproto::PropMode, property: xproto::Atom, type_: xproto::Atom, num_items: u32, items: &'input XIChangePropertyAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIChangePropertyRequest {
        deviceid,
        mode,
        property,
        type_,
        num_items,
        items: Cow::Borrowed(items),
    };
    send_xi_change_property(request0, conn)
}

fn send_xi_delete_property<'c, Conn>(req: XIDeletePropertyRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_delete_property<Conn, A>(conn: &Conn, deviceid: A, property: xproto::Atom) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIDeletePropertyRequest {
        deviceid,
        property,
    };
    send_xi_delete_property(request0, conn)
}

fn send_xi_get_property<'c, Conn>(req: XIGetPropertyRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIGetPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_get_property<Conn, A>(conn: &Conn, deviceid: A, delete: bool, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32) -> Result<Cookie<'_, Conn, XIGetPropertyReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<DeviceId>,
{
    let deviceid: DeviceId = deviceid.into();
    let request0 = XIGetPropertyRequest {
        deviceid,
        delete,
        property,
        type_,
        offset,
        len,
    };
    send_xi_get_property(request0, conn)
}

fn send_xi_get_selected_events<'c, Conn>(req: XIGetSelectedEventsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, XIGetSelectedEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn xi_get_selected_events<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, XIGetSelectedEventsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XIGetSelectedEventsRequest {
        window,
    };
    send_xi_get_selected_events(request0, conn)
}

fn send_xi_barrier_release_pointer<'c, Conn>(req: XIBarrierReleasePointerRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn xi_barrier_release_pointer<'c, 'input, Conn>(conn: &'c Conn, barriers: &'input [BarrierReleasePointerInfo]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = XIBarrierReleasePointerRequest {
        barriers: Cow::Borrowed(barriers),
    };
    send_xi_barrier_release_pointer(request0, conn)
}

fn send_send_extension_event<'c, Conn>(req: SendExtensionEventRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn send_extension_event<'c, 'input, Conn>(conn: &'c Conn, destination: xproto::Window, device_id: u8, propagate: bool, events: &'input [EventForSend], classes: &'input [EventClass]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SendExtensionEventRequest {
        destination,
        device_id,
        propagate,
        events: Cow::Borrowed(events),
        classes: Cow::Borrowed(classes),
    };
    send_send_extension_event(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xinput_get_extension_version<'c, 'input>(&'c self, name: &'input [u8]) -> Result<Cookie<'c, Self, GetExtensionVersionReply>, ConnectionError>
    {
        get_extension_version(self, name)
    }
    fn xinput_list_input_devices(&self) -> Result<Cookie<'_, Self, ListInputDevicesReply>, ConnectionError>
    {
        list_input_devices(self)
    }
    fn xinput_open_device(&self, device_id: u8) -> Result<Cookie<'_, Self, OpenDeviceReply>, ConnectionError>
    {
        open_device(self, device_id)
    }
    fn xinput_close_device(&self, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        close_device(self, device_id)
    }
    fn xinput_set_device_mode(&self, device_id: u8, mode: ValuatorMode) -> Result<Cookie<'_, Self, SetDeviceModeReply>, ConnectionError>
    {
        set_device_mode(self, device_id, mode)
    }
    fn xinput_select_extension_event<'c, 'input>(&'c self, window: xproto::Window, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        select_extension_event(self, window, classes)
    }
    fn xinput_get_selected_extension_events(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetSelectedExtensionEventsReply>, ConnectionError>
    {
        get_selected_extension_events(self, window)
    }
    fn xinput_change_device_dont_propagate_list<'c, 'input>(&'c self, window: xproto::Window, mode: PropagateMode, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_device_dont_propagate_list(self, window, mode, classes)
    }
    fn xinput_get_device_dont_propagate_list(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetDeviceDontPropagateListReply>, ConnectionError>
    {
        get_device_dont_propagate_list(self, window)
    }
    fn xinput_get_device_motion_events<A>(&self, start: xproto::Timestamp, stop: A, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceMotionEventsReply>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        get_device_motion_events(self, start, stop, device_id)
    }
    fn xinput_change_keyboard_device(&self, device_id: u8) -> Result<Cookie<'_, Self, ChangeKeyboardDeviceReply>, ConnectionError>
    {
        change_keyboard_device(self, device_id)
    }
    fn xinput_change_pointer_device(&self, x_axis: u8, y_axis: u8, device_id: u8) -> Result<Cookie<'_, Self, ChangePointerDeviceReply>, ConnectionError>
    {
        change_pointer_device(self, x_axis, y_axis, device_id)
    }
    fn xinput_grab_device<'c, 'input, A>(&'c self, grab_window: xproto::Window, time: A, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, owner_events: bool, device_id: u8, classes: &'input [EventClass]) -> Result<Cookie<'c, Self, GrabDeviceReply>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        grab_device(self, grab_window, time, this_device_mode, other_device_mode, owner_events, device_id, classes)
    }
    fn xinput_ungrab_device<A>(&self, time: A, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        ungrab_device(self, time, device_id)
    }
    fn xinput_grab_device_key<'c, 'input, A, B, C>(&'c self, grab_window: xproto::Window, modifiers: A, modifier_device: B, grabbed_device: u8, key: C, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, owner_events: bool, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u8>,
        C: Into<u8>,
    {
        grab_device_key(self, grab_window, modifiers, modifier_device, grabbed_device, key, this_device_mode, other_device_mode, owner_events, classes)
    }
    fn xinput_ungrab_device_key<A, B, C>(&self, grab_window: xproto::Window, modifiers: A, modifier_device: B, key: C, grabbed_device: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u8>,
        C: Into<u8>,
    {
        ungrab_device_key(self, grab_window, modifiers, modifier_device, key, grabbed_device)
    }
    fn xinput_grab_device_button<'c, 'input, A, B, C>(&'c self, grab_window: xproto::Window, grabbed_device: u8, modifier_device: A, modifiers: B, this_device_mode: xproto::GrabMode, other_device_mode: xproto::GrabMode, button: C, owner_events: bool, classes: &'input [EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u8>,
        B: Into<u16>,
        C: Into<u8>,
    {
        grab_device_button(self, grab_window, grabbed_device, modifier_device, modifiers, this_device_mode, other_device_mode, button, owner_events, classes)
    }
    fn xinput_ungrab_device_button<A, B, C>(&self, grab_window: xproto::Window, modifiers: A, modifier_device: B, button: C, grabbed_device: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u16>,
        B: Into<u8>,
        C: Into<u8>,
    {
        ungrab_device_button(self, grab_window, modifiers, modifier_device, button, grabbed_device)
    }
    fn xinput_allow_device_events<A>(&self, time: A, mode: DeviceInputMode, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
    {
        allow_device_events(self, time, mode, device_id)
    }
    fn xinput_get_device_focus(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceFocusReply>, ConnectionError>
    {
        get_device_focus(self, device_id)
    }
    fn xinput_set_device_focus<A, B>(&self, focus: A, time: B, revert_to: xproto::InputFocus, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Window>,
        B: Into<xproto::Timestamp>,
    {
        set_device_focus(self, focus, time, revert_to, device_id)
    }
    fn xinput_get_feedback_control(&self, device_id: u8) -> Result<Cookie<'_, Self, GetFeedbackControlReply>, ConnectionError>
    {
        get_feedback_control(self, device_id)
    }
    fn xinput_change_feedback_control<A>(&self, mask: A, device_id: u8, feedback_id: u8, feedback: FeedbackCtl) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        change_feedback_control(self, mask, device_id, feedback_id, feedback)
    }
    fn xinput_get_device_key_mapping(&self, device_id: u8, first_keycode: KeyCode, count: u8) -> Result<Cookie<'_, Self, GetDeviceKeyMappingReply>, ConnectionError>
    {
        get_device_key_mapping(self, device_id, first_keycode, count)
    }
    fn xinput_change_device_key_mapping<'c, 'input>(&'c self, device_id: u8, first_keycode: KeyCode, keysyms_per_keycode: u8, keycode_count: u8, keysyms: &'input [xproto::Keysym]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_device_key_mapping(self, device_id, first_keycode, keysyms_per_keycode, keycode_count, keysyms)
    }
    fn xinput_get_device_modifier_mapping(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceModifierMappingReply>, ConnectionError>
    {
        get_device_modifier_mapping(self, device_id)
    }
    fn xinput_set_device_modifier_mapping<'c, 'input>(&'c self, device_id: u8, keymaps: &'input [u8]) -> Result<Cookie<'c, Self, SetDeviceModifierMappingReply>, ConnectionError>
    {
        set_device_modifier_mapping(self, device_id, keymaps)
    }
    fn xinput_get_device_button_mapping(&self, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceButtonMappingReply>, ConnectionError>
    {
        get_device_button_mapping(self, device_id)
    }
    fn xinput_set_device_button_mapping<'c, 'input>(&'c self, device_id: u8, map: &'input [u8]) -> Result<Cookie<'c, Self, SetDeviceButtonMappingReply>, ConnectionError>
    {
        set_device_button_mapping(self, device_id, map)
    }
    fn xinput_query_device_state(&self, device_id: u8) -> Result<Cookie<'_, Self, QueryDeviceStateReply>, ConnectionError>
    {
        query_device_state(self, device_id)
    }
    fn xinput_device_bell(&self, device_id: u8, feedback_id: u8, feedback_class: u8, percent: i8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        device_bell(self, device_id, feedback_id, feedback_class, percent)
    }
    fn xinput_set_device_valuators<'c, 'input>(&'c self, device_id: u8, first_valuator: u8, valuators: &'input [i32]) -> Result<Cookie<'c, Self, SetDeviceValuatorsReply>, ConnectionError>
    {
        set_device_valuators(self, device_id, first_valuator, valuators)
    }
    fn xinput_get_device_control(&self, control_id: DeviceControl, device_id: u8) -> Result<Cookie<'_, Self, GetDeviceControlReply>, ConnectionError>
    {
        get_device_control(self, control_id, device_id)
    }
    fn xinput_change_device_control(&self, control_id: DeviceControl, device_id: u8, control: DeviceCtl) -> Result<Cookie<'_, Self, ChangeDeviceControlReply>, ConnectionError>
    {
        change_device_control(self, control_id, device_id, control)
    }
    fn xinput_list_device_properties(&self, device_id: u8) -> Result<Cookie<'_, Self, ListDevicePropertiesReply>, ConnectionError>
    {
        list_device_properties(self, device_id)
    }
    fn xinput_change_device_property<'c, 'input>(&'c self, property: xproto::Atom, type_: xproto::Atom, device_id: u8, mode: xproto::PropMode, num_items: u32, items: &'input ChangeDevicePropertyAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        change_device_property(self, property, type_, device_id, mode, num_items, items)
    }
    fn xinput_delete_device_property(&self, property: xproto::Atom, device_id: u8) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        delete_device_property(self, property, device_id)
    }
    fn xinput_get_device_property(&self, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32, device_id: u8, delete: bool) -> Result<Cookie<'_, Self, GetDevicePropertyReply>, ConnectionError>
    {
        get_device_property(self, property, type_, offset, len, device_id, delete)
    }
    fn xinput_xi_query_pointer<A>(&self, window: xproto::Window, deviceid: A) -> Result<Cookie<'_, Self, XIQueryPointerReply>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_query_pointer(self, window, deviceid)
    }
    fn xinput_xi_warp_pointer<A>(&self, src_win: xproto::Window, dst_win: xproto::Window, src_x: Fp1616, src_y: Fp1616, src_width: u16, src_height: u16, dst_x: Fp1616, dst_y: Fp1616, deviceid: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_warp_pointer(self, src_win, dst_win, src_x, src_y, src_width, src_height, dst_x, dst_y, deviceid)
    }
    fn xinput_xi_change_cursor<A>(&self, window: xproto::Window, cursor: xproto::Cursor, deviceid: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_change_cursor(self, window, cursor, deviceid)
    }
    fn xinput_xi_change_hierarchy<'c, 'input>(&'c self, changes: &'input [HierarchyChange]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_change_hierarchy(self, changes)
    }
    fn xinput_xi_set_client_pointer<A>(&self, window: xproto::Window, deviceid: A) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_set_client_pointer(self, window, deviceid)
    }
    fn xinput_xi_get_client_pointer(&self, window: xproto::Window) -> Result<Cookie<'_, Self, XIGetClientPointerReply>, ConnectionError>
    {
        xi_get_client_pointer(self, window)
    }
    fn xinput_xi_select_events<'c, 'input>(&'c self, window: xproto::Window, masks: &'input [EventMask]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_select_events(self, window, masks)
    }
    fn xinput_xi_query_version(&self, major_version: u16, minor_version: u16) -> Result<Cookie<'_, Self, XIQueryVersionReply>, ConnectionError>
    {
        xi_query_version(self, major_version, minor_version)
    }
    fn xinput_xi_query_device<A>(&self, deviceid: A) -> Result<Cookie<'_, Self, XIQueryDeviceReply>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_query_device(self, deviceid)
    }
    fn xinput_xi_set_focus<A, B>(&self, window: xproto::Window, time: A, deviceid: B) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
        B: Into<DeviceId>,
    {
        xi_set_focus(self, window, time, deviceid)
    }
    fn xinput_xi_get_focus<A>(&self, deviceid: A) -> Result<Cookie<'_, Self, XIGetFocusReply>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_get_focus(self, deviceid)
    }
    fn xinput_xi_grab_device<'c, 'input, A, B>(&'c self, window: xproto::Window, time: A, cursor: xproto::Cursor, deviceid: B, mode: xproto::GrabMode, paired_device_mode: xproto::GrabMode, owner_events: GrabOwner, mask: &'input [u32]) -> Result<Cookie<'c, Self, XIGrabDeviceReply>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
        B: Into<DeviceId>,
    {
        xi_grab_device(self, window, time, cursor, deviceid, mode, paired_device_mode, owner_events, mask)
    }
    fn xinput_xi_ungrab_device<A, B>(&self, time: A, deviceid: B) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
        B: Into<DeviceId>,
    {
        xi_ungrab_device(self, time, deviceid)
    }
    fn xinput_xi_allow_events<A, B>(&self, time: A, deviceid: B, event_mode: EventMode, touchid: u32, grab_window: xproto::Window) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
        B: Into<DeviceId>,
    {
        xi_allow_events(self, time, deviceid, event_mode, touchid, grab_window)
    }
    fn xinput_xi_passive_grab_device<'c, 'input, A, B>(&'c self, time: A, grab_window: xproto::Window, cursor: xproto::Cursor, detail: u32, deviceid: B, grab_type: GrabType, grab_mode: GrabMode22, paired_device_mode: xproto::GrabMode, owner_events: GrabOwner, mask: &'input [u32], modifiers: &'input [u32]) -> Result<Cookie<'c, Self, XIPassiveGrabDeviceReply>, ConnectionError>
    where
        A: Into<xproto::Timestamp>,
        B: Into<DeviceId>,
    {
        xi_passive_grab_device(self, time, grab_window, cursor, detail, deviceid, grab_type, grab_mode, paired_device_mode, owner_events, mask, modifiers)
    }
    fn xinput_xi_passive_ungrab_device<'c, 'input, A>(&'c self, grab_window: xproto::Window, detail: u32, deviceid: A, grab_type: GrabType, modifiers: &'input [u32]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_passive_ungrab_device(self, grab_window, detail, deviceid, grab_type, modifiers)
    }
    fn xinput_xi_list_properties<A>(&self, deviceid: A) -> Result<Cookie<'_, Self, XIListPropertiesReply>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_list_properties(self, deviceid)
    }
    fn xinput_xi_change_property<'c, 'input, A>(&'c self, deviceid: A, mode: xproto::PropMode, property: xproto::Atom, type_: xproto::Atom, num_items: u32, items: &'input XIChangePropertyAux) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_change_property(self, deviceid, mode, property, type_, num_items, items)
    }
    fn xinput_xi_delete_property<A>(&self, deviceid: A, property: xproto::Atom) -> Result<VoidCookie<'_, Self>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_delete_property(self, deviceid, property)
    }
    fn xinput_xi_get_property<A>(&self, deviceid: A, delete: bool, property: xproto::Atom, type_: xproto::Atom, offset: u32, len: u32) -> Result<Cookie<'_, Self, XIGetPropertyReply>, ConnectionError>
    where
        A: Into<DeviceId>,
    {
        xi_get_property(self, deviceid, delete, property, type_, offset, len)
    }
    fn xinput_xi_get_selected_events(&self, window: xproto::Window) -> Result<Cookie<'_, Self, XIGetSelectedEventsReply>, ConnectionError>
    {
        xi_get_selected_events(self, window)
    }
    fn xinput_xi_barrier_release_pointer<'c, 'input>(&'c self, barriers: &'input [BarrierReleasePointerInfo]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        xi_barrier_release_pointer(self, barriers)
    }
    fn xinput_send_extension_event<'c, 'input>(&'c self, destination: xproto::Window, device_id: u8, propagate: bool, events: &'input [EventForSend], classes: &'input [EventClass]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        send_extension_event(self, destination, device_id, propagate, events, classes)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

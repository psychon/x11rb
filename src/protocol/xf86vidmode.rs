// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `XF86VidMode` X11 extension.

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

pub use x11rb_protocol::protocol::xf86vidmode::*;

/// Get the major opcode of this extension
fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME)?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

fn send_query_version<'c, Conn>(req: QueryVersionRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest;
    send_query_version(request0, conn)
}

fn send_get_mode_line<'c, Conn>(req: GetModeLineRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_mode_line<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetModeLineRequest {
        screen,
    };
    send_get_mode_line(request0, conn)
}

fn send_mod_mode_line<'c, Conn>(req: ModModeLineRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn mod_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ModModeLineRequest {
        screen,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    send_mod_mode_line(request0, conn)
}

fn send_switch_mode<'c, Conn>(req: SwitchModeRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn switch_mode<Conn>(conn: &Conn, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SwitchModeRequest {
        screen,
        zoom,
    };
    send_switch_mode(request0, conn)
}

fn send_get_monitor<'c, Conn>(req: GetMonitorRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetMonitorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_monitor<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetMonitorReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetMonitorRequest {
        screen,
    };
    send_get_monitor(request0, conn)
}

fn send_lock_mode_switch<'c, Conn>(req: LockModeSwitchRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn lock_mode_switch<Conn>(conn: &Conn, screen: u16, lock: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = LockModeSwitchRequest {
        screen,
        lock,
    };
    send_lock_mode_switch(request0, conn)
}

fn send_get_all_mode_lines<'c, Conn>(req: GetAllModeLinesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetAllModeLinesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_all_mode_lines<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetAllModeLinesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetAllModeLinesRequest {
        screen,
    };
    send_get_all_mode_lines(request0, conn)
}

fn send_add_mode_line<'c, Conn>(req: AddModeLineRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn add_mode_line<'c, 'input, Conn, A, B>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, after_dotclock: Dotclock, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: B, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
    B: Into<u32>,
{
    let flags: u32 = flags.into();
    let after_flags: u32 = after_flags.into();
    let request0 = AddModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        after_dotclock,
        after_hdisplay,
        after_hsyncstart,
        after_hsyncend,
        after_htotal,
        after_hskew,
        after_vdisplay,
        after_vsyncstart,
        after_vsyncend,
        after_vtotal,
        after_flags,
        private: Cow::Borrowed(private),
    };
    send_add_mode_line(request0, conn)
}

fn send_delete_mode_line<'c, Conn>(req: DeleteModeLineRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn delete_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = DeleteModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    send_delete_mode_line(request0, conn)
}

fn send_validate_mode_line<'c, Conn>(req: ValidateModeLineRequest<'_>, conn: &'c Conn) -> Result<Cookie<'c, Conn, ValidateModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn validate_mode_line<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<Cookie<'c, Conn, ValidateModeLineReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = ValidateModeLineRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    send_validate_mode_line(request0, conn)
}

fn send_switch_to_mode<'c, Conn>(req: SwitchToModeRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn switch_to_mode<'c, 'input, Conn, A>(conn: &'c Conn, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<u32>,
{
    let flags: u32 = flags.into();
    let request0 = SwitchToModeRequest {
        screen,
        dotclock,
        hdisplay,
        hsyncstart,
        hsyncend,
        htotal,
        hskew,
        vdisplay,
        vsyncstart,
        vsyncend,
        vtotal,
        flags,
        private: Cow::Borrowed(private),
    };
    send_switch_to_mode(request0, conn)
}

fn send_get_view_port<'c, Conn>(req: GetViewPortRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetViewPortReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_view_port<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetViewPortReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetViewPortRequest {
        screen,
    };
    send_get_view_port(request0, conn)
}

fn send_set_view_port<'c, Conn>(req: SetViewPortRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_view_port<Conn>(conn: &Conn, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetViewPortRequest {
        screen,
        x,
        y,
    };
    send_set_view_port(request0, conn)
}

fn send_get_dot_clocks<'c, Conn>(req: GetDotClocksRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDotClocksReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_dot_clocks<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetDotClocksReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDotClocksRequest {
        screen,
    };
    send_get_dot_clocks(request0, conn)
}

fn send_set_client_version<'c, Conn>(req: SetClientVersionRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_client_version<Conn>(conn: &Conn, major: u16, minor: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetClientVersionRequest {
        major,
        minor,
    };
    send_set_client_version(request0, conn)
}

fn send_set_gamma<'c, Conn>(req: SetGammaRequest, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_gamma<Conn>(conn: &Conn, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetGammaRequest {
        screen,
        red,
        green,
        blue,
    };
    send_set_gamma(request0, conn)
}

fn send_get_gamma<'c, Conn>(req: GetGammaRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetGammaReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_gamma<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRequest {
        screen,
    };
    send_get_gamma(request0, conn)
}

fn send_get_gamma_ramp<'c, Conn>(req: GetGammaRampRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetGammaRampReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_gamma_ramp<Conn>(conn: &Conn, screen: u16, size: u16) -> Result<Cookie<'_, Conn, GetGammaRampReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRampRequest {
        screen,
        size,
    };
    send_get_gamma_ramp(request0, conn)
}

fn send_set_gamma_ramp<'c, Conn>(req: SetGammaRampRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_gamma_ramp<'c, 'input, Conn>(conn: &'c Conn, screen: u16, size: u16, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetGammaRampRequest {
        screen,
        size,
        red: Cow::Borrowed(red),
        green: Cow::Borrowed(green),
        blue: Cow::Borrowed(blue),
    };
    send_set_gamma_ramp(request0, conn)
}

fn send_get_gamma_ramp_size<'c, Conn>(req: GetGammaRampSizeRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetGammaRampSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_gamma_ramp_size<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaRampSizeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetGammaRampSizeRequest {
        screen,
    };
    send_get_gamma_ramp_size(request0, conn)
}

fn send_get_permissions<'c, Conn>(req: GetPermissionsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPermissionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_permissions<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetPermissionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPermissionsRequest {
        screen,
    };
    send_get_permissions(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86vidmode_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }
    fn xf86vidmode_get_mode_line(&self, screen: u16) -> Result<Cookie<'_, Self, GetModeLineReply>, ConnectionError>
    {
        get_mode_line(self, screen)
    }
    fn xf86vidmode_mod_mode_line<'c, 'input, A>(&'c self, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        mod_mode_line(self, screen, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_switch_mode(&self, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        switch_mode(self, screen, zoom)
    }
    fn xf86vidmode_get_monitor(&self, screen: u16) -> Result<Cookie<'_, Self, GetMonitorReply>, ConnectionError>
    {
        get_monitor(self, screen)
    }
    fn xf86vidmode_lock_mode_switch(&self, screen: u16, lock: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        lock_mode_switch(self, screen, lock)
    }
    fn xf86vidmode_get_all_mode_lines(&self, screen: u16) -> Result<Cookie<'_, Self, GetAllModeLinesReply>, ConnectionError>
    {
        get_all_mode_lines(self, screen)
    }
    fn xf86vidmode_add_mode_line<'c, 'input, A, B>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, after_dotclock: Dotclock, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: B, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
        B: Into<u32>,
    {
        add_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, after_dotclock, after_hdisplay, after_hsyncstart, after_hsyncend, after_htotal, after_hskew, after_vdisplay, after_vsyncstart, after_vsyncend, after_vtotal, after_flags, private)
    }
    fn xf86vidmode_delete_mode_line<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        delete_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_validate_mode_line<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<Cookie<'c, Self, ValidateModeLineReply>, ConnectionError>
    where
        A: Into<u32>,
    {
        validate_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_switch_to_mode<'c, 'input, A>(&'c self, screen: u32, dotclock: Dotclock, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: A, private: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    where
        A: Into<u32>,
    {
        switch_to_mode(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }
    fn xf86vidmode_get_view_port(&self, screen: u16) -> Result<Cookie<'_, Self, GetViewPortReply>, ConnectionError>
    {
        get_view_port(self, screen)
    }
    fn xf86vidmode_set_view_port(&self, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_view_port(self, screen, x, y)
    }
    fn xf86vidmode_get_dot_clocks(&self, screen: u16) -> Result<Cookie<'_, Self, GetDotClocksReply>, ConnectionError>
    {
        get_dot_clocks(self, screen)
    }
    fn xf86vidmode_set_client_version(&self, major: u16, minor: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_client_version(self, major, minor)
    }
    fn xf86vidmode_set_gamma(&self, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_gamma(self, screen, red, green, blue)
    }
    fn xf86vidmode_get_gamma(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaReply>, ConnectionError>
    {
        get_gamma(self, screen)
    }
    fn xf86vidmode_get_gamma_ramp(&self, screen: u16, size: u16) -> Result<Cookie<'_, Self, GetGammaRampReply>, ConnectionError>
    {
        get_gamma_ramp(self, screen, size)
    }
    fn xf86vidmode_set_gamma_ramp<'c, 'input>(&'c self, screen: u16, size: u16, red: &'input [u16], green: &'input [u16], blue: &'input [u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_gamma_ramp(self, screen, size, red, green, blue)
    }
    fn xf86vidmode_get_gamma_ramp_size(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaRampSizeReply>, ConnectionError>
    {
        get_gamma_ramp_size(self, screen)
    }
    fn xf86vidmode_get_permissions(&self, screen: u16) -> Result<Cookie<'_, Self, GetPermissionsReply>, ConnectionError>
    {
        get_permissions(self, screen)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

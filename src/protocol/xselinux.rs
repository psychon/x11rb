// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `SELinux` X11 extension.

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
use super::xproto;

pub use x11rb_protocol::protocol::xselinux::*;

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
pub fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
    };
    send_query_version(request0, conn)
}

fn send_set_device_create_context<'c, Conn>(req: SetDeviceCreateContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_device_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_device_create_context(request0, conn)
}

fn send_get_device_create_context<'c, Conn>(req: GetDeviceCreateContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceCreateContextRequest;
    send_get_device_create_context(request0, conn)
}

fn send_set_device_context<'c, Conn>(req: SetDeviceContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_device_context<'c, 'input, Conn>(conn: &'c Conn, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceContextRequest {
        device,
        context: Cow::Borrowed(context),
    };
    send_set_device_context(request0, conn)
}

fn send_get_device_context<'c, Conn>(req: GetDeviceContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetDeviceContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_device_context<Conn>(conn: &Conn, device: u32) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceContextRequest {
        device,
    };
    send_get_device_context(request0, conn)
}

fn send_set_window_create_context<'c, Conn>(req: SetWindowCreateContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_window_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetWindowCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_window_create_context(request0, conn)
}

fn send_get_window_create_context<'c, Conn>(req: GetWindowCreateContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetWindowCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_window_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowCreateContextRequest;
    send_get_window_create_context(request0, conn)
}

fn send_get_window_context<'c, Conn>(req: GetWindowContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetWindowContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_window_context<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowContextRequest {
        window,
    };
    send_get_window_context(request0, conn)
}

fn send_set_property_create_context<'c, Conn>(req: SetPropertyCreateContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_property_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_property_create_context(request0, conn)
}

fn send_get_property_create_context<'c, Conn>(req: GetPropertyCreateContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPropertyCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_property_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyCreateContextRequest;
    send_get_property_create_context(request0, conn)
}

fn send_set_property_use_context<'c, Conn>(req: SetPropertyUseContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_property_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyUseContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_property_use_context(request0, conn)
}

fn send_get_property_use_context<'c, Conn>(req: GetPropertyUseContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPropertyUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_property_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyUseContextRequest;
    send_get_property_use_context(request0, conn)
}

fn send_get_property_context<'c, Conn>(req: GetPropertyContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPropertyContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_property_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyContextRequest {
        window,
        property,
    };
    send_get_property_context(request0, conn)
}

fn send_get_property_data_context<'c, Conn>(req: GetPropertyDataContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetPropertyDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_property_data_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyDataContextRequest {
        window,
        property,
    };
    send_get_property_data_context(request0, conn)
}

fn send_list_properties<'c, Conn>(req: ListPropertiesRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn list_properties<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListPropertiesRequest {
        window,
    };
    send_list_properties(request0, conn)
}

fn send_set_selection_create_context<'c, Conn>(req: SetSelectionCreateContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_selection_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_selection_create_context(request0, conn)
}

fn send_get_selection_create_context<'c, Conn>(req: GetSelectionCreateContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSelectionCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_selection_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionCreateContextRequest;
    send_get_selection_create_context(request0, conn)
}

fn send_set_selection_use_context<'c, Conn>(req: SetSelectionUseContextRequest<'_>, conn: &'c Conn) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_without_reply(&slices, fds)
}
pub fn set_selection_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionUseContextRequest {
        context: Cow::Borrowed(context),
    };
    send_set_selection_use_context(request0, conn)
}

fn send_get_selection_use_context<'c, Conn>(req: GetSelectionUseContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSelectionUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_selection_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionUseContextRequest;
    send_get_selection_use_context(request0, conn)
}

fn send_get_selection_context<'c, Conn>(req: GetSelectionContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSelectionContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_selection_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionContextRequest {
        selection,
    };
    send_get_selection_context(request0, conn)
}

fn send_get_selection_data_context<'c, Conn>(req: GetSelectionDataContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetSelectionDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_selection_data_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionDataContextRequest {
        selection,
    };
    send_get_selection_data_context(request0, conn)
}

fn send_list_selections<'c, Conn>(req: ListSelectionsRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, ListSelectionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn list_selections<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSelectionsRequest;
    send_list_selections(request0, conn)
}

fn send_get_client_context<'c, Conn>(req: GetClientContextRequest, conn: &'c Conn) -> Result<Cookie<'c, Conn, GetClientContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let (bytes, fds) = req.serialize(major_opcode(conn)?);
    let slices = bytes.iter().map(|b| IoSlice::new(&*b)).collect::<Vec<_>>();
    conn.send_request_with_reply(&slices, fds)
}
pub fn get_client_context<Conn>(conn: &Conn, resource: u32) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientContextRequest {
        resource,
    };
    send_get_client_context(request0, conn)
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xselinux_query_version(&self, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self, client_major, client_minor)
    }
    fn xselinux_set_device_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_create_context(self, context)
    }
    fn xselinux_get_device_create_context(&self) -> Result<Cookie<'_, Self, GetDeviceCreateContextReply>, ConnectionError>
    {
        get_device_create_context(self)
    }
    fn xselinux_set_device_context<'c, 'input>(&'c self, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_device_context(self, device, context)
    }
    fn xselinux_get_device_context(&self, device: u32) -> Result<Cookie<'_, Self, GetDeviceContextReply>, ConnectionError>
    {
        get_device_context(self, device)
    }
    fn xselinux_set_window_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_window_create_context(self, context)
    }
    fn xselinux_get_window_create_context(&self) -> Result<Cookie<'_, Self, GetWindowCreateContextReply>, ConnectionError>
    {
        get_window_create_context(self)
    }
    fn xselinux_get_window_context(&self, window: xproto::Window) -> Result<Cookie<'_, Self, GetWindowContextReply>, ConnectionError>
    {
        get_window_context(self, window)
    }
    fn xselinux_set_property_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_create_context(self, context)
    }
    fn xselinux_get_property_create_context(&self) -> Result<Cookie<'_, Self, GetPropertyCreateContextReply>, ConnectionError>
    {
        get_property_create_context(self)
    }
    fn xselinux_set_property_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_property_use_context(self, context)
    }
    fn xselinux_get_property_use_context(&self) -> Result<Cookie<'_, Self, GetPropertyUseContextReply>, ConnectionError>
    {
        get_property_use_context(self)
    }
    fn xselinux_get_property_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyContextReply>, ConnectionError>
    {
        get_property_context(self, window, property)
    }
    fn xselinux_get_property_data_context(&self, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Self, GetPropertyDataContextReply>, ConnectionError>
    {
        get_property_data_context(self, window, property)
    }
    fn xselinux_list_properties(&self, window: xproto::Window) -> Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>
    {
        list_properties(self, window)
    }
    fn xselinux_set_selection_create_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_create_context(self, context)
    }
    fn xselinux_get_selection_create_context(&self) -> Result<Cookie<'_, Self, GetSelectionCreateContextReply>, ConnectionError>
    {
        get_selection_create_context(self)
    }
    fn xselinux_set_selection_use_context<'c, 'input>(&'c self, context: &'input [u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_selection_use_context(self, context)
    }
    fn xselinux_get_selection_use_context(&self) -> Result<Cookie<'_, Self, GetSelectionUseContextReply>, ConnectionError>
    {
        get_selection_use_context(self)
    }
    fn xselinux_get_selection_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionContextReply>, ConnectionError>
    {
        get_selection_context(self, selection)
    }
    fn xselinux_get_selection_data_context(&self, selection: xproto::Atom) -> Result<Cookie<'_, Self, GetSelectionDataContextReply>, ConnectionError>
    {
        get_selection_data_context(self, selection)
    }
    fn xselinux_list_selections(&self) -> Result<Cookie<'_, Self, ListSelectionsReply>, ConnectionError>
    {
        list_selections(self)
    }
    fn xselinux_get_client_context(&self, resource: u32) -> Result<Cookie<'_, Self, GetClientContextReply>, ConnectionError>
    {
        get_client_context(self, resource)
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

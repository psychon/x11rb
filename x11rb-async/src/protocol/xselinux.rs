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
use std::future::Future;
use std::pin::Pin;
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::xselinux::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn query_version<Conn>(conn: &Conn, client_major: u8, client_minor: u8) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major,
        client_minor,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_device_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_device_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetDeviceCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceCreateContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_device_context<'c, 'input, Conn>(conn: &'c Conn, device: u32, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetDeviceContextRequest {
        device,
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_device_context<Conn>(conn: &Conn, device: u32) -> Result<Cookie<'_, Conn, GetDeviceContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetDeviceContextRequest {
        device,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_window_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetWindowCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_window_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetWindowCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowCreateContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_window_context<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, GetWindowContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetWindowContextRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_property_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_property_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyCreateContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_property_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPropertyUseContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_property_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetPropertyUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyUseContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_property_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyContextRequest {
        window,
        property,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_property_data_context<Conn>(conn: &Conn, window: xproto::Window, property: xproto::Atom) -> Result<Cookie<'_, Conn, GetPropertyDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPropertyDataContextRequest {
        window,
        property,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn list_properties<Conn>(conn: &Conn, window: xproto::Window) -> Result<Cookie<'_, Conn, ListPropertiesReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListPropertiesRequest {
        window,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_selection_create_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionCreateContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_selection_create_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionCreateContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionCreateContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_selection_use_context<'c, 'input, Conn>(conn: &'c Conn, context: &'input [u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetSelectionUseContextRequest {
        context: Cow::Borrowed(context),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_selection_use_context<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, GetSelectionUseContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionUseContextRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_selection_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionContextRequest {
        selection,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_selection_data_context<Conn>(conn: &Conn, selection: xproto::Atom) -> Result<Cookie<'_, Conn, GetSelectionDataContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetSelectionDataContextRequest {
        selection,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn list_selections<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSelectionsReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSelectionsRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn get_client_context<Conn>(conn: &Conn, resource: u32) -> Result<Cookie<'_, Conn, GetClientContextReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetClientContextRequest {
        resource,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xselinux_query_version(&self, client_major: u8, client_minor: u8) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, client_major, client_minor))
    }
    fn xselinux_set_device_create_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_device_create_context(self, context))
    }
    fn xselinux_get_device_create_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetDeviceCreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_device_create_context(self))
    }
    fn xselinux_set_device_context<'c, 'input, 'future>(&'c self, device: u32, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_device_context(self, device, context))
    }
    fn xselinux_get_device_context(&self, device: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetDeviceContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_device_context(self, device))
    }
    fn xselinux_set_window_create_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_window_create_context(self, context))
    }
    fn xselinux_get_window_create_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetWindowCreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_window_create_context(self))
    }
    fn xselinux_get_window_context(&self, window: xproto::Window) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetWindowContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_window_context(self, window))
    }
    fn xselinux_set_property_create_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_property_create_context(self, context))
    }
    fn xselinux_get_property_create_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetPropertyCreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_property_create_context(self))
    }
    fn xselinux_set_property_use_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_property_use_context(self, context))
    }
    fn xselinux_get_property_use_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetPropertyUseContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_property_use_context(self))
    }
    fn xselinux_get_property_context(&self, window: xproto::Window, property: xproto::Atom) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetPropertyContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_property_context(self, window, property))
    }
    fn xselinux_get_property_data_context(&self, window: xproto::Window, property: xproto::Atom) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetPropertyDataContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_property_data_context(self, window, property))
    }
    fn xselinux_list_properties(&self, window: xproto::Window) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ListPropertiesReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(list_properties(self, window))
    }
    fn xselinux_set_selection_create_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_selection_create_context(self, context))
    }
    fn xselinux_get_selection_create_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetSelectionCreateContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_selection_create_context(self))
    }
    fn xselinux_set_selection_use_context<'c, 'input, 'future>(&'c self, context: &'input [u8]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(set_selection_use_context(self, context))
    }
    fn xselinux_get_selection_use_context(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetSelectionUseContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_selection_use_context(self))
    }
    fn xselinux_get_selection_context(&self, selection: xproto::Atom) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetSelectionContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_selection_context(self, selection))
    }
    fn xselinux_get_selection_data_context(&self, selection: xproto::Atom) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetSelectionDataContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_selection_data_context(self, selection))
    }
    fn xselinux_list_selections(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ListSelectionsReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(list_selections(self))
    }
    fn xselinux_get_client_context(&self, resource: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetClientContextReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_client_context(self, resource))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

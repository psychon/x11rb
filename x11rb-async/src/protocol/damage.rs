// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Damage` X11 extension.

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
use super::xfixes;
#[allow(unused_imports)]
use super::xproto;

pub use x11rb_protocol::protocol::damage::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

/// Negotiate the version of the DAMAGE extension.
///
/// This negotiates the version of the DAMAGE extension.  It must precede any other
/// request using the DAMAGE extension.  Failure to do so will cause a BadRequest
/// error for those requests.
///
/// # Fields
///
/// * `client_major_version` - The major version supported by the client.
/// * `client_minor_version` - The minor version supported by the client.
pub async fn query_version<Conn>(conn: &Conn, client_major_version: u32, client_minor_version: u32) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryVersionRequest {
        client_major_version,
        client_minor_version,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
/// Creates a Damage object to monitor changes to a drawable..
///
/// This creates a Damage object to monitor changes to a drawable, and specifies
/// the level of detail to be reported for changes.
///
/// We call changes made to pixel contents of windows and pixmaps 'damage'
/// throughout this extension.
///
/// Damage accumulates as drawing occurs in the drawable.  Each drawing operation
/// 'damages' one or more rectangular areas within the drawable.  The rectangles
/// are guaranteed to include the set of pixels modified by each operation, but
/// may include significantly more than just those pixels.  The desire is for
/// the damage to strike a balance between the number of rectangles reported and
/// the extraneous area included.  A reasonable goal is for each primitive
/// object drawn (line, string, rectangle) to be represented as a single
/// rectangle and for the damage area of the operation to be the union of these
/// rectangles.
///
/// The DAMAGE extension allows applications to either receive the raw
/// rectangles as a stream of events, or to have them partially processed within
/// the X server to reduce the amount of data transmitted as well as reduce the
/// processing latency once the repaint operation has started.
///
/// The Damage object holds any accumulated damage region and reflects the
/// relationship between the drawable selected for damage notification and the
/// drawable for which damage is tracked.
///
/// # Fields
///
/// * `damage` - The ID with which you will refer to the new Damage object, created by
/// `xcb_generate_id`.
/// * `drawable` - The ID of the drawable to be monitored.
/// * `level` - The level of detail to be provided in Damage events.
pub async fn create<Conn>(conn: &Conn, damage: Damage, drawable: xproto::Drawable, level: ReportLevel) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateRequest {
        damage,
        drawable,
        level,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Destroys a previously created Damage object..
///
/// This destroys a Damage object and requests the X server stop reporting
/// the changes it was tracking.
///
/// # Fields
///
/// * `damage` - The ID you provided to `xcb_create_damage`.
pub async fn destroy<Conn>(conn: &Conn, damage: Damage) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyRequest {
        damage,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Remove regions from a previously created Damage object..
///
/// This updates the regions of damage recorded in a a Damage object.
/// See https://www.x.org/releases/current/doc/damageproto/damageproto.txt
/// for details.
///
/// # Fields
///
/// * `damage` - The ID you provided to `xcb_create_damage`.
pub async fn subtract<Conn, A, B>(conn: &Conn, damage: Damage, repair: A, parts: B) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
    A: Into<xfixes::Region> + Send,
    B: Into<xfixes::Region> + Send,
{
    let repair: xfixes::Region = repair.into();
    let parts: xfixes::Region = parts.into();
    let request0 = SubtractRequest {
        damage,
        repair,
        parts,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Add a region to a previously created Damage object..
///
/// This updates the regions of damage recorded in a a Damage object.
/// See https://www.x.org/releases/current/doc/damageproto/damageproto.txt
/// for details.
///
/// # Fields
///
/// * `damage` - The ID you provided to `xcb_create_damage`.
pub async fn add<Conn>(conn: &Conn, drawable: xproto::Drawable, region: xfixes::Region) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AddRequest {
        drawable,
        region,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    /// Negotiate the version of the DAMAGE extension.
    ///
    /// This negotiates the version of the DAMAGE extension.  It must precede any other
    /// request using the DAMAGE extension.  Failure to do so will cause a BadRequest
    /// error for those requests.
    ///
    /// # Fields
    ///
    /// * `client_major_version` - The major version supported by the client.
    /// * `client_minor_version` - The minor version supported by the client.
    fn damage_query_version(&self, client_major_version: u32, client_minor_version: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_version(self, client_major_version, client_minor_version))
    }
    /// Creates a Damage object to monitor changes to a drawable..
    ///
    /// This creates a Damage object to monitor changes to a drawable, and specifies
    /// the level of detail to be reported for changes.
    ///
    /// We call changes made to pixel contents of windows and pixmaps 'damage'
    /// throughout this extension.
    ///
    /// Damage accumulates as drawing occurs in the drawable.  Each drawing operation
    /// 'damages' one or more rectangular areas within the drawable.  The rectangles
    /// are guaranteed to include the set of pixels modified by each operation, but
    /// may include significantly more than just those pixels.  The desire is for
    /// the damage to strike a balance between the number of rectangles reported and
    /// the extraneous area included.  A reasonable goal is for each primitive
    /// object drawn (line, string, rectangle) to be represented as a single
    /// rectangle and for the damage area of the operation to be the union of these
    /// rectangles.
    ///
    /// The DAMAGE extension allows applications to either receive the raw
    /// rectangles as a stream of events, or to have them partially processed within
    /// the X server to reduce the amount of data transmitted as well as reduce the
    /// processing latency once the repaint operation has started.
    ///
    /// The Damage object holds any accumulated damage region and reflects the
    /// relationship between the drawable selected for damage notification and the
    /// drawable for which damage is tracked.
    ///
    /// # Fields
    ///
    /// * `damage` - The ID with which you will refer to the new Damage object, created by
    /// `xcb_generate_id`.
    /// * `drawable` - The ID of the drawable to be monitored.
    /// * `level` - The level of detail to be provided in Damage events.
    fn damage_create(&self, damage: Damage, drawable: xproto::Drawable, level: ReportLevel) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create(self, damage, drawable, level))
    }
    /// Destroys a previously created Damage object..
    ///
    /// This destroys a Damage object and requests the X server stop reporting
    /// the changes it was tracking.
    ///
    /// # Fields
    ///
    /// * `damage` - The ID you provided to `xcb_create_damage`.
    fn damage_destroy(&self, damage: Damage) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy(self, damage))
    }
    /// Remove regions from a previously created Damage object..
    ///
    /// This updates the regions of damage recorded in a a Damage object.
    /// See https://www.x.org/releases/current/doc/damageproto/damageproto.txt
    /// for details.
    ///
    /// # Fields
    ///
    /// * `damage` - The ID you provided to `xcb_create_damage`.
    fn damage_subtract<A, B>(&self, damage: Damage, repair: A, parts: B) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    where
        A: Into<xfixes::Region> + Send + 'static,
        B: Into<xfixes::Region> + Send + 'static,
    {
        Box::pin(subtract(self, damage, repair, parts))
    }
    /// Add a region to a previously created Damage object..
    ///
    /// This updates the regions of damage recorded in a a Damage object.
    /// See https://www.x.org/releases/current/doc/damageproto/damageproto.txt
    /// for details.
    ///
    /// # Fields
    ///
    /// * `damage` - The ID you provided to `xcb_create_damage`.
    fn damage_add(&self, drawable: xproto::Drawable, region: xfixes::Region) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(add(self, drawable, region))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

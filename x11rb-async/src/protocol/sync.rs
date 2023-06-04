// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the `Sync` X11 extension.

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

pub use x11rb_protocol::protocol::sync::*;

/// Get the major opcode of this extension
async fn major_opcode<Conn: RequestConnection + ?Sized>(conn: &Conn) -> Result<u8, ConnectionError> {
    let info = conn.extension_information(X11_EXTENSION_NAME).await?;
    let info = info.ok_or(ConnectionError::UnsupportedExtension)?;
    Ok(info.major_opcode)
}

pub async fn initialize<Conn>(conn: &Conn, desired_major_version: u8, desired_minor_version: u8) -> Result<Cookie<'_, Conn, InitializeReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = InitializeRequest {
        desired_major_version,
        desired_minor_version,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn list_system_counters<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, ListSystemCountersReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ListSystemCountersRequest;
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_counter<Conn>(conn: &Conn, id: Counter, initial_value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateCounterRequest {
        id,
        initial_value,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn destroy_counter<Conn>(conn: &Conn, counter: Counter) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyCounterRequest {
        counter,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn query_counter<Conn>(conn: &Conn, counter: Counter) -> Result<Cookie<'_, Conn, QueryCounterReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryCounterRequest {
        counter,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn await_<'c, 'input, Conn>(conn: &'c Conn, wait_list: &'input [Waitcondition]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitRequest {
        wait_list: Cow::Borrowed(wait_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn change_counter<Conn>(conn: &Conn, counter: Counter, amount: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeCounterRequest {
        counter,
        amount,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn set_counter<Conn>(conn: &Conn, counter: Counter, value: Int64) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetCounterRequest {
        counter,
        value,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn create_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input CreateAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn change_alarm<'c, 'input, Conn>(conn: &'c Conn, id: Alarm, value_list: &'input ChangeAlarmAux) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ChangeAlarmRequest {
        id,
        value_list: Cow::Borrowed(value_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn destroy_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyAlarmRequest {
        alarm,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn query_alarm<Conn>(conn: &Conn, alarm: Alarm) -> Result<Cookie<'_, Conn, QueryAlarmReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryAlarmRequest {
        alarm,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn set_priority<Conn>(conn: &Conn, id: u32, priority: i32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = SetPriorityRequest {
        id,
        priority,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn get_priority<Conn>(conn: &Conn, id: u32) -> Result<Cookie<'_, Conn, GetPriorityReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = GetPriorityRequest {
        id,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn create_fence<Conn>(conn: &Conn, drawable: xproto::Drawable, fence: Fence, initially_triggered: bool) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = CreateFenceRequest {
        drawable,
        fence,
        initially_triggered,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn trigger_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = TriggerFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn reset_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = ResetFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn destroy_fence<Conn>(conn: &Conn, fence: Fence) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = DestroyFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
pub async fn query_fence<Conn>(conn: &Conn, fence: Fence) -> Result<Cookie<'_, Conn, QueryFenceReply>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = QueryFenceRequest {
        fence,
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_with_reply(&slices, fds).await
}
pub async fn await_fence<'c, 'input, Conn>(conn: &'c Conn, fence_list: &'input [Fence]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where
    Conn: RequestConnection + ?Sized,
{
    let request0 = AwaitFenceRequest {
        fence_list: Cow::Borrowed(fence_list),
    };
    let (bytes, fds) = request0.serialize(major_opcode(conn).await?);
    let slices = [IoSlice::new(&bytes[0]), IoSlice::new(&bytes[1]), IoSlice::new(&bytes[2])];
    assert_eq!(slices.len(), bytes.len());
    conn.send_request_without_reply(&slices, fds).await
}
/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn sync_initialize(&self, desired_major_version: u8, desired_minor_version: u8) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, InitializeReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(initialize(self, desired_major_version, desired_minor_version))
    }
    fn sync_list_system_counters(&self) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, ListSystemCountersReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(list_system_counters(self))
    }
    fn sync_create_counter(&self, id: Counter, initial_value: Int64) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_counter(self, id, initial_value))
    }
    fn sync_destroy_counter(&self, counter: Counter) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_counter(self, counter))
    }
    fn sync_query_counter(&self, counter: Counter) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryCounterReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_counter(self, counter))
    }
    fn sync_await_<'c, 'input, 'future>(&'c self, wait_list: &'input [Waitcondition]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(await_(self, wait_list))
    }
    fn sync_change_counter(&self, counter: Counter, amount: Int64) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(change_counter(self, counter, amount))
    }
    fn sync_set_counter(&self, counter: Counter, value: Int64) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(set_counter(self, counter, value))
    }
    fn sync_create_alarm<'c, 'input, 'future>(&'c self, id: Alarm, value_list: &'input CreateAlarmAux) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(create_alarm(self, id, value_list))
    }
    fn sync_change_alarm<'c, 'input, 'future>(&'c self, id: Alarm, value_list: &'input ChangeAlarmAux) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(change_alarm(self, id, value_list))
    }
    fn sync_destroy_alarm(&self, alarm: Alarm) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_alarm(self, alarm))
    }
    fn sync_query_alarm(&self, alarm: Alarm) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryAlarmReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_alarm(self, alarm))
    }
    fn sync_set_priority(&self, id: u32, priority: i32) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(set_priority(self, id, priority))
    }
    fn sync_get_priority(&self, id: u32) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, GetPriorityReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(get_priority(self, id))
    }
    fn sync_create_fence(&self, drawable: xproto::Drawable, fence: Fence, initially_triggered: bool) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(create_fence(self, drawable, fence, initially_triggered))
    }
    fn sync_trigger_fence(&self, fence: Fence) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(trigger_fence(self, fence))
    }
    fn sync_reset_fence(&self, fence: Fence) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(reset_fence(self, fence))
    }
    fn sync_destroy_fence(&self, fence: Fence) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'_, Self>, ConnectionError>> + Send + '_>>
    {
        Box::pin(destroy_fence(self, fence))
    }
    fn sync_query_fence(&self, fence: Fence) -> Pin<Box<dyn Future<Output = Result<Cookie<'_, Self, QueryFenceReply>, ConnectionError>> + Send + '_>>
    {
        Box::pin(query_fence(self, fence))
    }
    fn sync_await_fence<'c, 'input, 'future>(&'c self, fence_list: &'input [Fence]) -> Pin<Box<dyn Future<Output = Result<VoidCookie<'c, Self>, ConnectionError>> + Send + 'future>>
    where
        'c: 'future,
        'input: 'future,
    {
        Box::pin(await_fence(self, fence_list))
    }
}

impl<C: RequestConnection + ?Sized> ConnectionExt for C {}

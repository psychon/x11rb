// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the X11 protocol.
//!
//! Each sub-module of this module corresponds to one X11 extension. It contains all the
//! definitions from that extension. The core X11 protocol is in [`xproto`](xproto/index.html).

use std::borrow::Cow;
use std::convert::{TryFrom, TryInto};
use crate::errors::ParseError;
use crate::utils::RawFdContainer;
use crate::x11_utils::{ExtInfoProvider, ReplyParsingFunction, Request as RequestTrait, RequestHeader};

pub mod xproto;
pub mod bigreq;
#[cfg(feature = "composite")]
pub mod composite;
#[cfg(feature = "damage")]
pub mod damage;
#[cfg(feature = "dpms")]
pub mod dpms;
#[cfg(feature = "dri2")]
pub mod dri2;
#[cfg(feature = "dri3")]
pub mod dri3;
pub mod ge;
#[cfg(feature = "glx")]
pub mod glx;
#[cfg(feature = "present")]
pub mod present;
#[cfg(feature = "randr")]
pub mod randr;
#[cfg(feature = "record")]
pub mod record;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "res")]
pub mod res;
#[cfg(feature = "screensaver")]
pub mod screensaver;
#[cfg(feature = "shape")]
pub mod shape;
#[cfg(feature = "shm")]
pub mod shm;
#[cfg(feature = "sync")]
pub mod sync;
pub mod xc_misc;
#[cfg(feature = "xevie")]
pub mod xevie;
#[cfg(feature = "xf86dri")]
pub mod xf86dri;
#[cfg(feature = "xf86vidmode")]
pub mod xf86vidmode;
#[cfg(feature = "xfixes")]
pub mod xfixes;
#[cfg(feature = "xinerama")]
pub mod xinerama;
#[cfg(feature = "xinput")]
pub mod xinput;
#[cfg(feature = "xkb")]
pub mod xkb;
#[cfg(feature = "xprint")]
pub mod xprint;
#[cfg(feature = "xselinux")]
pub mod xselinux;
#[cfg(feature = "xtest")]
pub mod xtest;
#[cfg(feature = "xv")]
pub mod xv;
#[cfg(feature = "xvmc")]
pub mod xvmc;

/// Enumeration of all possible X11 requests.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Request<'input> {
    Unknown(RequestHeader, Cow<'input, [u8]>),
    CreateWindow(xproto::CreateWindowRequest<'input>),
    ChangeWindowAttributes(xproto::ChangeWindowAttributesRequest<'input>),
    GetWindowAttributes(xproto::GetWindowAttributesRequest),
    DestroyWindow(xproto::DestroyWindowRequest),
    DestroySubwindows(xproto::DestroySubwindowsRequest),
    ChangeSaveSet(xproto::ChangeSaveSetRequest),
    ReparentWindow(xproto::ReparentWindowRequest),
    MapWindow(xproto::MapWindowRequest),
    MapSubwindows(xproto::MapSubwindowsRequest),
    UnmapWindow(xproto::UnmapWindowRequest),
    UnmapSubwindows(xproto::UnmapSubwindowsRequest),
    ConfigureWindow(xproto::ConfigureWindowRequest<'input>),
    CirculateWindow(xproto::CirculateWindowRequest),
    GetGeometry(xproto::GetGeometryRequest),
    QueryTree(xproto::QueryTreeRequest),
    InternAtom(xproto::InternAtomRequest<'input>),
    GetAtomName(xproto::GetAtomNameRequest),
    ChangeProperty(xproto::ChangePropertyRequest<'input>),
    DeleteProperty(xproto::DeletePropertyRequest),
    GetProperty(xproto::GetPropertyRequest),
    ListProperties(xproto::ListPropertiesRequest),
    SetSelectionOwner(xproto::SetSelectionOwnerRequest),
    GetSelectionOwner(xproto::GetSelectionOwnerRequest),
    ConvertSelection(xproto::ConvertSelectionRequest),
    SendEvent(xproto::SendEventRequest<'input>),
    GrabPointer(xproto::GrabPointerRequest),
    UngrabPointer(xproto::UngrabPointerRequest),
    GrabButton(xproto::GrabButtonRequest),
    UngrabButton(xproto::UngrabButtonRequest),
    ChangeActivePointerGrab(xproto::ChangeActivePointerGrabRequest),
    GrabKeyboard(xproto::GrabKeyboardRequest),
    UngrabKeyboard(xproto::UngrabKeyboardRequest),
    GrabKey(xproto::GrabKeyRequest),
    UngrabKey(xproto::UngrabKeyRequest),
    AllowEvents(xproto::AllowEventsRequest),
    GrabServer(xproto::GrabServerRequest),
    UngrabServer(xproto::UngrabServerRequest),
    QueryPointer(xproto::QueryPointerRequest),
    GetMotionEvents(xproto::GetMotionEventsRequest),
    TranslateCoordinates(xproto::TranslateCoordinatesRequest),
    WarpPointer(xproto::WarpPointerRequest),
    SetInputFocus(xproto::SetInputFocusRequest),
    GetInputFocus(xproto::GetInputFocusRequest),
    QueryKeymap(xproto::QueryKeymapRequest),
    OpenFont(xproto::OpenFontRequest<'input>),
    CloseFont(xproto::CloseFontRequest),
    QueryFont(xproto::QueryFontRequest),
    QueryTextExtents(xproto::QueryTextExtentsRequest<'input>),
    ListFonts(xproto::ListFontsRequest<'input>),
    ListFontsWithInfo(xproto::ListFontsWithInfoRequest<'input>),
    SetFontPath(xproto::SetFontPathRequest<'input>),
    GetFontPath(xproto::GetFontPathRequest),
    CreatePixmap(xproto::CreatePixmapRequest),
    FreePixmap(xproto::FreePixmapRequest),
    CreateGC(xproto::CreateGCRequest<'input>),
    ChangeGC(xproto::ChangeGCRequest<'input>),
    CopyGC(xproto::CopyGCRequest),
    SetDashes(xproto::SetDashesRequest<'input>),
    SetClipRectangles(xproto::SetClipRectanglesRequest<'input>),
    FreeGC(xproto::FreeGCRequest),
    ClearArea(xproto::ClearAreaRequest),
    CopyArea(xproto::CopyAreaRequest),
    CopyPlane(xproto::CopyPlaneRequest),
    PolyPoint(xproto::PolyPointRequest<'input>),
    PolyLine(xproto::PolyLineRequest<'input>),
    PolySegment(xproto::PolySegmentRequest<'input>),
    PolyRectangle(xproto::PolyRectangleRequest<'input>),
    PolyArc(xproto::PolyArcRequest<'input>),
    FillPoly(xproto::FillPolyRequest<'input>),
    PolyFillRectangle(xproto::PolyFillRectangleRequest<'input>),
    PolyFillArc(xproto::PolyFillArcRequest<'input>),
    PutImage(xproto::PutImageRequest<'input>),
    GetImage(xproto::GetImageRequest),
    PolyText8(xproto::PolyText8Request<'input>),
    PolyText16(xproto::PolyText16Request<'input>),
    ImageText8(xproto::ImageText8Request<'input>),
    ImageText16(xproto::ImageText16Request<'input>),
    CreateColormap(xproto::CreateColormapRequest),
    FreeColormap(xproto::FreeColormapRequest),
    CopyColormapAndFree(xproto::CopyColormapAndFreeRequest),
    InstallColormap(xproto::InstallColormapRequest),
    UninstallColormap(xproto::UninstallColormapRequest),
    ListInstalledColormaps(xproto::ListInstalledColormapsRequest),
    AllocColor(xproto::AllocColorRequest),
    AllocNamedColor(xproto::AllocNamedColorRequest<'input>),
    AllocColorCells(xproto::AllocColorCellsRequest),
    AllocColorPlanes(xproto::AllocColorPlanesRequest),
    FreeColors(xproto::FreeColorsRequest<'input>),
    StoreColors(xproto::StoreColorsRequest<'input>),
    StoreNamedColor(xproto::StoreNamedColorRequest<'input>),
    QueryColors(xproto::QueryColorsRequest<'input>),
    LookupColor(xproto::LookupColorRequest<'input>),
    CreateCursor(xproto::CreateCursorRequest),
    CreateGlyphCursor(xproto::CreateGlyphCursorRequest),
    FreeCursor(xproto::FreeCursorRequest),
    RecolorCursor(xproto::RecolorCursorRequest),
    QueryBestSize(xproto::QueryBestSizeRequest),
    QueryExtension(xproto::QueryExtensionRequest<'input>),
    ListExtensions(xproto::ListExtensionsRequest),
    ChangeKeyboardMapping(xproto::ChangeKeyboardMappingRequest<'input>),
    GetKeyboardMapping(xproto::GetKeyboardMappingRequest),
    ChangeKeyboardControl(xproto::ChangeKeyboardControlRequest<'input>),
    GetKeyboardControl(xproto::GetKeyboardControlRequest),
    Bell(xproto::BellRequest),
    ChangePointerControl(xproto::ChangePointerControlRequest),
    GetPointerControl(xproto::GetPointerControlRequest),
    SetScreenSaver(xproto::SetScreenSaverRequest),
    GetScreenSaver(xproto::GetScreenSaverRequest),
    ChangeHosts(xproto::ChangeHostsRequest<'input>),
    ListHosts(xproto::ListHostsRequest),
    SetAccessControl(xproto::SetAccessControlRequest),
    SetCloseDownMode(xproto::SetCloseDownModeRequest),
    KillClient(xproto::KillClientRequest),
    RotateProperties(xproto::RotatePropertiesRequest<'input>),
    ForceScreenSaver(xproto::ForceScreenSaverRequest),
    SetPointerMapping(xproto::SetPointerMappingRequest<'input>),
    GetPointerMapping(xproto::GetPointerMappingRequest),
    SetModifierMapping(xproto::SetModifierMappingRequest<'input>),
    GetModifierMapping(xproto::GetModifierMappingRequest),
    NoOperation(xproto::NoOperationRequest),
    BigreqEnable(bigreq::EnableRequest),
    #[cfg(feature = "composite")]
    CompositeQueryVersion(composite::QueryVersionRequest),
    #[cfg(feature = "composite")]
    CompositeRedirectWindow(composite::RedirectWindowRequest),
    #[cfg(feature = "composite")]
    CompositeRedirectSubwindows(composite::RedirectSubwindowsRequest),
    #[cfg(feature = "composite")]
    CompositeUnredirectWindow(composite::UnredirectWindowRequest),
    #[cfg(feature = "composite")]
    CompositeUnredirectSubwindows(composite::UnredirectSubwindowsRequest),
    #[cfg(feature = "composite")]
    CompositeCreateRegionFromBorderClip(composite::CreateRegionFromBorderClipRequest),
    #[cfg(feature = "composite")]
    CompositeNameWindowPixmap(composite::NameWindowPixmapRequest),
    #[cfg(feature = "composite")]
    CompositeGetOverlayWindow(composite::GetOverlayWindowRequest),
    #[cfg(feature = "composite")]
    CompositeReleaseOverlayWindow(composite::ReleaseOverlayWindowRequest),
    #[cfg(feature = "damage")]
    DamageQueryVersion(damage::QueryVersionRequest),
    #[cfg(feature = "damage")]
    DamageCreate(damage::CreateRequest),
    #[cfg(feature = "damage")]
    DamageDestroy(damage::DestroyRequest),
    #[cfg(feature = "damage")]
    DamageSubtract(damage::SubtractRequest),
    #[cfg(feature = "damage")]
    DamageAdd(damage::AddRequest),
    #[cfg(feature = "dpms")]
    DpmsGetVersion(dpms::GetVersionRequest),
    #[cfg(feature = "dpms")]
    DpmsCapable(dpms::CapableRequest),
    #[cfg(feature = "dpms")]
    DpmsGetTimeouts(dpms::GetTimeoutsRequest),
    #[cfg(feature = "dpms")]
    DpmsSetTimeouts(dpms::SetTimeoutsRequest),
    #[cfg(feature = "dpms")]
    DpmsEnable(dpms::EnableRequest),
    #[cfg(feature = "dpms")]
    DpmsDisable(dpms::DisableRequest),
    #[cfg(feature = "dpms")]
    DpmsForceLevel(dpms::ForceLevelRequest),
    #[cfg(feature = "dpms")]
    DpmsInfo(dpms::InfoRequest),
    #[cfg(feature = "dri2")]
    Dri2QueryVersion(dri2::QueryVersionRequest),
    #[cfg(feature = "dri2")]
    Dri2Connect(dri2::ConnectRequest),
    #[cfg(feature = "dri2")]
    Dri2Authenticate(dri2::AuthenticateRequest),
    #[cfg(feature = "dri2")]
    Dri2CreateDrawable(dri2::CreateDrawableRequest),
    #[cfg(feature = "dri2")]
    Dri2DestroyDrawable(dri2::DestroyDrawableRequest),
    #[cfg(feature = "dri2")]
    Dri2GetBuffers(dri2::GetBuffersRequest<'input>),
    #[cfg(feature = "dri2")]
    Dri2CopyRegion(dri2::CopyRegionRequest),
    #[cfg(feature = "dri2")]
    Dri2GetBuffersWithFormat(dri2::GetBuffersWithFormatRequest<'input>),
    #[cfg(feature = "dri2")]
    Dri2SwapBuffers(dri2::SwapBuffersRequest),
    #[cfg(feature = "dri2")]
    Dri2GetMSC(dri2::GetMSCRequest),
    #[cfg(feature = "dri2")]
    Dri2WaitMSC(dri2::WaitMSCRequest),
    #[cfg(feature = "dri2")]
    Dri2WaitSBC(dri2::WaitSBCRequest),
    #[cfg(feature = "dri2")]
    Dri2SwapInterval(dri2::SwapIntervalRequest),
    #[cfg(feature = "dri2")]
    Dri2GetParam(dri2::GetParamRequest),
    #[cfg(feature = "dri3")]
    Dri3QueryVersion(dri3::QueryVersionRequest),
    #[cfg(feature = "dri3")]
    Dri3Open(dri3::OpenRequest),
    #[cfg(feature = "dri3")]
    Dri3PixmapFromBuffer(dri3::PixmapFromBufferRequest),
    #[cfg(feature = "dri3")]
    Dri3BufferFromPixmap(dri3::BufferFromPixmapRequest),
    #[cfg(feature = "dri3")]
    Dri3FenceFromFD(dri3::FenceFromFDRequest),
    #[cfg(feature = "dri3")]
    Dri3FDFromFence(dri3::FDFromFenceRequest),
    #[cfg(feature = "dri3")]
    Dri3GetSupportedModifiers(dri3::GetSupportedModifiersRequest),
    #[cfg(feature = "dri3")]
    Dri3PixmapFromBuffers(dri3::PixmapFromBuffersRequest),
    #[cfg(feature = "dri3")]
    Dri3BuffersFromPixmap(dri3::BuffersFromPixmapRequest),
    GeQueryVersion(ge::QueryVersionRequest),
    #[cfg(feature = "glx")]
    GlxRender(glx::RenderRequest<'input>),
    #[cfg(feature = "glx")]
    GlxRenderLarge(glx::RenderLargeRequest<'input>),
    #[cfg(feature = "glx")]
    GlxCreateContext(glx::CreateContextRequest),
    #[cfg(feature = "glx")]
    GlxDestroyContext(glx::DestroyContextRequest),
    #[cfg(feature = "glx")]
    GlxMakeCurrent(glx::MakeCurrentRequest),
    #[cfg(feature = "glx")]
    GlxIsDirect(glx::IsDirectRequest),
    #[cfg(feature = "glx")]
    GlxQueryVersion(glx::QueryVersionRequest),
    #[cfg(feature = "glx")]
    GlxWaitGL(glx::WaitGLRequest),
    #[cfg(feature = "glx")]
    GlxWaitX(glx::WaitXRequest),
    #[cfg(feature = "glx")]
    GlxCopyContext(glx::CopyContextRequest),
    #[cfg(feature = "glx")]
    GlxSwapBuffers(glx::SwapBuffersRequest),
    #[cfg(feature = "glx")]
    GlxUseXFont(glx::UseXFontRequest),
    #[cfg(feature = "glx")]
    GlxCreateGLXPixmap(glx::CreateGLXPixmapRequest),
    #[cfg(feature = "glx")]
    GlxGetVisualConfigs(glx::GetVisualConfigsRequest),
    #[cfg(feature = "glx")]
    GlxDestroyGLXPixmap(glx::DestroyGLXPixmapRequest),
    #[cfg(feature = "glx")]
    GlxVendorPrivate(glx::VendorPrivateRequest<'input>),
    #[cfg(feature = "glx")]
    GlxVendorPrivateWithReply(glx::VendorPrivateWithReplyRequest<'input>),
    #[cfg(feature = "glx")]
    GlxQueryExtensionsString(glx::QueryExtensionsStringRequest),
    #[cfg(feature = "glx")]
    GlxQueryServerString(glx::QueryServerStringRequest),
    #[cfg(feature = "glx")]
    GlxClientInfo(glx::ClientInfoRequest<'input>),
    #[cfg(feature = "glx")]
    GlxGetFBConfigs(glx::GetFBConfigsRequest),
    #[cfg(feature = "glx")]
    GlxCreatePixmap(glx::CreatePixmapRequest<'input>),
    #[cfg(feature = "glx")]
    GlxDestroyPixmap(glx::DestroyPixmapRequest),
    #[cfg(feature = "glx")]
    GlxCreateNewContext(glx::CreateNewContextRequest),
    #[cfg(feature = "glx")]
    GlxQueryContext(glx::QueryContextRequest),
    #[cfg(feature = "glx")]
    GlxMakeContextCurrent(glx::MakeContextCurrentRequest),
    #[cfg(feature = "glx")]
    GlxCreatePbuffer(glx::CreatePbufferRequest<'input>),
    #[cfg(feature = "glx")]
    GlxDestroyPbuffer(glx::DestroyPbufferRequest),
    #[cfg(feature = "glx")]
    GlxGetDrawableAttributes(glx::GetDrawableAttributesRequest),
    #[cfg(feature = "glx")]
    GlxChangeDrawableAttributes(glx::ChangeDrawableAttributesRequest<'input>),
    #[cfg(feature = "glx")]
    GlxCreateWindow(glx::CreateWindowRequest<'input>),
    #[cfg(feature = "glx")]
    GlxDeleteWindow(glx::DeleteWindowRequest),
    #[cfg(feature = "glx")]
    GlxSetClientInfoARB(glx::SetClientInfoARBRequest<'input>),
    #[cfg(feature = "glx")]
    GlxCreateContextAttribsARB(glx::CreateContextAttribsARBRequest<'input>),
    #[cfg(feature = "glx")]
    GlxSetClientInfo2ARB(glx::SetClientInfo2ARBRequest<'input>),
    #[cfg(feature = "glx")]
    GlxNewList(glx::NewListRequest),
    #[cfg(feature = "glx")]
    GlxEndList(glx::EndListRequest),
    #[cfg(feature = "glx")]
    GlxDeleteLists(glx::DeleteListsRequest),
    #[cfg(feature = "glx")]
    GlxGenLists(glx::GenListsRequest),
    #[cfg(feature = "glx")]
    GlxFeedbackBuffer(glx::FeedbackBufferRequest),
    #[cfg(feature = "glx")]
    GlxSelectBuffer(glx::SelectBufferRequest),
    #[cfg(feature = "glx")]
    GlxRenderMode(glx::RenderModeRequest),
    #[cfg(feature = "glx")]
    GlxFinish(glx::FinishRequest),
    #[cfg(feature = "glx")]
    GlxPixelStoref(glx::PixelStorefRequest),
    #[cfg(feature = "glx")]
    GlxPixelStorei(glx::PixelStoreiRequest),
    #[cfg(feature = "glx")]
    GlxReadPixels(glx::ReadPixelsRequest),
    #[cfg(feature = "glx")]
    GlxGetBooleanv(glx::GetBooleanvRequest),
    #[cfg(feature = "glx")]
    GlxGetClipPlane(glx::GetClipPlaneRequest),
    #[cfg(feature = "glx")]
    GlxGetDoublev(glx::GetDoublevRequest),
    #[cfg(feature = "glx")]
    GlxGetError(glx::GetErrorRequest),
    #[cfg(feature = "glx")]
    GlxGetFloatv(glx::GetFloatvRequest),
    #[cfg(feature = "glx")]
    GlxGetIntegerv(glx::GetIntegervRequest),
    #[cfg(feature = "glx")]
    GlxGetLightfv(glx::GetLightfvRequest),
    #[cfg(feature = "glx")]
    GlxGetLightiv(glx::GetLightivRequest),
    #[cfg(feature = "glx")]
    GlxGetMapdv(glx::GetMapdvRequest),
    #[cfg(feature = "glx")]
    GlxGetMapfv(glx::GetMapfvRequest),
    #[cfg(feature = "glx")]
    GlxGetMapiv(glx::GetMapivRequest),
    #[cfg(feature = "glx")]
    GlxGetMaterialfv(glx::GetMaterialfvRequest),
    #[cfg(feature = "glx")]
    GlxGetMaterialiv(glx::GetMaterialivRequest),
    #[cfg(feature = "glx")]
    GlxGetPixelMapfv(glx::GetPixelMapfvRequest),
    #[cfg(feature = "glx")]
    GlxGetPixelMapuiv(glx::GetPixelMapuivRequest),
    #[cfg(feature = "glx")]
    GlxGetPixelMapusv(glx::GetPixelMapusvRequest),
    #[cfg(feature = "glx")]
    GlxGetPolygonStipple(glx::GetPolygonStippleRequest),
    #[cfg(feature = "glx")]
    GlxGetString(glx::GetStringRequest),
    #[cfg(feature = "glx")]
    GlxGetTexEnvfv(glx::GetTexEnvfvRequest),
    #[cfg(feature = "glx")]
    GlxGetTexEnviv(glx::GetTexEnvivRequest),
    #[cfg(feature = "glx")]
    GlxGetTexGendv(glx::GetTexGendvRequest),
    #[cfg(feature = "glx")]
    GlxGetTexGenfv(glx::GetTexGenfvRequest),
    #[cfg(feature = "glx")]
    GlxGetTexGeniv(glx::GetTexGenivRequest),
    #[cfg(feature = "glx")]
    GlxGetTexImage(glx::GetTexImageRequest),
    #[cfg(feature = "glx")]
    GlxGetTexParameterfv(glx::GetTexParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetTexParameteriv(glx::GetTexParameterivRequest),
    #[cfg(feature = "glx")]
    GlxGetTexLevelParameterfv(glx::GetTexLevelParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetTexLevelParameteriv(glx::GetTexLevelParameterivRequest),
    #[cfg(feature = "glx")]
    GlxIsEnabled(glx::IsEnabledRequest),
    #[cfg(feature = "glx")]
    GlxIsList(glx::IsListRequest),
    #[cfg(feature = "glx")]
    GlxFlush(glx::FlushRequest),
    #[cfg(feature = "glx")]
    GlxAreTexturesResident(glx::AreTexturesResidentRequest<'input>),
    #[cfg(feature = "glx")]
    GlxDeleteTextures(glx::DeleteTexturesRequest<'input>),
    #[cfg(feature = "glx")]
    GlxGenTextures(glx::GenTexturesRequest),
    #[cfg(feature = "glx")]
    GlxIsTexture(glx::IsTextureRequest),
    #[cfg(feature = "glx")]
    GlxGetColorTable(glx::GetColorTableRequest),
    #[cfg(feature = "glx")]
    GlxGetColorTableParameterfv(glx::GetColorTableParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetColorTableParameteriv(glx::GetColorTableParameterivRequest),
    #[cfg(feature = "glx")]
    GlxGetConvolutionFilter(glx::GetConvolutionFilterRequest),
    #[cfg(feature = "glx")]
    GlxGetConvolutionParameterfv(glx::GetConvolutionParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetConvolutionParameteriv(glx::GetConvolutionParameterivRequest),
    #[cfg(feature = "glx")]
    GlxGetSeparableFilter(glx::GetSeparableFilterRequest),
    #[cfg(feature = "glx")]
    GlxGetHistogram(glx::GetHistogramRequest),
    #[cfg(feature = "glx")]
    GlxGetHistogramParameterfv(glx::GetHistogramParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetHistogramParameteriv(glx::GetHistogramParameterivRequest),
    #[cfg(feature = "glx")]
    GlxGetMinmax(glx::GetMinmaxRequest),
    #[cfg(feature = "glx")]
    GlxGetMinmaxParameterfv(glx::GetMinmaxParameterfvRequest),
    #[cfg(feature = "glx")]
    GlxGetMinmaxParameteriv(glx::GetMinmaxParameterivRequest),
    #[cfg(feature = "glx")]
    GlxGetCompressedTexImageARB(glx::GetCompressedTexImageARBRequest),
    #[cfg(feature = "glx")]
    GlxDeleteQueriesARB(glx::DeleteQueriesARBRequest<'input>),
    #[cfg(feature = "glx")]
    GlxGenQueriesARB(glx::GenQueriesARBRequest),
    #[cfg(feature = "glx")]
    GlxIsQueryARB(glx::IsQueryARBRequest),
    #[cfg(feature = "glx")]
    GlxGetQueryivARB(glx::GetQueryivARBRequest),
    #[cfg(feature = "glx")]
    GlxGetQueryObjectivARB(glx::GetQueryObjectivARBRequest),
    #[cfg(feature = "glx")]
    GlxGetQueryObjectuivARB(glx::GetQueryObjectuivARBRequest),
    #[cfg(feature = "present")]
    PresentQueryVersion(present::QueryVersionRequest),
    #[cfg(feature = "present")]
    PresentPixmap(present::PixmapRequest<'input>),
    #[cfg(feature = "present")]
    PresentNotifyMSC(present::NotifyMSCRequest),
    #[cfg(feature = "present")]
    PresentSelectInput(present::SelectInputRequest),
    #[cfg(feature = "present")]
    PresentQueryCapabilities(present::QueryCapabilitiesRequest),
    #[cfg(feature = "randr")]
    RandrQueryVersion(randr::QueryVersionRequest),
    #[cfg(feature = "randr")]
    RandrSetScreenConfig(randr::SetScreenConfigRequest),
    #[cfg(feature = "randr")]
    RandrSelectInput(randr::SelectInputRequest),
    #[cfg(feature = "randr")]
    RandrGetScreenInfo(randr::GetScreenInfoRequest),
    #[cfg(feature = "randr")]
    RandrGetScreenSizeRange(randr::GetScreenSizeRangeRequest),
    #[cfg(feature = "randr")]
    RandrSetScreenSize(randr::SetScreenSizeRequest),
    #[cfg(feature = "randr")]
    RandrGetScreenResources(randr::GetScreenResourcesRequest),
    #[cfg(feature = "randr")]
    RandrGetOutputInfo(randr::GetOutputInfoRequest),
    #[cfg(feature = "randr")]
    RandrListOutputProperties(randr::ListOutputPropertiesRequest),
    #[cfg(feature = "randr")]
    RandrQueryOutputProperty(randr::QueryOutputPropertyRequest),
    #[cfg(feature = "randr")]
    RandrConfigureOutputProperty(randr::ConfigureOutputPropertyRequest<'input>),
    #[cfg(feature = "randr")]
    RandrChangeOutputProperty(randr::ChangeOutputPropertyRequest<'input>),
    #[cfg(feature = "randr")]
    RandrDeleteOutputProperty(randr::DeleteOutputPropertyRequest),
    #[cfg(feature = "randr")]
    RandrGetOutputProperty(randr::GetOutputPropertyRequest),
    #[cfg(feature = "randr")]
    RandrCreateMode(randr::CreateModeRequest<'input>),
    #[cfg(feature = "randr")]
    RandrDestroyMode(randr::DestroyModeRequest),
    #[cfg(feature = "randr")]
    RandrAddOutputMode(randr::AddOutputModeRequest),
    #[cfg(feature = "randr")]
    RandrDeleteOutputMode(randr::DeleteOutputModeRequest),
    #[cfg(feature = "randr")]
    RandrGetCrtcInfo(randr::GetCrtcInfoRequest),
    #[cfg(feature = "randr")]
    RandrSetCrtcConfig(randr::SetCrtcConfigRequest<'input>),
    #[cfg(feature = "randr")]
    RandrGetCrtcGammaSize(randr::GetCrtcGammaSizeRequest),
    #[cfg(feature = "randr")]
    RandrGetCrtcGamma(randr::GetCrtcGammaRequest),
    #[cfg(feature = "randr")]
    RandrSetCrtcGamma(randr::SetCrtcGammaRequest<'input>),
    #[cfg(feature = "randr")]
    RandrGetScreenResourcesCurrent(randr::GetScreenResourcesCurrentRequest),
    #[cfg(feature = "randr")]
    RandrSetCrtcTransform(randr::SetCrtcTransformRequest<'input>),
    #[cfg(feature = "randr")]
    RandrGetCrtcTransform(randr::GetCrtcTransformRequest),
    #[cfg(feature = "randr")]
    RandrGetPanning(randr::GetPanningRequest),
    #[cfg(feature = "randr")]
    RandrSetPanning(randr::SetPanningRequest),
    #[cfg(feature = "randr")]
    RandrSetOutputPrimary(randr::SetOutputPrimaryRequest),
    #[cfg(feature = "randr")]
    RandrGetOutputPrimary(randr::GetOutputPrimaryRequest),
    #[cfg(feature = "randr")]
    RandrGetProviders(randr::GetProvidersRequest),
    #[cfg(feature = "randr")]
    RandrGetProviderInfo(randr::GetProviderInfoRequest),
    #[cfg(feature = "randr")]
    RandrSetProviderOffloadSink(randr::SetProviderOffloadSinkRequest),
    #[cfg(feature = "randr")]
    RandrSetProviderOutputSource(randr::SetProviderOutputSourceRequest),
    #[cfg(feature = "randr")]
    RandrListProviderProperties(randr::ListProviderPropertiesRequest),
    #[cfg(feature = "randr")]
    RandrQueryProviderProperty(randr::QueryProviderPropertyRequest),
    #[cfg(feature = "randr")]
    RandrConfigureProviderProperty(randr::ConfigureProviderPropertyRequest<'input>),
    #[cfg(feature = "randr")]
    RandrChangeProviderProperty(randr::ChangeProviderPropertyRequest<'input>),
    #[cfg(feature = "randr")]
    RandrDeleteProviderProperty(randr::DeleteProviderPropertyRequest),
    #[cfg(feature = "randr")]
    RandrGetProviderProperty(randr::GetProviderPropertyRequest),
    #[cfg(feature = "randr")]
    RandrGetMonitors(randr::GetMonitorsRequest),
    #[cfg(feature = "randr")]
    RandrSetMonitor(randr::SetMonitorRequest),
    #[cfg(feature = "randr")]
    RandrDeleteMonitor(randr::DeleteMonitorRequest),
    #[cfg(feature = "randr")]
    RandrCreateLease(randr::CreateLeaseRequest<'input>),
    #[cfg(feature = "randr")]
    RandrFreeLease(randr::FreeLeaseRequest),
    #[cfg(feature = "record")]
    RecordQueryVersion(record::QueryVersionRequest),
    #[cfg(feature = "record")]
    RecordCreateContext(record::CreateContextRequest<'input>),
    #[cfg(feature = "record")]
    RecordRegisterClients(record::RegisterClientsRequest<'input>),
    #[cfg(feature = "record")]
    RecordUnregisterClients(record::UnregisterClientsRequest<'input>),
    #[cfg(feature = "record")]
    RecordGetContext(record::GetContextRequest),
    #[cfg(feature = "record")]
    RecordEnableContext(record::EnableContextRequest),
    #[cfg(feature = "record")]
    RecordDisableContext(record::DisableContextRequest),
    #[cfg(feature = "record")]
    RecordFreeContext(record::FreeContextRequest),
    #[cfg(feature = "render")]
    RenderQueryVersion(render::QueryVersionRequest),
    #[cfg(feature = "render")]
    RenderQueryPictFormats(render::QueryPictFormatsRequest),
    #[cfg(feature = "render")]
    RenderQueryPictIndexValues(render::QueryPictIndexValuesRequest),
    #[cfg(feature = "render")]
    RenderCreatePicture(render::CreatePictureRequest<'input>),
    #[cfg(feature = "render")]
    RenderChangePicture(render::ChangePictureRequest<'input>),
    #[cfg(feature = "render")]
    RenderSetPictureClipRectangles(render::SetPictureClipRectanglesRequest<'input>),
    #[cfg(feature = "render")]
    RenderFreePicture(render::FreePictureRequest),
    #[cfg(feature = "render")]
    RenderComposite(render::CompositeRequest),
    #[cfg(feature = "render")]
    RenderTrapezoids(render::TrapezoidsRequest<'input>),
    #[cfg(feature = "render")]
    RenderTriangles(render::TrianglesRequest<'input>),
    #[cfg(feature = "render")]
    RenderTriStrip(render::TriStripRequest<'input>),
    #[cfg(feature = "render")]
    RenderTriFan(render::TriFanRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateGlyphSet(render::CreateGlyphSetRequest),
    #[cfg(feature = "render")]
    RenderReferenceGlyphSet(render::ReferenceGlyphSetRequest),
    #[cfg(feature = "render")]
    RenderFreeGlyphSet(render::FreeGlyphSetRequest),
    #[cfg(feature = "render")]
    RenderAddGlyphs(render::AddGlyphsRequest<'input>),
    #[cfg(feature = "render")]
    RenderFreeGlyphs(render::FreeGlyphsRequest<'input>),
    #[cfg(feature = "render")]
    RenderCompositeGlyphs8(render::CompositeGlyphs8Request<'input>),
    #[cfg(feature = "render")]
    RenderCompositeGlyphs16(render::CompositeGlyphs16Request<'input>),
    #[cfg(feature = "render")]
    RenderCompositeGlyphs32(render::CompositeGlyphs32Request<'input>),
    #[cfg(feature = "render")]
    RenderFillRectangles(render::FillRectanglesRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateCursor(render::CreateCursorRequest),
    #[cfg(feature = "render")]
    RenderSetPictureTransform(render::SetPictureTransformRequest),
    #[cfg(feature = "render")]
    RenderQueryFilters(render::QueryFiltersRequest),
    #[cfg(feature = "render")]
    RenderSetPictureFilter(render::SetPictureFilterRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateAnimCursor(render::CreateAnimCursorRequest<'input>),
    #[cfg(feature = "render")]
    RenderAddTraps(render::AddTrapsRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateSolidFill(render::CreateSolidFillRequest),
    #[cfg(feature = "render")]
    RenderCreateLinearGradient(render::CreateLinearGradientRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateRadialGradient(render::CreateRadialGradientRequest<'input>),
    #[cfg(feature = "render")]
    RenderCreateConicalGradient(render::CreateConicalGradientRequest<'input>),
    #[cfg(feature = "res")]
    ResQueryVersion(res::QueryVersionRequest),
    #[cfg(feature = "res")]
    ResQueryClients(res::QueryClientsRequest),
    #[cfg(feature = "res")]
    ResQueryClientResources(res::QueryClientResourcesRequest),
    #[cfg(feature = "res")]
    ResQueryClientPixmapBytes(res::QueryClientPixmapBytesRequest),
    #[cfg(feature = "res")]
    ResQueryClientIds(res::QueryClientIdsRequest<'input>),
    #[cfg(feature = "res")]
    ResQueryResourceBytes(res::QueryResourceBytesRequest<'input>),
    #[cfg(feature = "screensaver")]
    ScreensaverQueryVersion(screensaver::QueryVersionRequest),
    #[cfg(feature = "screensaver")]
    ScreensaverQueryInfo(screensaver::QueryInfoRequest),
    #[cfg(feature = "screensaver")]
    ScreensaverSelectInput(screensaver::SelectInputRequest),
    #[cfg(feature = "screensaver")]
    ScreensaverSetAttributes(screensaver::SetAttributesRequest<'input>),
    #[cfg(feature = "screensaver")]
    ScreensaverUnsetAttributes(screensaver::UnsetAttributesRequest),
    #[cfg(feature = "screensaver")]
    ScreensaverSuspend(screensaver::SuspendRequest),
    #[cfg(feature = "shape")]
    ShapeQueryVersion(shape::QueryVersionRequest),
    #[cfg(feature = "shape")]
    ShapeRectangles(shape::RectanglesRequest<'input>),
    #[cfg(feature = "shape")]
    ShapeMask(shape::MaskRequest),
    #[cfg(feature = "shape")]
    ShapeCombine(shape::CombineRequest),
    #[cfg(feature = "shape")]
    ShapeOffset(shape::OffsetRequest),
    #[cfg(feature = "shape")]
    ShapeQueryExtents(shape::QueryExtentsRequest),
    #[cfg(feature = "shape")]
    ShapeSelectInput(shape::SelectInputRequest),
    #[cfg(feature = "shape")]
    ShapeInputSelected(shape::InputSelectedRequest),
    #[cfg(feature = "shape")]
    ShapeGetRectangles(shape::GetRectanglesRequest),
    #[cfg(feature = "shm")]
    ShmQueryVersion(shm::QueryVersionRequest),
    #[cfg(feature = "shm")]
    ShmAttach(shm::AttachRequest),
    #[cfg(feature = "shm")]
    ShmDetach(shm::DetachRequest),
    #[cfg(feature = "shm")]
    ShmPutImage(shm::PutImageRequest),
    #[cfg(feature = "shm")]
    ShmGetImage(shm::GetImageRequest),
    #[cfg(feature = "shm")]
    ShmCreatePixmap(shm::CreatePixmapRequest),
    #[cfg(feature = "shm")]
    ShmAttachFd(shm::AttachFdRequest),
    #[cfg(feature = "shm")]
    ShmCreateSegment(shm::CreateSegmentRequest),
    #[cfg(feature = "sync")]
    SyncInitialize(sync::InitializeRequest),
    #[cfg(feature = "sync")]
    SyncListSystemCounters(sync::ListSystemCountersRequest),
    #[cfg(feature = "sync")]
    SyncCreateCounter(sync::CreateCounterRequest),
    #[cfg(feature = "sync")]
    SyncDestroyCounter(sync::DestroyCounterRequest),
    #[cfg(feature = "sync")]
    SyncQueryCounter(sync::QueryCounterRequest),
    #[cfg(feature = "sync")]
    SyncAwait(sync::AwaitRequest<'input>),
    #[cfg(feature = "sync")]
    SyncChangeCounter(sync::ChangeCounterRequest),
    #[cfg(feature = "sync")]
    SyncSetCounter(sync::SetCounterRequest),
    #[cfg(feature = "sync")]
    SyncCreateAlarm(sync::CreateAlarmRequest<'input>),
    #[cfg(feature = "sync")]
    SyncChangeAlarm(sync::ChangeAlarmRequest<'input>),
    #[cfg(feature = "sync")]
    SyncDestroyAlarm(sync::DestroyAlarmRequest),
    #[cfg(feature = "sync")]
    SyncQueryAlarm(sync::QueryAlarmRequest),
    #[cfg(feature = "sync")]
    SyncSetPriority(sync::SetPriorityRequest),
    #[cfg(feature = "sync")]
    SyncGetPriority(sync::GetPriorityRequest),
    #[cfg(feature = "sync")]
    SyncCreateFence(sync::CreateFenceRequest),
    #[cfg(feature = "sync")]
    SyncTriggerFence(sync::TriggerFenceRequest),
    #[cfg(feature = "sync")]
    SyncResetFence(sync::ResetFenceRequest),
    #[cfg(feature = "sync")]
    SyncDestroyFence(sync::DestroyFenceRequest),
    #[cfg(feature = "sync")]
    SyncQueryFence(sync::QueryFenceRequest),
    #[cfg(feature = "sync")]
    SyncAwaitFence(sync::AwaitFenceRequest<'input>),
    XcMiscGetVersion(xc_misc::GetVersionRequest),
    XcMiscGetXIDRange(xc_misc::GetXIDRangeRequest),
    XcMiscGetXIDList(xc_misc::GetXIDListRequest),
    #[cfg(feature = "xevie")]
    XevieQueryVersion(xevie::QueryVersionRequest),
    #[cfg(feature = "xevie")]
    XevieStart(xevie::StartRequest),
    #[cfg(feature = "xevie")]
    XevieEnd(xevie::EndRequest),
    #[cfg(feature = "xevie")]
    XevieSend(xevie::SendRequest),
    #[cfg(feature = "xevie")]
    XevieSelectInput(xevie::SelectInputRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driQueryVersion(xf86dri::QueryVersionRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driQueryDirectRenderingCapable(xf86dri::QueryDirectRenderingCapableRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driOpenConnection(xf86dri::OpenConnectionRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driCloseConnection(xf86dri::CloseConnectionRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driGetClientDriverName(xf86dri::GetClientDriverNameRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driCreateContext(xf86dri::CreateContextRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driDestroyContext(xf86dri::DestroyContextRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driCreateDrawable(xf86dri::CreateDrawableRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driDestroyDrawable(xf86dri::DestroyDrawableRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driGetDrawableInfo(xf86dri::GetDrawableInfoRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driGetDeviceInfo(xf86dri::GetDeviceInfoRequest),
    #[cfg(feature = "xf86dri")]
    Xf86driAuthConnection(xf86dri::AuthConnectionRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeQueryVersion(xf86vidmode::QueryVersionRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetModeLine(xf86vidmode::GetModeLineRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeModModeLine(xf86vidmode::ModModeLineRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSwitchMode(xf86vidmode::SwitchModeRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetMonitor(xf86vidmode::GetMonitorRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeLockModeSwitch(xf86vidmode::LockModeSwitchRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetAllModeLines(xf86vidmode::GetAllModeLinesRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeAddModeLine(xf86vidmode::AddModeLineRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeDeleteModeLine(xf86vidmode::DeleteModeLineRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeValidateModeLine(xf86vidmode::ValidateModeLineRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSwitchToMode(xf86vidmode::SwitchToModeRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetViewPort(xf86vidmode::GetViewPortRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSetViewPort(xf86vidmode::SetViewPortRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetDotClocks(xf86vidmode::GetDotClocksRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSetClientVersion(xf86vidmode::SetClientVersionRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSetGamma(xf86vidmode::SetGammaRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGamma(xf86vidmode::GetGammaRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGammaRamp(xf86vidmode::GetGammaRampRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeSetGammaRamp(xf86vidmode::SetGammaRampRequest<'input>),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGammaRampSize(xf86vidmode::GetGammaRampSizeRequest),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetPermissions(xf86vidmode::GetPermissionsRequest),
    #[cfg(feature = "xfixes")]
    XfixesQueryVersion(xfixes::QueryVersionRequest),
    #[cfg(feature = "xfixes")]
    XfixesChangeSaveSet(xfixes::ChangeSaveSetRequest),
    #[cfg(feature = "xfixes")]
    XfixesSelectSelectionInput(xfixes::SelectSelectionInputRequest),
    #[cfg(feature = "xfixes")]
    XfixesSelectCursorInput(xfixes::SelectCursorInputRequest),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorImage(xfixes::GetCursorImageRequest),
    #[cfg(feature = "xfixes")]
    XfixesCreateRegion(xfixes::CreateRegionRequest<'input>),
    #[cfg(feature = "xfixes")]
    XfixesCreateRegionFromBitmap(xfixes::CreateRegionFromBitmapRequest),
    #[cfg(feature = "xfixes")]
    XfixesCreateRegionFromWindow(xfixes::CreateRegionFromWindowRequest),
    #[cfg(feature = "xfixes")]
    XfixesCreateRegionFromGC(xfixes::CreateRegionFromGCRequest),
    #[cfg(feature = "xfixes")]
    XfixesCreateRegionFromPicture(xfixes::CreateRegionFromPictureRequest),
    #[cfg(feature = "xfixes")]
    XfixesDestroyRegion(xfixes::DestroyRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSetRegion(xfixes::SetRegionRequest<'input>),
    #[cfg(feature = "xfixes")]
    XfixesCopyRegion(xfixes::CopyRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesUnionRegion(xfixes::UnionRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesIntersectRegion(xfixes::IntersectRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSubtractRegion(xfixes::SubtractRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesInvertRegion(xfixes::InvertRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesTranslateRegion(xfixes::TranslateRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesRegionExtents(xfixes::RegionExtentsRequest),
    #[cfg(feature = "xfixes")]
    XfixesFetchRegion(xfixes::FetchRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSetGCClipRegion(xfixes::SetGCClipRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSetWindowShapeRegion(xfixes::SetWindowShapeRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSetPictureClipRegion(xfixes::SetPictureClipRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesSetCursorName(xfixes::SetCursorNameRequest<'input>),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorName(xfixes::GetCursorNameRequest),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorImageAndName(xfixes::GetCursorImageAndNameRequest),
    #[cfg(feature = "xfixes")]
    XfixesChangeCursor(xfixes::ChangeCursorRequest),
    #[cfg(feature = "xfixes")]
    XfixesChangeCursorByName(xfixes::ChangeCursorByNameRequest<'input>),
    #[cfg(feature = "xfixes")]
    XfixesExpandRegion(xfixes::ExpandRegionRequest),
    #[cfg(feature = "xfixes")]
    XfixesHideCursor(xfixes::HideCursorRequest),
    #[cfg(feature = "xfixes")]
    XfixesShowCursor(xfixes::ShowCursorRequest),
    #[cfg(feature = "xfixes")]
    XfixesCreatePointerBarrier(xfixes::CreatePointerBarrierRequest<'input>),
    #[cfg(feature = "xfixes")]
    XfixesDeletePointerBarrier(xfixes::DeletePointerBarrierRequest),
    #[cfg(feature = "xinerama")]
    XineramaQueryVersion(xinerama::QueryVersionRequest),
    #[cfg(feature = "xinerama")]
    XineramaGetState(xinerama::GetStateRequest),
    #[cfg(feature = "xinerama")]
    XineramaGetScreenCount(xinerama::GetScreenCountRequest),
    #[cfg(feature = "xinerama")]
    XineramaGetScreenSize(xinerama::GetScreenSizeRequest),
    #[cfg(feature = "xinerama")]
    XineramaIsActive(xinerama::IsActiveRequest),
    #[cfg(feature = "xinerama")]
    XineramaQueryScreens(xinerama::QueryScreensRequest),
    #[cfg(feature = "xinput")]
    XinputGetExtensionVersion(xinput::GetExtensionVersionRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputListInputDevices(xinput::ListInputDevicesRequest),
    #[cfg(feature = "xinput")]
    XinputOpenDevice(xinput::OpenDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputCloseDevice(xinput::CloseDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputSetDeviceMode(xinput::SetDeviceModeRequest),
    #[cfg(feature = "xinput")]
    XinputSelectExtensionEvent(xinput::SelectExtensionEventRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputGetSelectedExtensionEvents(xinput::GetSelectedExtensionEventsRequest),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceDontPropagateList(xinput::ChangeDeviceDontPropagateListRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputGetDeviceDontPropagateList(xinput::GetDeviceDontPropagateListRequest),
    #[cfg(feature = "xinput")]
    XinputGetDeviceMotionEvents(xinput::GetDeviceMotionEventsRequest),
    #[cfg(feature = "xinput")]
    XinputChangeKeyboardDevice(xinput::ChangeKeyboardDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputChangePointerDevice(xinput::ChangePointerDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputGrabDevice(xinput::GrabDeviceRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputUngrabDevice(xinput::UngrabDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputGrabDeviceKey(xinput::GrabDeviceKeyRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputUngrabDeviceKey(xinput::UngrabDeviceKeyRequest),
    #[cfg(feature = "xinput")]
    XinputGrabDeviceButton(xinput::GrabDeviceButtonRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputUngrabDeviceButton(xinput::UngrabDeviceButtonRequest),
    #[cfg(feature = "xinput")]
    XinputAllowDeviceEvents(xinput::AllowDeviceEventsRequest),
    #[cfg(feature = "xinput")]
    XinputGetDeviceFocus(xinput::GetDeviceFocusRequest),
    #[cfg(feature = "xinput")]
    XinputSetDeviceFocus(xinput::SetDeviceFocusRequest),
    #[cfg(feature = "xinput")]
    XinputGetFeedbackControl(xinput::GetFeedbackControlRequest),
    #[cfg(feature = "xinput")]
    XinputChangeFeedbackControl(xinput::ChangeFeedbackControlRequest),
    #[cfg(feature = "xinput")]
    XinputGetDeviceKeyMapping(xinput::GetDeviceKeyMappingRequest),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceKeyMapping(xinput::ChangeDeviceKeyMappingRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputGetDeviceModifierMapping(xinput::GetDeviceModifierMappingRequest),
    #[cfg(feature = "xinput")]
    XinputSetDeviceModifierMapping(xinput::SetDeviceModifierMappingRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputGetDeviceButtonMapping(xinput::GetDeviceButtonMappingRequest),
    #[cfg(feature = "xinput")]
    XinputSetDeviceButtonMapping(xinput::SetDeviceButtonMappingRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputQueryDeviceState(xinput::QueryDeviceStateRequest),
    #[cfg(feature = "xinput")]
    XinputDeviceBell(xinput::DeviceBellRequest),
    #[cfg(feature = "xinput")]
    XinputSetDeviceValuators(xinput::SetDeviceValuatorsRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputGetDeviceControl(xinput::GetDeviceControlRequest),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceControl(xinput::ChangeDeviceControlRequest),
    #[cfg(feature = "xinput")]
    XinputListDeviceProperties(xinput::ListDevicePropertiesRequest),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceProperty(xinput::ChangeDevicePropertyRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputDeleteDeviceProperty(xinput::DeleteDevicePropertyRequest),
    #[cfg(feature = "xinput")]
    XinputGetDeviceProperty(xinput::GetDevicePropertyRequest),
    #[cfg(feature = "xinput")]
    XinputXIQueryPointer(xinput::XIQueryPointerRequest),
    #[cfg(feature = "xinput")]
    XinputXIWarpPointer(xinput::XIWarpPointerRequest),
    #[cfg(feature = "xinput")]
    XinputXIChangeCursor(xinput::XIChangeCursorRequest),
    #[cfg(feature = "xinput")]
    XinputXIChangeHierarchy(xinput::XIChangeHierarchyRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXISetClientPointer(xinput::XISetClientPointerRequest),
    #[cfg(feature = "xinput")]
    XinputXIGetClientPointer(xinput::XIGetClientPointerRequest),
    #[cfg(feature = "xinput")]
    XinputXISelectEvents(xinput::XISelectEventsRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXIQueryVersion(xinput::XIQueryVersionRequest),
    #[cfg(feature = "xinput")]
    XinputXIQueryDevice(xinput::XIQueryDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputXISetFocus(xinput::XISetFocusRequest),
    #[cfg(feature = "xinput")]
    XinputXIGetFocus(xinput::XIGetFocusRequest),
    #[cfg(feature = "xinput")]
    XinputXIGrabDevice(xinput::XIGrabDeviceRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXIUngrabDevice(xinput::XIUngrabDeviceRequest),
    #[cfg(feature = "xinput")]
    XinputXIAllowEvents(xinput::XIAllowEventsRequest),
    #[cfg(feature = "xinput")]
    XinputXIPassiveGrabDevice(xinput::XIPassiveGrabDeviceRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXIPassiveUngrabDevice(xinput::XIPassiveUngrabDeviceRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXIListProperties(xinput::XIListPropertiesRequest),
    #[cfg(feature = "xinput")]
    XinputXIChangeProperty(xinput::XIChangePropertyRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputXIDeleteProperty(xinput::XIDeletePropertyRequest),
    #[cfg(feature = "xinput")]
    XinputXIGetProperty(xinput::XIGetPropertyRequest),
    #[cfg(feature = "xinput")]
    XinputXIGetSelectedEvents(xinput::XIGetSelectedEventsRequest),
    #[cfg(feature = "xinput")]
    XinputXIBarrierReleasePointer(xinput::XIBarrierReleasePointerRequest<'input>),
    #[cfg(feature = "xinput")]
    XinputSendExtensionEvent(xinput::SendExtensionEventRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbUseExtension(xkb::UseExtensionRequest),
    #[cfg(feature = "xkb")]
    XkbSelectEvents(xkb::SelectEventsRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbBell(xkb::BellRequest),
    #[cfg(feature = "xkb")]
    XkbGetState(xkb::GetStateRequest),
    #[cfg(feature = "xkb")]
    XkbLatchLockState(xkb::LatchLockStateRequest),
    #[cfg(feature = "xkb")]
    XkbGetControls(xkb::GetControlsRequest),
    #[cfg(feature = "xkb")]
    XkbSetControls(xkb::SetControlsRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbGetMap(xkb::GetMapRequest),
    #[cfg(feature = "xkb")]
    XkbSetMap(xkb::SetMapRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbGetCompatMap(xkb::GetCompatMapRequest),
    #[cfg(feature = "xkb")]
    XkbSetCompatMap(xkb::SetCompatMapRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbGetIndicatorState(xkb::GetIndicatorStateRequest),
    #[cfg(feature = "xkb")]
    XkbGetIndicatorMap(xkb::GetIndicatorMapRequest),
    #[cfg(feature = "xkb")]
    XkbSetIndicatorMap(xkb::SetIndicatorMapRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbGetNamedIndicator(xkb::GetNamedIndicatorRequest),
    #[cfg(feature = "xkb")]
    XkbSetNamedIndicator(xkb::SetNamedIndicatorRequest),
    #[cfg(feature = "xkb")]
    XkbGetNames(xkb::GetNamesRequest),
    #[cfg(feature = "xkb")]
    XkbSetNames(xkb::SetNamesRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbPerClientFlags(xkb::PerClientFlagsRequest),
    #[cfg(feature = "xkb")]
    XkbListComponents(xkb::ListComponentsRequest),
    #[cfg(feature = "xkb")]
    XkbGetKbdByName(xkb::GetKbdByNameRequest),
    #[cfg(feature = "xkb")]
    XkbGetDeviceInfo(xkb::GetDeviceInfoRequest),
    #[cfg(feature = "xkb")]
    XkbSetDeviceInfo(xkb::SetDeviceInfoRequest<'input>),
    #[cfg(feature = "xkb")]
    XkbSetDebuggingFlags(xkb::SetDebuggingFlagsRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintQueryVersion(xprint::PrintQueryVersionRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetPrinterList(xprint::PrintGetPrinterListRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintRehashPrinterList(xprint::PrintRehashPrinterListRequest),
    #[cfg(feature = "xprint")]
    XprintCreateContext(xprint::CreateContextRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintSetContext(xprint::PrintSetContextRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetContext(xprint::PrintGetContextRequest),
    #[cfg(feature = "xprint")]
    XprintPrintDestroyContext(xprint::PrintDestroyContextRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetScreenOfContext(xprint::PrintGetScreenOfContextRequest),
    #[cfg(feature = "xprint")]
    XprintPrintStartJob(xprint::PrintStartJobRequest),
    #[cfg(feature = "xprint")]
    XprintPrintEndJob(xprint::PrintEndJobRequest),
    #[cfg(feature = "xprint")]
    XprintPrintStartDoc(xprint::PrintStartDocRequest),
    #[cfg(feature = "xprint")]
    XprintPrintEndDoc(xprint::PrintEndDocRequest),
    #[cfg(feature = "xprint")]
    XprintPrintPutDocumentData(xprint::PrintPutDocumentDataRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintGetDocumentData(xprint::PrintGetDocumentDataRequest),
    #[cfg(feature = "xprint")]
    XprintPrintStartPage(xprint::PrintStartPageRequest),
    #[cfg(feature = "xprint")]
    XprintPrintEndPage(xprint::PrintEndPageRequest),
    #[cfg(feature = "xprint")]
    XprintPrintSelectInput(xprint::PrintSelectInputRequest),
    #[cfg(feature = "xprint")]
    XprintPrintInputSelected(xprint::PrintInputSelectedRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetAttributes(xprint::PrintGetAttributesRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetOneAttributes(xprint::PrintGetOneAttributesRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintSetAttributes(xprint::PrintSetAttributesRequest<'input>),
    #[cfg(feature = "xprint")]
    XprintPrintGetPageDimensions(xprint::PrintGetPageDimensionsRequest),
    #[cfg(feature = "xprint")]
    XprintPrintQueryScreens(xprint::PrintQueryScreensRequest),
    #[cfg(feature = "xprint")]
    XprintPrintSetImageResolution(xprint::PrintSetImageResolutionRequest),
    #[cfg(feature = "xprint")]
    XprintPrintGetImageResolution(xprint::PrintGetImageResolutionRequest),
    #[cfg(feature = "xselinux")]
    XselinuxQueryVersion(xselinux::QueryVersionRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetDeviceCreateContext(xselinux::SetDeviceCreateContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetDeviceCreateContext(xselinux::GetDeviceCreateContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetDeviceContext(xselinux::SetDeviceContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetDeviceContext(xselinux::GetDeviceContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetWindowCreateContext(xselinux::SetWindowCreateContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetWindowCreateContext(xselinux::GetWindowCreateContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetWindowContext(xselinux::GetWindowContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetPropertyCreateContext(xselinux::SetPropertyCreateContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyCreateContext(xselinux::GetPropertyCreateContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetPropertyUseContext(xselinux::SetPropertyUseContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyUseContext(xselinux::GetPropertyUseContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyContext(xselinux::GetPropertyContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyDataContext(xselinux::GetPropertyDataContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxListProperties(xselinux::ListPropertiesRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetSelectionCreateContext(xselinux::SetSelectionCreateContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionCreateContext(xselinux::GetSelectionCreateContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxSetSelectionUseContext(xselinux::SetSelectionUseContextRequest<'input>),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionUseContext(xselinux::GetSelectionUseContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionContext(xselinux::GetSelectionContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionDataContext(xselinux::GetSelectionDataContextRequest),
    #[cfg(feature = "xselinux")]
    XselinuxListSelections(xselinux::ListSelectionsRequest),
    #[cfg(feature = "xselinux")]
    XselinuxGetClientContext(xselinux::GetClientContextRequest),
    #[cfg(feature = "xtest")]
    XtestGetVersion(xtest::GetVersionRequest),
    #[cfg(feature = "xtest")]
    XtestCompareCursor(xtest::CompareCursorRequest),
    #[cfg(feature = "xtest")]
    XtestFakeInput(xtest::FakeInputRequest),
    #[cfg(feature = "xtest")]
    XtestGrabControl(xtest::GrabControlRequest),
    #[cfg(feature = "xv")]
    XvQueryExtension(xv::QueryExtensionRequest),
    #[cfg(feature = "xv")]
    XvQueryAdaptors(xv::QueryAdaptorsRequest),
    #[cfg(feature = "xv")]
    XvQueryEncodings(xv::QueryEncodingsRequest),
    #[cfg(feature = "xv")]
    XvGrabPort(xv::GrabPortRequest),
    #[cfg(feature = "xv")]
    XvUngrabPort(xv::UngrabPortRequest),
    #[cfg(feature = "xv")]
    XvPutVideo(xv::PutVideoRequest),
    #[cfg(feature = "xv")]
    XvPutStill(xv::PutStillRequest),
    #[cfg(feature = "xv")]
    XvGetVideo(xv::GetVideoRequest),
    #[cfg(feature = "xv")]
    XvGetStill(xv::GetStillRequest),
    #[cfg(feature = "xv")]
    XvStopVideo(xv::StopVideoRequest),
    #[cfg(feature = "xv")]
    XvSelectVideoNotify(xv::SelectVideoNotifyRequest),
    #[cfg(feature = "xv")]
    XvSelectPortNotify(xv::SelectPortNotifyRequest),
    #[cfg(feature = "xv")]
    XvQueryBestSize(xv::QueryBestSizeRequest),
    #[cfg(feature = "xv")]
    XvSetPortAttribute(xv::SetPortAttributeRequest),
    #[cfg(feature = "xv")]
    XvGetPortAttribute(xv::GetPortAttributeRequest),
    #[cfg(feature = "xv")]
    XvQueryPortAttributes(xv::QueryPortAttributesRequest),
    #[cfg(feature = "xv")]
    XvListImageFormats(xv::ListImageFormatsRequest),
    #[cfg(feature = "xv")]
    XvQueryImageAttributes(xv::QueryImageAttributesRequest),
    #[cfg(feature = "xv")]
    XvPutImage(xv::PutImageRequest<'input>),
    #[cfg(feature = "xv")]
    XvShmPutImage(xv::ShmPutImageRequest),
    #[cfg(feature = "xvmc")]
    XvmcQueryVersion(xvmc::QueryVersionRequest),
    #[cfg(feature = "xvmc")]
    XvmcListSurfaceTypes(xvmc::ListSurfaceTypesRequest),
    #[cfg(feature = "xvmc")]
    XvmcCreateContext(xvmc::CreateContextRequest),
    #[cfg(feature = "xvmc")]
    XvmcDestroyContext(xvmc::DestroyContextRequest),
    #[cfg(feature = "xvmc")]
    XvmcCreateSurface(xvmc::CreateSurfaceRequest),
    #[cfg(feature = "xvmc")]
    XvmcDestroySurface(xvmc::DestroySurfaceRequest),
    #[cfg(feature = "xvmc")]
    XvmcCreateSubpicture(xvmc::CreateSubpictureRequest),
    #[cfg(feature = "xvmc")]
    XvmcDestroySubpicture(xvmc::DestroySubpictureRequest),
    #[cfg(feature = "xvmc")]
    XvmcListSubpictureTypes(xvmc::ListSubpictureTypesRequest),
}

impl<'input> Request<'input> {
    // Parse a X11 request into a concrete type
    #[allow(clippy::cognitive_complexity, clippy::single_match)]
    pub fn parse(
        header: RequestHeader,
        body: &'input [u8],
        fds: &mut Vec<RawFdContainer>,
        ext_info_provider: &dyn ExtInfoProvider,
    ) -> Result<Self, ParseError> {
        // Might not be used if none of the extensions that use FD passing is enabled
        // The `allow` is not in the function argument because it is not stable in Rust 1.37
        #[allow(unused_variables)]
        let fds = fds;
        let remaining = body;
        // Check if this is a core protocol request.
        match header.major_opcode {
            xproto::CREATE_WINDOW_REQUEST => return Ok(Request::CreateWindow(xproto::CreateWindowRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_WINDOW_ATTRIBUTES_REQUEST => return Ok(Request::ChangeWindowAttributes(xproto::ChangeWindowAttributesRequest::try_parse_request(header, remaining)?)),
            xproto::GET_WINDOW_ATTRIBUTES_REQUEST => return Ok(Request::GetWindowAttributes(xproto::GetWindowAttributesRequest::try_parse_request(header, remaining)?)),
            xproto::DESTROY_WINDOW_REQUEST => return Ok(Request::DestroyWindow(xproto::DestroyWindowRequest::try_parse_request(header, remaining)?)),
            xproto::DESTROY_SUBWINDOWS_REQUEST => return Ok(Request::DestroySubwindows(xproto::DestroySubwindowsRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_SAVE_SET_REQUEST => return Ok(Request::ChangeSaveSet(xproto::ChangeSaveSetRequest::try_parse_request(header, remaining)?)),
            xproto::REPARENT_WINDOW_REQUEST => return Ok(Request::ReparentWindow(xproto::ReparentWindowRequest::try_parse_request(header, remaining)?)),
            xproto::MAP_WINDOW_REQUEST => return Ok(Request::MapWindow(xproto::MapWindowRequest::try_parse_request(header, remaining)?)),
            xproto::MAP_SUBWINDOWS_REQUEST => return Ok(Request::MapSubwindows(xproto::MapSubwindowsRequest::try_parse_request(header, remaining)?)),
            xproto::UNMAP_WINDOW_REQUEST => return Ok(Request::UnmapWindow(xproto::UnmapWindowRequest::try_parse_request(header, remaining)?)),
            xproto::UNMAP_SUBWINDOWS_REQUEST => return Ok(Request::UnmapSubwindows(xproto::UnmapSubwindowsRequest::try_parse_request(header, remaining)?)),
            xproto::CONFIGURE_WINDOW_REQUEST => return Ok(Request::ConfigureWindow(xproto::ConfigureWindowRequest::try_parse_request(header, remaining)?)),
            xproto::CIRCULATE_WINDOW_REQUEST => return Ok(Request::CirculateWindow(xproto::CirculateWindowRequest::try_parse_request(header, remaining)?)),
            xproto::GET_GEOMETRY_REQUEST => return Ok(Request::GetGeometry(xproto::GetGeometryRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_TREE_REQUEST => return Ok(Request::QueryTree(xproto::QueryTreeRequest::try_parse_request(header, remaining)?)),
            xproto::INTERN_ATOM_REQUEST => return Ok(Request::InternAtom(xproto::InternAtomRequest::try_parse_request(header, remaining)?)),
            xproto::GET_ATOM_NAME_REQUEST => return Ok(Request::GetAtomName(xproto::GetAtomNameRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_PROPERTY_REQUEST => return Ok(Request::ChangeProperty(xproto::ChangePropertyRequest::try_parse_request(header, remaining)?)),
            xproto::DELETE_PROPERTY_REQUEST => return Ok(Request::DeleteProperty(xproto::DeletePropertyRequest::try_parse_request(header, remaining)?)),
            xproto::GET_PROPERTY_REQUEST => return Ok(Request::GetProperty(xproto::GetPropertyRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_PROPERTIES_REQUEST => return Ok(Request::ListProperties(xproto::ListPropertiesRequest::try_parse_request(header, remaining)?)),
            xproto::SET_SELECTION_OWNER_REQUEST => return Ok(Request::SetSelectionOwner(xproto::SetSelectionOwnerRequest::try_parse_request(header, remaining)?)),
            xproto::GET_SELECTION_OWNER_REQUEST => return Ok(Request::GetSelectionOwner(xproto::GetSelectionOwnerRequest::try_parse_request(header, remaining)?)),
            xproto::CONVERT_SELECTION_REQUEST => return Ok(Request::ConvertSelection(xproto::ConvertSelectionRequest::try_parse_request(header, remaining)?)),
            xproto::SEND_EVENT_REQUEST => return Ok(Request::SendEvent(xproto::SendEventRequest::try_parse_request(header, remaining)?)),
            xproto::GRAB_POINTER_REQUEST => return Ok(Request::GrabPointer(xproto::GrabPointerRequest::try_parse_request(header, remaining)?)),
            xproto::UNGRAB_POINTER_REQUEST => return Ok(Request::UngrabPointer(xproto::UngrabPointerRequest::try_parse_request(header, remaining)?)),
            xproto::GRAB_BUTTON_REQUEST => return Ok(Request::GrabButton(xproto::GrabButtonRequest::try_parse_request(header, remaining)?)),
            xproto::UNGRAB_BUTTON_REQUEST => return Ok(Request::UngrabButton(xproto::UngrabButtonRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_ACTIVE_POINTER_GRAB_REQUEST => return Ok(Request::ChangeActivePointerGrab(xproto::ChangeActivePointerGrabRequest::try_parse_request(header, remaining)?)),
            xproto::GRAB_KEYBOARD_REQUEST => return Ok(Request::GrabKeyboard(xproto::GrabKeyboardRequest::try_parse_request(header, remaining)?)),
            xproto::UNGRAB_KEYBOARD_REQUEST => return Ok(Request::UngrabKeyboard(xproto::UngrabKeyboardRequest::try_parse_request(header, remaining)?)),
            xproto::GRAB_KEY_REQUEST => return Ok(Request::GrabKey(xproto::GrabKeyRequest::try_parse_request(header, remaining)?)),
            xproto::UNGRAB_KEY_REQUEST => return Ok(Request::UngrabKey(xproto::UngrabKeyRequest::try_parse_request(header, remaining)?)),
            xproto::ALLOW_EVENTS_REQUEST => return Ok(Request::AllowEvents(xproto::AllowEventsRequest::try_parse_request(header, remaining)?)),
            xproto::GRAB_SERVER_REQUEST => return Ok(Request::GrabServer(xproto::GrabServerRequest::try_parse_request(header, remaining)?)),
            xproto::UNGRAB_SERVER_REQUEST => return Ok(Request::UngrabServer(xproto::UngrabServerRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_POINTER_REQUEST => return Ok(Request::QueryPointer(xproto::QueryPointerRequest::try_parse_request(header, remaining)?)),
            xproto::GET_MOTION_EVENTS_REQUEST => return Ok(Request::GetMotionEvents(xproto::GetMotionEventsRequest::try_parse_request(header, remaining)?)),
            xproto::TRANSLATE_COORDINATES_REQUEST => return Ok(Request::TranslateCoordinates(xproto::TranslateCoordinatesRequest::try_parse_request(header, remaining)?)),
            xproto::WARP_POINTER_REQUEST => return Ok(Request::WarpPointer(xproto::WarpPointerRequest::try_parse_request(header, remaining)?)),
            xproto::SET_INPUT_FOCUS_REQUEST => return Ok(Request::SetInputFocus(xproto::SetInputFocusRequest::try_parse_request(header, remaining)?)),
            xproto::GET_INPUT_FOCUS_REQUEST => return Ok(Request::GetInputFocus(xproto::GetInputFocusRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_KEYMAP_REQUEST => return Ok(Request::QueryKeymap(xproto::QueryKeymapRequest::try_parse_request(header, remaining)?)),
            xproto::OPEN_FONT_REQUEST => return Ok(Request::OpenFont(xproto::OpenFontRequest::try_parse_request(header, remaining)?)),
            xproto::CLOSE_FONT_REQUEST => return Ok(Request::CloseFont(xproto::CloseFontRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_FONT_REQUEST => return Ok(Request::QueryFont(xproto::QueryFontRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_TEXT_EXTENTS_REQUEST => return Ok(Request::QueryTextExtents(xproto::QueryTextExtentsRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_FONTS_REQUEST => return Ok(Request::ListFonts(xproto::ListFontsRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_FONTS_WITH_INFO_REQUEST => return Ok(Request::ListFontsWithInfo(xproto::ListFontsWithInfoRequest::try_parse_request(header, remaining)?)),
            xproto::SET_FONT_PATH_REQUEST => return Ok(Request::SetFontPath(xproto::SetFontPathRequest::try_parse_request(header, remaining)?)),
            xproto::GET_FONT_PATH_REQUEST => return Ok(Request::GetFontPath(xproto::GetFontPathRequest::try_parse_request(header, remaining)?)),
            xproto::CREATE_PIXMAP_REQUEST => return Ok(Request::CreatePixmap(xproto::CreatePixmapRequest::try_parse_request(header, remaining)?)),
            xproto::FREE_PIXMAP_REQUEST => return Ok(Request::FreePixmap(xproto::FreePixmapRequest::try_parse_request(header, remaining)?)),
            xproto::CREATE_GC_REQUEST => return Ok(Request::CreateGC(xproto::CreateGCRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_GC_REQUEST => return Ok(Request::ChangeGC(xproto::ChangeGCRequest::try_parse_request(header, remaining)?)),
            xproto::COPY_GC_REQUEST => return Ok(Request::CopyGC(xproto::CopyGCRequest::try_parse_request(header, remaining)?)),
            xproto::SET_DASHES_REQUEST => return Ok(Request::SetDashes(xproto::SetDashesRequest::try_parse_request(header, remaining)?)),
            xproto::SET_CLIP_RECTANGLES_REQUEST => return Ok(Request::SetClipRectangles(xproto::SetClipRectanglesRequest::try_parse_request(header, remaining)?)),
            xproto::FREE_GC_REQUEST => return Ok(Request::FreeGC(xproto::FreeGCRequest::try_parse_request(header, remaining)?)),
            xproto::CLEAR_AREA_REQUEST => return Ok(Request::ClearArea(xproto::ClearAreaRequest::try_parse_request(header, remaining)?)),
            xproto::COPY_AREA_REQUEST => return Ok(Request::CopyArea(xproto::CopyAreaRequest::try_parse_request(header, remaining)?)),
            xproto::COPY_PLANE_REQUEST => return Ok(Request::CopyPlane(xproto::CopyPlaneRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_POINT_REQUEST => return Ok(Request::PolyPoint(xproto::PolyPointRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_LINE_REQUEST => return Ok(Request::PolyLine(xproto::PolyLineRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_SEGMENT_REQUEST => return Ok(Request::PolySegment(xproto::PolySegmentRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_RECTANGLE_REQUEST => return Ok(Request::PolyRectangle(xproto::PolyRectangleRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_ARC_REQUEST => return Ok(Request::PolyArc(xproto::PolyArcRequest::try_parse_request(header, remaining)?)),
            xproto::FILL_POLY_REQUEST => return Ok(Request::FillPoly(xproto::FillPolyRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_FILL_RECTANGLE_REQUEST => return Ok(Request::PolyFillRectangle(xproto::PolyFillRectangleRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_FILL_ARC_REQUEST => return Ok(Request::PolyFillArc(xproto::PolyFillArcRequest::try_parse_request(header, remaining)?)),
            xproto::PUT_IMAGE_REQUEST => return Ok(Request::PutImage(xproto::PutImageRequest::try_parse_request(header, remaining)?)),
            xproto::GET_IMAGE_REQUEST => return Ok(Request::GetImage(xproto::GetImageRequest::try_parse_request(header, remaining)?)),
            xproto::POLY_TEXT8_REQUEST => return Ok(Request::PolyText8(xproto::PolyText8Request::try_parse_request(header, remaining)?)),
            xproto::POLY_TEXT16_REQUEST => return Ok(Request::PolyText16(xproto::PolyText16Request::try_parse_request(header, remaining)?)),
            xproto::IMAGE_TEXT8_REQUEST => return Ok(Request::ImageText8(xproto::ImageText8Request::try_parse_request(header, remaining)?)),
            xproto::IMAGE_TEXT16_REQUEST => return Ok(Request::ImageText16(xproto::ImageText16Request::try_parse_request(header, remaining)?)),
            xproto::CREATE_COLORMAP_REQUEST => return Ok(Request::CreateColormap(xproto::CreateColormapRequest::try_parse_request(header, remaining)?)),
            xproto::FREE_COLORMAP_REQUEST => return Ok(Request::FreeColormap(xproto::FreeColormapRequest::try_parse_request(header, remaining)?)),
            xproto::COPY_COLORMAP_AND_FREE_REQUEST => return Ok(Request::CopyColormapAndFree(xproto::CopyColormapAndFreeRequest::try_parse_request(header, remaining)?)),
            xproto::INSTALL_COLORMAP_REQUEST => return Ok(Request::InstallColormap(xproto::InstallColormapRequest::try_parse_request(header, remaining)?)),
            xproto::UNINSTALL_COLORMAP_REQUEST => return Ok(Request::UninstallColormap(xproto::UninstallColormapRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_INSTALLED_COLORMAPS_REQUEST => return Ok(Request::ListInstalledColormaps(xproto::ListInstalledColormapsRequest::try_parse_request(header, remaining)?)),
            xproto::ALLOC_COLOR_REQUEST => return Ok(Request::AllocColor(xproto::AllocColorRequest::try_parse_request(header, remaining)?)),
            xproto::ALLOC_NAMED_COLOR_REQUEST => return Ok(Request::AllocNamedColor(xproto::AllocNamedColorRequest::try_parse_request(header, remaining)?)),
            xproto::ALLOC_COLOR_CELLS_REQUEST => return Ok(Request::AllocColorCells(xproto::AllocColorCellsRequest::try_parse_request(header, remaining)?)),
            xproto::ALLOC_COLOR_PLANES_REQUEST => return Ok(Request::AllocColorPlanes(xproto::AllocColorPlanesRequest::try_parse_request(header, remaining)?)),
            xproto::FREE_COLORS_REQUEST => return Ok(Request::FreeColors(xproto::FreeColorsRequest::try_parse_request(header, remaining)?)),
            xproto::STORE_COLORS_REQUEST => return Ok(Request::StoreColors(xproto::StoreColorsRequest::try_parse_request(header, remaining)?)),
            xproto::STORE_NAMED_COLOR_REQUEST => return Ok(Request::StoreNamedColor(xproto::StoreNamedColorRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_COLORS_REQUEST => return Ok(Request::QueryColors(xproto::QueryColorsRequest::try_parse_request(header, remaining)?)),
            xproto::LOOKUP_COLOR_REQUEST => return Ok(Request::LookupColor(xproto::LookupColorRequest::try_parse_request(header, remaining)?)),
            xproto::CREATE_CURSOR_REQUEST => return Ok(Request::CreateCursor(xproto::CreateCursorRequest::try_parse_request(header, remaining)?)),
            xproto::CREATE_GLYPH_CURSOR_REQUEST => return Ok(Request::CreateGlyphCursor(xproto::CreateGlyphCursorRequest::try_parse_request(header, remaining)?)),
            xproto::FREE_CURSOR_REQUEST => return Ok(Request::FreeCursor(xproto::FreeCursorRequest::try_parse_request(header, remaining)?)),
            xproto::RECOLOR_CURSOR_REQUEST => return Ok(Request::RecolorCursor(xproto::RecolorCursorRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_BEST_SIZE_REQUEST => return Ok(Request::QueryBestSize(xproto::QueryBestSizeRequest::try_parse_request(header, remaining)?)),
            xproto::QUERY_EXTENSION_REQUEST => return Ok(Request::QueryExtension(xproto::QueryExtensionRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_EXTENSIONS_REQUEST => return Ok(Request::ListExtensions(xproto::ListExtensionsRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_KEYBOARD_MAPPING_REQUEST => return Ok(Request::ChangeKeyboardMapping(xproto::ChangeKeyboardMappingRequest::try_parse_request(header, remaining)?)),
            xproto::GET_KEYBOARD_MAPPING_REQUEST => return Ok(Request::GetKeyboardMapping(xproto::GetKeyboardMappingRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_KEYBOARD_CONTROL_REQUEST => return Ok(Request::ChangeKeyboardControl(xproto::ChangeKeyboardControlRequest::try_parse_request(header, remaining)?)),
            xproto::GET_KEYBOARD_CONTROL_REQUEST => return Ok(Request::GetKeyboardControl(xproto::GetKeyboardControlRequest::try_parse_request(header, remaining)?)),
            xproto::BELL_REQUEST => return Ok(Request::Bell(xproto::BellRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_POINTER_CONTROL_REQUEST => return Ok(Request::ChangePointerControl(xproto::ChangePointerControlRequest::try_parse_request(header, remaining)?)),
            xproto::GET_POINTER_CONTROL_REQUEST => return Ok(Request::GetPointerControl(xproto::GetPointerControlRequest::try_parse_request(header, remaining)?)),
            xproto::SET_SCREEN_SAVER_REQUEST => return Ok(Request::SetScreenSaver(xproto::SetScreenSaverRequest::try_parse_request(header, remaining)?)),
            xproto::GET_SCREEN_SAVER_REQUEST => return Ok(Request::GetScreenSaver(xproto::GetScreenSaverRequest::try_parse_request(header, remaining)?)),
            xproto::CHANGE_HOSTS_REQUEST => return Ok(Request::ChangeHosts(xproto::ChangeHostsRequest::try_parse_request(header, remaining)?)),
            xproto::LIST_HOSTS_REQUEST => return Ok(Request::ListHosts(xproto::ListHostsRequest::try_parse_request(header, remaining)?)),
            xproto::SET_ACCESS_CONTROL_REQUEST => return Ok(Request::SetAccessControl(xproto::SetAccessControlRequest::try_parse_request(header, remaining)?)),
            xproto::SET_CLOSE_DOWN_MODE_REQUEST => return Ok(Request::SetCloseDownMode(xproto::SetCloseDownModeRequest::try_parse_request(header, remaining)?)),
            xproto::KILL_CLIENT_REQUEST => return Ok(Request::KillClient(xproto::KillClientRequest::try_parse_request(header, remaining)?)),
            xproto::ROTATE_PROPERTIES_REQUEST => return Ok(Request::RotateProperties(xproto::RotatePropertiesRequest::try_parse_request(header, remaining)?)),
            xproto::FORCE_SCREEN_SAVER_REQUEST => return Ok(Request::ForceScreenSaver(xproto::ForceScreenSaverRequest::try_parse_request(header, remaining)?)),
            xproto::SET_POINTER_MAPPING_REQUEST => return Ok(Request::SetPointerMapping(xproto::SetPointerMappingRequest::try_parse_request(header, remaining)?)),
            xproto::GET_POINTER_MAPPING_REQUEST => return Ok(Request::GetPointerMapping(xproto::GetPointerMappingRequest::try_parse_request(header, remaining)?)),
            xproto::SET_MODIFIER_MAPPING_REQUEST => return Ok(Request::SetModifierMapping(xproto::SetModifierMappingRequest::try_parse_request(header, remaining)?)),
            xproto::GET_MODIFIER_MAPPING_REQUEST => return Ok(Request::GetModifierMapping(xproto::GetModifierMappingRequest::try_parse_request(header, remaining)?)),
            xproto::NO_OPERATION_REQUEST => return Ok(Request::NoOperation(xproto::NoOperationRequest::try_parse_request(header, remaining)?)),
            _ => (),
        }
        // Find the extension that this request could belong to
        let ext_info = ext_info_provider.get_from_major_opcode(header.major_opcode);
        match ext_info {
            Some((bigreq::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    bigreq::ENABLE_REQUEST => return Ok(Request::BigreqEnable(bigreq::EnableRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "composite")]
            Some((composite::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    composite::QUERY_VERSION_REQUEST => return Ok(Request::CompositeQueryVersion(composite::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    composite::REDIRECT_WINDOW_REQUEST => return Ok(Request::CompositeRedirectWindow(composite::RedirectWindowRequest::try_parse_request(header, remaining)?)),
                    composite::REDIRECT_SUBWINDOWS_REQUEST => return Ok(Request::CompositeRedirectSubwindows(composite::RedirectSubwindowsRequest::try_parse_request(header, remaining)?)),
                    composite::UNREDIRECT_WINDOW_REQUEST => return Ok(Request::CompositeUnredirectWindow(composite::UnredirectWindowRequest::try_parse_request(header, remaining)?)),
                    composite::UNREDIRECT_SUBWINDOWS_REQUEST => return Ok(Request::CompositeUnredirectSubwindows(composite::UnredirectSubwindowsRequest::try_parse_request(header, remaining)?)),
                    composite::CREATE_REGION_FROM_BORDER_CLIP_REQUEST => return Ok(Request::CompositeCreateRegionFromBorderClip(composite::CreateRegionFromBorderClipRequest::try_parse_request(header, remaining)?)),
                    composite::NAME_WINDOW_PIXMAP_REQUEST => return Ok(Request::CompositeNameWindowPixmap(composite::NameWindowPixmapRequest::try_parse_request(header, remaining)?)),
                    composite::GET_OVERLAY_WINDOW_REQUEST => return Ok(Request::CompositeGetOverlayWindow(composite::GetOverlayWindowRequest::try_parse_request(header, remaining)?)),
                    composite::RELEASE_OVERLAY_WINDOW_REQUEST => return Ok(Request::CompositeReleaseOverlayWindow(composite::ReleaseOverlayWindowRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    damage::QUERY_VERSION_REQUEST => return Ok(Request::DamageQueryVersion(damage::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    damage::CREATE_REQUEST => return Ok(Request::DamageCreate(damage::CreateRequest::try_parse_request(header, remaining)?)),
                    damage::DESTROY_REQUEST => return Ok(Request::DamageDestroy(damage::DestroyRequest::try_parse_request(header, remaining)?)),
                    damage::SUBTRACT_REQUEST => return Ok(Request::DamageSubtract(damage::SubtractRequest::try_parse_request(header, remaining)?)),
                    damage::ADD_REQUEST => return Ok(Request::DamageAdd(damage::AddRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "dpms")]
            Some((dpms::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    dpms::GET_VERSION_REQUEST => return Ok(Request::DpmsGetVersion(dpms::GetVersionRequest::try_parse_request(header, remaining)?)),
                    dpms::CAPABLE_REQUEST => return Ok(Request::DpmsCapable(dpms::CapableRequest::try_parse_request(header, remaining)?)),
                    dpms::GET_TIMEOUTS_REQUEST => return Ok(Request::DpmsGetTimeouts(dpms::GetTimeoutsRequest::try_parse_request(header, remaining)?)),
                    dpms::SET_TIMEOUTS_REQUEST => return Ok(Request::DpmsSetTimeouts(dpms::SetTimeoutsRequest::try_parse_request(header, remaining)?)),
                    dpms::ENABLE_REQUEST => return Ok(Request::DpmsEnable(dpms::EnableRequest::try_parse_request(header, remaining)?)),
                    dpms::DISABLE_REQUEST => return Ok(Request::DpmsDisable(dpms::DisableRequest::try_parse_request(header, remaining)?)),
                    dpms::FORCE_LEVEL_REQUEST => return Ok(Request::DpmsForceLevel(dpms::ForceLevelRequest::try_parse_request(header, remaining)?)),
                    dpms::INFO_REQUEST => return Ok(Request::DpmsInfo(dpms::InfoRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "dri2")]
            Some((dri2::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    dri2::QUERY_VERSION_REQUEST => return Ok(Request::Dri2QueryVersion(dri2::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    dri2::CONNECT_REQUEST => return Ok(Request::Dri2Connect(dri2::ConnectRequest::try_parse_request(header, remaining)?)),
                    dri2::AUTHENTICATE_REQUEST => return Ok(Request::Dri2Authenticate(dri2::AuthenticateRequest::try_parse_request(header, remaining)?)),
                    dri2::CREATE_DRAWABLE_REQUEST => return Ok(Request::Dri2CreateDrawable(dri2::CreateDrawableRequest::try_parse_request(header, remaining)?)),
                    dri2::DESTROY_DRAWABLE_REQUEST => return Ok(Request::Dri2DestroyDrawable(dri2::DestroyDrawableRequest::try_parse_request(header, remaining)?)),
                    dri2::GET_BUFFERS_REQUEST => return Ok(Request::Dri2GetBuffers(dri2::GetBuffersRequest::try_parse_request(header, remaining)?)),
                    dri2::COPY_REGION_REQUEST => return Ok(Request::Dri2CopyRegion(dri2::CopyRegionRequest::try_parse_request(header, remaining)?)),
                    dri2::GET_BUFFERS_WITH_FORMAT_REQUEST => return Ok(Request::Dri2GetBuffersWithFormat(dri2::GetBuffersWithFormatRequest::try_parse_request(header, remaining)?)),
                    dri2::SWAP_BUFFERS_REQUEST => return Ok(Request::Dri2SwapBuffers(dri2::SwapBuffersRequest::try_parse_request(header, remaining)?)),
                    dri2::GET_MSC_REQUEST => return Ok(Request::Dri2GetMSC(dri2::GetMSCRequest::try_parse_request(header, remaining)?)),
                    dri2::WAIT_MSC_REQUEST => return Ok(Request::Dri2WaitMSC(dri2::WaitMSCRequest::try_parse_request(header, remaining)?)),
                    dri2::WAIT_SBC_REQUEST => return Ok(Request::Dri2WaitSBC(dri2::WaitSBCRequest::try_parse_request(header, remaining)?)),
                    dri2::SWAP_INTERVAL_REQUEST => return Ok(Request::Dri2SwapInterval(dri2::SwapIntervalRequest::try_parse_request(header, remaining)?)),
                    dri2::GET_PARAM_REQUEST => return Ok(Request::Dri2GetParam(dri2::GetParamRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "dri3")]
            Some((dri3::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    dri3::QUERY_VERSION_REQUEST => return Ok(Request::Dri3QueryVersion(dri3::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    dri3::OPEN_REQUEST => return Ok(Request::Dri3Open(dri3::OpenRequest::try_parse_request(header, remaining)?)),
                    dri3::PIXMAP_FROM_BUFFER_REQUEST => return Ok(Request::Dri3PixmapFromBuffer(dri3::PixmapFromBufferRequest::try_parse_request_fd(header, remaining, fds)?)),
                    dri3::BUFFER_FROM_PIXMAP_REQUEST => return Ok(Request::Dri3BufferFromPixmap(dri3::BufferFromPixmapRequest::try_parse_request(header, remaining)?)),
                    dri3::FENCE_FROM_FD_REQUEST => return Ok(Request::Dri3FenceFromFD(dri3::FenceFromFDRequest::try_parse_request_fd(header, remaining, fds)?)),
                    dri3::FD_FROM_FENCE_REQUEST => return Ok(Request::Dri3FDFromFence(dri3::FDFromFenceRequest::try_parse_request(header, remaining)?)),
                    dri3::GET_SUPPORTED_MODIFIERS_REQUEST => return Ok(Request::Dri3GetSupportedModifiers(dri3::GetSupportedModifiersRequest::try_parse_request(header, remaining)?)),
                    dri3::PIXMAP_FROM_BUFFERS_REQUEST => return Ok(Request::Dri3PixmapFromBuffers(dri3::PixmapFromBuffersRequest::try_parse_request_fd(header, remaining, fds)?)),
                    dri3::BUFFERS_FROM_PIXMAP_REQUEST => return Ok(Request::Dri3BuffersFromPixmap(dri3::BuffersFromPixmapRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            Some((ge::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    ge::QUERY_VERSION_REQUEST => return Ok(Request::GeQueryVersion(ge::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    glx::RENDER_REQUEST => return Ok(Request::GlxRender(glx::RenderRequest::try_parse_request(header, remaining)?)),
                    glx::RENDER_LARGE_REQUEST => return Ok(Request::GlxRenderLarge(glx::RenderLargeRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_CONTEXT_REQUEST => return Ok(Request::GlxCreateContext(glx::CreateContextRequest::try_parse_request(header, remaining)?)),
                    glx::DESTROY_CONTEXT_REQUEST => return Ok(Request::GlxDestroyContext(glx::DestroyContextRequest::try_parse_request(header, remaining)?)),
                    glx::MAKE_CURRENT_REQUEST => return Ok(Request::GlxMakeCurrent(glx::MakeCurrentRequest::try_parse_request(header, remaining)?)),
                    glx::IS_DIRECT_REQUEST => return Ok(Request::GlxIsDirect(glx::IsDirectRequest::try_parse_request(header, remaining)?)),
                    glx::QUERY_VERSION_REQUEST => return Ok(Request::GlxQueryVersion(glx::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    glx::WAIT_GL_REQUEST => return Ok(Request::GlxWaitGL(glx::WaitGLRequest::try_parse_request(header, remaining)?)),
                    glx::WAIT_X_REQUEST => return Ok(Request::GlxWaitX(glx::WaitXRequest::try_parse_request(header, remaining)?)),
                    glx::COPY_CONTEXT_REQUEST => return Ok(Request::GlxCopyContext(glx::CopyContextRequest::try_parse_request(header, remaining)?)),
                    glx::SWAP_BUFFERS_REQUEST => return Ok(Request::GlxSwapBuffers(glx::SwapBuffersRequest::try_parse_request(header, remaining)?)),
                    glx::USE_X_FONT_REQUEST => return Ok(Request::GlxUseXFont(glx::UseXFontRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_GLX_PIXMAP_REQUEST => return Ok(Request::GlxCreateGLXPixmap(glx::CreateGLXPixmapRequest::try_parse_request(header, remaining)?)),
                    glx::GET_VISUAL_CONFIGS_REQUEST => return Ok(Request::GlxGetVisualConfigs(glx::GetVisualConfigsRequest::try_parse_request(header, remaining)?)),
                    glx::DESTROY_GLX_PIXMAP_REQUEST => return Ok(Request::GlxDestroyGLXPixmap(glx::DestroyGLXPixmapRequest::try_parse_request(header, remaining)?)),
                    glx::VENDOR_PRIVATE_REQUEST => return Ok(Request::GlxVendorPrivate(glx::VendorPrivateRequest::try_parse_request(header, remaining)?)),
                    glx::VENDOR_PRIVATE_WITH_REPLY_REQUEST => return Ok(Request::GlxVendorPrivateWithReply(glx::VendorPrivateWithReplyRequest::try_parse_request(header, remaining)?)),
                    glx::QUERY_EXTENSIONS_STRING_REQUEST => return Ok(Request::GlxQueryExtensionsString(glx::QueryExtensionsStringRequest::try_parse_request(header, remaining)?)),
                    glx::QUERY_SERVER_STRING_REQUEST => return Ok(Request::GlxQueryServerString(glx::QueryServerStringRequest::try_parse_request(header, remaining)?)),
                    glx::CLIENT_INFO_REQUEST => return Ok(Request::GlxClientInfo(glx::ClientInfoRequest::try_parse_request(header, remaining)?)),
                    glx::GET_FB_CONFIGS_REQUEST => return Ok(Request::GlxGetFBConfigs(glx::GetFBConfigsRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_PIXMAP_REQUEST => return Ok(Request::GlxCreatePixmap(glx::CreatePixmapRequest::try_parse_request(header, remaining)?)),
                    glx::DESTROY_PIXMAP_REQUEST => return Ok(Request::GlxDestroyPixmap(glx::DestroyPixmapRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_NEW_CONTEXT_REQUEST => return Ok(Request::GlxCreateNewContext(glx::CreateNewContextRequest::try_parse_request(header, remaining)?)),
                    glx::QUERY_CONTEXT_REQUEST => return Ok(Request::GlxQueryContext(glx::QueryContextRequest::try_parse_request(header, remaining)?)),
                    glx::MAKE_CONTEXT_CURRENT_REQUEST => return Ok(Request::GlxMakeContextCurrent(glx::MakeContextCurrentRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_PBUFFER_REQUEST => return Ok(Request::GlxCreatePbuffer(glx::CreatePbufferRequest::try_parse_request(header, remaining)?)),
                    glx::DESTROY_PBUFFER_REQUEST => return Ok(Request::GlxDestroyPbuffer(glx::DestroyPbufferRequest::try_parse_request(header, remaining)?)),
                    glx::GET_DRAWABLE_ATTRIBUTES_REQUEST => return Ok(Request::GlxGetDrawableAttributes(glx::GetDrawableAttributesRequest::try_parse_request(header, remaining)?)),
                    glx::CHANGE_DRAWABLE_ATTRIBUTES_REQUEST => return Ok(Request::GlxChangeDrawableAttributes(glx::ChangeDrawableAttributesRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_WINDOW_REQUEST => return Ok(Request::GlxCreateWindow(glx::CreateWindowRequest::try_parse_request(header, remaining)?)),
                    glx::DELETE_WINDOW_REQUEST => return Ok(Request::GlxDeleteWindow(glx::DeleteWindowRequest::try_parse_request(header, remaining)?)),
                    glx::SET_CLIENT_INFO_ARB_REQUEST => return Ok(Request::GlxSetClientInfoARB(glx::SetClientInfoARBRequest::try_parse_request(header, remaining)?)),
                    glx::CREATE_CONTEXT_ATTRIBS_ARB_REQUEST => return Ok(Request::GlxCreateContextAttribsARB(glx::CreateContextAttribsARBRequest::try_parse_request(header, remaining)?)),
                    glx::SET_CLIENT_INFO2_ARB_REQUEST => return Ok(Request::GlxSetClientInfo2ARB(glx::SetClientInfo2ARBRequest::try_parse_request(header, remaining)?)),
                    glx::NEW_LIST_REQUEST => return Ok(Request::GlxNewList(glx::NewListRequest::try_parse_request(header, remaining)?)),
                    glx::END_LIST_REQUEST => return Ok(Request::GlxEndList(glx::EndListRequest::try_parse_request(header, remaining)?)),
                    glx::DELETE_LISTS_REQUEST => return Ok(Request::GlxDeleteLists(glx::DeleteListsRequest::try_parse_request(header, remaining)?)),
                    glx::GEN_LISTS_REQUEST => return Ok(Request::GlxGenLists(glx::GenListsRequest::try_parse_request(header, remaining)?)),
                    glx::FEEDBACK_BUFFER_REQUEST => return Ok(Request::GlxFeedbackBuffer(glx::FeedbackBufferRequest::try_parse_request(header, remaining)?)),
                    glx::SELECT_BUFFER_REQUEST => return Ok(Request::GlxSelectBuffer(glx::SelectBufferRequest::try_parse_request(header, remaining)?)),
                    glx::RENDER_MODE_REQUEST => return Ok(Request::GlxRenderMode(glx::RenderModeRequest::try_parse_request(header, remaining)?)),
                    glx::FINISH_REQUEST => return Ok(Request::GlxFinish(glx::FinishRequest::try_parse_request(header, remaining)?)),
                    glx::PIXEL_STOREF_REQUEST => return Ok(Request::GlxPixelStoref(glx::PixelStorefRequest::try_parse_request(header, remaining)?)),
                    glx::PIXEL_STOREI_REQUEST => return Ok(Request::GlxPixelStorei(glx::PixelStoreiRequest::try_parse_request(header, remaining)?)),
                    glx::READ_PIXELS_REQUEST => return Ok(Request::GlxReadPixels(glx::ReadPixelsRequest::try_parse_request(header, remaining)?)),
                    glx::GET_BOOLEANV_REQUEST => return Ok(Request::GlxGetBooleanv(glx::GetBooleanvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_CLIP_PLANE_REQUEST => return Ok(Request::GlxGetClipPlane(glx::GetClipPlaneRequest::try_parse_request(header, remaining)?)),
                    glx::GET_DOUBLEV_REQUEST => return Ok(Request::GlxGetDoublev(glx::GetDoublevRequest::try_parse_request(header, remaining)?)),
                    glx::GET_ERROR_REQUEST => return Ok(Request::GlxGetError(glx::GetErrorRequest::try_parse_request(header, remaining)?)),
                    glx::GET_FLOATV_REQUEST => return Ok(Request::GlxGetFloatv(glx::GetFloatvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_INTEGERV_REQUEST => return Ok(Request::GlxGetIntegerv(glx::GetIntegervRequest::try_parse_request(header, remaining)?)),
                    glx::GET_LIGHTFV_REQUEST => return Ok(Request::GlxGetLightfv(glx::GetLightfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_LIGHTIV_REQUEST => return Ok(Request::GlxGetLightiv(glx::GetLightivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MAPDV_REQUEST => return Ok(Request::GlxGetMapdv(glx::GetMapdvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MAPFV_REQUEST => return Ok(Request::GlxGetMapfv(glx::GetMapfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MAPIV_REQUEST => return Ok(Request::GlxGetMapiv(glx::GetMapivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MATERIALFV_REQUEST => return Ok(Request::GlxGetMaterialfv(glx::GetMaterialfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MATERIALIV_REQUEST => return Ok(Request::GlxGetMaterialiv(glx::GetMaterialivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_PIXEL_MAPFV_REQUEST => return Ok(Request::GlxGetPixelMapfv(glx::GetPixelMapfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_PIXEL_MAPUIV_REQUEST => return Ok(Request::GlxGetPixelMapuiv(glx::GetPixelMapuivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_PIXEL_MAPUSV_REQUEST => return Ok(Request::GlxGetPixelMapusv(glx::GetPixelMapusvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_POLYGON_STIPPLE_REQUEST => return Ok(Request::GlxGetPolygonStipple(glx::GetPolygonStippleRequest::try_parse_request(header, remaining)?)),
                    glx::GET_STRING_REQUEST => return Ok(Request::GlxGetString(glx::GetStringRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_ENVFV_REQUEST => return Ok(Request::GlxGetTexEnvfv(glx::GetTexEnvfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_ENVIV_REQUEST => return Ok(Request::GlxGetTexEnviv(glx::GetTexEnvivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_GENDV_REQUEST => return Ok(Request::GlxGetTexGendv(glx::GetTexGendvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_GENFV_REQUEST => return Ok(Request::GlxGetTexGenfv(glx::GetTexGenfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_GENIV_REQUEST => return Ok(Request::GlxGetTexGeniv(glx::GetTexGenivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_IMAGE_REQUEST => return Ok(Request::GlxGetTexImage(glx::GetTexImageRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_PARAMETERFV_REQUEST => return Ok(Request::GlxGetTexParameterfv(glx::GetTexParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_PARAMETERIV_REQUEST => return Ok(Request::GlxGetTexParameteriv(glx::GetTexParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_LEVEL_PARAMETERFV_REQUEST => return Ok(Request::GlxGetTexLevelParameterfv(glx::GetTexLevelParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_TEX_LEVEL_PARAMETERIV_REQUEST => return Ok(Request::GlxGetTexLevelParameteriv(glx::GetTexLevelParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::IS_ENABLED_REQUEST => return Ok(Request::GlxIsEnabled(glx::IsEnabledRequest::try_parse_request(header, remaining)?)),
                    glx::IS_LIST_REQUEST => return Ok(Request::GlxIsList(glx::IsListRequest::try_parse_request(header, remaining)?)),
                    glx::FLUSH_REQUEST => return Ok(Request::GlxFlush(glx::FlushRequest::try_parse_request(header, remaining)?)),
                    glx::ARE_TEXTURES_RESIDENT_REQUEST => return Ok(Request::GlxAreTexturesResident(glx::AreTexturesResidentRequest::try_parse_request(header, remaining)?)),
                    glx::DELETE_TEXTURES_REQUEST => return Ok(Request::GlxDeleteTextures(glx::DeleteTexturesRequest::try_parse_request(header, remaining)?)),
                    glx::GEN_TEXTURES_REQUEST => return Ok(Request::GlxGenTextures(glx::GenTexturesRequest::try_parse_request(header, remaining)?)),
                    glx::IS_TEXTURE_REQUEST => return Ok(Request::GlxIsTexture(glx::IsTextureRequest::try_parse_request(header, remaining)?)),
                    glx::GET_COLOR_TABLE_REQUEST => return Ok(Request::GlxGetColorTable(glx::GetColorTableRequest::try_parse_request(header, remaining)?)),
                    glx::GET_COLOR_TABLE_PARAMETERFV_REQUEST => return Ok(Request::GlxGetColorTableParameterfv(glx::GetColorTableParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_COLOR_TABLE_PARAMETERIV_REQUEST => return Ok(Request::GlxGetColorTableParameteriv(glx::GetColorTableParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_CONVOLUTION_FILTER_REQUEST => return Ok(Request::GlxGetConvolutionFilter(glx::GetConvolutionFilterRequest::try_parse_request(header, remaining)?)),
                    glx::GET_CONVOLUTION_PARAMETERFV_REQUEST => return Ok(Request::GlxGetConvolutionParameterfv(glx::GetConvolutionParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_CONVOLUTION_PARAMETERIV_REQUEST => return Ok(Request::GlxGetConvolutionParameteriv(glx::GetConvolutionParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_SEPARABLE_FILTER_REQUEST => return Ok(Request::GlxGetSeparableFilter(glx::GetSeparableFilterRequest::try_parse_request(header, remaining)?)),
                    glx::GET_HISTOGRAM_REQUEST => return Ok(Request::GlxGetHistogram(glx::GetHistogramRequest::try_parse_request(header, remaining)?)),
                    glx::GET_HISTOGRAM_PARAMETERFV_REQUEST => return Ok(Request::GlxGetHistogramParameterfv(glx::GetHistogramParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_HISTOGRAM_PARAMETERIV_REQUEST => return Ok(Request::GlxGetHistogramParameteriv(glx::GetHistogramParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MINMAX_REQUEST => return Ok(Request::GlxGetMinmax(glx::GetMinmaxRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MINMAX_PARAMETERFV_REQUEST => return Ok(Request::GlxGetMinmaxParameterfv(glx::GetMinmaxParameterfvRequest::try_parse_request(header, remaining)?)),
                    glx::GET_MINMAX_PARAMETERIV_REQUEST => return Ok(Request::GlxGetMinmaxParameteriv(glx::GetMinmaxParameterivRequest::try_parse_request(header, remaining)?)),
                    glx::GET_COMPRESSED_TEX_IMAGE_ARB_REQUEST => return Ok(Request::GlxGetCompressedTexImageARB(glx::GetCompressedTexImageARBRequest::try_parse_request(header, remaining)?)),
                    glx::DELETE_QUERIES_ARB_REQUEST => return Ok(Request::GlxDeleteQueriesARB(glx::DeleteQueriesARBRequest::try_parse_request(header, remaining)?)),
                    glx::GEN_QUERIES_ARB_REQUEST => return Ok(Request::GlxGenQueriesARB(glx::GenQueriesARBRequest::try_parse_request(header, remaining)?)),
                    glx::IS_QUERY_ARB_REQUEST => return Ok(Request::GlxIsQueryARB(glx::IsQueryARBRequest::try_parse_request(header, remaining)?)),
                    glx::GET_QUERYIV_ARB_REQUEST => return Ok(Request::GlxGetQueryivARB(glx::GetQueryivARBRequest::try_parse_request(header, remaining)?)),
                    glx::GET_QUERY_OBJECTIV_ARB_REQUEST => return Ok(Request::GlxGetQueryObjectivARB(glx::GetQueryObjectivARBRequest::try_parse_request(header, remaining)?)),
                    glx::GET_QUERY_OBJECTUIV_ARB_REQUEST => return Ok(Request::GlxGetQueryObjectuivARB(glx::GetQueryObjectuivARBRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "present")]
            Some((present::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    present::QUERY_VERSION_REQUEST => return Ok(Request::PresentQueryVersion(present::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    present::PIXMAP_REQUEST => return Ok(Request::PresentPixmap(present::PixmapRequest::try_parse_request(header, remaining)?)),
                    present::NOTIFY_MSC_REQUEST => return Ok(Request::PresentNotifyMSC(present::NotifyMSCRequest::try_parse_request(header, remaining)?)),
                    present::SELECT_INPUT_REQUEST => return Ok(Request::PresentSelectInput(present::SelectInputRequest::try_parse_request(header, remaining)?)),
                    present::QUERY_CAPABILITIES_REQUEST => return Ok(Request::PresentQueryCapabilities(present::QueryCapabilitiesRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    randr::QUERY_VERSION_REQUEST => return Ok(Request::RandrQueryVersion(randr::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    randr::SET_SCREEN_CONFIG_REQUEST => return Ok(Request::RandrSetScreenConfig(randr::SetScreenConfigRequest::try_parse_request(header, remaining)?)),
                    randr::SELECT_INPUT_REQUEST => return Ok(Request::RandrSelectInput(randr::SelectInputRequest::try_parse_request(header, remaining)?)),
                    randr::GET_SCREEN_INFO_REQUEST => return Ok(Request::RandrGetScreenInfo(randr::GetScreenInfoRequest::try_parse_request(header, remaining)?)),
                    randr::GET_SCREEN_SIZE_RANGE_REQUEST => return Ok(Request::RandrGetScreenSizeRange(randr::GetScreenSizeRangeRequest::try_parse_request(header, remaining)?)),
                    randr::SET_SCREEN_SIZE_REQUEST => return Ok(Request::RandrSetScreenSize(randr::SetScreenSizeRequest::try_parse_request(header, remaining)?)),
                    randr::GET_SCREEN_RESOURCES_REQUEST => return Ok(Request::RandrGetScreenResources(randr::GetScreenResourcesRequest::try_parse_request(header, remaining)?)),
                    randr::GET_OUTPUT_INFO_REQUEST => return Ok(Request::RandrGetOutputInfo(randr::GetOutputInfoRequest::try_parse_request(header, remaining)?)),
                    randr::LIST_OUTPUT_PROPERTIES_REQUEST => return Ok(Request::RandrListOutputProperties(randr::ListOutputPropertiesRequest::try_parse_request(header, remaining)?)),
                    randr::QUERY_OUTPUT_PROPERTY_REQUEST => return Ok(Request::RandrQueryOutputProperty(randr::QueryOutputPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::CONFIGURE_OUTPUT_PROPERTY_REQUEST => return Ok(Request::RandrConfigureOutputProperty(randr::ConfigureOutputPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::CHANGE_OUTPUT_PROPERTY_REQUEST => return Ok(Request::RandrChangeOutputProperty(randr::ChangeOutputPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::DELETE_OUTPUT_PROPERTY_REQUEST => return Ok(Request::RandrDeleteOutputProperty(randr::DeleteOutputPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::GET_OUTPUT_PROPERTY_REQUEST => return Ok(Request::RandrGetOutputProperty(randr::GetOutputPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::CREATE_MODE_REQUEST => return Ok(Request::RandrCreateMode(randr::CreateModeRequest::try_parse_request(header, remaining)?)),
                    randr::DESTROY_MODE_REQUEST => return Ok(Request::RandrDestroyMode(randr::DestroyModeRequest::try_parse_request(header, remaining)?)),
                    randr::ADD_OUTPUT_MODE_REQUEST => return Ok(Request::RandrAddOutputMode(randr::AddOutputModeRequest::try_parse_request(header, remaining)?)),
                    randr::DELETE_OUTPUT_MODE_REQUEST => return Ok(Request::RandrDeleteOutputMode(randr::DeleteOutputModeRequest::try_parse_request(header, remaining)?)),
                    randr::GET_CRTC_INFO_REQUEST => return Ok(Request::RandrGetCrtcInfo(randr::GetCrtcInfoRequest::try_parse_request(header, remaining)?)),
                    randr::SET_CRTC_CONFIG_REQUEST => return Ok(Request::RandrSetCrtcConfig(randr::SetCrtcConfigRequest::try_parse_request(header, remaining)?)),
                    randr::GET_CRTC_GAMMA_SIZE_REQUEST => return Ok(Request::RandrGetCrtcGammaSize(randr::GetCrtcGammaSizeRequest::try_parse_request(header, remaining)?)),
                    randr::GET_CRTC_GAMMA_REQUEST => return Ok(Request::RandrGetCrtcGamma(randr::GetCrtcGammaRequest::try_parse_request(header, remaining)?)),
                    randr::SET_CRTC_GAMMA_REQUEST => return Ok(Request::RandrSetCrtcGamma(randr::SetCrtcGammaRequest::try_parse_request(header, remaining)?)),
                    randr::GET_SCREEN_RESOURCES_CURRENT_REQUEST => return Ok(Request::RandrGetScreenResourcesCurrent(randr::GetScreenResourcesCurrentRequest::try_parse_request(header, remaining)?)),
                    randr::SET_CRTC_TRANSFORM_REQUEST => return Ok(Request::RandrSetCrtcTransform(randr::SetCrtcTransformRequest::try_parse_request(header, remaining)?)),
                    randr::GET_CRTC_TRANSFORM_REQUEST => return Ok(Request::RandrGetCrtcTransform(randr::GetCrtcTransformRequest::try_parse_request(header, remaining)?)),
                    randr::GET_PANNING_REQUEST => return Ok(Request::RandrGetPanning(randr::GetPanningRequest::try_parse_request(header, remaining)?)),
                    randr::SET_PANNING_REQUEST => return Ok(Request::RandrSetPanning(randr::SetPanningRequest::try_parse_request(header, remaining)?)),
                    randr::SET_OUTPUT_PRIMARY_REQUEST => return Ok(Request::RandrSetOutputPrimary(randr::SetOutputPrimaryRequest::try_parse_request(header, remaining)?)),
                    randr::GET_OUTPUT_PRIMARY_REQUEST => return Ok(Request::RandrGetOutputPrimary(randr::GetOutputPrimaryRequest::try_parse_request(header, remaining)?)),
                    randr::GET_PROVIDERS_REQUEST => return Ok(Request::RandrGetProviders(randr::GetProvidersRequest::try_parse_request(header, remaining)?)),
                    randr::GET_PROVIDER_INFO_REQUEST => return Ok(Request::RandrGetProviderInfo(randr::GetProviderInfoRequest::try_parse_request(header, remaining)?)),
                    randr::SET_PROVIDER_OFFLOAD_SINK_REQUEST => return Ok(Request::RandrSetProviderOffloadSink(randr::SetProviderOffloadSinkRequest::try_parse_request(header, remaining)?)),
                    randr::SET_PROVIDER_OUTPUT_SOURCE_REQUEST => return Ok(Request::RandrSetProviderOutputSource(randr::SetProviderOutputSourceRequest::try_parse_request(header, remaining)?)),
                    randr::LIST_PROVIDER_PROPERTIES_REQUEST => return Ok(Request::RandrListProviderProperties(randr::ListProviderPropertiesRequest::try_parse_request(header, remaining)?)),
                    randr::QUERY_PROVIDER_PROPERTY_REQUEST => return Ok(Request::RandrQueryProviderProperty(randr::QueryProviderPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::CONFIGURE_PROVIDER_PROPERTY_REQUEST => return Ok(Request::RandrConfigureProviderProperty(randr::ConfigureProviderPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::CHANGE_PROVIDER_PROPERTY_REQUEST => return Ok(Request::RandrChangeProviderProperty(randr::ChangeProviderPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::DELETE_PROVIDER_PROPERTY_REQUEST => return Ok(Request::RandrDeleteProviderProperty(randr::DeleteProviderPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::GET_PROVIDER_PROPERTY_REQUEST => return Ok(Request::RandrGetProviderProperty(randr::GetProviderPropertyRequest::try_parse_request(header, remaining)?)),
                    randr::GET_MONITORS_REQUEST => return Ok(Request::RandrGetMonitors(randr::GetMonitorsRequest::try_parse_request(header, remaining)?)),
                    randr::SET_MONITOR_REQUEST => return Ok(Request::RandrSetMonitor(randr::SetMonitorRequest::try_parse_request(header, remaining)?)),
                    randr::DELETE_MONITOR_REQUEST => return Ok(Request::RandrDeleteMonitor(randr::DeleteMonitorRequest::try_parse_request(header, remaining)?)),
                    randr::CREATE_LEASE_REQUEST => return Ok(Request::RandrCreateLease(randr::CreateLeaseRequest::try_parse_request(header, remaining)?)),
                    randr::FREE_LEASE_REQUEST => return Ok(Request::RandrFreeLease(randr::FreeLeaseRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "record")]
            Some((record::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    record::QUERY_VERSION_REQUEST => return Ok(Request::RecordQueryVersion(record::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    record::CREATE_CONTEXT_REQUEST => return Ok(Request::RecordCreateContext(record::CreateContextRequest::try_parse_request(header, remaining)?)),
                    record::REGISTER_CLIENTS_REQUEST => return Ok(Request::RecordRegisterClients(record::RegisterClientsRequest::try_parse_request(header, remaining)?)),
                    record::UNREGISTER_CLIENTS_REQUEST => return Ok(Request::RecordUnregisterClients(record::UnregisterClientsRequest::try_parse_request(header, remaining)?)),
                    record::GET_CONTEXT_REQUEST => return Ok(Request::RecordGetContext(record::GetContextRequest::try_parse_request(header, remaining)?)),
                    record::ENABLE_CONTEXT_REQUEST => return Ok(Request::RecordEnableContext(record::EnableContextRequest::try_parse_request(header, remaining)?)),
                    record::DISABLE_CONTEXT_REQUEST => return Ok(Request::RecordDisableContext(record::DisableContextRequest::try_parse_request(header, remaining)?)),
                    record::FREE_CONTEXT_REQUEST => return Ok(Request::RecordFreeContext(record::FreeContextRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "render")]
            Some((render::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    render::QUERY_VERSION_REQUEST => return Ok(Request::RenderQueryVersion(render::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    render::QUERY_PICT_FORMATS_REQUEST => return Ok(Request::RenderQueryPictFormats(render::QueryPictFormatsRequest::try_parse_request(header, remaining)?)),
                    render::QUERY_PICT_INDEX_VALUES_REQUEST => return Ok(Request::RenderQueryPictIndexValues(render::QueryPictIndexValuesRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_PICTURE_REQUEST => return Ok(Request::RenderCreatePicture(render::CreatePictureRequest::try_parse_request(header, remaining)?)),
                    render::CHANGE_PICTURE_REQUEST => return Ok(Request::RenderChangePicture(render::ChangePictureRequest::try_parse_request(header, remaining)?)),
                    render::SET_PICTURE_CLIP_RECTANGLES_REQUEST => return Ok(Request::RenderSetPictureClipRectangles(render::SetPictureClipRectanglesRequest::try_parse_request(header, remaining)?)),
                    render::FREE_PICTURE_REQUEST => return Ok(Request::RenderFreePicture(render::FreePictureRequest::try_parse_request(header, remaining)?)),
                    render::COMPOSITE_REQUEST => return Ok(Request::RenderComposite(render::CompositeRequest::try_parse_request(header, remaining)?)),
                    render::TRAPEZOIDS_REQUEST => return Ok(Request::RenderTrapezoids(render::TrapezoidsRequest::try_parse_request(header, remaining)?)),
                    render::TRIANGLES_REQUEST => return Ok(Request::RenderTriangles(render::TrianglesRequest::try_parse_request(header, remaining)?)),
                    render::TRI_STRIP_REQUEST => return Ok(Request::RenderTriStrip(render::TriStripRequest::try_parse_request(header, remaining)?)),
                    render::TRI_FAN_REQUEST => return Ok(Request::RenderTriFan(render::TriFanRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_GLYPH_SET_REQUEST => return Ok(Request::RenderCreateGlyphSet(render::CreateGlyphSetRequest::try_parse_request(header, remaining)?)),
                    render::REFERENCE_GLYPH_SET_REQUEST => return Ok(Request::RenderReferenceGlyphSet(render::ReferenceGlyphSetRequest::try_parse_request(header, remaining)?)),
                    render::FREE_GLYPH_SET_REQUEST => return Ok(Request::RenderFreeGlyphSet(render::FreeGlyphSetRequest::try_parse_request(header, remaining)?)),
                    render::ADD_GLYPHS_REQUEST => return Ok(Request::RenderAddGlyphs(render::AddGlyphsRequest::try_parse_request(header, remaining)?)),
                    render::FREE_GLYPHS_REQUEST => return Ok(Request::RenderFreeGlyphs(render::FreeGlyphsRequest::try_parse_request(header, remaining)?)),
                    render::COMPOSITE_GLYPHS8_REQUEST => return Ok(Request::RenderCompositeGlyphs8(render::CompositeGlyphs8Request::try_parse_request(header, remaining)?)),
                    render::COMPOSITE_GLYPHS16_REQUEST => return Ok(Request::RenderCompositeGlyphs16(render::CompositeGlyphs16Request::try_parse_request(header, remaining)?)),
                    render::COMPOSITE_GLYPHS32_REQUEST => return Ok(Request::RenderCompositeGlyphs32(render::CompositeGlyphs32Request::try_parse_request(header, remaining)?)),
                    render::FILL_RECTANGLES_REQUEST => return Ok(Request::RenderFillRectangles(render::FillRectanglesRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_CURSOR_REQUEST => return Ok(Request::RenderCreateCursor(render::CreateCursorRequest::try_parse_request(header, remaining)?)),
                    render::SET_PICTURE_TRANSFORM_REQUEST => return Ok(Request::RenderSetPictureTransform(render::SetPictureTransformRequest::try_parse_request(header, remaining)?)),
                    render::QUERY_FILTERS_REQUEST => return Ok(Request::RenderQueryFilters(render::QueryFiltersRequest::try_parse_request(header, remaining)?)),
                    render::SET_PICTURE_FILTER_REQUEST => return Ok(Request::RenderSetPictureFilter(render::SetPictureFilterRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_ANIM_CURSOR_REQUEST => return Ok(Request::RenderCreateAnimCursor(render::CreateAnimCursorRequest::try_parse_request(header, remaining)?)),
                    render::ADD_TRAPS_REQUEST => return Ok(Request::RenderAddTraps(render::AddTrapsRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_SOLID_FILL_REQUEST => return Ok(Request::RenderCreateSolidFill(render::CreateSolidFillRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_LINEAR_GRADIENT_REQUEST => return Ok(Request::RenderCreateLinearGradient(render::CreateLinearGradientRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_RADIAL_GRADIENT_REQUEST => return Ok(Request::RenderCreateRadialGradient(render::CreateRadialGradientRequest::try_parse_request(header, remaining)?)),
                    render::CREATE_CONICAL_GRADIENT_REQUEST => return Ok(Request::RenderCreateConicalGradient(render::CreateConicalGradientRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "res")]
            Some((res::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    res::QUERY_VERSION_REQUEST => return Ok(Request::ResQueryVersion(res::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    res::QUERY_CLIENTS_REQUEST => return Ok(Request::ResQueryClients(res::QueryClientsRequest::try_parse_request(header, remaining)?)),
                    res::QUERY_CLIENT_RESOURCES_REQUEST => return Ok(Request::ResQueryClientResources(res::QueryClientResourcesRequest::try_parse_request(header, remaining)?)),
                    res::QUERY_CLIENT_PIXMAP_BYTES_REQUEST => return Ok(Request::ResQueryClientPixmapBytes(res::QueryClientPixmapBytesRequest::try_parse_request(header, remaining)?)),
                    res::QUERY_CLIENT_IDS_REQUEST => return Ok(Request::ResQueryClientIds(res::QueryClientIdsRequest::try_parse_request(header, remaining)?)),
                    res::QUERY_RESOURCE_BYTES_REQUEST => return Ok(Request::ResQueryResourceBytes(res::QueryResourceBytesRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "screensaver")]
            Some((screensaver::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    screensaver::QUERY_VERSION_REQUEST => return Ok(Request::ScreensaverQueryVersion(screensaver::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    screensaver::QUERY_INFO_REQUEST => return Ok(Request::ScreensaverQueryInfo(screensaver::QueryInfoRequest::try_parse_request(header, remaining)?)),
                    screensaver::SELECT_INPUT_REQUEST => return Ok(Request::ScreensaverSelectInput(screensaver::SelectInputRequest::try_parse_request(header, remaining)?)),
                    screensaver::SET_ATTRIBUTES_REQUEST => return Ok(Request::ScreensaverSetAttributes(screensaver::SetAttributesRequest::try_parse_request(header, remaining)?)),
                    screensaver::UNSET_ATTRIBUTES_REQUEST => return Ok(Request::ScreensaverUnsetAttributes(screensaver::UnsetAttributesRequest::try_parse_request(header, remaining)?)),
                    screensaver::SUSPEND_REQUEST => return Ok(Request::ScreensaverSuspend(screensaver::SuspendRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "shape")]
            Some((shape::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    shape::QUERY_VERSION_REQUEST => return Ok(Request::ShapeQueryVersion(shape::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    shape::RECTANGLES_REQUEST => return Ok(Request::ShapeRectangles(shape::RectanglesRequest::try_parse_request(header, remaining)?)),
                    shape::MASK_REQUEST => return Ok(Request::ShapeMask(shape::MaskRequest::try_parse_request(header, remaining)?)),
                    shape::COMBINE_REQUEST => return Ok(Request::ShapeCombine(shape::CombineRequest::try_parse_request(header, remaining)?)),
                    shape::OFFSET_REQUEST => return Ok(Request::ShapeOffset(shape::OffsetRequest::try_parse_request(header, remaining)?)),
                    shape::QUERY_EXTENTS_REQUEST => return Ok(Request::ShapeQueryExtents(shape::QueryExtentsRequest::try_parse_request(header, remaining)?)),
                    shape::SELECT_INPUT_REQUEST => return Ok(Request::ShapeSelectInput(shape::SelectInputRequest::try_parse_request(header, remaining)?)),
                    shape::INPUT_SELECTED_REQUEST => return Ok(Request::ShapeInputSelected(shape::InputSelectedRequest::try_parse_request(header, remaining)?)),
                    shape::GET_RECTANGLES_REQUEST => return Ok(Request::ShapeGetRectangles(shape::GetRectanglesRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    shm::QUERY_VERSION_REQUEST => return Ok(Request::ShmQueryVersion(shm::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    shm::ATTACH_REQUEST => return Ok(Request::ShmAttach(shm::AttachRequest::try_parse_request(header, remaining)?)),
                    shm::DETACH_REQUEST => return Ok(Request::ShmDetach(shm::DetachRequest::try_parse_request(header, remaining)?)),
                    shm::PUT_IMAGE_REQUEST => return Ok(Request::ShmPutImage(shm::PutImageRequest::try_parse_request(header, remaining)?)),
                    shm::GET_IMAGE_REQUEST => return Ok(Request::ShmGetImage(shm::GetImageRequest::try_parse_request(header, remaining)?)),
                    shm::CREATE_PIXMAP_REQUEST => return Ok(Request::ShmCreatePixmap(shm::CreatePixmapRequest::try_parse_request(header, remaining)?)),
                    shm::ATTACH_FD_REQUEST => return Ok(Request::ShmAttachFd(shm::AttachFdRequest::try_parse_request_fd(header, remaining, fds)?)),
                    shm::CREATE_SEGMENT_REQUEST => return Ok(Request::ShmCreateSegment(shm::CreateSegmentRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    sync::INITIALIZE_REQUEST => return Ok(Request::SyncInitialize(sync::InitializeRequest::try_parse_request(header, remaining)?)),
                    sync::LIST_SYSTEM_COUNTERS_REQUEST => return Ok(Request::SyncListSystemCounters(sync::ListSystemCountersRequest::try_parse_request(header, remaining)?)),
                    sync::CREATE_COUNTER_REQUEST => return Ok(Request::SyncCreateCounter(sync::CreateCounterRequest::try_parse_request(header, remaining)?)),
                    sync::DESTROY_COUNTER_REQUEST => return Ok(Request::SyncDestroyCounter(sync::DestroyCounterRequest::try_parse_request(header, remaining)?)),
                    sync::QUERY_COUNTER_REQUEST => return Ok(Request::SyncQueryCounter(sync::QueryCounterRequest::try_parse_request(header, remaining)?)),
                    sync::AWAIT_REQUEST => return Ok(Request::SyncAwait(sync::AwaitRequest::try_parse_request(header, remaining)?)),
                    sync::CHANGE_COUNTER_REQUEST => return Ok(Request::SyncChangeCounter(sync::ChangeCounterRequest::try_parse_request(header, remaining)?)),
                    sync::SET_COUNTER_REQUEST => return Ok(Request::SyncSetCounter(sync::SetCounterRequest::try_parse_request(header, remaining)?)),
                    sync::CREATE_ALARM_REQUEST => return Ok(Request::SyncCreateAlarm(sync::CreateAlarmRequest::try_parse_request(header, remaining)?)),
                    sync::CHANGE_ALARM_REQUEST => return Ok(Request::SyncChangeAlarm(sync::ChangeAlarmRequest::try_parse_request(header, remaining)?)),
                    sync::DESTROY_ALARM_REQUEST => return Ok(Request::SyncDestroyAlarm(sync::DestroyAlarmRequest::try_parse_request(header, remaining)?)),
                    sync::QUERY_ALARM_REQUEST => return Ok(Request::SyncQueryAlarm(sync::QueryAlarmRequest::try_parse_request(header, remaining)?)),
                    sync::SET_PRIORITY_REQUEST => return Ok(Request::SyncSetPriority(sync::SetPriorityRequest::try_parse_request(header, remaining)?)),
                    sync::GET_PRIORITY_REQUEST => return Ok(Request::SyncGetPriority(sync::GetPriorityRequest::try_parse_request(header, remaining)?)),
                    sync::CREATE_FENCE_REQUEST => return Ok(Request::SyncCreateFence(sync::CreateFenceRequest::try_parse_request(header, remaining)?)),
                    sync::TRIGGER_FENCE_REQUEST => return Ok(Request::SyncTriggerFence(sync::TriggerFenceRequest::try_parse_request(header, remaining)?)),
                    sync::RESET_FENCE_REQUEST => return Ok(Request::SyncResetFence(sync::ResetFenceRequest::try_parse_request(header, remaining)?)),
                    sync::DESTROY_FENCE_REQUEST => return Ok(Request::SyncDestroyFence(sync::DestroyFenceRequest::try_parse_request(header, remaining)?)),
                    sync::QUERY_FENCE_REQUEST => return Ok(Request::SyncQueryFence(sync::QueryFenceRequest::try_parse_request(header, remaining)?)),
                    sync::AWAIT_FENCE_REQUEST => return Ok(Request::SyncAwaitFence(sync::AwaitFenceRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            Some((xc_misc::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xc_misc::GET_VERSION_REQUEST => return Ok(Request::XcMiscGetVersion(xc_misc::GetVersionRequest::try_parse_request(header, remaining)?)),
                    xc_misc::GET_XID_RANGE_REQUEST => return Ok(Request::XcMiscGetXIDRange(xc_misc::GetXIDRangeRequest::try_parse_request(header, remaining)?)),
                    xc_misc::GET_XID_LIST_REQUEST => return Ok(Request::XcMiscGetXIDList(xc_misc::GetXIDListRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xevie")]
            Some((xevie::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xevie::QUERY_VERSION_REQUEST => return Ok(Request::XevieQueryVersion(xevie::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xevie::START_REQUEST => return Ok(Request::XevieStart(xevie::StartRequest::try_parse_request(header, remaining)?)),
                    xevie::END_REQUEST => return Ok(Request::XevieEnd(xevie::EndRequest::try_parse_request(header, remaining)?)),
                    xevie::SEND_REQUEST => return Ok(Request::XevieSend(xevie::SendRequest::try_parse_request(header, remaining)?)),
                    xevie::SELECT_INPUT_REQUEST => return Ok(Request::XevieSelectInput(xevie::SelectInputRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xf86dri")]
            Some((xf86dri::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xf86dri::QUERY_VERSION_REQUEST => return Ok(Request::Xf86driQueryVersion(xf86dri::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xf86dri::QUERY_DIRECT_RENDERING_CAPABLE_REQUEST => return Ok(Request::Xf86driQueryDirectRenderingCapable(xf86dri::QueryDirectRenderingCapableRequest::try_parse_request(header, remaining)?)),
                    xf86dri::OPEN_CONNECTION_REQUEST => return Ok(Request::Xf86driOpenConnection(xf86dri::OpenConnectionRequest::try_parse_request(header, remaining)?)),
                    xf86dri::CLOSE_CONNECTION_REQUEST => return Ok(Request::Xf86driCloseConnection(xf86dri::CloseConnectionRequest::try_parse_request(header, remaining)?)),
                    xf86dri::GET_CLIENT_DRIVER_NAME_REQUEST => return Ok(Request::Xf86driGetClientDriverName(xf86dri::GetClientDriverNameRequest::try_parse_request(header, remaining)?)),
                    xf86dri::CREATE_CONTEXT_REQUEST => return Ok(Request::Xf86driCreateContext(xf86dri::CreateContextRequest::try_parse_request(header, remaining)?)),
                    xf86dri::DESTROY_CONTEXT_REQUEST => return Ok(Request::Xf86driDestroyContext(xf86dri::DestroyContextRequest::try_parse_request(header, remaining)?)),
                    xf86dri::CREATE_DRAWABLE_REQUEST => return Ok(Request::Xf86driCreateDrawable(xf86dri::CreateDrawableRequest::try_parse_request(header, remaining)?)),
                    xf86dri::DESTROY_DRAWABLE_REQUEST => return Ok(Request::Xf86driDestroyDrawable(xf86dri::DestroyDrawableRequest::try_parse_request(header, remaining)?)),
                    xf86dri::GET_DRAWABLE_INFO_REQUEST => return Ok(Request::Xf86driGetDrawableInfo(xf86dri::GetDrawableInfoRequest::try_parse_request(header, remaining)?)),
                    xf86dri::GET_DEVICE_INFO_REQUEST => return Ok(Request::Xf86driGetDeviceInfo(xf86dri::GetDeviceInfoRequest::try_parse_request(header, remaining)?)),
                    xf86dri::AUTH_CONNECTION_REQUEST => return Ok(Request::Xf86driAuthConnection(xf86dri::AuthConnectionRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xf86vidmode")]
            Some((xf86vidmode::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xf86vidmode::QUERY_VERSION_REQUEST => return Ok(Request::Xf86vidmodeQueryVersion(xf86vidmode::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_MODE_LINE_REQUEST => return Ok(Request::Xf86vidmodeGetModeLine(xf86vidmode::GetModeLineRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::MOD_MODE_LINE_REQUEST => return Ok(Request::Xf86vidmodeModModeLine(xf86vidmode::ModModeLineRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SWITCH_MODE_REQUEST => return Ok(Request::Xf86vidmodeSwitchMode(xf86vidmode::SwitchModeRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_MONITOR_REQUEST => return Ok(Request::Xf86vidmodeGetMonitor(xf86vidmode::GetMonitorRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::LOCK_MODE_SWITCH_REQUEST => return Ok(Request::Xf86vidmodeLockModeSwitch(xf86vidmode::LockModeSwitchRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_ALL_MODE_LINES_REQUEST => return Ok(Request::Xf86vidmodeGetAllModeLines(xf86vidmode::GetAllModeLinesRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::ADD_MODE_LINE_REQUEST => return Ok(Request::Xf86vidmodeAddModeLine(xf86vidmode::AddModeLineRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::DELETE_MODE_LINE_REQUEST => return Ok(Request::Xf86vidmodeDeleteModeLine(xf86vidmode::DeleteModeLineRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::VALIDATE_MODE_LINE_REQUEST => return Ok(Request::Xf86vidmodeValidateModeLine(xf86vidmode::ValidateModeLineRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SWITCH_TO_MODE_REQUEST => return Ok(Request::Xf86vidmodeSwitchToMode(xf86vidmode::SwitchToModeRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_VIEW_PORT_REQUEST => return Ok(Request::Xf86vidmodeGetViewPort(xf86vidmode::GetViewPortRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SET_VIEW_PORT_REQUEST => return Ok(Request::Xf86vidmodeSetViewPort(xf86vidmode::SetViewPortRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_DOT_CLOCKS_REQUEST => return Ok(Request::Xf86vidmodeGetDotClocks(xf86vidmode::GetDotClocksRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SET_CLIENT_VERSION_REQUEST => return Ok(Request::Xf86vidmodeSetClientVersion(xf86vidmode::SetClientVersionRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SET_GAMMA_REQUEST => return Ok(Request::Xf86vidmodeSetGamma(xf86vidmode::SetGammaRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_GAMMA_REQUEST => return Ok(Request::Xf86vidmodeGetGamma(xf86vidmode::GetGammaRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_GAMMA_RAMP_REQUEST => return Ok(Request::Xf86vidmodeGetGammaRamp(xf86vidmode::GetGammaRampRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::SET_GAMMA_RAMP_REQUEST => return Ok(Request::Xf86vidmodeSetGammaRamp(xf86vidmode::SetGammaRampRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_GAMMA_RAMP_SIZE_REQUEST => return Ok(Request::Xf86vidmodeGetGammaRampSize(xf86vidmode::GetGammaRampSizeRequest::try_parse_request(header, remaining)?)),
                    xf86vidmode::GET_PERMISSIONS_REQUEST => return Ok(Request::Xf86vidmodeGetPermissions(xf86vidmode::GetPermissionsRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xfixes::QUERY_VERSION_REQUEST => return Ok(Request::XfixesQueryVersion(xfixes::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xfixes::CHANGE_SAVE_SET_REQUEST => return Ok(Request::XfixesChangeSaveSet(xfixes::ChangeSaveSetRequest::try_parse_request(header, remaining)?)),
                    xfixes::SELECT_SELECTION_INPUT_REQUEST => return Ok(Request::XfixesSelectSelectionInput(xfixes::SelectSelectionInputRequest::try_parse_request(header, remaining)?)),
                    xfixes::SELECT_CURSOR_INPUT_REQUEST => return Ok(Request::XfixesSelectCursorInput(xfixes::SelectCursorInputRequest::try_parse_request(header, remaining)?)),
                    xfixes::GET_CURSOR_IMAGE_REQUEST => return Ok(Request::XfixesGetCursorImage(xfixes::GetCursorImageRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_REGION_REQUEST => return Ok(Request::XfixesCreateRegion(xfixes::CreateRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_REGION_FROM_BITMAP_REQUEST => return Ok(Request::XfixesCreateRegionFromBitmap(xfixes::CreateRegionFromBitmapRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_REGION_FROM_WINDOW_REQUEST => return Ok(Request::XfixesCreateRegionFromWindow(xfixes::CreateRegionFromWindowRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_REGION_FROM_GC_REQUEST => return Ok(Request::XfixesCreateRegionFromGC(xfixes::CreateRegionFromGCRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_REGION_FROM_PICTURE_REQUEST => return Ok(Request::XfixesCreateRegionFromPicture(xfixes::CreateRegionFromPictureRequest::try_parse_request(header, remaining)?)),
                    xfixes::DESTROY_REGION_REQUEST => return Ok(Request::XfixesDestroyRegion(xfixes::DestroyRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SET_REGION_REQUEST => return Ok(Request::XfixesSetRegion(xfixes::SetRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::COPY_REGION_REQUEST => return Ok(Request::XfixesCopyRegion(xfixes::CopyRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::UNION_REGION_REQUEST => return Ok(Request::XfixesUnionRegion(xfixes::UnionRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::INTERSECT_REGION_REQUEST => return Ok(Request::XfixesIntersectRegion(xfixes::IntersectRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SUBTRACT_REGION_REQUEST => return Ok(Request::XfixesSubtractRegion(xfixes::SubtractRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::INVERT_REGION_REQUEST => return Ok(Request::XfixesInvertRegion(xfixes::InvertRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::TRANSLATE_REGION_REQUEST => return Ok(Request::XfixesTranslateRegion(xfixes::TranslateRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::REGION_EXTENTS_REQUEST => return Ok(Request::XfixesRegionExtents(xfixes::RegionExtentsRequest::try_parse_request(header, remaining)?)),
                    xfixes::FETCH_REGION_REQUEST => return Ok(Request::XfixesFetchRegion(xfixes::FetchRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SET_GC_CLIP_REGION_REQUEST => return Ok(Request::XfixesSetGCClipRegion(xfixes::SetGCClipRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SET_WINDOW_SHAPE_REGION_REQUEST => return Ok(Request::XfixesSetWindowShapeRegion(xfixes::SetWindowShapeRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SET_PICTURE_CLIP_REGION_REQUEST => return Ok(Request::XfixesSetPictureClipRegion(xfixes::SetPictureClipRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::SET_CURSOR_NAME_REQUEST => return Ok(Request::XfixesSetCursorName(xfixes::SetCursorNameRequest::try_parse_request(header, remaining)?)),
                    xfixes::GET_CURSOR_NAME_REQUEST => return Ok(Request::XfixesGetCursorName(xfixes::GetCursorNameRequest::try_parse_request(header, remaining)?)),
                    xfixes::GET_CURSOR_IMAGE_AND_NAME_REQUEST => return Ok(Request::XfixesGetCursorImageAndName(xfixes::GetCursorImageAndNameRequest::try_parse_request(header, remaining)?)),
                    xfixes::CHANGE_CURSOR_REQUEST => return Ok(Request::XfixesChangeCursor(xfixes::ChangeCursorRequest::try_parse_request(header, remaining)?)),
                    xfixes::CHANGE_CURSOR_BY_NAME_REQUEST => return Ok(Request::XfixesChangeCursorByName(xfixes::ChangeCursorByNameRequest::try_parse_request(header, remaining)?)),
                    xfixes::EXPAND_REGION_REQUEST => return Ok(Request::XfixesExpandRegion(xfixes::ExpandRegionRequest::try_parse_request(header, remaining)?)),
                    xfixes::HIDE_CURSOR_REQUEST => return Ok(Request::XfixesHideCursor(xfixes::HideCursorRequest::try_parse_request(header, remaining)?)),
                    xfixes::SHOW_CURSOR_REQUEST => return Ok(Request::XfixesShowCursor(xfixes::ShowCursorRequest::try_parse_request(header, remaining)?)),
                    xfixes::CREATE_POINTER_BARRIER_REQUEST => return Ok(Request::XfixesCreatePointerBarrier(xfixes::CreatePointerBarrierRequest::try_parse_request(header, remaining)?)),
                    xfixes::DELETE_POINTER_BARRIER_REQUEST => return Ok(Request::XfixesDeletePointerBarrier(xfixes::DeletePointerBarrierRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xinerama")]
            Some((xinerama::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xinerama::QUERY_VERSION_REQUEST => return Ok(Request::XineramaQueryVersion(xinerama::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xinerama::GET_STATE_REQUEST => return Ok(Request::XineramaGetState(xinerama::GetStateRequest::try_parse_request(header, remaining)?)),
                    xinerama::GET_SCREEN_COUNT_REQUEST => return Ok(Request::XineramaGetScreenCount(xinerama::GetScreenCountRequest::try_parse_request(header, remaining)?)),
                    xinerama::GET_SCREEN_SIZE_REQUEST => return Ok(Request::XineramaGetScreenSize(xinerama::GetScreenSizeRequest::try_parse_request(header, remaining)?)),
                    xinerama::IS_ACTIVE_REQUEST => return Ok(Request::XineramaIsActive(xinerama::IsActiveRequest::try_parse_request(header, remaining)?)),
                    xinerama::QUERY_SCREENS_REQUEST => return Ok(Request::XineramaQueryScreens(xinerama::QueryScreensRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xinput::GET_EXTENSION_VERSION_REQUEST => return Ok(Request::XinputGetExtensionVersion(xinput::GetExtensionVersionRequest::try_parse_request(header, remaining)?)),
                    xinput::LIST_INPUT_DEVICES_REQUEST => return Ok(Request::XinputListInputDevices(xinput::ListInputDevicesRequest::try_parse_request(header, remaining)?)),
                    xinput::OPEN_DEVICE_REQUEST => return Ok(Request::XinputOpenDevice(xinput::OpenDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::CLOSE_DEVICE_REQUEST => return Ok(Request::XinputCloseDevice(xinput::CloseDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::SET_DEVICE_MODE_REQUEST => return Ok(Request::XinputSetDeviceMode(xinput::SetDeviceModeRequest::try_parse_request(header, remaining)?)),
                    xinput::SELECT_EXTENSION_EVENT_REQUEST => return Ok(Request::XinputSelectExtensionEvent(xinput::SelectExtensionEventRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_SELECTED_EXTENSION_EVENTS_REQUEST => return Ok(Request::XinputGetSelectedExtensionEvents(xinput::GetSelectedExtensionEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_DEVICE_DONT_PROPAGATE_LIST_REQUEST => return Ok(Request::XinputChangeDeviceDontPropagateList(xinput::ChangeDeviceDontPropagateListRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_DONT_PROPAGATE_LIST_REQUEST => return Ok(Request::XinputGetDeviceDontPropagateList(xinput::GetDeviceDontPropagateListRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_MOTION_EVENTS_REQUEST => return Ok(Request::XinputGetDeviceMotionEvents(xinput::GetDeviceMotionEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_KEYBOARD_DEVICE_REQUEST => return Ok(Request::XinputChangeKeyboardDevice(xinput::ChangeKeyboardDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_POINTER_DEVICE_REQUEST => return Ok(Request::XinputChangePointerDevice(xinput::ChangePointerDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::GRAB_DEVICE_REQUEST => return Ok(Request::XinputGrabDevice(xinput::GrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::UNGRAB_DEVICE_REQUEST => return Ok(Request::XinputUngrabDevice(xinput::UngrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::GRAB_DEVICE_KEY_REQUEST => return Ok(Request::XinputGrabDeviceKey(xinput::GrabDeviceKeyRequest::try_parse_request(header, remaining)?)),
                    xinput::UNGRAB_DEVICE_KEY_REQUEST => return Ok(Request::XinputUngrabDeviceKey(xinput::UngrabDeviceKeyRequest::try_parse_request(header, remaining)?)),
                    xinput::GRAB_DEVICE_BUTTON_REQUEST => return Ok(Request::XinputGrabDeviceButton(xinput::GrabDeviceButtonRequest::try_parse_request(header, remaining)?)),
                    xinput::UNGRAB_DEVICE_BUTTON_REQUEST => return Ok(Request::XinputUngrabDeviceButton(xinput::UngrabDeviceButtonRequest::try_parse_request(header, remaining)?)),
                    xinput::ALLOW_DEVICE_EVENTS_REQUEST => return Ok(Request::XinputAllowDeviceEvents(xinput::AllowDeviceEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_FOCUS_REQUEST => return Ok(Request::XinputGetDeviceFocus(xinput::GetDeviceFocusRequest::try_parse_request(header, remaining)?)),
                    xinput::SET_DEVICE_FOCUS_REQUEST => return Ok(Request::XinputSetDeviceFocus(xinput::SetDeviceFocusRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_FEEDBACK_CONTROL_REQUEST => return Ok(Request::XinputGetFeedbackControl(xinput::GetFeedbackControlRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_FEEDBACK_CONTROL_REQUEST => return Ok(Request::XinputChangeFeedbackControl(xinput::ChangeFeedbackControlRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_KEY_MAPPING_REQUEST => return Ok(Request::XinputGetDeviceKeyMapping(xinput::GetDeviceKeyMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_DEVICE_KEY_MAPPING_REQUEST => return Ok(Request::XinputChangeDeviceKeyMapping(xinput::ChangeDeviceKeyMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_MODIFIER_MAPPING_REQUEST => return Ok(Request::XinputGetDeviceModifierMapping(xinput::GetDeviceModifierMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::SET_DEVICE_MODIFIER_MAPPING_REQUEST => return Ok(Request::XinputSetDeviceModifierMapping(xinput::SetDeviceModifierMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_BUTTON_MAPPING_REQUEST => return Ok(Request::XinputGetDeviceButtonMapping(xinput::GetDeviceButtonMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::SET_DEVICE_BUTTON_MAPPING_REQUEST => return Ok(Request::XinputSetDeviceButtonMapping(xinput::SetDeviceButtonMappingRequest::try_parse_request(header, remaining)?)),
                    xinput::QUERY_DEVICE_STATE_REQUEST => return Ok(Request::XinputQueryDeviceState(xinput::QueryDeviceStateRequest::try_parse_request(header, remaining)?)),
                    xinput::DEVICE_BELL_REQUEST => return Ok(Request::XinputDeviceBell(xinput::DeviceBellRequest::try_parse_request(header, remaining)?)),
                    xinput::SET_DEVICE_VALUATORS_REQUEST => return Ok(Request::XinputSetDeviceValuators(xinput::SetDeviceValuatorsRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_CONTROL_REQUEST => return Ok(Request::XinputGetDeviceControl(xinput::GetDeviceControlRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_DEVICE_CONTROL_REQUEST => return Ok(Request::XinputChangeDeviceControl(xinput::ChangeDeviceControlRequest::try_parse_request(header, remaining)?)),
                    xinput::LIST_DEVICE_PROPERTIES_REQUEST => return Ok(Request::XinputListDeviceProperties(xinput::ListDevicePropertiesRequest::try_parse_request(header, remaining)?)),
                    xinput::CHANGE_DEVICE_PROPERTY_REQUEST => return Ok(Request::XinputChangeDeviceProperty(xinput::ChangeDevicePropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::DELETE_DEVICE_PROPERTY_REQUEST => return Ok(Request::XinputDeleteDeviceProperty(xinput::DeleteDevicePropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::GET_DEVICE_PROPERTY_REQUEST => return Ok(Request::XinputGetDeviceProperty(xinput::GetDevicePropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_QUERY_POINTER_REQUEST => return Ok(Request::XinputXIQueryPointer(xinput::XIQueryPointerRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_WARP_POINTER_REQUEST => return Ok(Request::XinputXIWarpPointer(xinput::XIWarpPointerRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_CHANGE_CURSOR_REQUEST => return Ok(Request::XinputXIChangeCursor(xinput::XIChangeCursorRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_CHANGE_HIERARCHY_REQUEST => return Ok(Request::XinputXIChangeHierarchy(xinput::XIChangeHierarchyRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_SET_CLIENT_POINTER_REQUEST => return Ok(Request::XinputXISetClientPointer(xinput::XISetClientPointerRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_GET_CLIENT_POINTER_REQUEST => return Ok(Request::XinputXIGetClientPointer(xinput::XIGetClientPointerRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_SELECT_EVENTS_REQUEST => return Ok(Request::XinputXISelectEvents(xinput::XISelectEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_QUERY_VERSION_REQUEST => return Ok(Request::XinputXIQueryVersion(xinput::XIQueryVersionRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_QUERY_DEVICE_REQUEST => return Ok(Request::XinputXIQueryDevice(xinput::XIQueryDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_SET_FOCUS_REQUEST => return Ok(Request::XinputXISetFocus(xinput::XISetFocusRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_GET_FOCUS_REQUEST => return Ok(Request::XinputXIGetFocus(xinput::XIGetFocusRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_GRAB_DEVICE_REQUEST => return Ok(Request::XinputXIGrabDevice(xinput::XIGrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_UNGRAB_DEVICE_REQUEST => return Ok(Request::XinputXIUngrabDevice(xinput::XIUngrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_ALLOW_EVENTS_REQUEST => return Ok(Request::XinputXIAllowEvents(xinput::XIAllowEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_PASSIVE_GRAB_DEVICE_REQUEST => return Ok(Request::XinputXIPassiveGrabDevice(xinput::XIPassiveGrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_PASSIVE_UNGRAB_DEVICE_REQUEST => return Ok(Request::XinputXIPassiveUngrabDevice(xinput::XIPassiveUngrabDeviceRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_LIST_PROPERTIES_REQUEST => return Ok(Request::XinputXIListProperties(xinput::XIListPropertiesRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_CHANGE_PROPERTY_REQUEST => return Ok(Request::XinputXIChangeProperty(xinput::XIChangePropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_DELETE_PROPERTY_REQUEST => return Ok(Request::XinputXIDeleteProperty(xinput::XIDeletePropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_GET_PROPERTY_REQUEST => return Ok(Request::XinputXIGetProperty(xinput::XIGetPropertyRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_GET_SELECTED_EVENTS_REQUEST => return Ok(Request::XinputXIGetSelectedEvents(xinput::XIGetSelectedEventsRequest::try_parse_request(header, remaining)?)),
                    xinput::XI_BARRIER_RELEASE_POINTER_REQUEST => return Ok(Request::XinputXIBarrierReleasePointer(xinput::XIBarrierReleasePointerRequest::try_parse_request(header, remaining)?)),
                    xinput::SEND_EXTENSION_EVENT_REQUEST => return Ok(Request::XinputSendExtensionEvent(xinput::SendExtensionEventRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xkb::USE_EXTENSION_REQUEST => return Ok(Request::XkbUseExtension(xkb::UseExtensionRequest::try_parse_request(header, remaining)?)),
                    xkb::SELECT_EVENTS_REQUEST => return Ok(Request::XkbSelectEvents(xkb::SelectEventsRequest::try_parse_request(header, remaining)?)),
                    xkb::BELL_REQUEST => return Ok(Request::XkbBell(xkb::BellRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_STATE_REQUEST => return Ok(Request::XkbGetState(xkb::GetStateRequest::try_parse_request(header, remaining)?)),
                    xkb::LATCH_LOCK_STATE_REQUEST => return Ok(Request::XkbLatchLockState(xkb::LatchLockStateRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_CONTROLS_REQUEST => return Ok(Request::XkbGetControls(xkb::GetControlsRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_CONTROLS_REQUEST => return Ok(Request::XkbSetControls(xkb::SetControlsRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_MAP_REQUEST => return Ok(Request::XkbGetMap(xkb::GetMapRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_MAP_REQUEST => return Ok(Request::XkbSetMap(xkb::SetMapRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_COMPAT_MAP_REQUEST => return Ok(Request::XkbGetCompatMap(xkb::GetCompatMapRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_COMPAT_MAP_REQUEST => return Ok(Request::XkbSetCompatMap(xkb::SetCompatMapRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_INDICATOR_STATE_REQUEST => return Ok(Request::XkbGetIndicatorState(xkb::GetIndicatorStateRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_INDICATOR_MAP_REQUEST => return Ok(Request::XkbGetIndicatorMap(xkb::GetIndicatorMapRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_INDICATOR_MAP_REQUEST => return Ok(Request::XkbSetIndicatorMap(xkb::SetIndicatorMapRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_NAMED_INDICATOR_REQUEST => return Ok(Request::XkbGetNamedIndicator(xkb::GetNamedIndicatorRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_NAMED_INDICATOR_REQUEST => return Ok(Request::XkbSetNamedIndicator(xkb::SetNamedIndicatorRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_NAMES_REQUEST => return Ok(Request::XkbGetNames(xkb::GetNamesRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_NAMES_REQUEST => return Ok(Request::XkbSetNames(xkb::SetNamesRequest::try_parse_request(header, remaining)?)),
                    xkb::PER_CLIENT_FLAGS_REQUEST => return Ok(Request::XkbPerClientFlags(xkb::PerClientFlagsRequest::try_parse_request(header, remaining)?)),
                    xkb::LIST_COMPONENTS_REQUEST => return Ok(Request::XkbListComponents(xkb::ListComponentsRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_KBD_BY_NAME_REQUEST => return Ok(Request::XkbGetKbdByName(xkb::GetKbdByNameRequest::try_parse_request(header, remaining)?)),
                    xkb::GET_DEVICE_INFO_REQUEST => return Ok(Request::XkbGetDeviceInfo(xkb::GetDeviceInfoRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_DEVICE_INFO_REQUEST => return Ok(Request::XkbSetDeviceInfo(xkb::SetDeviceInfoRequest::try_parse_request(header, remaining)?)),
                    xkb::SET_DEBUGGING_FLAGS_REQUEST => return Ok(Request::XkbSetDebuggingFlags(xkb::SetDebuggingFlagsRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xprint::PRINT_QUERY_VERSION_REQUEST => return Ok(Request::XprintPrintQueryVersion(xprint::PrintQueryVersionRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_PRINTER_LIST_REQUEST => return Ok(Request::XprintPrintGetPrinterList(xprint::PrintGetPrinterListRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_REHASH_PRINTER_LIST_REQUEST => return Ok(Request::XprintPrintRehashPrinterList(xprint::PrintRehashPrinterListRequest::try_parse_request(header, remaining)?)),
                    xprint::CREATE_CONTEXT_REQUEST => return Ok(Request::XprintCreateContext(xprint::CreateContextRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_SET_CONTEXT_REQUEST => return Ok(Request::XprintPrintSetContext(xprint::PrintSetContextRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_CONTEXT_REQUEST => return Ok(Request::XprintPrintGetContext(xprint::PrintGetContextRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_DESTROY_CONTEXT_REQUEST => return Ok(Request::XprintPrintDestroyContext(xprint::PrintDestroyContextRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_SCREEN_OF_CONTEXT_REQUEST => return Ok(Request::XprintPrintGetScreenOfContext(xprint::PrintGetScreenOfContextRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_START_JOB_REQUEST => return Ok(Request::XprintPrintStartJob(xprint::PrintStartJobRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_END_JOB_REQUEST => return Ok(Request::XprintPrintEndJob(xprint::PrintEndJobRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_START_DOC_REQUEST => return Ok(Request::XprintPrintStartDoc(xprint::PrintStartDocRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_END_DOC_REQUEST => return Ok(Request::XprintPrintEndDoc(xprint::PrintEndDocRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_PUT_DOCUMENT_DATA_REQUEST => return Ok(Request::XprintPrintPutDocumentData(xprint::PrintPutDocumentDataRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_DOCUMENT_DATA_REQUEST => return Ok(Request::XprintPrintGetDocumentData(xprint::PrintGetDocumentDataRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_START_PAGE_REQUEST => return Ok(Request::XprintPrintStartPage(xprint::PrintStartPageRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_END_PAGE_REQUEST => return Ok(Request::XprintPrintEndPage(xprint::PrintEndPageRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_SELECT_INPUT_REQUEST => return Ok(Request::XprintPrintSelectInput(xprint::PrintSelectInputRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_INPUT_SELECTED_REQUEST => return Ok(Request::XprintPrintInputSelected(xprint::PrintInputSelectedRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_ATTRIBUTES_REQUEST => return Ok(Request::XprintPrintGetAttributes(xprint::PrintGetAttributesRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_ONE_ATTRIBUTES_REQUEST => return Ok(Request::XprintPrintGetOneAttributes(xprint::PrintGetOneAttributesRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_SET_ATTRIBUTES_REQUEST => return Ok(Request::XprintPrintSetAttributes(xprint::PrintSetAttributesRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_PAGE_DIMENSIONS_REQUEST => return Ok(Request::XprintPrintGetPageDimensions(xprint::PrintGetPageDimensionsRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_QUERY_SCREENS_REQUEST => return Ok(Request::XprintPrintQueryScreens(xprint::PrintQueryScreensRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_SET_IMAGE_RESOLUTION_REQUEST => return Ok(Request::XprintPrintSetImageResolution(xprint::PrintSetImageResolutionRequest::try_parse_request(header, remaining)?)),
                    xprint::PRINT_GET_IMAGE_RESOLUTION_REQUEST => return Ok(Request::XprintPrintGetImageResolution(xprint::PrintGetImageResolutionRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xselinux")]
            Some((xselinux::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xselinux::QUERY_VERSION_REQUEST => return Ok(Request::XselinuxQueryVersion(xselinux::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_DEVICE_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetDeviceCreateContext(xselinux::SetDeviceCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_DEVICE_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetDeviceCreateContext(xselinux::GetDeviceCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_DEVICE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetDeviceContext(xselinux::SetDeviceContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_DEVICE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetDeviceContext(xselinux::GetDeviceContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_WINDOW_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetWindowCreateContext(xselinux::SetWindowCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_WINDOW_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetWindowCreateContext(xselinux::GetWindowCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_WINDOW_CONTEXT_REQUEST => return Ok(Request::XselinuxGetWindowContext(xselinux::GetWindowContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_PROPERTY_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetPropertyCreateContext(xselinux::SetPropertyCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_PROPERTY_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetPropertyCreateContext(xselinux::GetPropertyCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_PROPERTY_USE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetPropertyUseContext(xselinux::SetPropertyUseContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_PROPERTY_USE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetPropertyUseContext(xselinux::GetPropertyUseContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_PROPERTY_CONTEXT_REQUEST => return Ok(Request::XselinuxGetPropertyContext(xselinux::GetPropertyContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_PROPERTY_DATA_CONTEXT_REQUEST => return Ok(Request::XselinuxGetPropertyDataContext(xselinux::GetPropertyDataContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::LIST_PROPERTIES_REQUEST => return Ok(Request::XselinuxListProperties(xselinux::ListPropertiesRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_SELECTION_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetSelectionCreateContext(xselinux::SetSelectionCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_SELECTION_CREATE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetSelectionCreateContext(xselinux::GetSelectionCreateContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::SET_SELECTION_USE_CONTEXT_REQUEST => return Ok(Request::XselinuxSetSelectionUseContext(xselinux::SetSelectionUseContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_SELECTION_USE_CONTEXT_REQUEST => return Ok(Request::XselinuxGetSelectionUseContext(xselinux::GetSelectionUseContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_SELECTION_CONTEXT_REQUEST => return Ok(Request::XselinuxGetSelectionContext(xselinux::GetSelectionContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_SELECTION_DATA_CONTEXT_REQUEST => return Ok(Request::XselinuxGetSelectionDataContext(xselinux::GetSelectionDataContextRequest::try_parse_request(header, remaining)?)),
                    xselinux::LIST_SELECTIONS_REQUEST => return Ok(Request::XselinuxListSelections(xselinux::ListSelectionsRequest::try_parse_request(header, remaining)?)),
                    xselinux::GET_CLIENT_CONTEXT_REQUEST => return Ok(Request::XselinuxGetClientContext(xselinux::GetClientContextRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xtest")]
            Some((xtest::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xtest::GET_VERSION_REQUEST => return Ok(Request::XtestGetVersion(xtest::GetVersionRequest::try_parse_request(header, remaining)?)),
                    xtest::COMPARE_CURSOR_REQUEST => return Ok(Request::XtestCompareCursor(xtest::CompareCursorRequest::try_parse_request(header, remaining)?)),
                    xtest::FAKE_INPUT_REQUEST => return Ok(Request::XtestFakeInput(xtest::FakeInputRequest::try_parse_request(header, remaining)?)),
                    xtest::GRAB_CONTROL_REQUEST => return Ok(Request::XtestGrabControl(xtest::GrabControlRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xv::QUERY_EXTENSION_REQUEST => return Ok(Request::XvQueryExtension(xv::QueryExtensionRequest::try_parse_request(header, remaining)?)),
                    xv::QUERY_ADAPTORS_REQUEST => return Ok(Request::XvQueryAdaptors(xv::QueryAdaptorsRequest::try_parse_request(header, remaining)?)),
                    xv::QUERY_ENCODINGS_REQUEST => return Ok(Request::XvQueryEncodings(xv::QueryEncodingsRequest::try_parse_request(header, remaining)?)),
                    xv::GRAB_PORT_REQUEST => return Ok(Request::XvGrabPort(xv::GrabPortRequest::try_parse_request(header, remaining)?)),
                    xv::UNGRAB_PORT_REQUEST => return Ok(Request::XvUngrabPort(xv::UngrabPortRequest::try_parse_request(header, remaining)?)),
                    xv::PUT_VIDEO_REQUEST => return Ok(Request::XvPutVideo(xv::PutVideoRequest::try_parse_request(header, remaining)?)),
                    xv::PUT_STILL_REQUEST => return Ok(Request::XvPutStill(xv::PutStillRequest::try_parse_request(header, remaining)?)),
                    xv::GET_VIDEO_REQUEST => return Ok(Request::XvGetVideo(xv::GetVideoRequest::try_parse_request(header, remaining)?)),
                    xv::GET_STILL_REQUEST => return Ok(Request::XvGetStill(xv::GetStillRequest::try_parse_request(header, remaining)?)),
                    xv::STOP_VIDEO_REQUEST => return Ok(Request::XvStopVideo(xv::StopVideoRequest::try_parse_request(header, remaining)?)),
                    xv::SELECT_VIDEO_NOTIFY_REQUEST => return Ok(Request::XvSelectVideoNotify(xv::SelectVideoNotifyRequest::try_parse_request(header, remaining)?)),
                    xv::SELECT_PORT_NOTIFY_REQUEST => return Ok(Request::XvSelectPortNotify(xv::SelectPortNotifyRequest::try_parse_request(header, remaining)?)),
                    xv::QUERY_BEST_SIZE_REQUEST => return Ok(Request::XvQueryBestSize(xv::QueryBestSizeRequest::try_parse_request(header, remaining)?)),
                    xv::SET_PORT_ATTRIBUTE_REQUEST => return Ok(Request::XvSetPortAttribute(xv::SetPortAttributeRequest::try_parse_request(header, remaining)?)),
                    xv::GET_PORT_ATTRIBUTE_REQUEST => return Ok(Request::XvGetPortAttribute(xv::GetPortAttributeRequest::try_parse_request(header, remaining)?)),
                    xv::QUERY_PORT_ATTRIBUTES_REQUEST => return Ok(Request::XvQueryPortAttributes(xv::QueryPortAttributesRequest::try_parse_request(header, remaining)?)),
                    xv::LIST_IMAGE_FORMATS_REQUEST => return Ok(Request::XvListImageFormats(xv::ListImageFormatsRequest::try_parse_request(header, remaining)?)),
                    xv::QUERY_IMAGE_ATTRIBUTES_REQUEST => return Ok(Request::XvQueryImageAttributes(xv::QueryImageAttributesRequest::try_parse_request(header, remaining)?)),
                    xv::PUT_IMAGE_REQUEST => return Ok(Request::XvPutImage(xv::PutImageRequest::try_parse_request(header, remaining)?)),
                    xv::SHM_PUT_IMAGE_REQUEST => return Ok(Request::XvShmPutImage(xv::ShmPutImageRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            #[cfg(feature = "xvmc")]
            Some((xvmc::X11_EXTENSION_NAME, _)) => {
                match header.minor_opcode {
                    xvmc::QUERY_VERSION_REQUEST => return Ok(Request::XvmcQueryVersion(xvmc::QueryVersionRequest::try_parse_request(header, remaining)?)),
                    xvmc::LIST_SURFACE_TYPES_REQUEST => return Ok(Request::XvmcListSurfaceTypes(xvmc::ListSurfaceTypesRequest::try_parse_request(header, remaining)?)),
                    xvmc::CREATE_CONTEXT_REQUEST => return Ok(Request::XvmcCreateContext(xvmc::CreateContextRequest::try_parse_request(header, remaining)?)),
                    xvmc::DESTROY_CONTEXT_REQUEST => return Ok(Request::XvmcDestroyContext(xvmc::DestroyContextRequest::try_parse_request(header, remaining)?)),
                    xvmc::CREATE_SURFACE_REQUEST => return Ok(Request::XvmcCreateSurface(xvmc::CreateSurfaceRequest::try_parse_request(header, remaining)?)),
                    xvmc::DESTROY_SURFACE_REQUEST => return Ok(Request::XvmcDestroySurface(xvmc::DestroySurfaceRequest::try_parse_request(header, remaining)?)),
                    xvmc::CREATE_SUBPICTURE_REQUEST => return Ok(Request::XvmcCreateSubpicture(xvmc::CreateSubpictureRequest::try_parse_request(header, remaining)?)),
                    xvmc::DESTROY_SUBPICTURE_REQUEST => return Ok(Request::XvmcDestroySubpicture(xvmc::DestroySubpictureRequest::try_parse_request(header, remaining)?)),
                    xvmc::LIST_SUBPICTURE_TYPES_REQUEST => return Ok(Request::XvmcListSubpictureTypes(xvmc::ListSubpictureTypesRequest::try_parse_request(header, remaining)?)),
                    _ => (),
                }
            }
            _ => (),
        }
        Ok(Request::Unknown(header, Cow::Borrowed(remaining)))
    }
    /// Get the matching reply parser (if any) for this request.
    /// For `Request::Unknown`, `None` is also returned.
    pub fn reply_parser(&self) -> Option<ReplyParsingFunction> {
        match self {
            Request::Unknown(_, _) => None,
            Request::CreateWindow(_) => None,
            Request::ChangeWindowAttributes(_) => None,
            Request::GetWindowAttributes(_) => Some(xproto::GetWindowAttributesRequest::parse_reply),
            Request::DestroyWindow(_) => None,
            Request::DestroySubwindows(_) => None,
            Request::ChangeSaveSet(_) => None,
            Request::ReparentWindow(_) => None,
            Request::MapWindow(_) => None,
            Request::MapSubwindows(_) => None,
            Request::UnmapWindow(_) => None,
            Request::UnmapSubwindows(_) => None,
            Request::ConfigureWindow(_) => None,
            Request::CirculateWindow(_) => None,
            Request::GetGeometry(_) => Some(xproto::GetGeometryRequest::parse_reply),
            Request::QueryTree(_) => Some(xproto::QueryTreeRequest::parse_reply),
            Request::InternAtom(_) => Some(xproto::InternAtomRequest::parse_reply),
            Request::GetAtomName(_) => Some(xproto::GetAtomNameRequest::parse_reply),
            Request::ChangeProperty(_) => None,
            Request::DeleteProperty(_) => None,
            Request::GetProperty(_) => Some(xproto::GetPropertyRequest::parse_reply),
            Request::ListProperties(_) => Some(xproto::ListPropertiesRequest::parse_reply),
            Request::SetSelectionOwner(_) => None,
            Request::GetSelectionOwner(_) => Some(xproto::GetSelectionOwnerRequest::parse_reply),
            Request::ConvertSelection(_) => None,
            Request::SendEvent(_) => None,
            Request::GrabPointer(_) => Some(xproto::GrabPointerRequest::parse_reply),
            Request::UngrabPointer(_) => None,
            Request::GrabButton(_) => None,
            Request::UngrabButton(_) => None,
            Request::ChangeActivePointerGrab(_) => None,
            Request::GrabKeyboard(_) => Some(xproto::GrabKeyboardRequest::parse_reply),
            Request::UngrabKeyboard(_) => None,
            Request::GrabKey(_) => None,
            Request::UngrabKey(_) => None,
            Request::AllowEvents(_) => None,
            Request::GrabServer(_) => None,
            Request::UngrabServer(_) => None,
            Request::QueryPointer(_) => Some(xproto::QueryPointerRequest::parse_reply),
            Request::GetMotionEvents(_) => Some(xproto::GetMotionEventsRequest::parse_reply),
            Request::TranslateCoordinates(_) => Some(xproto::TranslateCoordinatesRequest::parse_reply),
            Request::WarpPointer(_) => None,
            Request::SetInputFocus(_) => None,
            Request::GetInputFocus(_) => Some(xproto::GetInputFocusRequest::parse_reply),
            Request::QueryKeymap(_) => Some(xproto::QueryKeymapRequest::parse_reply),
            Request::OpenFont(_) => None,
            Request::CloseFont(_) => None,
            Request::QueryFont(_) => Some(xproto::QueryFontRequest::parse_reply),
            Request::QueryTextExtents(_) => Some(xproto::QueryTextExtentsRequest::parse_reply),
            Request::ListFonts(_) => Some(xproto::ListFontsRequest::parse_reply),
            Request::ListFontsWithInfo(_) => Some(xproto::ListFontsWithInfoRequest::parse_reply),
            Request::SetFontPath(_) => None,
            Request::GetFontPath(_) => Some(xproto::GetFontPathRequest::parse_reply),
            Request::CreatePixmap(_) => None,
            Request::FreePixmap(_) => None,
            Request::CreateGC(_) => None,
            Request::ChangeGC(_) => None,
            Request::CopyGC(_) => None,
            Request::SetDashes(_) => None,
            Request::SetClipRectangles(_) => None,
            Request::FreeGC(_) => None,
            Request::ClearArea(_) => None,
            Request::CopyArea(_) => None,
            Request::CopyPlane(_) => None,
            Request::PolyPoint(_) => None,
            Request::PolyLine(_) => None,
            Request::PolySegment(_) => None,
            Request::PolyRectangle(_) => None,
            Request::PolyArc(_) => None,
            Request::FillPoly(_) => None,
            Request::PolyFillRectangle(_) => None,
            Request::PolyFillArc(_) => None,
            Request::PutImage(_) => None,
            Request::GetImage(_) => Some(xproto::GetImageRequest::parse_reply),
            Request::PolyText8(_) => None,
            Request::PolyText16(_) => None,
            Request::ImageText8(_) => None,
            Request::ImageText16(_) => None,
            Request::CreateColormap(_) => None,
            Request::FreeColormap(_) => None,
            Request::CopyColormapAndFree(_) => None,
            Request::InstallColormap(_) => None,
            Request::UninstallColormap(_) => None,
            Request::ListInstalledColormaps(_) => Some(xproto::ListInstalledColormapsRequest::parse_reply),
            Request::AllocColor(_) => Some(xproto::AllocColorRequest::parse_reply),
            Request::AllocNamedColor(_) => Some(xproto::AllocNamedColorRequest::parse_reply),
            Request::AllocColorCells(_) => Some(xproto::AllocColorCellsRequest::parse_reply),
            Request::AllocColorPlanes(_) => Some(xproto::AllocColorPlanesRequest::parse_reply),
            Request::FreeColors(_) => None,
            Request::StoreColors(_) => None,
            Request::StoreNamedColor(_) => None,
            Request::QueryColors(_) => Some(xproto::QueryColorsRequest::parse_reply),
            Request::LookupColor(_) => Some(xproto::LookupColorRequest::parse_reply),
            Request::CreateCursor(_) => None,
            Request::CreateGlyphCursor(_) => None,
            Request::FreeCursor(_) => None,
            Request::RecolorCursor(_) => None,
            Request::QueryBestSize(_) => Some(xproto::QueryBestSizeRequest::parse_reply),
            Request::QueryExtension(_) => Some(xproto::QueryExtensionRequest::parse_reply),
            Request::ListExtensions(_) => Some(xproto::ListExtensionsRequest::parse_reply),
            Request::ChangeKeyboardMapping(_) => None,
            Request::GetKeyboardMapping(_) => Some(xproto::GetKeyboardMappingRequest::parse_reply),
            Request::ChangeKeyboardControl(_) => None,
            Request::GetKeyboardControl(_) => Some(xproto::GetKeyboardControlRequest::parse_reply),
            Request::Bell(_) => None,
            Request::ChangePointerControl(_) => None,
            Request::GetPointerControl(_) => Some(xproto::GetPointerControlRequest::parse_reply),
            Request::SetScreenSaver(_) => None,
            Request::GetScreenSaver(_) => Some(xproto::GetScreenSaverRequest::parse_reply),
            Request::ChangeHosts(_) => None,
            Request::ListHosts(_) => Some(xproto::ListHostsRequest::parse_reply),
            Request::SetAccessControl(_) => None,
            Request::SetCloseDownMode(_) => None,
            Request::KillClient(_) => None,
            Request::RotateProperties(_) => None,
            Request::ForceScreenSaver(_) => None,
            Request::SetPointerMapping(_) => Some(xproto::SetPointerMappingRequest::parse_reply),
            Request::GetPointerMapping(_) => Some(xproto::GetPointerMappingRequest::parse_reply),
            Request::SetModifierMapping(_) => Some(xproto::SetModifierMappingRequest::parse_reply),
            Request::GetModifierMapping(_) => Some(xproto::GetModifierMappingRequest::parse_reply),
            Request::NoOperation(_) => None,
            Request::BigreqEnable(_) => Some(bigreq::EnableRequest::parse_reply),
            #[cfg(feature = "composite")]
            Request::CompositeQueryVersion(_) => Some(composite::QueryVersionRequest::parse_reply),
            #[cfg(feature = "composite")]
            Request::CompositeRedirectWindow(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeRedirectSubwindows(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeUnredirectWindow(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeUnredirectSubwindows(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeCreateRegionFromBorderClip(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeNameWindowPixmap(_) => None,
            #[cfg(feature = "composite")]
            Request::CompositeGetOverlayWindow(_) => Some(composite::GetOverlayWindowRequest::parse_reply),
            #[cfg(feature = "composite")]
            Request::CompositeReleaseOverlayWindow(_) => None,
            #[cfg(feature = "damage")]
            Request::DamageQueryVersion(_) => Some(damage::QueryVersionRequest::parse_reply),
            #[cfg(feature = "damage")]
            Request::DamageCreate(_) => None,
            #[cfg(feature = "damage")]
            Request::DamageDestroy(_) => None,
            #[cfg(feature = "damage")]
            Request::DamageSubtract(_) => None,
            #[cfg(feature = "damage")]
            Request::DamageAdd(_) => None,
            #[cfg(feature = "dpms")]
            Request::DpmsGetVersion(_) => Some(dpms::GetVersionRequest::parse_reply),
            #[cfg(feature = "dpms")]
            Request::DpmsCapable(_) => Some(dpms::CapableRequest::parse_reply),
            #[cfg(feature = "dpms")]
            Request::DpmsGetTimeouts(_) => Some(dpms::GetTimeoutsRequest::parse_reply),
            #[cfg(feature = "dpms")]
            Request::DpmsSetTimeouts(_) => None,
            #[cfg(feature = "dpms")]
            Request::DpmsEnable(_) => None,
            #[cfg(feature = "dpms")]
            Request::DpmsDisable(_) => None,
            #[cfg(feature = "dpms")]
            Request::DpmsForceLevel(_) => None,
            #[cfg(feature = "dpms")]
            Request::DpmsInfo(_) => Some(dpms::InfoRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2QueryVersion(_) => Some(dri2::QueryVersionRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2Connect(_) => Some(dri2::ConnectRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2Authenticate(_) => Some(dri2::AuthenticateRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2CreateDrawable(_) => None,
            #[cfg(feature = "dri2")]
            Request::Dri2DestroyDrawable(_) => None,
            #[cfg(feature = "dri2")]
            Request::Dri2GetBuffers(_) => Some(dri2::GetBuffersRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2CopyRegion(_) => Some(dri2::CopyRegionRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2GetBuffersWithFormat(_) => Some(dri2::GetBuffersWithFormatRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2SwapBuffers(_) => Some(dri2::SwapBuffersRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2GetMSC(_) => Some(dri2::GetMSCRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2WaitMSC(_) => Some(dri2::WaitMSCRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2WaitSBC(_) => Some(dri2::WaitSBCRequest::parse_reply),
            #[cfg(feature = "dri2")]
            Request::Dri2SwapInterval(_) => None,
            #[cfg(feature = "dri2")]
            Request::Dri2GetParam(_) => Some(dri2::GetParamRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3QueryVersion(_) => Some(dri3::QueryVersionRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3Open(_) => Some(dri3::OpenRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3PixmapFromBuffer(_) => None,
            #[cfg(feature = "dri3")]
            Request::Dri3BufferFromPixmap(_) => Some(dri3::BufferFromPixmapRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3FenceFromFD(_) => None,
            #[cfg(feature = "dri3")]
            Request::Dri3FDFromFence(_) => Some(dri3::FDFromFenceRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3GetSupportedModifiers(_) => Some(dri3::GetSupportedModifiersRequest::parse_reply),
            #[cfg(feature = "dri3")]
            Request::Dri3PixmapFromBuffers(_) => None,
            #[cfg(feature = "dri3")]
            Request::Dri3BuffersFromPixmap(_) => Some(dri3::BuffersFromPixmapRequest::parse_reply),
            Request::GeQueryVersion(_) => Some(ge::QueryVersionRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxRender(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxRenderLarge(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCreateContext(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxDestroyContext(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxMakeCurrent(_) => Some(glx::MakeCurrentRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxIsDirect(_) => Some(glx::IsDirectRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxQueryVersion(_) => Some(glx::QueryVersionRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxWaitGL(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxWaitX(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCopyContext(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxSwapBuffers(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxUseXFont(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCreateGLXPixmap(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGetVisualConfigs(_) => Some(glx::GetVisualConfigsRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxDestroyGLXPixmap(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxVendorPrivate(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxVendorPrivateWithReply(_) => Some(glx::VendorPrivateWithReplyRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxQueryExtensionsString(_) => Some(glx::QueryExtensionsStringRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxQueryServerString(_) => Some(glx::QueryServerStringRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxClientInfo(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGetFBConfigs(_) => Some(glx::GetFBConfigsRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxCreatePixmap(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxDestroyPixmap(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCreateNewContext(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxQueryContext(_) => Some(glx::QueryContextRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxMakeContextCurrent(_) => Some(glx::MakeContextCurrentRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxCreatePbuffer(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxDestroyPbuffer(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGetDrawableAttributes(_) => Some(glx::GetDrawableAttributesRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxChangeDrawableAttributes(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCreateWindow(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxDeleteWindow(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxSetClientInfoARB(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxCreateContextAttribsARB(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxSetClientInfo2ARB(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxNewList(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxEndList(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxDeleteLists(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGenLists(_) => Some(glx::GenListsRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxFeedbackBuffer(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxSelectBuffer(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxRenderMode(_) => Some(glx::RenderModeRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxFinish(_) => Some(glx::FinishRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxPixelStoref(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxPixelStorei(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxReadPixels(_) => Some(glx::ReadPixelsRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetBooleanv(_) => Some(glx::GetBooleanvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetClipPlane(_) => Some(glx::GetClipPlaneRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetDoublev(_) => Some(glx::GetDoublevRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetError(_) => Some(glx::GetErrorRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetFloatv(_) => Some(glx::GetFloatvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetIntegerv(_) => Some(glx::GetIntegervRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetLightfv(_) => Some(glx::GetLightfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetLightiv(_) => Some(glx::GetLightivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMapdv(_) => Some(glx::GetMapdvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMapfv(_) => Some(glx::GetMapfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMapiv(_) => Some(glx::GetMapivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMaterialfv(_) => Some(glx::GetMaterialfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMaterialiv(_) => Some(glx::GetMaterialivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapfv(_) => Some(glx::GetPixelMapfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapuiv(_) => Some(glx::GetPixelMapuivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapusv(_) => Some(glx::GetPixelMapusvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetPolygonStipple(_) => Some(glx::GetPolygonStippleRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetString(_) => Some(glx::GetStringRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexEnvfv(_) => Some(glx::GetTexEnvfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexEnviv(_) => Some(glx::GetTexEnvivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGendv(_) => Some(glx::GetTexGendvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGenfv(_) => Some(glx::GetTexGenfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGeniv(_) => Some(glx::GetTexGenivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexImage(_) => Some(glx::GetTexImageRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexParameterfv(_) => Some(glx::GetTexParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexParameteriv(_) => Some(glx::GetTexParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexLevelParameterfv(_) => Some(glx::GetTexLevelParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetTexLevelParameteriv(_) => Some(glx::GetTexLevelParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxIsEnabled(_) => Some(glx::IsEnabledRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxIsList(_) => Some(glx::IsListRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxFlush(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxAreTexturesResident(_) => Some(glx::AreTexturesResidentRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxDeleteTextures(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGenTextures(_) => Some(glx::GenTexturesRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxIsTexture(_) => Some(glx::IsTextureRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTable(_) => Some(glx::GetColorTableRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTableParameterfv(_) => Some(glx::GetColorTableParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTableParameteriv(_) => Some(glx::GetColorTableParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionFilter(_) => Some(glx::GetConvolutionFilterRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionParameterfv(_) => Some(glx::GetConvolutionParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionParameteriv(_) => Some(glx::GetConvolutionParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetSeparableFilter(_) => Some(glx::GetSeparableFilterRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogram(_) => Some(glx::GetHistogramRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogramParameterfv(_) => Some(glx::GetHistogramParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogramParameteriv(_) => Some(glx::GetHistogramParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmax(_) => Some(glx::GetMinmaxRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmaxParameterfv(_) => Some(glx::GetMinmaxParameterfvRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmaxParameteriv(_) => Some(glx::GetMinmaxParameterivRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetCompressedTexImageARB(_) => Some(glx::GetCompressedTexImageARBRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxDeleteQueriesARB(_) => None,
            #[cfg(feature = "glx")]
            Request::GlxGenQueriesARB(_) => Some(glx::GenQueriesARBRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxIsQueryARB(_) => Some(glx::IsQueryARBRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryivARB(_) => Some(glx::GetQueryivARBRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryObjectivARB(_) => Some(glx::GetQueryObjectivARBRequest::parse_reply),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryObjectuivARB(_) => Some(glx::GetQueryObjectuivARBRequest::parse_reply),
            #[cfg(feature = "present")]
            Request::PresentQueryVersion(_) => Some(present::QueryVersionRequest::parse_reply),
            #[cfg(feature = "present")]
            Request::PresentPixmap(_) => None,
            #[cfg(feature = "present")]
            Request::PresentNotifyMSC(_) => None,
            #[cfg(feature = "present")]
            Request::PresentSelectInput(_) => None,
            #[cfg(feature = "present")]
            Request::PresentQueryCapabilities(_) => Some(present::QueryCapabilitiesRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrQueryVersion(_) => Some(randr::QueryVersionRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetScreenConfig(_) => Some(randr::SetScreenConfigRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSelectInput(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetScreenInfo(_) => Some(randr::GetScreenInfoRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetScreenSizeRange(_) => Some(randr::GetScreenSizeRangeRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetScreenSize(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetScreenResources(_) => Some(randr::GetScreenResourcesRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetOutputInfo(_) => Some(randr::GetOutputInfoRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrListOutputProperties(_) => Some(randr::ListOutputPropertiesRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrQueryOutputProperty(_) => Some(randr::QueryOutputPropertyRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrConfigureOutputProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrChangeOutputProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrDeleteOutputProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetOutputProperty(_) => Some(randr::GetOutputPropertyRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrCreateMode(_) => Some(randr::CreateModeRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrDestroyMode(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrAddOutputMode(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrDeleteOutputMode(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcInfo(_) => Some(randr::GetCrtcInfoRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcConfig(_) => Some(randr::SetCrtcConfigRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcGammaSize(_) => Some(randr::GetCrtcGammaSizeRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcGamma(_) => Some(randr::GetCrtcGammaRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcGamma(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetScreenResourcesCurrent(_) => Some(randr::GetScreenResourcesCurrentRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcTransform(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcTransform(_) => Some(randr::GetCrtcTransformRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetPanning(_) => Some(randr::GetPanningRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetPanning(_) => Some(randr::SetPanningRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetOutputPrimary(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetOutputPrimary(_) => Some(randr::GetOutputPrimaryRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetProviders(_) => Some(randr::GetProvidersRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetProviderInfo(_) => Some(randr::GetProviderInfoRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetProviderOffloadSink(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrSetProviderOutputSource(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrListProviderProperties(_) => Some(randr::ListProviderPropertiesRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrQueryProviderProperty(_) => Some(randr::QueryProviderPropertyRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrConfigureProviderProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrChangeProviderProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrDeleteProviderProperty(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrGetProviderProperty(_) => Some(randr::GetProviderPropertyRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrGetMonitors(_) => Some(randr::GetMonitorsRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrSetMonitor(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrDeleteMonitor(_) => None,
            #[cfg(feature = "randr")]
            Request::RandrCreateLease(_) => Some(randr::CreateLeaseRequest::parse_reply),
            #[cfg(feature = "randr")]
            Request::RandrFreeLease(_) => None,
            #[cfg(feature = "record")]
            Request::RecordQueryVersion(_) => Some(record::QueryVersionRequest::parse_reply),
            #[cfg(feature = "record")]
            Request::RecordCreateContext(_) => None,
            #[cfg(feature = "record")]
            Request::RecordRegisterClients(_) => None,
            #[cfg(feature = "record")]
            Request::RecordUnregisterClients(_) => None,
            #[cfg(feature = "record")]
            Request::RecordGetContext(_) => Some(record::GetContextRequest::parse_reply),
            #[cfg(feature = "record")]
            Request::RecordEnableContext(_) => Some(record::EnableContextRequest::parse_reply),
            #[cfg(feature = "record")]
            Request::RecordDisableContext(_) => None,
            #[cfg(feature = "record")]
            Request::RecordFreeContext(_) => None,
            #[cfg(feature = "render")]
            Request::RenderQueryVersion(_) => Some(render::QueryVersionRequest::parse_reply),
            #[cfg(feature = "render")]
            Request::RenderQueryPictFormats(_) => Some(render::QueryPictFormatsRequest::parse_reply),
            #[cfg(feature = "render")]
            Request::RenderQueryPictIndexValues(_) => Some(render::QueryPictIndexValuesRequest::parse_reply),
            #[cfg(feature = "render")]
            Request::RenderCreatePicture(_) => None,
            #[cfg(feature = "render")]
            Request::RenderChangePicture(_) => None,
            #[cfg(feature = "render")]
            Request::RenderSetPictureClipRectangles(_) => None,
            #[cfg(feature = "render")]
            Request::RenderFreePicture(_) => None,
            #[cfg(feature = "render")]
            Request::RenderComposite(_) => None,
            #[cfg(feature = "render")]
            Request::RenderTrapezoids(_) => None,
            #[cfg(feature = "render")]
            Request::RenderTriangles(_) => None,
            #[cfg(feature = "render")]
            Request::RenderTriStrip(_) => None,
            #[cfg(feature = "render")]
            Request::RenderTriFan(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateGlyphSet(_) => None,
            #[cfg(feature = "render")]
            Request::RenderReferenceGlyphSet(_) => None,
            #[cfg(feature = "render")]
            Request::RenderFreeGlyphSet(_) => None,
            #[cfg(feature = "render")]
            Request::RenderAddGlyphs(_) => None,
            #[cfg(feature = "render")]
            Request::RenderFreeGlyphs(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs8(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs16(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs32(_) => None,
            #[cfg(feature = "render")]
            Request::RenderFillRectangles(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateCursor(_) => None,
            #[cfg(feature = "render")]
            Request::RenderSetPictureTransform(_) => None,
            #[cfg(feature = "render")]
            Request::RenderQueryFilters(_) => Some(render::QueryFiltersRequest::parse_reply),
            #[cfg(feature = "render")]
            Request::RenderSetPictureFilter(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateAnimCursor(_) => None,
            #[cfg(feature = "render")]
            Request::RenderAddTraps(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateSolidFill(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateLinearGradient(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateRadialGradient(_) => None,
            #[cfg(feature = "render")]
            Request::RenderCreateConicalGradient(_) => None,
            #[cfg(feature = "res")]
            Request::ResQueryVersion(_) => Some(res::QueryVersionRequest::parse_reply),
            #[cfg(feature = "res")]
            Request::ResQueryClients(_) => Some(res::QueryClientsRequest::parse_reply),
            #[cfg(feature = "res")]
            Request::ResQueryClientResources(_) => Some(res::QueryClientResourcesRequest::parse_reply),
            #[cfg(feature = "res")]
            Request::ResQueryClientPixmapBytes(_) => Some(res::QueryClientPixmapBytesRequest::parse_reply),
            #[cfg(feature = "res")]
            Request::ResQueryClientIds(_) => Some(res::QueryClientIdsRequest::parse_reply),
            #[cfg(feature = "res")]
            Request::ResQueryResourceBytes(_) => Some(res::QueryResourceBytesRequest::parse_reply),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverQueryVersion(_) => Some(screensaver::QueryVersionRequest::parse_reply),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverQueryInfo(_) => Some(screensaver::QueryInfoRequest::parse_reply),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSelectInput(_) => None,
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSetAttributes(_) => None,
            #[cfg(feature = "screensaver")]
            Request::ScreensaverUnsetAttributes(_) => None,
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSuspend(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeQueryVersion(_) => Some(shape::QueryVersionRequest::parse_reply),
            #[cfg(feature = "shape")]
            Request::ShapeRectangles(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeMask(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeCombine(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeOffset(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeQueryExtents(_) => Some(shape::QueryExtentsRequest::parse_reply),
            #[cfg(feature = "shape")]
            Request::ShapeSelectInput(_) => None,
            #[cfg(feature = "shape")]
            Request::ShapeInputSelected(_) => Some(shape::InputSelectedRequest::parse_reply),
            #[cfg(feature = "shape")]
            Request::ShapeGetRectangles(_) => Some(shape::GetRectanglesRequest::parse_reply),
            #[cfg(feature = "shm")]
            Request::ShmQueryVersion(_) => Some(shm::QueryVersionRequest::parse_reply),
            #[cfg(feature = "shm")]
            Request::ShmAttach(_) => None,
            #[cfg(feature = "shm")]
            Request::ShmDetach(_) => None,
            #[cfg(feature = "shm")]
            Request::ShmPutImage(_) => None,
            #[cfg(feature = "shm")]
            Request::ShmGetImage(_) => Some(shm::GetImageRequest::parse_reply),
            #[cfg(feature = "shm")]
            Request::ShmCreatePixmap(_) => None,
            #[cfg(feature = "shm")]
            Request::ShmAttachFd(_) => None,
            #[cfg(feature = "shm")]
            Request::ShmCreateSegment(_) => Some(shm::CreateSegmentRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncInitialize(_) => Some(sync::InitializeRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncListSystemCounters(_) => Some(sync::ListSystemCountersRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncCreateCounter(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncDestroyCounter(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncQueryCounter(_) => Some(sync::QueryCounterRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncAwait(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncChangeCounter(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncSetCounter(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncCreateAlarm(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncChangeAlarm(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncDestroyAlarm(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncQueryAlarm(_) => Some(sync::QueryAlarmRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncSetPriority(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncGetPriority(_) => Some(sync::GetPriorityRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncCreateFence(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncTriggerFence(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncResetFence(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncDestroyFence(_) => None,
            #[cfg(feature = "sync")]
            Request::SyncQueryFence(_) => Some(sync::QueryFenceRequest::parse_reply),
            #[cfg(feature = "sync")]
            Request::SyncAwaitFence(_) => None,
            Request::XcMiscGetVersion(_) => Some(xc_misc::GetVersionRequest::parse_reply),
            Request::XcMiscGetXIDRange(_) => Some(xc_misc::GetXIDRangeRequest::parse_reply),
            Request::XcMiscGetXIDList(_) => Some(xc_misc::GetXIDListRequest::parse_reply),
            #[cfg(feature = "xevie")]
            Request::XevieQueryVersion(_) => Some(xevie::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xevie")]
            Request::XevieStart(_) => Some(xevie::StartRequest::parse_reply),
            #[cfg(feature = "xevie")]
            Request::XevieEnd(_) => Some(xevie::EndRequest::parse_reply),
            #[cfg(feature = "xevie")]
            Request::XevieSend(_) => Some(xevie::SendRequest::parse_reply),
            #[cfg(feature = "xevie")]
            Request::XevieSelectInput(_) => Some(xevie::SelectInputRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driQueryVersion(_) => Some(xf86dri::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driQueryDirectRenderingCapable(_) => Some(xf86dri::QueryDirectRenderingCapableRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driOpenConnection(_) => Some(xf86dri::OpenConnectionRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCloseConnection(_) => None,
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetClientDriverName(_) => Some(xf86dri::GetClientDriverNameRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCreateContext(_) => Some(xf86dri::CreateContextRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driDestroyContext(_) => None,
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCreateDrawable(_) => Some(xf86dri::CreateDrawableRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driDestroyDrawable(_) => None,
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetDrawableInfo(_) => Some(xf86dri::GetDrawableInfoRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetDeviceInfo(_) => Some(xf86dri::GetDeviceInfoRequest::parse_reply),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driAuthConnection(_) => Some(xf86dri::AuthConnectionRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeQueryVersion(_) => Some(xf86vidmode::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetModeLine(_) => Some(xf86vidmode::GetModeLineRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeModModeLine(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSwitchMode(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetMonitor(_) => Some(xf86vidmode::GetMonitorRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeLockModeSwitch(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetAllModeLines(_) => Some(xf86vidmode::GetAllModeLinesRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeAddModeLine(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeDeleteModeLine(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeValidateModeLine(_) => Some(xf86vidmode::ValidateModeLineRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSwitchToMode(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetViewPort(_) => Some(xf86vidmode::GetViewPortRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetViewPort(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetDotClocks(_) => Some(xf86vidmode::GetDotClocksRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetClientVersion(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetGamma(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGamma(_) => Some(xf86vidmode::GetGammaRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGammaRamp(_) => Some(xf86vidmode::GetGammaRampRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetGammaRamp(_) => None,
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGammaRampSize(_) => Some(xf86vidmode::GetGammaRampSizeRequest::parse_reply),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetPermissions(_) => Some(xf86vidmode::GetPermissionsRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesQueryVersion(_) => Some(xfixes::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeSaveSet(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSelectSelectionInput(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSelectCursorInput(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorImage(_) => Some(xfixes::GetCursorImageRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromBitmap(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromWindow(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromGC(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromPicture(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesDestroyRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSetRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCopyRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesUnionRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesIntersectRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSubtractRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesInvertRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesTranslateRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesRegionExtents(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesFetchRegion(_) => Some(xfixes::FetchRegionRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetGCClipRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSetWindowShapeRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSetPictureClipRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesSetCursorName(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorName(_) => Some(xfixes::GetCursorNameRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorImageAndName(_) => Some(xfixes::GetCursorImageAndNameRequest::parse_reply),
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeCursor(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeCursorByName(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesExpandRegion(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesHideCursor(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesShowCursor(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesCreatePointerBarrier(_) => None,
            #[cfg(feature = "xfixes")]
            Request::XfixesDeletePointerBarrier(_) => None,
            #[cfg(feature = "xinerama")]
            Request::XineramaQueryVersion(_) => Some(xinerama::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetState(_) => Some(xinerama::GetStateRequest::parse_reply),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetScreenCount(_) => Some(xinerama::GetScreenCountRequest::parse_reply),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetScreenSize(_) => Some(xinerama::GetScreenSizeRequest::parse_reply),
            #[cfg(feature = "xinerama")]
            Request::XineramaIsActive(_) => Some(xinerama::IsActiveRequest::parse_reply),
            #[cfg(feature = "xinerama")]
            Request::XineramaQueryScreens(_) => Some(xinerama::QueryScreensRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputGetExtensionVersion(_) => Some(xinput::GetExtensionVersionRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputListInputDevices(_) => Some(xinput::ListInputDevicesRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputOpenDevice(_) => Some(xinput::OpenDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputCloseDevice(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceMode(_) => Some(xinput::SetDeviceModeRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputSelectExtensionEvent(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetSelectedExtensionEvents(_) => Some(xinput::GetSelectedExtensionEventsRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceDontPropagateList(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceDontPropagateList(_) => Some(xinput::GetDeviceDontPropagateListRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceMotionEvents(_) => Some(xinput::GetDeviceMotionEventsRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeKeyboardDevice(_) => Some(xinput::ChangeKeyboardDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangePointerDevice(_) => Some(xinput::ChangePointerDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputGrabDevice(_) => Some(xinput::GrabDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDevice(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGrabDeviceKey(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDeviceKey(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGrabDeviceButton(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDeviceButton(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputAllowDeviceEvents(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceFocus(_) => Some(xinput::GetDeviceFocusRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceFocus(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetFeedbackControl(_) => Some(xinput::GetFeedbackControlRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeFeedbackControl(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceKeyMapping(_) => Some(xinput::GetDeviceKeyMappingRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceKeyMapping(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceModifierMapping(_) => Some(xinput::GetDeviceModifierMappingRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceModifierMapping(_) => Some(xinput::SetDeviceModifierMappingRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceButtonMapping(_) => Some(xinput::GetDeviceButtonMappingRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceButtonMapping(_) => Some(xinput::SetDeviceButtonMappingRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputQueryDeviceState(_) => Some(xinput::QueryDeviceStateRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputDeviceBell(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceValuators(_) => Some(xinput::SetDeviceValuatorsRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceControl(_) => Some(xinput::GetDeviceControlRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceControl(_) => Some(xinput::ChangeDeviceControlRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputListDeviceProperties(_) => Some(xinput::ListDevicePropertiesRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceProperty(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputDeleteDeviceProperty(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceProperty(_) => Some(xinput::GetDevicePropertyRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryPointer(_) => Some(xinput::XIQueryPointerRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIWarpPointer(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeCursor(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeHierarchy(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXISetClientPointer(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIGetClientPointer(_) => Some(xinput::XIGetClientPointerRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXISelectEvents(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryVersion(_) => Some(xinput::XIQueryVersionRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryDevice(_) => Some(xinput::XIQueryDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXISetFocus(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIGetFocus(_) => Some(xinput::XIGetFocusRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIGrabDevice(_) => Some(xinput::XIGrabDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIUngrabDevice(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIAllowEvents(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIPassiveGrabDevice(_) => Some(xinput::XIPassiveGrabDeviceRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIPassiveUngrabDevice(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIListProperties(_) => Some(xinput::XIListPropertiesRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeProperty(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIDeleteProperty(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputXIGetProperty(_) => Some(xinput::XIGetPropertyRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIGetSelectedEvents(_) => Some(xinput::XIGetSelectedEventsRequest::parse_reply),
            #[cfg(feature = "xinput")]
            Request::XinputXIBarrierReleasePointer(_) => None,
            #[cfg(feature = "xinput")]
            Request::XinputSendExtensionEvent(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbUseExtension(_) => Some(xkb::UseExtensionRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSelectEvents(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbBell(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetState(_) => Some(xkb::GetStateRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbLatchLockState(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetControls(_) => Some(xkb::GetControlsRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetControls(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetMap(_) => Some(xkb::GetMapRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetMap(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetCompatMap(_) => Some(xkb::GetCompatMapRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetCompatMap(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetIndicatorState(_) => Some(xkb::GetIndicatorStateRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbGetIndicatorMap(_) => Some(xkb::GetIndicatorMapRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetIndicatorMap(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetNamedIndicator(_) => Some(xkb::GetNamedIndicatorRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetNamedIndicator(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbGetNames(_) => Some(xkb::GetNamesRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetNames(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbPerClientFlags(_) => Some(xkb::PerClientFlagsRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbListComponents(_) => Some(xkb::ListComponentsRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbGetKbdByName(_) => Some(xkb::GetKbdByNameRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbGetDeviceInfo(_) => Some(xkb::GetDeviceInfoRequest::parse_reply),
            #[cfg(feature = "xkb")]
            Request::XkbSetDeviceInfo(_) => None,
            #[cfg(feature = "xkb")]
            Request::XkbSetDebuggingFlags(_) => Some(xkb::SetDebuggingFlagsRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintQueryVersion(_) => Some(xprint::PrintQueryVersionRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetPrinterList(_) => Some(xprint::PrintGetPrinterListRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintRehashPrinterList(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintCreateContext(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetContext(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetContext(_) => Some(xprint::PrintGetContextRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintDestroyContext(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetScreenOfContext(_) => Some(xprint::PrintGetScreenOfContextRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartJob(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndJob(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartDoc(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndDoc(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintPutDocumentData(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetDocumentData(_) => Some(xprint::PrintGetDocumentDataRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartPage(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndPage(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintSelectInput(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintInputSelected(_) => Some(xprint::PrintInputSelectedRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetAttributes(_) => Some(xprint::PrintGetAttributesRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetOneAttributes(_) => Some(xprint::PrintGetOneAttributesRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetAttributes(_) => None,
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetPageDimensions(_) => Some(xprint::PrintGetPageDimensionsRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintQueryScreens(_) => Some(xprint::PrintQueryScreensRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetImageResolution(_) => Some(xprint::PrintSetImageResolutionRequest::parse_reply),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetImageResolution(_) => Some(xprint::PrintGetImageResolutionRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxQueryVersion(_) => Some(xselinux::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetDeviceCreateContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetDeviceCreateContext(_) => Some(xselinux::GetDeviceCreateContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetDeviceContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetDeviceContext(_) => Some(xselinux::GetDeviceContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetWindowCreateContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetWindowCreateContext(_) => Some(xselinux::GetWindowCreateContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetWindowContext(_) => Some(xselinux::GetWindowContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetPropertyCreateContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyCreateContext(_) => Some(xselinux::GetPropertyCreateContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetPropertyUseContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyUseContext(_) => Some(xselinux::GetPropertyUseContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyContext(_) => Some(xselinux::GetPropertyContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyDataContext(_) => Some(xselinux::GetPropertyDataContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxListProperties(_) => Some(xselinux::ListPropertiesRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetSelectionCreateContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionCreateContext(_) => Some(xselinux::GetSelectionCreateContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetSelectionUseContext(_) => None,
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionUseContext(_) => Some(xselinux::GetSelectionUseContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionContext(_) => Some(xselinux::GetSelectionContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionDataContext(_) => Some(xselinux::GetSelectionDataContextRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxListSelections(_) => Some(xselinux::ListSelectionsRequest::parse_reply),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetClientContext(_) => Some(xselinux::GetClientContextRequest::parse_reply),
            #[cfg(feature = "xtest")]
            Request::XtestGetVersion(_) => Some(xtest::GetVersionRequest::parse_reply),
            #[cfg(feature = "xtest")]
            Request::XtestCompareCursor(_) => Some(xtest::CompareCursorRequest::parse_reply),
            #[cfg(feature = "xtest")]
            Request::XtestFakeInput(_) => None,
            #[cfg(feature = "xtest")]
            Request::XtestGrabControl(_) => None,
            #[cfg(feature = "xv")]
            Request::XvQueryExtension(_) => Some(xv::QueryExtensionRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvQueryAdaptors(_) => Some(xv::QueryAdaptorsRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvQueryEncodings(_) => Some(xv::QueryEncodingsRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvGrabPort(_) => Some(xv::GrabPortRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvUngrabPort(_) => None,
            #[cfg(feature = "xv")]
            Request::XvPutVideo(_) => None,
            #[cfg(feature = "xv")]
            Request::XvPutStill(_) => None,
            #[cfg(feature = "xv")]
            Request::XvGetVideo(_) => None,
            #[cfg(feature = "xv")]
            Request::XvGetStill(_) => None,
            #[cfg(feature = "xv")]
            Request::XvStopVideo(_) => None,
            #[cfg(feature = "xv")]
            Request::XvSelectVideoNotify(_) => None,
            #[cfg(feature = "xv")]
            Request::XvSelectPortNotify(_) => None,
            #[cfg(feature = "xv")]
            Request::XvQueryBestSize(_) => Some(xv::QueryBestSizeRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvSetPortAttribute(_) => None,
            #[cfg(feature = "xv")]
            Request::XvGetPortAttribute(_) => Some(xv::GetPortAttributeRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvQueryPortAttributes(_) => Some(xv::QueryPortAttributesRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvListImageFormats(_) => Some(xv::ListImageFormatsRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvQueryImageAttributes(_) => Some(xv::QueryImageAttributesRequest::parse_reply),
            #[cfg(feature = "xv")]
            Request::XvPutImage(_) => None,
            #[cfg(feature = "xv")]
            Request::XvShmPutImage(_) => None,
            #[cfg(feature = "xvmc")]
            Request::XvmcQueryVersion(_) => Some(xvmc::QueryVersionRequest::parse_reply),
            #[cfg(feature = "xvmc")]
            Request::XvmcListSurfaceTypes(_) => Some(xvmc::ListSurfaceTypesRequest::parse_reply),
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateContext(_) => Some(xvmc::CreateContextRequest::parse_reply),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroyContext(_) => None,
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateSurface(_) => Some(xvmc::CreateSurfaceRequest::parse_reply),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroySurface(_) => None,
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateSubpicture(_) => Some(xvmc::CreateSubpictureRequest::parse_reply),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroySubpicture(_) => None,
            #[cfg(feature = "xvmc")]
            Request::XvmcListSubpictureTypes(_) => Some(xvmc::ListSubpictureTypesRequest::parse_reply),
        }
    }
    /// Convert this Request into an owned version with no borrows.
    pub fn into_owned(self) -> Request<'static> {
        match self {
            Request::Unknown(header, body) => Request::Unknown(header, Cow::Owned(body.into_owned())),
            Request::CreateWindow(req) => Request::CreateWindow(req.into_owned()),
            Request::ChangeWindowAttributes(req) => Request::ChangeWindowAttributes(req.into_owned()),
            Request::GetWindowAttributes(req) => Request::GetWindowAttributes(req),
            Request::DestroyWindow(req) => Request::DestroyWindow(req),
            Request::DestroySubwindows(req) => Request::DestroySubwindows(req),
            Request::ChangeSaveSet(req) => Request::ChangeSaveSet(req),
            Request::ReparentWindow(req) => Request::ReparentWindow(req),
            Request::MapWindow(req) => Request::MapWindow(req),
            Request::MapSubwindows(req) => Request::MapSubwindows(req),
            Request::UnmapWindow(req) => Request::UnmapWindow(req),
            Request::UnmapSubwindows(req) => Request::UnmapSubwindows(req),
            Request::ConfigureWindow(req) => Request::ConfigureWindow(req.into_owned()),
            Request::CirculateWindow(req) => Request::CirculateWindow(req),
            Request::GetGeometry(req) => Request::GetGeometry(req),
            Request::QueryTree(req) => Request::QueryTree(req),
            Request::InternAtom(req) => Request::InternAtom(req.into_owned()),
            Request::GetAtomName(req) => Request::GetAtomName(req),
            Request::ChangeProperty(req) => Request::ChangeProperty(req.into_owned()),
            Request::DeleteProperty(req) => Request::DeleteProperty(req),
            Request::GetProperty(req) => Request::GetProperty(req),
            Request::ListProperties(req) => Request::ListProperties(req),
            Request::SetSelectionOwner(req) => Request::SetSelectionOwner(req),
            Request::GetSelectionOwner(req) => Request::GetSelectionOwner(req),
            Request::ConvertSelection(req) => Request::ConvertSelection(req),
            Request::SendEvent(req) => Request::SendEvent(req.into_owned()),
            Request::GrabPointer(req) => Request::GrabPointer(req),
            Request::UngrabPointer(req) => Request::UngrabPointer(req),
            Request::GrabButton(req) => Request::GrabButton(req),
            Request::UngrabButton(req) => Request::UngrabButton(req),
            Request::ChangeActivePointerGrab(req) => Request::ChangeActivePointerGrab(req),
            Request::GrabKeyboard(req) => Request::GrabKeyboard(req),
            Request::UngrabKeyboard(req) => Request::UngrabKeyboard(req),
            Request::GrabKey(req) => Request::GrabKey(req),
            Request::UngrabKey(req) => Request::UngrabKey(req),
            Request::AllowEvents(req) => Request::AllowEvents(req),
            Request::GrabServer(req) => Request::GrabServer(req),
            Request::UngrabServer(req) => Request::UngrabServer(req),
            Request::QueryPointer(req) => Request::QueryPointer(req),
            Request::GetMotionEvents(req) => Request::GetMotionEvents(req),
            Request::TranslateCoordinates(req) => Request::TranslateCoordinates(req),
            Request::WarpPointer(req) => Request::WarpPointer(req),
            Request::SetInputFocus(req) => Request::SetInputFocus(req),
            Request::GetInputFocus(req) => Request::GetInputFocus(req),
            Request::QueryKeymap(req) => Request::QueryKeymap(req),
            Request::OpenFont(req) => Request::OpenFont(req.into_owned()),
            Request::CloseFont(req) => Request::CloseFont(req),
            Request::QueryFont(req) => Request::QueryFont(req),
            Request::QueryTextExtents(req) => Request::QueryTextExtents(req.into_owned()),
            Request::ListFonts(req) => Request::ListFonts(req.into_owned()),
            Request::ListFontsWithInfo(req) => Request::ListFontsWithInfo(req.into_owned()),
            Request::SetFontPath(req) => Request::SetFontPath(req.into_owned()),
            Request::GetFontPath(req) => Request::GetFontPath(req),
            Request::CreatePixmap(req) => Request::CreatePixmap(req),
            Request::FreePixmap(req) => Request::FreePixmap(req),
            Request::CreateGC(req) => Request::CreateGC(req.into_owned()),
            Request::ChangeGC(req) => Request::ChangeGC(req.into_owned()),
            Request::CopyGC(req) => Request::CopyGC(req),
            Request::SetDashes(req) => Request::SetDashes(req.into_owned()),
            Request::SetClipRectangles(req) => Request::SetClipRectangles(req.into_owned()),
            Request::FreeGC(req) => Request::FreeGC(req),
            Request::ClearArea(req) => Request::ClearArea(req),
            Request::CopyArea(req) => Request::CopyArea(req),
            Request::CopyPlane(req) => Request::CopyPlane(req),
            Request::PolyPoint(req) => Request::PolyPoint(req.into_owned()),
            Request::PolyLine(req) => Request::PolyLine(req.into_owned()),
            Request::PolySegment(req) => Request::PolySegment(req.into_owned()),
            Request::PolyRectangle(req) => Request::PolyRectangle(req.into_owned()),
            Request::PolyArc(req) => Request::PolyArc(req.into_owned()),
            Request::FillPoly(req) => Request::FillPoly(req.into_owned()),
            Request::PolyFillRectangle(req) => Request::PolyFillRectangle(req.into_owned()),
            Request::PolyFillArc(req) => Request::PolyFillArc(req.into_owned()),
            Request::PutImage(req) => Request::PutImage(req.into_owned()),
            Request::GetImage(req) => Request::GetImage(req),
            Request::PolyText8(req) => Request::PolyText8(req.into_owned()),
            Request::PolyText16(req) => Request::PolyText16(req.into_owned()),
            Request::ImageText8(req) => Request::ImageText8(req.into_owned()),
            Request::ImageText16(req) => Request::ImageText16(req.into_owned()),
            Request::CreateColormap(req) => Request::CreateColormap(req),
            Request::FreeColormap(req) => Request::FreeColormap(req),
            Request::CopyColormapAndFree(req) => Request::CopyColormapAndFree(req),
            Request::InstallColormap(req) => Request::InstallColormap(req),
            Request::UninstallColormap(req) => Request::UninstallColormap(req),
            Request::ListInstalledColormaps(req) => Request::ListInstalledColormaps(req),
            Request::AllocColor(req) => Request::AllocColor(req),
            Request::AllocNamedColor(req) => Request::AllocNamedColor(req.into_owned()),
            Request::AllocColorCells(req) => Request::AllocColorCells(req),
            Request::AllocColorPlanes(req) => Request::AllocColorPlanes(req),
            Request::FreeColors(req) => Request::FreeColors(req.into_owned()),
            Request::StoreColors(req) => Request::StoreColors(req.into_owned()),
            Request::StoreNamedColor(req) => Request::StoreNamedColor(req.into_owned()),
            Request::QueryColors(req) => Request::QueryColors(req.into_owned()),
            Request::LookupColor(req) => Request::LookupColor(req.into_owned()),
            Request::CreateCursor(req) => Request::CreateCursor(req),
            Request::CreateGlyphCursor(req) => Request::CreateGlyphCursor(req),
            Request::FreeCursor(req) => Request::FreeCursor(req),
            Request::RecolorCursor(req) => Request::RecolorCursor(req),
            Request::QueryBestSize(req) => Request::QueryBestSize(req),
            Request::QueryExtension(req) => Request::QueryExtension(req.into_owned()),
            Request::ListExtensions(req) => Request::ListExtensions(req),
            Request::ChangeKeyboardMapping(req) => Request::ChangeKeyboardMapping(req.into_owned()),
            Request::GetKeyboardMapping(req) => Request::GetKeyboardMapping(req),
            Request::ChangeKeyboardControl(req) => Request::ChangeKeyboardControl(req.into_owned()),
            Request::GetKeyboardControl(req) => Request::GetKeyboardControl(req),
            Request::Bell(req) => Request::Bell(req),
            Request::ChangePointerControl(req) => Request::ChangePointerControl(req),
            Request::GetPointerControl(req) => Request::GetPointerControl(req),
            Request::SetScreenSaver(req) => Request::SetScreenSaver(req),
            Request::GetScreenSaver(req) => Request::GetScreenSaver(req),
            Request::ChangeHosts(req) => Request::ChangeHosts(req.into_owned()),
            Request::ListHosts(req) => Request::ListHosts(req),
            Request::SetAccessControl(req) => Request::SetAccessControl(req),
            Request::SetCloseDownMode(req) => Request::SetCloseDownMode(req),
            Request::KillClient(req) => Request::KillClient(req),
            Request::RotateProperties(req) => Request::RotateProperties(req.into_owned()),
            Request::ForceScreenSaver(req) => Request::ForceScreenSaver(req),
            Request::SetPointerMapping(req) => Request::SetPointerMapping(req.into_owned()),
            Request::GetPointerMapping(req) => Request::GetPointerMapping(req),
            Request::SetModifierMapping(req) => Request::SetModifierMapping(req.into_owned()),
            Request::GetModifierMapping(req) => Request::GetModifierMapping(req),
            Request::NoOperation(req) => Request::NoOperation(req),
            Request::BigreqEnable(req) => Request::BigreqEnable(req),
            #[cfg(feature = "composite")]
            Request::CompositeQueryVersion(req) => Request::CompositeQueryVersion(req),
            #[cfg(feature = "composite")]
            Request::CompositeRedirectWindow(req) => Request::CompositeRedirectWindow(req),
            #[cfg(feature = "composite")]
            Request::CompositeRedirectSubwindows(req) => Request::CompositeRedirectSubwindows(req),
            #[cfg(feature = "composite")]
            Request::CompositeUnredirectWindow(req) => Request::CompositeUnredirectWindow(req),
            #[cfg(feature = "composite")]
            Request::CompositeUnredirectSubwindows(req) => Request::CompositeUnredirectSubwindows(req),
            #[cfg(feature = "composite")]
            Request::CompositeCreateRegionFromBorderClip(req) => Request::CompositeCreateRegionFromBorderClip(req),
            #[cfg(feature = "composite")]
            Request::CompositeNameWindowPixmap(req) => Request::CompositeNameWindowPixmap(req),
            #[cfg(feature = "composite")]
            Request::CompositeGetOverlayWindow(req) => Request::CompositeGetOverlayWindow(req),
            #[cfg(feature = "composite")]
            Request::CompositeReleaseOverlayWindow(req) => Request::CompositeReleaseOverlayWindow(req),
            #[cfg(feature = "damage")]
            Request::DamageQueryVersion(req) => Request::DamageQueryVersion(req),
            #[cfg(feature = "damage")]
            Request::DamageCreate(req) => Request::DamageCreate(req),
            #[cfg(feature = "damage")]
            Request::DamageDestroy(req) => Request::DamageDestroy(req),
            #[cfg(feature = "damage")]
            Request::DamageSubtract(req) => Request::DamageSubtract(req),
            #[cfg(feature = "damage")]
            Request::DamageAdd(req) => Request::DamageAdd(req),
            #[cfg(feature = "dpms")]
            Request::DpmsGetVersion(req) => Request::DpmsGetVersion(req),
            #[cfg(feature = "dpms")]
            Request::DpmsCapable(req) => Request::DpmsCapable(req),
            #[cfg(feature = "dpms")]
            Request::DpmsGetTimeouts(req) => Request::DpmsGetTimeouts(req),
            #[cfg(feature = "dpms")]
            Request::DpmsSetTimeouts(req) => Request::DpmsSetTimeouts(req),
            #[cfg(feature = "dpms")]
            Request::DpmsEnable(req) => Request::DpmsEnable(req),
            #[cfg(feature = "dpms")]
            Request::DpmsDisable(req) => Request::DpmsDisable(req),
            #[cfg(feature = "dpms")]
            Request::DpmsForceLevel(req) => Request::DpmsForceLevel(req),
            #[cfg(feature = "dpms")]
            Request::DpmsInfo(req) => Request::DpmsInfo(req),
            #[cfg(feature = "dri2")]
            Request::Dri2QueryVersion(req) => Request::Dri2QueryVersion(req),
            #[cfg(feature = "dri2")]
            Request::Dri2Connect(req) => Request::Dri2Connect(req),
            #[cfg(feature = "dri2")]
            Request::Dri2Authenticate(req) => Request::Dri2Authenticate(req),
            #[cfg(feature = "dri2")]
            Request::Dri2CreateDrawable(req) => Request::Dri2CreateDrawable(req),
            #[cfg(feature = "dri2")]
            Request::Dri2DestroyDrawable(req) => Request::Dri2DestroyDrawable(req),
            #[cfg(feature = "dri2")]
            Request::Dri2GetBuffers(req) => Request::Dri2GetBuffers(req.into_owned()),
            #[cfg(feature = "dri2")]
            Request::Dri2CopyRegion(req) => Request::Dri2CopyRegion(req),
            #[cfg(feature = "dri2")]
            Request::Dri2GetBuffersWithFormat(req) => Request::Dri2GetBuffersWithFormat(req.into_owned()),
            #[cfg(feature = "dri2")]
            Request::Dri2SwapBuffers(req) => Request::Dri2SwapBuffers(req),
            #[cfg(feature = "dri2")]
            Request::Dri2GetMSC(req) => Request::Dri2GetMSC(req),
            #[cfg(feature = "dri2")]
            Request::Dri2WaitMSC(req) => Request::Dri2WaitMSC(req),
            #[cfg(feature = "dri2")]
            Request::Dri2WaitSBC(req) => Request::Dri2WaitSBC(req),
            #[cfg(feature = "dri2")]
            Request::Dri2SwapInterval(req) => Request::Dri2SwapInterval(req),
            #[cfg(feature = "dri2")]
            Request::Dri2GetParam(req) => Request::Dri2GetParam(req),
            #[cfg(feature = "dri3")]
            Request::Dri3QueryVersion(req) => Request::Dri3QueryVersion(req),
            #[cfg(feature = "dri3")]
            Request::Dri3Open(req) => Request::Dri3Open(req),
            #[cfg(feature = "dri3")]
            Request::Dri3PixmapFromBuffer(req) => Request::Dri3PixmapFromBuffer(req),
            #[cfg(feature = "dri3")]
            Request::Dri3BufferFromPixmap(req) => Request::Dri3BufferFromPixmap(req),
            #[cfg(feature = "dri3")]
            Request::Dri3FenceFromFD(req) => Request::Dri3FenceFromFD(req),
            #[cfg(feature = "dri3")]
            Request::Dri3FDFromFence(req) => Request::Dri3FDFromFence(req),
            #[cfg(feature = "dri3")]
            Request::Dri3GetSupportedModifiers(req) => Request::Dri3GetSupportedModifiers(req),
            #[cfg(feature = "dri3")]
            Request::Dri3PixmapFromBuffers(req) => Request::Dri3PixmapFromBuffers(req),
            #[cfg(feature = "dri3")]
            Request::Dri3BuffersFromPixmap(req) => Request::Dri3BuffersFromPixmap(req),
            Request::GeQueryVersion(req) => Request::GeQueryVersion(req),
            #[cfg(feature = "glx")]
            Request::GlxRender(req) => Request::GlxRender(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxRenderLarge(req) => Request::GlxRenderLarge(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxCreateContext(req) => Request::GlxCreateContext(req),
            #[cfg(feature = "glx")]
            Request::GlxDestroyContext(req) => Request::GlxDestroyContext(req),
            #[cfg(feature = "glx")]
            Request::GlxMakeCurrent(req) => Request::GlxMakeCurrent(req),
            #[cfg(feature = "glx")]
            Request::GlxIsDirect(req) => Request::GlxIsDirect(req),
            #[cfg(feature = "glx")]
            Request::GlxQueryVersion(req) => Request::GlxQueryVersion(req),
            #[cfg(feature = "glx")]
            Request::GlxWaitGL(req) => Request::GlxWaitGL(req),
            #[cfg(feature = "glx")]
            Request::GlxWaitX(req) => Request::GlxWaitX(req),
            #[cfg(feature = "glx")]
            Request::GlxCopyContext(req) => Request::GlxCopyContext(req),
            #[cfg(feature = "glx")]
            Request::GlxSwapBuffers(req) => Request::GlxSwapBuffers(req),
            #[cfg(feature = "glx")]
            Request::GlxUseXFont(req) => Request::GlxUseXFont(req),
            #[cfg(feature = "glx")]
            Request::GlxCreateGLXPixmap(req) => Request::GlxCreateGLXPixmap(req),
            #[cfg(feature = "glx")]
            Request::GlxGetVisualConfigs(req) => Request::GlxGetVisualConfigs(req),
            #[cfg(feature = "glx")]
            Request::GlxDestroyGLXPixmap(req) => Request::GlxDestroyGLXPixmap(req),
            #[cfg(feature = "glx")]
            Request::GlxVendorPrivate(req) => Request::GlxVendorPrivate(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxVendorPrivateWithReply(req) => Request::GlxVendorPrivateWithReply(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxQueryExtensionsString(req) => Request::GlxQueryExtensionsString(req),
            #[cfg(feature = "glx")]
            Request::GlxQueryServerString(req) => Request::GlxQueryServerString(req),
            #[cfg(feature = "glx")]
            Request::GlxClientInfo(req) => Request::GlxClientInfo(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxGetFBConfigs(req) => Request::GlxGetFBConfigs(req),
            #[cfg(feature = "glx")]
            Request::GlxCreatePixmap(req) => Request::GlxCreatePixmap(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxDestroyPixmap(req) => Request::GlxDestroyPixmap(req),
            #[cfg(feature = "glx")]
            Request::GlxCreateNewContext(req) => Request::GlxCreateNewContext(req),
            #[cfg(feature = "glx")]
            Request::GlxQueryContext(req) => Request::GlxQueryContext(req),
            #[cfg(feature = "glx")]
            Request::GlxMakeContextCurrent(req) => Request::GlxMakeContextCurrent(req),
            #[cfg(feature = "glx")]
            Request::GlxCreatePbuffer(req) => Request::GlxCreatePbuffer(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxDestroyPbuffer(req) => Request::GlxDestroyPbuffer(req),
            #[cfg(feature = "glx")]
            Request::GlxGetDrawableAttributes(req) => Request::GlxGetDrawableAttributes(req),
            #[cfg(feature = "glx")]
            Request::GlxChangeDrawableAttributes(req) => Request::GlxChangeDrawableAttributes(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxCreateWindow(req) => Request::GlxCreateWindow(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxDeleteWindow(req) => Request::GlxDeleteWindow(req),
            #[cfg(feature = "glx")]
            Request::GlxSetClientInfoARB(req) => Request::GlxSetClientInfoARB(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxCreateContextAttribsARB(req) => Request::GlxCreateContextAttribsARB(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxSetClientInfo2ARB(req) => Request::GlxSetClientInfo2ARB(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxNewList(req) => Request::GlxNewList(req),
            #[cfg(feature = "glx")]
            Request::GlxEndList(req) => Request::GlxEndList(req),
            #[cfg(feature = "glx")]
            Request::GlxDeleteLists(req) => Request::GlxDeleteLists(req),
            #[cfg(feature = "glx")]
            Request::GlxGenLists(req) => Request::GlxGenLists(req),
            #[cfg(feature = "glx")]
            Request::GlxFeedbackBuffer(req) => Request::GlxFeedbackBuffer(req),
            #[cfg(feature = "glx")]
            Request::GlxSelectBuffer(req) => Request::GlxSelectBuffer(req),
            #[cfg(feature = "glx")]
            Request::GlxRenderMode(req) => Request::GlxRenderMode(req),
            #[cfg(feature = "glx")]
            Request::GlxFinish(req) => Request::GlxFinish(req),
            #[cfg(feature = "glx")]
            Request::GlxPixelStoref(req) => Request::GlxPixelStoref(req),
            #[cfg(feature = "glx")]
            Request::GlxPixelStorei(req) => Request::GlxPixelStorei(req),
            #[cfg(feature = "glx")]
            Request::GlxReadPixels(req) => Request::GlxReadPixels(req),
            #[cfg(feature = "glx")]
            Request::GlxGetBooleanv(req) => Request::GlxGetBooleanv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetClipPlane(req) => Request::GlxGetClipPlane(req),
            #[cfg(feature = "glx")]
            Request::GlxGetDoublev(req) => Request::GlxGetDoublev(req),
            #[cfg(feature = "glx")]
            Request::GlxGetError(req) => Request::GlxGetError(req),
            #[cfg(feature = "glx")]
            Request::GlxGetFloatv(req) => Request::GlxGetFloatv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetIntegerv(req) => Request::GlxGetIntegerv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetLightfv(req) => Request::GlxGetLightfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetLightiv(req) => Request::GlxGetLightiv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMapdv(req) => Request::GlxGetMapdv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMapfv(req) => Request::GlxGetMapfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMapiv(req) => Request::GlxGetMapiv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMaterialfv(req) => Request::GlxGetMaterialfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMaterialiv(req) => Request::GlxGetMaterialiv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapfv(req) => Request::GlxGetPixelMapfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapuiv(req) => Request::GlxGetPixelMapuiv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetPixelMapusv(req) => Request::GlxGetPixelMapusv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetPolygonStipple(req) => Request::GlxGetPolygonStipple(req),
            #[cfg(feature = "glx")]
            Request::GlxGetString(req) => Request::GlxGetString(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexEnvfv(req) => Request::GlxGetTexEnvfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexEnviv(req) => Request::GlxGetTexEnviv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGendv(req) => Request::GlxGetTexGendv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGenfv(req) => Request::GlxGetTexGenfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexGeniv(req) => Request::GlxGetTexGeniv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexImage(req) => Request::GlxGetTexImage(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexParameterfv(req) => Request::GlxGetTexParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexParameteriv(req) => Request::GlxGetTexParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexLevelParameterfv(req) => Request::GlxGetTexLevelParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetTexLevelParameteriv(req) => Request::GlxGetTexLevelParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxIsEnabled(req) => Request::GlxIsEnabled(req),
            #[cfg(feature = "glx")]
            Request::GlxIsList(req) => Request::GlxIsList(req),
            #[cfg(feature = "glx")]
            Request::GlxFlush(req) => Request::GlxFlush(req),
            #[cfg(feature = "glx")]
            Request::GlxAreTexturesResident(req) => Request::GlxAreTexturesResident(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxDeleteTextures(req) => Request::GlxDeleteTextures(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxGenTextures(req) => Request::GlxGenTextures(req),
            #[cfg(feature = "glx")]
            Request::GlxIsTexture(req) => Request::GlxIsTexture(req),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTable(req) => Request::GlxGetColorTable(req),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTableParameterfv(req) => Request::GlxGetColorTableParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetColorTableParameteriv(req) => Request::GlxGetColorTableParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionFilter(req) => Request::GlxGetConvolutionFilter(req),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionParameterfv(req) => Request::GlxGetConvolutionParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetConvolutionParameteriv(req) => Request::GlxGetConvolutionParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetSeparableFilter(req) => Request::GlxGetSeparableFilter(req),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogram(req) => Request::GlxGetHistogram(req),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogramParameterfv(req) => Request::GlxGetHistogramParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetHistogramParameteriv(req) => Request::GlxGetHistogramParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmax(req) => Request::GlxGetMinmax(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmaxParameterfv(req) => Request::GlxGetMinmaxParameterfv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetMinmaxParameteriv(req) => Request::GlxGetMinmaxParameteriv(req),
            #[cfg(feature = "glx")]
            Request::GlxGetCompressedTexImageARB(req) => Request::GlxGetCompressedTexImageARB(req),
            #[cfg(feature = "glx")]
            Request::GlxDeleteQueriesARB(req) => Request::GlxDeleteQueriesARB(req.into_owned()),
            #[cfg(feature = "glx")]
            Request::GlxGenQueriesARB(req) => Request::GlxGenQueriesARB(req),
            #[cfg(feature = "glx")]
            Request::GlxIsQueryARB(req) => Request::GlxIsQueryARB(req),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryivARB(req) => Request::GlxGetQueryivARB(req),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryObjectivARB(req) => Request::GlxGetQueryObjectivARB(req),
            #[cfg(feature = "glx")]
            Request::GlxGetQueryObjectuivARB(req) => Request::GlxGetQueryObjectuivARB(req),
            #[cfg(feature = "present")]
            Request::PresentQueryVersion(req) => Request::PresentQueryVersion(req),
            #[cfg(feature = "present")]
            Request::PresentPixmap(req) => Request::PresentPixmap(req.into_owned()),
            #[cfg(feature = "present")]
            Request::PresentNotifyMSC(req) => Request::PresentNotifyMSC(req),
            #[cfg(feature = "present")]
            Request::PresentSelectInput(req) => Request::PresentSelectInput(req),
            #[cfg(feature = "present")]
            Request::PresentQueryCapabilities(req) => Request::PresentQueryCapabilities(req),
            #[cfg(feature = "randr")]
            Request::RandrQueryVersion(req) => Request::RandrQueryVersion(req),
            #[cfg(feature = "randr")]
            Request::RandrSetScreenConfig(req) => Request::RandrSetScreenConfig(req),
            #[cfg(feature = "randr")]
            Request::RandrSelectInput(req) => Request::RandrSelectInput(req),
            #[cfg(feature = "randr")]
            Request::RandrGetScreenInfo(req) => Request::RandrGetScreenInfo(req),
            #[cfg(feature = "randr")]
            Request::RandrGetScreenSizeRange(req) => Request::RandrGetScreenSizeRange(req),
            #[cfg(feature = "randr")]
            Request::RandrSetScreenSize(req) => Request::RandrSetScreenSize(req),
            #[cfg(feature = "randr")]
            Request::RandrGetScreenResources(req) => Request::RandrGetScreenResources(req),
            #[cfg(feature = "randr")]
            Request::RandrGetOutputInfo(req) => Request::RandrGetOutputInfo(req),
            #[cfg(feature = "randr")]
            Request::RandrListOutputProperties(req) => Request::RandrListOutputProperties(req),
            #[cfg(feature = "randr")]
            Request::RandrQueryOutputProperty(req) => Request::RandrQueryOutputProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrConfigureOutputProperty(req) => Request::RandrConfigureOutputProperty(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrChangeOutputProperty(req) => Request::RandrChangeOutputProperty(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrDeleteOutputProperty(req) => Request::RandrDeleteOutputProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrGetOutputProperty(req) => Request::RandrGetOutputProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrCreateMode(req) => Request::RandrCreateMode(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrDestroyMode(req) => Request::RandrDestroyMode(req),
            #[cfg(feature = "randr")]
            Request::RandrAddOutputMode(req) => Request::RandrAddOutputMode(req),
            #[cfg(feature = "randr")]
            Request::RandrDeleteOutputMode(req) => Request::RandrDeleteOutputMode(req),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcInfo(req) => Request::RandrGetCrtcInfo(req),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcConfig(req) => Request::RandrSetCrtcConfig(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcGammaSize(req) => Request::RandrGetCrtcGammaSize(req),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcGamma(req) => Request::RandrGetCrtcGamma(req),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcGamma(req) => Request::RandrSetCrtcGamma(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrGetScreenResourcesCurrent(req) => Request::RandrGetScreenResourcesCurrent(req),
            #[cfg(feature = "randr")]
            Request::RandrSetCrtcTransform(req) => Request::RandrSetCrtcTransform(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrGetCrtcTransform(req) => Request::RandrGetCrtcTransform(req),
            #[cfg(feature = "randr")]
            Request::RandrGetPanning(req) => Request::RandrGetPanning(req),
            #[cfg(feature = "randr")]
            Request::RandrSetPanning(req) => Request::RandrSetPanning(req),
            #[cfg(feature = "randr")]
            Request::RandrSetOutputPrimary(req) => Request::RandrSetOutputPrimary(req),
            #[cfg(feature = "randr")]
            Request::RandrGetOutputPrimary(req) => Request::RandrGetOutputPrimary(req),
            #[cfg(feature = "randr")]
            Request::RandrGetProviders(req) => Request::RandrGetProviders(req),
            #[cfg(feature = "randr")]
            Request::RandrGetProviderInfo(req) => Request::RandrGetProviderInfo(req),
            #[cfg(feature = "randr")]
            Request::RandrSetProviderOffloadSink(req) => Request::RandrSetProviderOffloadSink(req),
            #[cfg(feature = "randr")]
            Request::RandrSetProviderOutputSource(req) => Request::RandrSetProviderOutputSource(req),
            #[cfg(feature = "randr")]
            Request::RandrListProviderProperties(req) => Request::RandrListProviderProperties(req),
            #[cfg(feature = "randr")]
            Request::RandrQueryProviderProperty(req) => Request::RandrQueryProviderProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrConfigureProviderProperty(req) => Request::RandrConfigureProviderProperty(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrChangeProviderProperty(req) => Request::RandrChangeProviderProperty(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrDeleteProviderProperty(req) => Request::RandrDeleteProviderProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrGetProviderProperty(req) => Request::RandrGetProviderProperty(req),
            #[cfg(feature = "randr")]
            Request::RandrGetMonitors(req) => Request::RandrGetMonitors(req),
            #[cfg(feature = "randr")]
            Request::RandrSetMonitor(req) => Request::RandrSetMonitor(req),
            #[cfg(feature = "randr")]
            Request::RandrDeleteMonitor(req) => Request::RandrDeleteMonitor(req),
            #[cfg(feature = "randr")]
            Request::RandrCreateLease(req) => Request::RandrCreateLease(req.into_owned()),
            #[cfg(feature = "randr")]
            Request::RandrFreeLease(req) => Request::RandrFreeLease(req),
            #[cfg(feature = "record")]
            Request::RecordQueryVersion(req) => Request::RecordQueryVersion(req),
            #[cfg(feature = "record")]
            Request::RecordCreateContext(req) => Request::RecordCreateContext(req.into_owned()),
            #[cfg(feature = "record")]
            Request::RecordRegisterClients(req) => Request::RecordRegisterClients(req.into_owned()),
            #[cfg(feature = "record")]
            Request::RecordUnregisterClients(req) => Request::RecordUnregisterClients(req.into_owned()),
            #[cfg(feature = "record")]
            Request::RecordGetContext(req) => Request::RecordGetContext(req),
            #[cfg(feature = "record")]
            Request::RecordEnableContext(req) => Request::RecordEnableContext(req),
            #[cfg(feature = "record")]
            Request::RecordDisableContext(req) => Request::RecordDisableContext(req),
            #[cfg(feature = "record")]
            Request::RecordFreeContext(req) => Request::RecordFreeContext(req),
            #[cfg(feature = "render")]
            Request::RenderQueryVersion(req) => Request::RenderQueryVersion(req),
            #[cfg(feature = "render")]
            Request::RenderQueryPictFormats(req) => Request::RenderQueryPictFormats(req),
            #[cfg(feature = "render")]
            Request::RenderQueryPictIndexValues(req) => Request::RenderQueryPictIndexValues(req),
            #[cfg(feature = "render")]
            Request::RenderCreatePicture(req) => Request::RenderCreatePicture(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderChangePicture(req) => Request::RenderChangePicture(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderSetPictureClipRectangles(req) => Request::RenderSetPictureClipRectangles(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderFreePicture(req) => Request::RenderFreePicture(req),
            #[cfg(feature = "render")]
            Request::RenderComposite(req) => Request::RenderComposite(req),
            #[cfg(feature = "render")]
            Request::RenderTrapezoids(req) => Request::RenderTrapezoids(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderTriangles(req) => Request::RenderTriangles(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderTriStrip(req) => Request::RenderTriStrip(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderTriFan(req) => Request::RenderTriFan(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateGlyphSet(req) => Request::RenderCreateGlyphSet(req),
            #[cfg(feature = "render")]
            Request::RenderReferenceGlyphSet(req) => Request::RenderReferenceGlyphSet(req),
            #[cfg(feature = "render")]
            Request::RenderFreeGlyphSet(req) => Request::RenderFreeGlyphSet(req),
            #[cfg(feature = "render")]
            Request::RenderAddGlyphs(req) => Request::RenderAddGlyphs(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderFreeGlyphs(req) => Request::RenderFreeGlyphs(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs8(req) => Request::RenderCompositeGlyphs8(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs16(req) => Request::RenderCompositeGlyphs16(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCompositeGlyphs32(req) => Request::RenderCompositeGlyphs32(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderFillRectangles(req) => Request::RenderFillRectangles(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateCursor(req) => Request::RenderCreateCursor(req),
            #[cfg(feature = "render")]
            Request::RenderSetPictureTransform(req) => Request::RenderSetPictureTransform(req),
            #[cfg(feature = "render")]
            Request::RenderQueryFilters(req) => Request::RenderQueryFilters(req),
            #[cfg(feature = "render")]
            Request::RenderSetPictureFilter(req) => Request::RenderSetPictureFilter(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateAnimCursor(req) => Request::RenderCreateAnimCursor(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderAddTraps(req) => Request::RenderAddTraps(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateSolidFill(req) => Request::RenderCreateSolidFill(req),
            #[cfg(feature = "render")]
            Request::RenderCreateLinearGradient(req) => Request::RenderCreateLinearGradient(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateRadialGradient(req) => Request::RenderCreateRadialGradient(req.into_owned()),
            #[cfg(feature = "render")]
            Request::RenderCreateConicalGradient(req) => Request::RenderCreateConicalGradient(req.into_owned()),
            #[cfg(feature = "res")]
            Request::ResQueryVersion(req) => Request::ResQueryVersion(req),
            #[cfg(feature = "res")]
            Request::ResQueryClients(req) => Request::ResQueryClients(req),
            #[cfg(feature = "res")]
            Request::ResQueryClientResources(req) => Request::ResQueryClientResources(req),
            #[cfg(feature = "res")]
            Request::ResQueryClientPixmapBytes(req) => Request::ResQueryClientPixmapBytes(req),
            #[cfg(feature = "res")]
            Request::ResQueryClientIds(req) => Request::ResQueryClientIds(req.into_owned()),
            #[cfg(feature = "res")]
            Request::ResQueryResourceBytes(req) => Request::ResQueryResourceBytes(req.into_owned()),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverQueryVersion(req) => Request::ScreensaverQueryVersion(req),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverQueryInfo(req) => Request::ScreensaverQueryInfo(req),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSelectInput(req) => Request::ScreensaverSelectInput(req),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSetAttributes(req) => Request::ScreensaverSetAttributes(req.into_owned()),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverUnsetAttributes(req) => Request::ScreensaverUnsetAttributes(req),
            #[cfg(feature = "screensaver")]
            Request::ScreensaverSuspend(req) => Request::ScreensaverSuspend(req),
            #[cfg(feature = "shape")]
            Request::ShapeQueryVersion(req) => Request::ShapeQueryVersion(req),
            #[cfg(feature = "shape")]
            Request::ShapeRectangles(req) => Request::ShapeRectangles(req.into_owned()),
            #[cfg(feature = "shape")]
            Request::ShapeMask(req) => Request::ShapeMask(req),
            #[cfg(feature = "shape")]
            Request::ShapeCombine(req) => Request::ShapeCombine(req),
            #[cfg(feature = "shape")]
            Request::ShapeOffset(req) => Request::ShapeOffset(req),
            #[cfg(feature = "shape")]
            Request::ShapeQueryExtents(req) => Request::ShapeQueryExtents(req),
            #[cfg(feature = "shape")]
            Request::ShapeSelectInput(req) => Request::ShapeSelectInput(req),
            #[cfg(feature = "shape")]
            Request::ShapeInputSelected(req) => Request::ShapeInputSelected(req),
            #[cfg(feature = "shape")]
            Request::ShapeGetRectangles(req) => Request::ShapeGetRectangles(req),
            #[cfg(feature = "shm")]
            Request::ShmQueryVersion(req) => Request::ShmQueryVersion(req),
            #[cfg(feature = "shm")]
            Request::ShmAttach(req) => Request::ShmAttach(req),
            #[cfg(feature = "shm")]
            Request::ShmDetach(req) => Request::ShmDetach(req),
            #[cfg(feature = "shm")]
            Request::ShmPutImage(req) => Request::ShmPutImage(req),
            #[cfg(feature = "shm")]
            Request::ShmGetImage(req) => Request::ShmGetImage(req),
            #[cfg(feature = "shm")]
            Request::ShmCreatePixmap(req) => Request::ShmCreatePixmap(req),
            #[cfg(feature = "shm")]
            Request::ShmAttachFd(req) => Request::ShmAttachFd(req),
            #[cfg(feature = "shm")]
            Request::ShmCreateSegment(req) => Request::ShmCreateSegment(req),
            #[cfg(feature = "sync")]
            Request::SyncInitialize(req) => Request::SyncInitialize(req),
            #[cfg(feature = "sync")]
            Request::SyncListSystemCounters(req) => Request::SyncListSystemCounters(req),
            #[cfg(feature = "sync")]
            Request::SyncCreateCounter(req) => Request::SyncCreateCounter(req),
            #[cfg(feature = "sync")]
            Request::SyncDestroyCounter(req) => Request::SyncDestroyCounter(req),
            #[cfg(feature = "sync")]
            Request::SyncQueryCounter(req) => Request::SyncQueryCounter(req),
            #[cfg(feature = "sync")]
            Request::SyncAwait(req) => Request::SyncAwait(req.into_owned()),
            #[cfg(feature = "sync")]
            Request::SyncChangeCounter(req) => Request::SyncChangeCounter(req),
            #[cfg(feature = "sync")]
            Request::SyncSetCounter(req) => Request::SyncSetCounter(req),
            #[cfg(feature = "sync")]
            Request::SyncCreateAlarm(req) => Request::SyncCreateAlarm(req.into_owned()),
            #[cfg(feature = "sync")]
            Request::SyncChangeAlarm(req) => Request::SyncChangeAlarm(req.into_owned()),
            #[cfg(feature = "sync")]
            Request::SyncDestroyAlarm(req) => Request::SyncDestroyAlarm(req),
            #[cfg(feature = "sync")]
            Request::SyncQueryAlarm(req) => Request::SyncQueryAlarm(req),
            #[cfg(feature = "sync")]
            Request::SyncSetPriority(req) => Request::SyncSetPriority(req),
            #[cfg(feature = "sync")]
            Request::SyncGetPriority(req) => Request::SyncGetPriority(req),
            #[cfg(feature = "sync")]
            Request::SyncCreateFence(req) => Request::SyncCreateFence(req),
            #[cfg(feature = "sync")]
            Request::SyncTriggerFence(req) => Request::SyncTriggerFence(req),
            #[cfg(feature = "sync")]
            Request::SyncResetFence(req) => Request::SyncResetFence(req),
            #[cfg(feature = "sync")]
            Request::SyncDestroyFence(req) => Request::SyncDestroyFence(req),
            #[cfg(feature = "sync")]
            Request::SyncQueryFence(req) => Request::SyncQueryFence(req),
            #[cfg(feature = "sync")]
            Request::SyncAwaitFence(req) => Request::SyncAwaitFence(req.into_owned()),
            Request::XcMiscGetVersion(req) => Request::XcMiscGetVersion(req),
            Request::XcMiscGetXIDRange(req) => Request::XcMiscGetXIDRange(req),
            Request::XcMiscGetXIDList(req) => Request::XcMiscGetXIDList(req),
            #[cfg(feature = "xevie")]
            Request::XevieQueryVersion(req) => Request::XevieQueryVersion(req),
            #[cfg(feature = "xevie")]
            Request::XevieStart(req) => Request::XevieStart(req),
            #[cfg(feature = "xevie")]
            Request::XevieEnd(req) => Request::XevieEnd(req),
            #[cfg(feature = "xevie")]
            Request::XevieSend(req) => Request::XevieSend(req),
            #[cfg(feature = "xevie")]
            Request::XevieSelectInput(req) => Request::XevieSelectInput(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driQueryVersion(req) => Request::Xf86driQueryVersion(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driQueryDirectRenderingCapable(req) => Request::Xf86driQueryDirectRenderingCapable(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driOpenConnection(req) => Request::Xf86driOpenConnection(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCloseConnection(req) => Request::Xf86driCloseConnection(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetClientDriverName(req) => Request::Xf86driGetClientDriverName(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCreateContext(req) => Request::Xf86driCreateContext(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driDestroyContext(req) => Request::Xf86driDestroyContext(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driCreateDrawable(req) => Request::Xf86driCreateDrawable(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driDestroyDrawable(req) => Request::Xf86driDestroyDrawable(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetDrawableInfo(req) => Request::Xf86driGetDrawableInfo(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driGetDeviceInfo(req) => Request::Xf86driGetDeviceInfo(req),
            #[cfg(feature = "xf86dri")]
            Request::Xf86driAuthConnection(req) => Request::Xf86driAuthConnection(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeQueryVersion(req) => Request::Xf86vidmodeQueryVersion(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetModeLine(req) => Request::Xf86vidmodeGetModeLine(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeModModeLine(req) => Request::Xf86vidmodeModModeLine(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSwitchMode(req) => Request::Xf86vidmodeSwitchMode(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetMonitor(req) => Request::Xf86vidmodeGetMonitor(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeLockModeSwitch(req) => Request::Xf86vidmodeLockModeSwitch(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetAllModeLines(req) => Request::Xf86vidmodeGetAllModeLines(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeAddModeLine(req) => Request::Xf86vidmodeAddModeLine(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeDeleteModeLine(req) => Request::Xf86vidmodeDeleteModeLine(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeValidateModeLine(req) => Request::Xf86vidmodeValidateModeLine(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSwitchToMode(req) => Request::Xf86vidmodeSwitchToMode(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetViewPort(req) => Request::Xf86vidmodeGetViewPort(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetViewPort(req) => Request::Xf86vidmodeSetViewPort(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetDotClocks(req) => Request::Xf86vidmodeGetDotClocks(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetClientVersion(req) => Request::Xf86vidmodeSetClientVersion(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetGamma(req) => Request::Xf86vidmodeSetGamma(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGamma(req) => Request::Xf86vidmodeGetGamma(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGammaRamp(req) => Request::Xf86vidmodeGetGammaRamp(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeSetGammaRamp(req) => Request::Xf86vidmodeSetGammaRamp(req.into_owned()),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetGammaRampSize(req) => Request::Xf86vidmodeGetGammaRampSize(req),
            #[cfg(feature = "xf86vidmode")]
            Request::Xf86vidmodeGetPermissions(req) => Request::Xf86vidmodeGetPermissions(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesQueryVersion(req) => Request::XfixesQueryVersion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeSaveSet(req) => Request::XfixesChangeSaveSet(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSelectSelectionInput(req) => Request::XfixesSelectSelectionInput(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSelectCursorInput(req) => Request::XfixesSelectCursorInput(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorImage(req) => Request::XfixesGetCursorImage(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegion(req) => Request::XfixesCreateRegion(req.into_owned()),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromBitmap(req) => Request::XfixesCreateRegionFromBitmap(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromWindow(req) => Request::XfixesCreateRegionFromWindow(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromGC(req) => Request::XfixesCreateRegionFromGC(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreateRegionFromPicture(req) => Request::XfixesCreateRegionFromPicture(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesDestroyRegion(req) => Request::XfixesDestroyRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetRegion(req) => Request::XfixesSetRegion(req.into_owned()),
            #[cfg(feature = "xfixes")]
            Request::XfixesCopyRegion(req) => Request::XfixesCopyRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesUnionRegion(req) => Request::XfixesUnionRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesIntersectRegion(req) => Request::XfixesIntersectRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSubtractRegion(req) => Request::XfixesSubtractRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesInvertRegion(req) => Request::XfixesInvertRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesTranslateRegion(req) => Request::XfixesTranslateRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesRegionExtents(req) => Request::XfixesRegionExtents(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesFetchRegion(req) => Request::XfixesFetchRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetGCClipRegion(req) => Request::XfixesSetGCClipRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetWindowShapeRegion(req) => Request::XfixesSetWindowShapeRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetPictureClipRegion(req) => Request::XfixesSetPictureClipRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesSetCursorName(req) => Request::XfixesSetCursorName(req.into_owned()),
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorName(req) => Request::XfixesGetCursorName(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesGetCursorImageAndName(req) => Request::XfixesGetCursorImageAndName(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeCursor(req) => Request::XfixesChangeCursor(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesChangeCursorByName(req) => Request::XfixesChangeCursorByName(req.into_owned()),
            #[cfg(feature = "xfixes")]
            Request::XfixesExpandRegion(req) => Request::XfixesExpandRegion(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesHideCursor(req) => Request::XfixesHideCursor(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesShowCursor(req) => Request::XfixesShowCursor(req),
            #[cfg(feature = "xfixes")]
            Request::XfixesCreatePointerBarrier(req) => Request::XfixesCreatePointerBarrier(req.into_owned()),
            #[cfg(feature = "xfixes")]
            Request::XfixesDeletePointerBarrier(req) => Request::XfixesDeletePointerBarrier(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaQueryVersion(req) => Request::XineramaQueryVersion(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetState(req) => Request::XineramaGetState(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetScreenCount(req) => Request::XineramaGetScreenCount(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaGetScreenSize(req) => Request::XineramaGetScreenSize(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaIsActive(req) => Request::XineramaIsActive(req),
            #[cfg(feature = "xinerama")]
            Request::XineramaQueryScreens(req) => Request::XineramaQueryScreens(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetExtensionVersion(req) => Request::XinputGetExtensionVersion(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputListInputDevices(req) => Request::XinputListInputDevices(req),
            #[cfg(feature = "xinput")]
            Request::XinputOpenDevice(req) => Request::XinputOpenDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputCloseDevice(req) => Request::XinputCloseDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceMode(req) => Request::XinputSetDeviceMode(req),
            #[cfg(feature = "xinput")]
            Request::XinputSelectExtensionEvent(req) => Request::XinputSelectExtensionEvent(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputGetSelectedExtensionEvents(req) => Request::XinputGetSelectedExtensionEvents(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceDontPropagateList(req) => Request::XinputChangeDeviceDontPropagateList(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceDontPropagateList(req) => Request::XinputGetDeviceDontPropagateList(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceMotionEvents(req) => Request::XinputGetDeviceMotionEvents(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeKeyboardDevice(req) => Request::XinputChangeKeyboardDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangePointerDevice(req) => Request::XinputChangePointerDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputGrabDevice(req) => Request::XinputGrabDevice(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDevice(req) => Request::XinputUngrabDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputGrabDeviceKey(req) => Request::XinputGrabDeviceKey(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDeviceKey(req) => Request::XinputUngrabDeviceKey(req),
            #[cfg(feature = "xinput")]
            Request::XinputGrabDeviceButton(req) => Request::XinputGrabDeviceButton(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputUngrabDeviceButton(req) => Request::XinputUngrabDeviceButton(req),
            #[cfg(feature = "xinput")]
            Request::XinputAllowDeviceEvents(req) => Request::XinputAllowDeviceEvents(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceFocus(req) => Request::XinputGetDeviceFocus(req),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceFocus(req) => Request::XinputSetDeviceFocus(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetFeedbackControl(req) => Request::XinputGetFeedbackControl(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeFeedbackControl(req) => Request::XinputChangeFeedbackControl(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceKeyMapping(req) => Request::XinputGetDeviceKeyMapping(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceKeyMapping(req) => Request::XinputChangeDeviceKeyMapping(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceModifierMapping(req) => Request::XinputGetDeviceModifierMapping(req),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceModifierMapping(req) => Request::XinputSetDeviceModifierMapping(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceButtonMapping(req) => Request::XinputGetDeviceButtonMapping(req),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceButtonMapping(req) => Request::XinputSetDeviceButtonMapping(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputQueryDeviceState(req) => Request::XinputQueryDeviceState(req),
            #[cfg(feature = "xinput")]
            Request::XinputDeviceBell(req) => Request::XinputDeviceBell(req),
            #[cfg(feature = "xinput")]
            Request::XinputSetDeviceValuators(req) => Request::XinputSetDeviceValuators(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceControl(req) => Request::XinputGetDeviceControl(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceControl(req) => Request::XinputChangeDeviceControl(req),
            #[cfg(feature = "xinput")]
            Request::XinputListDeviceProperties(req) => Request::XinputListDeviceProperties(req),
            #[cfg(feature = "xinput")]
            Request::XinputChangeDeviceProperty(req) => Request::XinputChangeDeviceProperty(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputDeleteDeviceProperty(req) => Request::XinputDeleteDeviceProperty(req),
            #[cfg(feature = "xinput")]
            Request::XinputGetDeviceProperty(req) => Request::XinputGetDeviceProperty(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryPointer(req) => Request::XinputXIQueryPointer(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIWarpPointer(req) => Request::XinputXIWarpPointer(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeCursor(req) => Request::XinputXIChangeCursor(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeHierarchy(req) => Request::XinputXIChangeHierarchy(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXISetClientPointer(req) => Request::XinputXISetClientPointer(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIGetClientPointer(req) => Request::XinputXIGetClientPointer(req),
            #[cfg(feature = "xinput")]
            Request::XinputXISelectEvents(req) => Request::XinputXISelectEvents(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryVersion(req) => Request::XinputXIQueryVersion(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIQueryDevice(req) => Request::XinputXIQueryDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputXISetFocus(req) => Request::XinputXISetFocus(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIGetFocus(req) => Request::XinputXIGetFocus(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIGrabDevice(req) => Request::XinputXIGrabDevice(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXIUngrabDevice(req) => Request::XinputXIUngrabDevice(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIAllowEvents(req) => Request::XinputXIAllowEvents(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIPassiveGrabDevice(req) => Request::XinputXIPassiveGrabDevice(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXIPassiveUngrabDevice(req) => Request::XinputXIPassiveUngrabDevice(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXIListProperties(req) => Request::XinputXIListProperties(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIChangeProperty(req) => Request::XinputXIChangeProperty(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputXIDeleteProperty(req) => Request::XinputXIDeleteProperty(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIGetProperty(req) => Request::XinputXIGetProperty(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIGetSelectedEvents(req) => Request::XinputXIGetSelectedEvents(req),
            #[cfg(feature = "xinput")]
            Request::XinputXIBarrierReleasePointer(req) => Request::XinputXIBarrierReleasePointer(req.into_owned()),
            #[cfg(feature = "xinput")]
            Request::XinputSendExtensionEvent(req) => Request::XinputSendExtensionEvent(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbUseExtension(req) => Request::XkbUseExtension(req),
            #[cfg(feature = "xkb")]
            Request::XkbSelectEvents(req) => Request::XkbSelectEvents(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbBell(req) => Request::XkbBell(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetState(req) => Request::XkbGetState(req),
            #[cfg(feature = "xkb")]
            Request::XkbLatchLockState(req) => Request::XkbLatchLockState(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetControls(req) => Request::XkbGetControls(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetControls(req) => Request::XkbSetControls(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbGetMap(req) => Request::XkbGetMap(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetMap(req) => Request::XkbSetMap(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbGetCompatMap(req) => Request::XkbGetCompatMap(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetCompatMap(req) => Request::XkbSetCompatMap(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbGetIndicatorState(req) => Request::XkbGetIndicatorState(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetIndicatorMap(req) => Request::XkbGetIndicatorMap(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetIndicatorMap(req) => Request::XkbSetIndicatorMap(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbGetNamedIndicator(req) => Request::XkbGetNamedIndicator(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetNamedIndicator(req) => Request::XkbSetNamedIndicator(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetNames(req) => Request::XkbGetNames(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetNames(req) => Request::XkbSetNames(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbPerClientFlags(req) => Request::XkbPerClientFlags(req),
            #[cfg(feature = "xkb")]
            Request::XkbListComponents(req) => Request::XkbListComponents(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetKbdByName(req) => Request::XkbGetKbdByName(req),
            #[cfg(feature = "xkb")]
            Request::XkbGetDeviceInfo(req) => Request::XkbGetDeviceInfo(req),
            #[cfg(feature = "xkb")]
            Request::XkbSetDeviceInfo(req) => Request::XkbSetDeviceInfo(req.into_owned()),
            #[cfg(feature = "xkb")]
            Request::XkbSetDebuggingFlags(req) => Request::XkbSetDebuggingFlags(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintQueryVersion(req) => Request::XprintPrintQueryVersion(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetPrinterList(req) => Request::XprintPrintGetPrinterList(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintRehashPrinterList(req) => Request::XprintPrintRehashPrinterList(req),
            #[cfg(feature = "xprint")]
            Request::XprintCreateContext(req) => Request::XprintCreateContext(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetContext(req) => Request::XprintPrintSetContext(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetContext(req) => Request::XprintPrintGetContext(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintDestroyContext(req) => Request::XprintPrintDestroyContext(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetScreenOfContext(req) => Request::XprintPrintGetScreenOfContext(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartJob(req) => Request::XprintPrintStartJob(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndJob(req) => Request::XprintPrintEndJob(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartDoc(req) => Request::XprintPrintStartDoc(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndDoc(req) => Request::XprintPrintEndDoc(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintPutDocumentData(req) => Request::XprintPrintPutDocumentData(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetDocumentData(req) => Request::XprintPrintGetDocumentData(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintStartPage(req) => Request::XprintPrintStartPage(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintEndPage(req) => Request::XprintPrintEndPage(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSelectInput(req) => Request::XprintPrintSelectInput(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintInputSelected(req) => Request::XprintPrintInputSelected(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetAttributes(req) => Request::XprintPrintGetAttributes(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetOneAttributes(req) => Request::XprintPrintGetOneAttributes(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetAttributes(req) => Request::XprintPrintSetAttributes(req.into_owned()),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetPageDimensions(req) => Request::XprintPrintGetPageDimensions(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintQueryScreens(req) => Request::XprintPrintQueryScreens(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintSetImageResolution(req) => Request::XprintPrintSetImageResolution(req),
            #[cfg(feature = "xprint")]
            Request::XprintPrintGetImageResolution(req) => Request::XprintPrintGetImageResolution(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxQueryVersion(req) => Request::XselinuxQueryVersion(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetDeviceCreateContext(req) => Request::XselinuxSetDeviceCreateContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetDeviceCreateContext(req) => Request::XselinuxGetDeviceCreateContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetDeviceContext(req) => Request::XselinuxSetDeviceContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetDeviceContext(req) => Request::XselinuxGetDeviceContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetWindowCreateContext(req) => Request::XselinuxSetWindowCreateContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetWindowCreateContext(req) => Request::XselinuxGetWindowCreateContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetWindowContext(req) => Request::XselinuxGetWindowContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetPropertyCreateContext(req) => Request::XselinuxSetPropertyCreateContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyCreateContext(req) => Request::XselinuxGetPropertyCreateContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetPropertyUseContext(req) => Request::XselinuxSetPropertyUseContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyUseContext(req) => Request::XselinuxGetPropertyUseContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyContext(req) => Request::XselinuxGetPropertyContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetPropertyDataContext(req) => Request::XselinuxGetPropertyDataContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxListProperties(req) => Request::XselinuxListProperties(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetSelectionCreateContext(req) => Request::XselinuxSetSelectionCreateContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionCreateContext(req) => Request::XselinuxGetSelectionCreateContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxSetSelectionUseContext(req) => Request::XselinuxSetSelectionUseContext(req.into_owned()),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionUseContext(req) => Request::XselinuxGetSelectionUseContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionContext(req) => Request::XselinuxGetSelectionContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetSelectionDataContext(req) => Request::XselinuxGetSelectionDataContext(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxListSelections(req) => Request::XselinuxListSelections(req),
            #[cfg(feature = "xselinux")]
            Request::XselinuxGetClientContext(req) => Request::XselinuxGetClientContext(req),
            #[cfg(feature = "xtest")]
            Request::XtestGetVersion(req) => Request::XtestGetVersion(req),
            #[cfg(feature = "xtest")]
            Request::XtestCompareCursor(req) => Request::XtestCompareCursor(req),
            #[cfg(feature = "xtest")]
            Request::XtestFakeInput(req) => Request::XtestFakeInput(req),
            #[cfg(feature = "xtest")]
            Request::XtestGrabControl(req) => Request::XtestGrabControl(req),
            #[cfg(feature = "xv")]
            Request::XvQueryExtension(req) => Request::XvQueryExtension(req),
            #[cfg(feature = "xv")]
            Request::XvQueryAdaptors(req) => Request::XvQueryAdaptors(req),
            #[cfg(feature = "xv")]
            Request::XvQueryEncodings(req) => Request::XvQueryEncodings(req),
            #[cfg(feature = "xv")]
            Request::XvGrabPort(req) => Request::XvGrabPort(req),
            #[cfg(feature = "xv")]
            Request::XvUngrabPort(req) => Request::XvUngrabPort(req),
            #[cfg(feature = "xv")]
            Request::XvPutVideo(req) => Request::XvPutVideo(req),
            #[cfg(feature = "xv")]
            Request::XvPutStill(req) => Request::XvPutStill(req),
            #[cfg(feature = "xv")]
            Request::XvGetVideo(req) => Request::XvGetVideo(req),
            #[cfg(feature = "xv")]
            Request::XvGetStill(req) => Request::XvGetStill(req),
            #[cfg(feature = "xv")]
            Request::XvStopVideo(req) => Request::XvStopVideo(req),
            #[cfg(feature = "xv")]
            Request::XvSelectVideoNotify(req) => Request::XvSelectVideoNotify(req),
            #[cfg(feature = "xv")]
            Request::XvSelectPortNotify(req) => Request::XvSelectPortNotify(req),
            #[cfg(feature = "xv")]
            Request::XvQueryBestSize(req) => Request::XvQueryBestSize(req),
            #[cfg(feature = "xv")]
            Request::XvSetPortAttribute(req) => Request::XvSetPortAttribute(req),
            #[cfg(feature = "xv")]
            Request::XvGetPortAttribute(req) => Request::XvGetPortAttribute(req),
            #[cfg(feature = "xv")]
            Request::XvQueryPortAttributes(req) => Request::XvQueryPortAttributes(req),
            #[cfg(feature = "xv")]
            Request::XvListImageFormats(req) => Request::XvListImageFormats(req),
            #[cfg(feature = "xv")]
            Request::XvQueryImageAttributes(req) => Request::XvQueryImageAttributes(req),
            #[cfg(feature = "xv")]
            Request::XvPutImage(req) => Request::XvPutImage(req.into_owned()),
            #[cfg(feature = "xv")]
            Request::XvShmPutImage(req) => Request::XvShmPutImage(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcQueryVersion(req) => Request::XvmcQueryVersion(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcListSurfaceTypes(req) => Request::XvmcListSurfaceTypes(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateContext(req) => Request::XvmcCreateContext(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroyContext(req) => Request::XvmcDestroyContext(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateSurface(req) => Request::XvmcCreateSurface(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroySurface(req) => Request::XvmcDestroySurface(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcCreateSubpicture(req) => Request::XvmcCreateSubpicture(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcDestroySubpicture(req) => Request::XvmcDestroySubpicture(req),
            #[cfg(feature = "xvmc")]
            Request::XvmcListSubpictureTypes(req) => Request::XvmcListSubpictureTypes(req),
        }
    }
}

/// Enumeration of all possible X11 replies.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Reply {
    Void,
    GetWindowAttributes(xproto::GetWindowAttributesReply),
    GetGeometry(xproto::GetGeometryReply),
    QueryTree(xproto::QueryTreeReply),
    InternAtom(xproto::InternAtomReply),
    GetAtomName(xproto::GetAtomNameReply),
    GetProperty(xproto::GetPropertyReply),
    ListProperties(xproto::ListPropertiesReply),
    GetSelectionOwner(xproto::GetSelectionOwnerReply),
    GrabPointer(xproto::GrabPointerReply),
    GrabKeyboard(xproto::GrabKeyboardReply),
    QueryPointer(xproto::QueryPointerReply),
    GetMotionEvents(xproto::GetMotionEventsReply),
    TranslateCoordinates(xproto::TranslateCoordinatesReply),
    GetInputFocus(xproto::GetInputFocusReply),
    QueryKeymap(xproto::QueryKeymapReply),
    QueryFont(xproto::QueryFontReply),
    QueryTextExtents(xproto::QueryTextExtentsReply),
    ListFonts(xproto::ListFontsReply),
    ListFontsWithInfo(xproto::ListFontsWithInfoReply),
    GetFontPath(xproto::GetFontPathReply),
    GetImage(xproto::GetImageReply),
    ListInstalledColormaps(xproto::ListInstalledColormapsReply),
    AllocColor(xproto::AllocColorReply),
    AllocNamedColor(xproto::AllocNamedColorReply),
    AllocColorCells(xproto::AllocColorCellsReply),
    AllocColorPlanes(xproto::AllocColorPlanesReply),
    QueryColors(xproto::QueryColorsReply),
    LookupColor(xproto::LookupColorReply),
    QueryBestSize(xproto::QueryBestSizeReply),
    QueryExtension(xproto::QueryExtensionReply),
    ListExtensions(xproto::ListExtensionsReply),
    GetKeyboardMapping(xproto::GetKeyboardMappingReply),
    GetKeyboardControl(xproto::GetKeyboardControlReply),
    GetPointerControl(xproto::GetPointerControlReply),
    GetScreenSaver(xproto::GetScreenSaverReply),
    ListHosts(xproto::ListHostsReply),
    SetPointerMapping(xproto::SetPointerMappingReply),
    GetPointerMapping(xproto::GetPointerMappingReply),
    SetModifierMapping(xproto::SetModifierMappingReply),
    GetModifierMapping(xproto::GetModifierMappingReply),
    BigreqEnable(bigreq::EnableReply),
    #[cfg(feature = "composite")]
    CompositeQueryVersion(composite::QueryVersionReply),
    #[cfg(feature = "composite")]
    CompositeGetOverlayWindow(composite::GetOverlayWindowReply),
    #[cfg(feature = "damage")]
    DamageQueryVersion(damage::QueryVersionReply),
    #[cfg(feature = "dpms")]
    DpmsGetVersion(dpms::GetVersionReply),
    #[cfg(feature = "dpms")]
    DpmsCapable(dpms::CapableReply),
    #[cfg(feature = "dpms")]
    DpmsGetTimeouts(dpms::GetTimeoutsReply),
    #[cfg(feature = "dpms")]
    DpmsInfo(dpms::InfoReply),
    #[cfg(feature = "dri2")]
    Dri2QueryVersion(dri2::QueryVersionReply),
    #[cfg(feature = "dri2")]
    Dri2Connect(dri2::ConnectReply),
    #[cfg(feature = "dri2")]
    Dri2Authenticate(dri2::AuthenticateReply),
    #[cfg(feature = "dri2")]
    Dri2GetBuffers(dri2::GetBuffersReply),
    #[cfg(feature = "dri2")]
    Dri2CopyRegion(dri2::CopyRegionReply),
    #[cfg(feature = "dri2")]
    Dri2GetBuffersWithFormat(dri2::GetBuffersWithFormatReply),
    #[cfg(feature = "dri2")]
    Dri2SwapBuffers(dri2::SwapBuffersReply),
    #[cfg(feature = "dri2")]
    Dri2GetMSC(dri2::GetMSCReply),
    #[cfg(feature = "dri2")]
    Dri2WaitMSC(dri2::WaitMSCReply),
    #[cfg(feature = "dri2")]
    Dri2WaitSBC(dri2::WaitSBCReply),
    #[cfg(feature = "dri2")]
    Dri2GetParam(dri2::GetParamReply),
    #[cfg(feature = "dri3")]
    Dri3QueryVersion(dri3::QueryVersionReply),
    #[cfg(feature = "dri3")]
    Dri3Open(dri3::OpenReply),
    #[cfg(feature = "dri3")]
    Dri3BufferFromPixmap(dri3::BufferFromPixmapReply),
    #[cfg(feature = "dri3")]
    Dri3FDFromFence(dri3::FDFromFenceReply),
    #[cfg(feature = "dri3")]
    Dri3GetSupportedModifiers(dri3::GetSupportedModifiersReply),
    #[cfg(feature = "dri3")]
    Dri3BuffersFromPixmap(dri3::BuffersFromPixmapReply),
    GeQueryVersion(ge::QueryVersionReply),
    #[cfg(feature = "glx")]
    GlxMakeCurrent(glx::MakeCurrentReply),
    #[cfg(feature = "glx")]
    GlxIsDirect(glx::IsDirectReply),
    #[cfg(feature = "glx")]
    GlxQueryVersion(glx::QueryVersionReply),
    #[cfg(feature = "glx")]
    GlxGetVisualConfigs(glx::GetVisualConfigsReply),
    #[cfg(feature = "glx")]
    GlxVendorPrivateWithReply(glx::VendorPrivateWithReplyReply),
    #[cfg(feature = "glx")]
    GlxQueryExtensionsString(glx::QueryExtensionsStringReply),
    #[cfg(feature = "glx")]
    GlxQueryServerString(glx::QueryServerStringReply),
    #[cfg(feature = "glx")]
    GlxGetFBConfigs(glx::GetFBConfigsReply),
    #[cfg(feature = "glx")]
    GlxQueryContext(glx::QueryContextReply),
    #[cfg(feature = "glx")]
    GlxMakeContextCurrent(glx::MakeContextCurrentReply),
    #[cfg(feature = "glx")]
    GlxGetDrawableAttributes(glx::GetDrawableAttributesReply),
    #[cfg(feature = "glx")]
    GlxGenLists(glx::GenListsReply),
    #[cfg(feature = "glx")]
    GlxRenderMode(glx::RenderModeReply),
    #[cfg(feature = "glx")]
    GlxFinish(glx::FinishReply),
    #[cfg(feature = "glx")]
    GlxReadPixels(glx::ReadPixelsReply),
    #[cfg(feature = "glx")]
    GlxGetBooleanv(glx::GetBooleanvReply),
    #[cfg(feature = "glx")]
    GlxGetClipPlane(glx::GetClipPlaneReply),
    #[cfg(feature = "glx")]
    GlxGetDoublev(glx::GetDoublevReply),
    #[cfg(feature = "glx")]
    GlxGetError(glx::GetErrorReply),
    #[cfg(feature = "glx")]
    GlxGetFloatv(glx::GetFloatvReply),
    #[cfg(feature = "glx")]
    GlxGetIntegerv(glx::GetIntegervReply),
    #[cfg(feature = "glx")]
    GlxGetLightfv(glx::GetLightfvReply),
    #[cfg(feature = "glx")]
    GlxGetLightiv(glx::GetLightivReply),
    #[cfg(feature = "glx")]
    GlxGetMapdv(glx::GetMapdvReply),
    #[cfg(feature = "glx")]
    GlxGetMapfv(glx::GetMapfvReply),
    #[cfg(feature = "glx")]
    GlxGetMapiv(glx::GetMapivReply),
    #[cfg(feature = "glx")]
    GlxGetMaterialfv(glx::GetMaterialfvReply),
    #[cfg(feature = "glx")]
    GlxGetMaterialiv(glx::GetMaterialivReply),
    #[cfg(feature = "glx")]
    GlxGetPixelMapfv(glx::GetPixelMapfvReply),
    #[cfg(feature = "glx")]
    GlxGetPixelMapuiv(glx::GetPixelMapuivReply),
    #[cfg(feature = "glx")]
    GlxGetPixelMapusv(glx::GetPixelMapusvReply),
    #[cfg(feature = "glx")]
    GlxGetPolygonStipple(glx::GetPolygonStippleReply),
    #[cfg(feature = "glx")]
    GlxGetString(glx::GetStringReply),
    #[cfg(feature = "glx")]
    GlxGetTexEnvfv(glx::GetTexEnvfvReply),
    #[cfg(feature = "glx")]
    GlxGetTexEnviv(glx::GetTexEnvivReply),
    #[cfg(feature = "glx")]
    GlxGetTexGendv(glx::GetTexGendvReply),
    #[cfg(feature = "glx")]
    GlxGetTexGenfv(glx::GetTexGenfvReply),
    #[cfg(feature = "glx")]
    GlxGetTexGeniv(glx::GetTexGenivReply),
    #[cfg(feature = "glx")]
    GlxGetTexImage(glx::GetTexImageReply),
    #[cfg(feature = "glx")]
    GlxGetTexParameterfv(glx::GetTexParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetTexParameteriv(glx::GetTexParameterivReply),
    #[cfg(feature = "glx")]
    GlxGetTexLevelParameterfv(glx::GetTexLevelParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetTexLevelParameteriv(glx::GetTexLevelParameterivReply),
    #[cfg(feature = "glx")]
    GlxIsEnabled(glx::IsEnabledReply),
    #[cfg(feature = "glx")]
    GlxIsList(glx::IsListReply),
    #[cfg(feature = "glx")]
    GlxAreTexturesResident(glx::AreTexturesResidentReply),
    #[cfg(feature = "glx")]
    GlxGenTextures(glx::GenTexturesReply),
    #[cfg(feature = "glx")]
    GlxIsTexture(glx::IsTextureReply),
    #[cfg(feature = "glx")]
    GlxGetColorTable(glx::GetColorTableReply),
    #[cfg(feature = "glx")]
    GlxGetColorTableParameterfv(glx::GetColorTableParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetColorTableParameteriv(glx::GetColorTableParameterivReply),
    #[cfg(feature = "glx")]
    GlxGetConvolutionFilter(glx::GetConvolutionFilterReply),
    #[cfg(feature = "glx")]
    GlxGetConvolutionParameterfv(glx::GetConvolutionParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetConvolutionParameteriv(glx::GetConvolutionParameterivReply),
    #[cfg(feature = "glx")]
    GlxGetSeparableFilter(glx::GetSeparableFilterReply),
    #[cfg(feature = "glx")]
    GlxGetHistogram(glx::GetHistogramReply),
    #[cfg(feature = "glx")]
    GlxGetHistogramParameterfv(glx::GetHistogramParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetHistogramParameteriv(glx::GetHistogramParameterivReply),
    #[cfg(feature = "glx")]
    GlxGetMinmax(glx::GetMinmaxReply),
    #[cfg(feature = "glx")]
    GlxGetMinmaxParameterfv(glx::GetMinmaxParameterfvReply),
    #[cfg(feature = "glx")]
    GlxGetMinmaxParameteriv(glx::GetMinmaxParameterivReply),
    #[cfg(feature = "glx")]
    GlxGetCompressedTexImageARB(glx::GetCompressedTexImageARBReply),
    #[cfg(feature = "glx")]
    GlxGenQueriesARB(glx::GenQueriesARBReply),
    #[cfg(feature = "glx")]
    GlxIsQueryARB(glx::IsQueryARBReply),
    #[cfg(feature = "glx")]
    GlxGetQueryivARB(glx::GetQueryivARBReply),
    #[cfg(feature = "glx")]
    GlxGetQueryObjectivARB(glx::GetQueryObjectivARBReply),
    #[cfg(feature = "glx")]
    GlxGetQueryObjectuivARB(glx::GetQueryObjectuivARBReply),
    #[cfg(feature = "present")]
    PresentQueryVersion(present::QueryVersionReply),
    #[cfg(feature = "present")]
    PresentQueryCapabilities(present::QueryCapabilitiesReply),
    #[cfg(feature = "randr")]
    RandrQueryVersion(randr::QueryVersionReply),
    #[cfg(feature = "randr")]
    RandrSetScreenConfig(randr::SetScreenConfigReply),
    #[cfg(feature = "randr")]
    RandrGetScreenInfo(randr::GetScreenInfoReply),
    #[cfg(feature = "randr")]
    RandrGetScreenSizeRange(randr::GetScreenSizeRangeReply),
    #[cfg(feature = "randr")]
    RandrGetScreenResources(randr::GetScreenResourcesReply),
    #[cfg(feature = "randr")]
    RandrGetOutputInfo(randr::GetOutputInfoReply),
    #[cfg(feature = "randr")]
    RandrListOutputProperties(randr::ListOutputPropertiesReply),
    #[cfg(feature = "randr")]
    RandrQueryOutputProperty(randr::QueryOutputPropertyReply),
    #[cfg(feature = "randr")]
    RandrGetOutputProperty(randr::GetOutputPropertyReply),
    #[cfg(feature = "randr")]
    RandrCreateMode(randr::CreateModeReply),
    #[cfg(feature = "randr")]
    RandrGetCrtcInfo(randr::GetCrtcInfoReply),
    #[cfg(feature = "randr")]
    RandrSetCrtcConfig(randr::SetCrtcConfigReply),
    #[cfg(feature = "randr")]
    RandrGetCrtcGammaSize(randr::GetCrtcGammaSizeReply),
    #[cfg(feature = "randr")]
    RandrGetCrtcGamma(randr::GetCrtcGammaReply),
    #[cfg(feature = "randr")]
    RandrGetScreenResourcesCurrent(randr::GetScreenResourcesCurrentReply),
    #[cfg(feature = "randr")]
    RandrGetCrtcTransform(randr::GetCrtcTransformReply),
    #[cfg(feature = "randr")]
    RandrGetPanning(randr::GetPanningReply),
    #[cfg(feature = "randr")]
    RandrSetPanning(randr::SetPanningReply),
    #[cfg(feature = "randr")]
    RandrGetOutputPrimary(randr::GetOutputPrimaryReply),
    #[cfg(feature = "randr")]
    RandrGetProviders(randr::GetProvidersReply),
    #[cfg(feature = "randr")]
    RandrGetProviderInfo(randr::GetProviderInfoReply),
    #[cfg(feature = "randr")]
    RandrListProviderProperties(randr::ListProviderPropertiesReply),
    #[cfg(feature = "randr")]
    RandrQueryProviderProperty(randr::QueryProviderPropertyReply),
    #[cfg(feature = "randr")]
    RandrGetProviderProperty(randr::GetProviderPropertyReply),
    #[cfg(feature = "randr")]
    RandrGetMonitors(randr::GetMonitorsReply),
    #[cfg(feature = "randr")]
    RandrCreateLease(randr::CreateLeaseReply),
    #[cfg(feature = "record")]
    RecordQueryVersion(record::QueryVersionReply),
    #[cfg(feature = "record")]
    RecordGetContext(record::GetContextReply),
    #[cfg(feature = "record")]
    RecordEnableContext(record::EnableContextReply),
    #[cfg(feature = "render")]
    RenderQueryVersion(render::QueryVersionReply),
    #[cfg(feature = "render")]
    RenderQueryPictFormats(render::QueryPictFormatsReply),
    #[cfg(feature = "render")]
    RenderQueryPictIndexValues(render::QueryPictIndexValuesReply),
    #[cfg(feature = "render")]
    RenderQueryFilters(render::QueryFiltersReply),
    #[cfg(feature = "res")]
    ResQueryVersion(res::QueryVersionReply),
    #[cfg(feature = "res")]
    ResQueryClients(res::QueryClientsReply),
    #[cfg(feature = "res")]
    ResQueryClientResources(res::QueryClientResourcesReply),
    #[cfg(feature = "res")]
    ResQueryClientPixmapBytes(res::QueryClientPixmapBytesReply),
    #[cfg(feature = "res")]
    ResQueryClientIds(res::QueryClientIdsReply),
    #[cfg(feature = "res")]
    ResQueryResourceBytes(res::QueryResourceBytesReply),
    #[cfg(feature = "screensaver")]
    ScreensaverQueryVersion(screensaver::QueryVersionReply),
    #[cfg(feature = "screensaver")]
    ScreensaverQueryInfo(screensaver::QueryInfoReply),
    #[cfg(feature = "shape")]
    ShapeQueryVersion(shape::QueryVersionReply),
    #[cfg(feature = "shape")]
    ShapeQueryExtents(shape::QueryExtentsReply),
    #[cfg(feature = "shape")]
    ShapeInputSelected(shape::InputSelectedReply),
    #[cfg(feature = "shape")]
    ShapeGetRectangles(shape::GetRectanglesReply),
    #[cfg(feature = "shm")]
    ShmQueryVersion(shm::QueryVersionReply),
    #[cfg(feature = "shm")]
    ShmGetImage(shm::GetImageReply),
    #[cfg(feature = "shm")]
    ShmCreateSegment(shm::CreateSegmentReply),
    #[cfg(feature = "sync")]
    SyncInitialize(sync::InitializeReply),
    #[cfg(feature = "sync")]
    SyncListSystemCounters(sync::ListSystemCountersReply),
    #[cfg(feature = "sync")]
    SyncQueryCounter(sync::QueryCounterReply),
    #[cfg(feature = "sync")]
    SyncQueryAlarm(sync::QueryAlarmReply),
    #[cfg(feature = "sync")]
    SyncGetPriority(sync::GetPriorityReply),
    #[cfg(feature = "sync")]
    SyncQueryFence(sync::QueryFenceReply),
    XcMiscGetVersion(xc_misc::GetVersionReply),
    XcMiscGetXIDRange(xc_misc::GetXIDRangeReply),
    XcMiscGetXIDList(xc_misc::GetXIDListReply),
    #[cfg(feature = "xevie")]
    XevieQueryVersion(xevie::QueryVersionReply),
    #[cfg(feature = "xevie")]
    XevieStart(xevie::StartReply),
    #[cfg(feature = "xevie")]
    XevieEnd(xevie::EndReply),
    #[cfg(feature = "xevie")]
    XevieSend(xevie::SendReply),
    #[cfg(feature = "xevie")]
    XevieSelectInput(xevie::SelectInputReply),
    #[cfg(feature = "xf86dri")]
    Xf86driQueryVersion(xf86dri::QueryVersionReply),
    #[cfg(feature = "xf86dri")]
    Xf86driQueryDirectRenderingCapable(xf86dri::QueryDirectRenderingCapableReply),
    #[cfg(feature = "xf86dri")]
    Xf86driOpenConnection(xf86dri::OpenConnectionReply),
    #[cfg(feature = "xf86dri")]
    Xf86driGetClientDriverName(xf86dri::GetClientDriverNameReply),
    #[cfg(feature = "xf86dri")]
    Xf86driCreateContext(xf86dri::CreateContextReply),
    #[cfg(feature = "xf86dri")]
    Xf86driCreateDrawable(xf86dri::CreateDrawableReply),
    #[cfg(feature = "xf86dri")]
    Xf86driGetDrawableInfo(xf86dri::GetDrawableInfoReply),
    #[cfg(feature = "xf86dri")]
    Xf86driGetDeviceInfo(xf86dri::GetDeviceInfoReply),
    #[cfg(feature = "xf86dri")]
    Xf86driAuthConnection(xf86dri::AuthConnectionReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeQueryVersion(xf86vidmode::QueryVersionReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetModeLine(xf86vidmode::GetModeLineReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetMonitor(xf86vidmode::GetMonitorReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetAllModeLines(xf86vidmode::GetAllModeLinesReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeValidateModeLine(xf86vidmode::ValidateModeLineReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetViewPort(xf86vidmode::GetViewPortReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetDotClocks(xf86vidmode::GetDotClocksReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGamma(xf86vidmode::GetGammaReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGammaRamp(xf86vidmode::GetGammaRampReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetGammaRampSize(xf86vidmode::GetGammaRampSizeReply),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeGetPermissions(xf86vidmode::GetPermissionsReply),
    #[cfg(feature = "xfixes")]
    XfixesQueryVersion(xfixes::QueryVersionReply),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorImage(xfixes::GetCursorImageReply),
    #[cfg(feature = "xfixes")]
    XfixesFetchRegion(xfixes::FetchRegionReply),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorName(xfixes::GetCursorNameReply),
    #[cfg(feature = "xfixes")]
    XfixesGetCursorImageAndName(xfixes::GetCursorImageAndNameReply),
    #[cfg(feature = "xinerama")]
    XineramaQueryVersion(xinerama::QueryVersionReply),
    #[cfg(feature = "xinerama")]
    XineramaGetState(xinerama::GetStateReply),
    #[cfg(feature = "xinerama")]
    XineramaGetScreenCount(xinerama::GetScreenCountReply),
    #[cfg(feature = "xinerama")]
    XineramaGetScreenSize(xinerama::GetScreenSizeReply),
    #[cfg(feature = "xinerama")]
    XineramaIsActive(xinerama::IsActiveReply),
    #[cfg(feature = "xinerama")]
    XineramaQueryScreens(xinerama::QueryScreensReply),
    #[cfg(feature = "xinput")]
    XinputGetExtensionVersion(xinput::GetExtensionVersionReply),
    #[cfg(feature = "xinput")]
    XinputListInputDevices(xinput::ListInputDevicesReply),
    #[cfg(feature = "xinput")]
    XinputOpenDevice(xinput::OpenDeviceReply),
    #[cfg(feature = "xinput")]
    XinputSetDeviceMode(xinput::SetDeviceModeReply),
    #[cfg(feature = "xinput")]
    XinputGetSelectedExtensionEvents(xinput::GetSelectedExtensionEventsReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceDontPropagateList(xinput::GetDeviceDontPropagateListReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceMotionEvents(xinput::GetDeviceMotionEventsReply),
    #[cfg(feature = "xinput")]
    XinputChangeKeyboardDevice(xinput::ChangeKeyboardDeviceReply),
    #[cfg(feature = "xinput")]
    XinputChangePointerDevice(xinput::ChangePointerDeviceReply),
    #[cfg(feature = "xinput")]
    XinputGrabDevice(xinput::GrabDeviceReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceFocus(xinput::GetDeviceFocusReply),
    #[cfg(feature = "xinput")]
    XinputGetFeedbackControl(xinput::GetFeedbackControlReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceKeyMapping(xinput::GetDeviceKeyMappingReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceModifierMapping(xinput::GetDeviceModifierMappingReply),
    #[cfg(feature = "xinput")]
    XinputSetDeviceModifierMapping(xinput::SetDeviceModifierMappingReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceButtonMapping(xinput::GetDeviceButtonMappingReply),
    #[cfg(feature = "xinput")]
    XinputSetDeviceButtonMapping(xinput::SetDeviceButtonMappingReply),
    #[cfg(feature = "xinput")]
    XinputQueryDeviceState(xinput::QueryDeviceStateReply),
    #[cfg(feature = "xinput")]
    XinputSetDeviceValuators(xinput::SetDeviceValuatorsReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceControl(xinput::GetDeviceControlReply),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceControl(xinput::ChangeDeviceControlReply),
    #[cfg(feature = "xinput")]
    XinputListDeviceProperties(xinput::ListDevicePropertiesReply),
    #[cfg(feature = "xinput")]
    XinputGetDeviceProperty(xinput::GetDevicePropertyReply),
    #[cfg(feature = "xinput")]
    XinputXIQueryPointer(xinput::XIQueryPointerReply),
    #[cfg(feature = "xinput")]
    XinputXIGetClientPointer(xinput::XIGetClientPointerReply),
    #[cfg(feature = "xinput")]
    XinputXIQueryVersion(xinput::XIQueryVersionReply),
    #[cfg(feature = "xinput")]
    XinputXIQueryDevice(xinput::XIQueryDeviceReply),
    #[cfg(feature = "xinput")]
    XinputXIGetFocus(xinput::XIGetFocusReply),
    #[cfg(feature = "xinput")]
    XinputXIGrabDevice(xinput::XIGrabDeviceReply),
    #[cfg(feature = "xinput")]
    XinputXIPassiveGrabDevice(xinput::XIPassiveGrabDeviceReply),
    #[cfg(feature = "xinput")]
    XinputXIListProperties(xinput::XIListPropertiesReply),
    #[cfg(feature = "xinput")]
    XinputXIGetProperty(xinput::XIGetPropertyReply),
    #[cfg(feature = "xinput")]
    XinputXIGetSelectedEvents(xinput::XIGetSelectedEventsReply),
    #[cfg(feature = "xkb")]
    XkbUseExtension(xkb::UseExtensionReply),
    #[cfg(feature = "xkb")]
    XkbGetState(xkb::GetStateReply),
    #[cfg(feature = "xkb")]
    XkbGetControls(xkb::GetControlsReply),
    #[cfg(feature = "xkb")]
    XkbGetMap(xkb::GetMapReply),
    #[cfg(feature = "xkb")]
    XkbGetCompatMap(xkb::GetCompatMapReply),
    #[cfg(feature = "xkb")]
    XkbGetIndicatorState(xkb::GetIndicatorStateReply),
    #[cfg(feature = "xkb")]
    XkbGetIndicatorMap(xkb::GetIndicatorMapReply),
    #[cfg(feature = "xkb")]
    XkbGetNamedIndicator(xkb::GetNamedIndicatorReply),
    #[cfg(feature = "xkb")]
    XkbGetNames(xkb::GetNamesReply),
    #[cfg(feature = "xkb")]
    XkbPerClientFlags(xkb::PerClientFlagsReply),
    #[cfg(feature = "xkb")]
    XkbListComponents(xkb::ListComponentsReply),
    #[cfg(feature = "xkb")]
    XkbGetKbdByName(xkb::GetKbdByNameReply),
    #[cfg(feature = "xkb")]
    XkbGetDeviceInfo(xkb::GetDeviceInfoReply),
    #[cfg(feature = "xkb")]
    XkbSetDebuggingFlags(xkb::SetDebuggingFlagsReply),
    #[cfg(feature = "xprint")]
    XprintPrintQueryVersion(xprint::PrintQueryVersionReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetPrinterList(xprint::PrintGetPrinterListReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetContext(xprint::PrintGetContextReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetScreenOfContext(xprint::PrintGetScreenOfContextReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetDocumentData(xprint::PrintGetDocumentDataReply),
    #[cfg(feature = "xprint")]
    XprintPrintInputSelected(xprint::PrintInputSelectedReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetAttributes(xprint::PrintGetAttributesReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetOneAttributes(xprint::PrintGetOneAttributesReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetPageDimensions(xprint::PrintGetPageDimensionsReply),
    #[cfg(feature = "xprint")]
    XprintPrintQueryScreens(xprint::PrintQueryScreensReply),
    #[cfg(feature = "xprint")]
    XprintPrintSetImageResolution(xprint::PrintSetImageResolutionReply),
    #[cfg(feature = "xprint")]
    XprintPrintGetImageResolution(xprint::PrintGetImageResolutionReply),
    #[cfg(feature = "xselinux")]
    XselinuxQueryVersion(xselinux::QueryVersionReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetDeviceCreateContext(xselinux::GetDeviceCreateContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetDeviceContext(xselinux::GetDeviceContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetWindowCreateContext(xselinux::GetWindowCreateContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetWindowContext(xselinux::GetWindowContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyCreateContext(xselinux::GetPropertyCreateContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyUseContext(xselinux::GetPropertyUseContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyContext(xselinux::GetPropertyContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetPropertyDataContext(xselinux::GetPropertyDataContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxListProperties(xselinux::ListPropertiesReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionCreateContext(xselinux::GetSelectionCreateContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionUseContext(xselinux::GetSelectionUseContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionContext(xselinux::GetSelectionContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetSelectionDataContext(xselinux::GetSelectionDataContextReply),
    #[cfg(feature = "xselinux")]
    XselinuxListSelections(xselinux::ListSelectionsReply),
    #[cfg(feature = "xselinux")]
    XselinuxGetClientContext(xselinux::GetClientContextReply),
    #[cfg(feature = "xtest")]
    XtestGetVersion(xtest::GetVersionReply),
    #[cfg(feature = "xtest")]
    XtestCompareCursor(xtest::CompareCursorReply),
    #[cfg(feature = "xv")]
    XvQueryExtension(xv::QueryExtensionReply),
    #[cfg(feature = "xv")]
    XvQueryAdaptors(xv::QueryAdaptorsReply),
    #[cfg(feature = "xv")]
    XvQueryEncodings(xv::QueryEncodingsReply),
    #[cfg(feature = "xv")]
    XvGrabPort(xv::GrabPortReply),
    #[cfg(feature = "xv")]
    XvQueryBestSize(xv::QueryBestSizeReply),
    #[cfg(feature = "xv")]
    XvGetPortAttribute(xv::GetPortAttributeReply),
    #[cfg(feature = "xv")]
    XvQueryPortAttributes(xv::QueryPortAttributesReply),
    #[cfg(feature = "xv")]
    XvListImageFormats(xv::ListImageFormatsReply),
    #[cfg(feature = "xv")]
    XvQueryImageAttributes(xv::QueryImageAttributesReply),
    #[cfg(feature = "xvmc")]
    XvmcQueryVersion(xvmc::QueryVersionReply),
    #[cfg(feature = "xvmc")]
    XvmcListSurfaceTypes(xvmc::ListSurfaceTypesReply),
    #[cfg(feature = "xvmc")]
    XvmcCreateContext(xvmc::CreateContextReply),
    #[cfg(feature = "xvmc")]
    XvmcCreateSurface(xvmc::CreateSurfaceReply),
    #[cfg(feature = "xvmc")]
    XvmcCreateSubpicture(xvmc::CreateSubpictureReply),
    #[cfg(feature = "xvmc")]
    XvmcListSubpictureTypes(xvmc::ListSubpictureTypesReply),
}
impl From<()> for Reply {
    fn from(_: ()) -> Reply {
        Reply::Void
    }
}
impl From<xproto::GetWindowAttributesReply> for Reply {
  fn from(reply: xproto::GetWindowAttributesReply) -> Reply {
    Reply::GetWindowAttributes(reply)
  }
}
impl From<xproto::GetGeometryReply> for Reply {
  fn from(reply: xproto::GetGeometryReply) -> Reply {
    Reply::GetGeometry(reply)
  }
}
impl From<xproto::QueryTreeReply> for Reply {
  fn from(reply: xproto::QueryTreeReply) -> Reply {
    Reply::QueryTree(reply)
  }
}
impl From<xproto::InternAtomReply> for Reply {
  fn from(reply: xproto::InternAtomReply) -> Reply {
    Reply::InternAtom(reply)
  }
}
impl From<xproto::GetAtomNameReply> for Reply {
  fn from(reply: xproto::GetAtomNameReply) -> Reply {
    Reply::GetAtomName(reply)
  }
}
impl From<xproto::GetPropertyReply> for Reply {
  fn from(reply: xproto::GetPropertyReply) -> Reply {
    Reply::GetProperty(reply)
  }
}
impl From<xproto::ListPropertiesReply> for Reply {
  fn from(reply: xproto::ListPropertiesReply) -> Reply {
    Reply::ListProperties(reply)
  }
}
impl From<xproto::GetSelectionOwnerReply> for Reply {
  fn from(reply: xproto::GetSelectionOwnerReply) -> Reply {
    Reply::GetSelectionOwner(reply)
  }
}
impl From<xproto::GrabPointerReply> for Reply {
  fn from(reply: xproto::GrabPointerReply) -> Reply {
    Reply::GrabPointer(reply)
  }
}
impl From<xproto::GrabKeyboardReply> for Reply {
  fn from(reply: xproto::GrabKeyboardReply) -> Reply {
    Reply::GrabKeyboard(reply)
  }
}
impl From<xproto::QueryPointerReply> for Reply {
  fn from(reply: xproto::QueryPointerReply) -> Reply {
    Reply::QueryPointer(reply)
  }
}
impl From<xproto::GetMotionEventsReply> for Reply {
  fn from(reply: xproto::GetMotionEventsReply) -> Reply {
    Reply::GetMotionEvents(reply)
  }
}
impl From<xproto::TranslateCoordinatesReply> for Reply {
  fn from(reply: xproto::TranslateCoordinatesReply) -> Reply {
    Reply::TranslateCoordinates(reply)
  }
}
impl From<xproto::GetInputFocusReply> for Reply {
  fn from(reply: xproto::GetInputFocusReply) -> Reply {
    Reply::GetInputFocus(reply)
  }
}
impl From<xproto::QueryKeymapReply> for Reply {
  fn from(reply: xproto::QueryKeymapReply) -> Reply {
    Reply::QueryKeymap(reply)
  }
}
impl From<xproto::QueryFontReply> for Reply {
  fn from(reply: xproto::QueryFontReply) -> Reply {
    Reply::QueryFont(reply)
  }
}
impl From<xproto::QueryTextExtentsReply> for Reply {
  fn from(reply: xproto::QueryTextExtentsReply) -> Reply {
    Reply::QueryTextExtents(reply)
  }
}
impl From<xproto::ListFontsReply> for Reply {
  fn from(reply: xproto::ListFontsReply) -> Reply {
    Reply::ListFonts(reply)
  }
}
impl From<xproto::ListFontsWithInfoReply> for Reply {
  fn from(reply: xproto::ListFontsWithInfoReply) -> Reply {
    Reply::ListFontsWithInfo(reply)
  }
}
impl From<xproto::GetFontPathReply> for Reply {
  fn from(reply: xproto::GetFontPathReply) -> Reply {
    Reply::GetFontPath(reply)
  }
}
impl From<xproto::GetImageReply> for Reply {
  fn from(reply: xproto::GetImageReply) -> Reply {
    Reply::GetImage(reply)
  }
}
impl From<xproto::ListInstalledColormapsReply> for Reply {
  fn from(reply: xproto::ListInstalledColormapsReply) -> Reply {
    Reply::ListInstalledColormaps(reply)
  }
}
impl From<xproto::AllocColorReply> for Reply {
  fn from(reply: xproto::AllocColorReply) -> Reply {
    Reply::AllocColor(reply)
  }
}
impl From<xproto::AllocNamedColorReply> for Reply {
  fn from(reply: xproto::AllocNamedColorReply) -> Reply {
    Reply::AllocNamedColor(reply)
  }
}
impl From<xproto::AllocColorCellsReply> for Reply {
  fn from(reply: xproto::AllocColorCellsReply) -> Reply {
    Reply::AllocColorCells(reply)
  }
}
impl From<xproto::AllocColorPlanesReply> for Reply {
  fn from(reply: xproto::AllocColorPlanesReply) -> Reply {
    Reply::AllocColorPlanes(reply)
  }
}
impl From<xproto::QueryColorsReply> for Reply {
  fn from(reply: xproto::QueryColorsReply) -> Reply {
    Reply::QueryColors(reply)
  }
}
impl From<xproto::LookupColorReply> for Reply {
  fn from(reply: xproto::LookupColorReply) -> Reply {
    Reply::LookupColor(reply)
  }
}
impl From<xproto::QueryBestSizeReply> for Reply {
  fn from(reply: xproto::QueryBestSizeReply) -> Reply {
    Reply::QueryBestSize(reply)
  }
}
impl From<xproto::QueryExtensionReply> for Reply {
  fn from(reply: xproto::QueryExtensionReply) -> Reply {
    Reply::QueryExtension(reply)
  }
}
impl From<xproto::ListExtensionsReply> for Reply {
  fn from(reply: xproto::ListExtensionsReply) -> Reply {
    Reply::ListExtensions(reply)
  }
}
impl From<xproto::GetKeyboardMappingReply> for Reply {
  fn from(reply: xproto::GetKeyboardMappingReply) -> Reply {
    Reply::GetKeyboardMapping(reply)
  }
}
impl From<xproto::GetKeyboardControlReply> for Reply {
  fn from(reply: xproto::GetKeyboardControlReply) -> Reply {
    Reply::GetKeyboardControl(reply)
  }
}
impl From<xproto::GetPointerControlReply> for Reply {
  fn from(reply: xproto::GetPointerControlReply) -> Reply {
    Reply::GetPointerControl(reply)
  }
}
impl From<xproto::GetScreenSaverReply> for Reply {
  fn from(reply: xproto::GetScreenSaverReply) -> Reply {
    Reply::GetScreenSaver(reply)
  }
}
impl From<xproto::ListHostsReply> for Reply {
  fn from(reply: xproto::ListHostsReply) -> Reply {
    Reply::ListHosts(reply)
  }
}
impl From<xproto::SetPointerMappingReply> for Reply {
  fn from(reply: xproto::SetPointerMappingReply) -> Reply {
    Reply::SetPointerMapping(reply)
  }
}
impl From<xproto::GetPointerMappingReply> for Reply {
  fn from(reply: xproto::GetPointerMappingReply) -> Reply {
    Reply::GetPointerMapping(reply)
  }
}
impl From<xproto::SetModifierMappingReply> for Reply {
  fn from(reply: xproto::SetModifierMappingReply) -> Reply {
    Reply::SetModifierMapping(reply)
  }
}
impl From<xproto::GetModifierMappingReply> for Reply {
  fn from(reply: xproto::GetModifierMappingReply) -> Reply {
    Reply::GetModifierMapping(reply)
  }
}
impl From<bigreq::EnableReply> for Reply {
  fn from(reply: bigreq::EnableReply) -> Reply {
    Reply::BigreqEnable(reply)
  }
}
#[cfg(feature = "composite")]
impl From<composite::QueryVersionReply> for Reply {
  fn from(reply: composite::QueryVersionReply) -> Reply {
    Reply::CompositeQueryVersion(reply)
  }
}
#[cfg(feature = "composite")]
impl From<composite::GetOverlayWindowReply> for Reply {
  fn from(reply: composite::GetOverlayWindowReply) -> Reply {
    Reply::CompositeGetOverlayWindow(reply)
  }
}
#[cfg(feature = "damage")]
impl From<damage::QueryVersionReply> for Reply {
  fn from(reply: damage::QueryVersionReply) -> Reply {
    Reply::DamageQueryVersion(reply)
  }
}
#[cfg(feature = "dpms")]
impl From<dpms::GetVersionReply> for Reply {
  fn from(reply: dpms::GetVersionReply) -> Reply {
    Reply::DpmsGetVersion(reply)
  }
}
#[cfg(feature = "dpms")]
impl From<dpms::CapableReply> for Reply {
  fn from(reply: dpms::CapableReply) -> Reply {
    Reply::DpmsCapable(reply)
  }
}
#[cfg(feature = "dpms")]
impl From<dpms::GetTimeoutsReply> for Reply {
  fn from(reply: dpms::GetTimeoutsReply) -> Reply {
    Reply::DpmsGetTimeouts(reply)
  }
}
#[cfg(feature = "dpms")]
impl From<dpms::InfoReply> for Reply {
  fn from(reply: dpms::InfoReply) -> Reply {
    Reply::DpmsInfo(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::QueryVersionReply> for Reply {
  fn from(reply: dri2::QueryVersionReply) -> Reply {
    Reply::Dri2QueryVersion(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::ConnectReply> for Reply {
  fn from(reply: dri2::ConnectReply) -> Reply {
    Reply::Dri2Connect(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::AuthenticateReply> for Reply {
  fn from(reply: dri2::AuthenticateReply) -> Reply {
    Reply::Dri2Authenticate(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::GetBuffersReply> for Reply {
  fn from(reply: dri2::GetBuffersReply) -> Reply {
    Reply::Dri2GetBuffers(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::CopyRegionReply> for Reply {
  fn from(reply: dri2::CopyRegionReply) -> Reply {
    Reply::Dri2CopyRegion(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::GetBuffersWithFormatReply> for Reply {
  fn from(reply: dri2::GetBuffersWithFormatReply) -> Reply {
    Reply::Dri2GetBuffersWithFormat(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::SwapBuffersReply> for Reply {
  fn from(reply: dri2::SwapBuffersReply) -> Reply {
    Reply::Dri2SwapBuffers(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::GetMSCReply> for Reply {
  fn from(reply: dri2::GetMSCReply) -> Reply {
    Reply::Dri2GetMSC(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::WaitMSCReply> for Reply {
  fn from(reply: dri2::WaitMSCReply) -> Reply {
    Reply::Dri2WaitMSC(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::WaitSBCReply> for Reply {
  fn from(reply: dri2::WaitSBCReply) -> Reply {
    Reply::Dri2WaitSBC(reply)
  }
}
#[cfg(feature = "dri2")]
impl From<dri2::GetParamReply> for Reply {
  fn from(reply: dri2::GetParamReply) -> Reply {
    Reply::Dri2GetParam(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::QueryVersionReply> for Reply {
  fn from(reply: dri3::QueryVersionReply) -> Reply {
    Reply::Dri3QueryVersion(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::OpenReply> for Reply {
  fn from(reply: dri3::OpenReply) -> Reply {
    Reply::Dri3Open(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::BufferFromPixmapReply> for Reply {
  fn from(reply: dri3::BufferFromPixmapReply) -> Reply {
    Reply::Dri3BufferFromPixmap(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::FDFromFenceReply> for Reply {
  fn from(reply: dri3::FDFromFenceReply) -> Reply {
    Reply::Dri3FDFromFence(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::GetSupportedModifiersReply> for Reply {
  fn from(reply: dri3::GetSupportedModifiersReply) -> Reply {
    Reply::Dri3GetSupportedModifiers(reply)
  }
}
#[cfg(feature = "dri3")]
impl From<dri3::BuffersFromPixmapReply> for Reply {
  fn from(reply: dri3::BuffersFromPixmapReply) -> Reply {
    Reply::Dri3BuffersFromPixmap(reply)
  }
}
impl From<ge::QueryVersionReply> for Reply {
  fn from(reply: ge::QueryVersionReply) -> Reply {
    Reply::GeQueryVersion(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::MakeCurrentReply> for Reply {
  fn from(reply: glx::MakeCurrentReply) -> Reply {
    Reply::GlxMakeCurrent(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::IsDirectReply> for Reply {
  fn from(reply: glx::IsDirectReply) -> Reply {
    Reply::GlxIsDirect(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::QueryVersionReply> for Reply {
  fn from(reply: glx::QueryVersionReply) -> Reply {
    Reply::GlxQueryVersion(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetVisualConfigsReply> for Reply {
  fn from(reply: glx::GetVisualConfigsReply) -> Reply {
    Reply::GlxGetVisualConfigs(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::VendorPrivateWithReplyReply> for Reply {
  fn from(reply: glx::VendorPrivateWithReplyReply) -> Reply {
    Reply::GlxVendorPrivateWithReply(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::QueryExtensionsStringReply> for Reply {
  fn from(reply: glx::QueryExtensionsStringReply) -> Reply {
    Reply::GlxQueryExtensionsString(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::QueryServerStringReply> for Reply {
  fn from(reply: glx::QueryServerStringReply) -> Reply {
    Reply::GlxQueryServerString(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetFBConfigsReply> for Reply {
  fn from(reply: glx::GetFBConfigsReply) -> Reply {
    Reply::GlxGetFBConfigs(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::QueryContextReply> for Reply {
  fn from(reply: glx::QueryContextReply) -> Reply {
    Reply::GlxQueryContext(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::MakeContextCurrentReply> for Reply {
  fn from(reply: glx::MakeContextCurrentReply) -> Reply {
    Reply::GlxMakeContextCurrent(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetDrawableAttributesReply> for Reply {
  fn from(reply: glx::GetDrawableAttributesReply) -> Reply {
    Reply::GlxGetDrawableAttributes(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GenListsReply> for Reply {
  fn from(reply: glx::GenListsReply) -> Reply {
    Reply::GlxGenLists(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::RenderModeReply> for Reply {
  fn from(reply: glx::RenderModeReply) -> Reply {
    Reply::GlxRenderMode(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::FinishReply> for Reply {
  fn from(reply: glx::FinishReply) -> Reply {
    Reply::GlxFinish(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::ReadPixelsReply> for Reply {
  fn from(reply: glx::ReadPixelsReply) -> Reply {
    Reply::GlxReadPixels(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetBooleanvReply> for Reply {
  fn from(reply: glx::GetBooleanvReply) -> Reply {
    Reply::GlxGetBooleanv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetClipPlaneReply> for Reply {
  fn from(reply: glx::GetClipPlaneReply) -> Reply {
    Reply::GlxGetClipPlane(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetDoublevReply> for Reply {
  fn from(reply: glx::GetDoublevReply) -> Reply {
    Reply::GlxGetDoublev(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetErrorReply> for Reply {
  fn from(reply: glx::GetErrorReply) -> Reply {
    Reply::GlxGetError(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetFloatvReply> for Reply {
  fn from(reply: glx::GetFloatvReply) -> Reply {
    Reply::GlxGetFloatv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetIntegervReply> for Reply {
  fn from(reply: glx::GetIntegervReply) -> Reply {
    Reply::GlxGetIntegerv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetLightfvReply> for Reply {
  fn from(reply: glx::GetLightfvReply) -> Reply {
    Reply::GlxGetLightfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetLightivReply> for Reply {
  fn from(reply: glx::GetLightivReply) -> Reply {
    Reply::GlxGetLightiv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMapdvReply> for Reply {
  fn from(reply: glx::GetMapdvReply) -> Reply {
    Reply::GlxGetMapdv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMapfvReply> for Reply {
  fn from(reply: glx::GetMapfvReply) -> Reply {
    Reply::GlxGetMapfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMapivReply> for Reply {
  fn from(reply: glx::GetMapivReply) -> Reply {
    Reply::GlxGetMapiv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMaterialfvReply> for Reply {
  fn from(reply: glx::GetMaterialfvReply) -> Reply {
    Reply::GlxGetMaterialfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMaterialivReply> for Reply {
  fn from(reply: glx::GetMaterialivReply) -> Reply {
    Reply::GlxGetMaterialiv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetPixelMapfvReply> for Reply {
  fn from(reply: glx::GetPixelMapfvReply) -> Reply {
    Reply::GlxGetPixelMapfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetPixelMapuivReply> for Reply {
  fn from(reply: glx::GetPixelMapuivReply) -> Reply {
    Reply::GlxGetPixelMapuiv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetPixelMapusvReply> for Reply {
  fn from(reply: glx::GetPixelMapusvReply) -> Reply {
    Reply::GlxGetPixelMapusv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetPolygonStippleReply> for Reply {
  fn from(reply: glx::GetPolygonStippleReply) -> Reply {
    Reply::GlxGetPolygonStipple(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetStringReply> for Reply {
  fn from(reply: glx::GetStringReply) -> Reply {
    Reply::GlxGetString(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexEnvfvReply> for Reply {
  fn from(reply: glx::GetTexEnvfvReply) -> Reply {
    Reply::GlxGetTexEnvfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexEnvivReply> for Reply {
  fn from(reply: glx::GetTexEnvivReply) -> Reply {
    Reply::GlxGetTexEnviv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexGendvReply> for Reply {
  fn from(reply: glx::GetTexGendvReply) -> Reply {
    Reply::GlxGetTexGendv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexGenfvReply> for Reply {
  fn from(reply: glx::GetTexGenfvReply) -> Reply {
    Reply::GlxGetTexGenfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexGenivReply> for Reply {
  fn from(reply: glx::GetTexGenivReply) -> Reply {
    Reply::GlxGetTexGeniv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexImageReply> for Reply {
  fn from(reply: glx::GetTexImageReply) -> Reply {
    Reply::GlxGetTexImage(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexParameterfvReply> for Reply {
  fn from(reply: glx::GetTexParameterfvReply) -> Reply {
    Reply::GlxGetTexParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexParameterivReply> for Reply {
  fn from(reply: glx::GetTexParameterivReply) -> Reply {
    Reply::GlxGetTexParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexLevelParameterfvReply> for Reply {
  fn from(reply: glx::GetTexLevelParameterfvReply) -> Reply {
    Reply::GlxGetTexLevelParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetTexLevelParameterivReply> for Reply {
  fn from(reply: glx::GetTexLevelParameterivReply) -> Reply {
    Reply::GlxGetTexLevelParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::IsEnabledReply> for Reply {
  fn from(reply: glx::IsEnabledReply) -> Reply {
    Reply::GlxIsEnabled(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::IsListReply> for Reply {
  fn from(reply: glx::IsListReply) -> Reply {
    Reply::GlxIsList(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::AreTexturesResidentReply> for Reply {
  fn from(reply: glx::AreTexturesResidentReply) -> Reply {
    Reply::GlxAreTexturesResident(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GenTexturesReply> for Reply {
  fn from(reply: glx::GenTexturesReply) -> Reply {
    Reply::GlxGenTextures(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::IsTextureReply> for Reply {
  fn from(reply: glx::IsTextureReply) -> Reply {
    Reply::GlxIsTexture(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetColorTableReply> for Reply {
  fn from(reply: glx::GetColorTableReply) -> Reply {
    Reply::GlxGetColorTable(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetColorTableParameterfvReply> for Reply {
  fn from(reply: glx::GetColorTableParameterfvReply) -> Reply {
    Reply::GlxGetColorTableParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetColorTableParameterivReply> for Reply {
  fn from(reply: glx::GetColorTableParameterivReply) -> Reply {
    Reply::GlxGetColorTableParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetConvolutionFilterReply> for Reply {
  fn from(reply: glx::GetConvolutionFilterReply) -> Reply {
    Reply::GlxGetConvolutionFilter(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetConvolutionParameterfvReply> for Reply {
  fn from(reply: glx::GetConvolutionParameterfvReply) -> Reply {
    Reply::GlxGetConvolutionParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetConvolutionParameterivReply> for Reply {
  fn from(reply: glx::GetConvolutionParameterivReply) -> Reply {
    Reply::GlxGetConvolutionParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetSeparableFilterReply> for Reply {
  fn from(reply: glx::GetSeparableFilterReply) -> Reply {
    Reply::GlxGetSeparableFilter(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetHistogramReply> for Reply {
  fn from(reply: glx::GetHistogramReply) -> Reply {
    Reply::GlxGetHistogram(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetHistogramParameterfvReply> for Reply {
  fn from(reply: glx::GetHistogramParameterfvReply) -> Reply {
    Reply::GlxGetHistogramParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetHistogramParameterivReply> for Reply {
  fn from(reply: glx::GetHistogramParameterivReply) -> Reply {
    Reply::GlxGetHistogramParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMinmaxReply> for Reply {
  fn from(reply: glx::GetMinmaxReply) -> Reply {
    Reply::GlxGetMinmax(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMinmaxParameterfvReply> for Reply {
  fn from(reply: glx::GetMinmaxParameterfvReply) -> Reply {
    Reply::GlxGetMinmaxParameterfv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetMinmaxParameterivReply> for Reply {
  fn from(reply: glx::GetMinmaxParameterivReply) -> Reply {
    Reply::GlxGetMinmaxParameteriv(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetCompressedTexImageARBReply> for Reply {
  fn from(reply: glx::GetCompressedTexImageARBReply) -> Reply {
    Reply::GlxGetCompressedTexImageARB(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GenQueriesARBReply> for Reply {
  fn from(reply: glx::GenQueriesARBReply) -> Reply {
    Reply::GlxGenQueriesARB(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::IsQueryARBReply> for Reply {
  fn from(reply: glx::IsQueryARBReply) -> Reply {
    Reply::GlxIsQueryARB(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetQueryivARBReply> for Reply {
  fn from(reply: glx::GetQueryivARBReply) -> Reply {
    Reply::GlxGetQueryivARB(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetQueryObjectivARBReply> for Reply {
  fn from(reply: glx::GetQueryObjectivARBReply) -> Reply {
    Reply::GlxGetQueryObjectivARB(reply)
  }
}
#[cfg(feature = "glx")]
impl From<glx::GetQueryObjectuivARBReply> for Reply {
  fn from(reply: glx::GetQueryObjectuivARBReply) -> Reply {
    Reply::GlxGetQueryObjectuivARB(reply)
  }
}
#[cfg(feature = "present")]
impl From<present::QueryVersionReply> for Reply {
  fn from(reply: present::QueryVersionReply) -> Reply {
    Reply::PresentQueryVersion(reply)
  }
}
#[cfg(feature = "present")]
impl From<present::QueryCapabilitiesReply> for Reply {
  fn from(reply: present::QueryCapabilitiesReply) -> Reply {
    Reply::PresentQueryCapabilities(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::QueryVersionReply> for Reply {
  fn from(reply: randr::QueryVersionReply) -> Reply {
    Reply::RandrQueryVersion(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::SetScreenConfigReply> for Reply {
  fn from(reply: randr::SetScreenConfigReply) -> Reply {
    Reply::RandrSetScreenConfig(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetScreenInfoReply> for Reply {
  fn from(reply: randr::GetScreenInfoReply) -> Reply {
    Reply::RandrGetScreenInfo(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetScreenSizeRangeReply> for Reply {
  fn from(reply: randr::GetScreenSizeRangeReply) -> Reply {
    Reply::RandrGetScreenSizeRange(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetScreenResourcesReply> for Reply {
  fn from(reply: randr::GetScreenResourcesReply) -> Reply {
    Reply::RandrGetScreenResources(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetOutputInfoReply> for Reply {
  fn from(reply: randr::GetOutputInfoReply) -> Reply {
    Reply::RandrGetOutputInfo(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::ListOutputPropertiesReply> for Reply {
  fn from(reply: randr::ListOutputPropertiesReply) -> Reply {
    Reply::RandrListOutputProperties(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::QueryOutputPropertyReply> for Reply {
  fn from(reply: randr::QueryOutputPropertyReply) -> Reply {
    Reply::RandrQueryOutputProperty(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetOutputPropertyReply> for Reply {
  fn from(reply: randr::GetOutputPropertyReply) -> Reply {
    Reply::RandrGetOutputProperty(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::CreateModeReply> for Reply {
  fn from(reply: randr::CreateModeReply) -> Reply {
    Reply::RandrCreateMode(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetCrtcInfoReply> for Reply {
  fn from(reply: randr::GetCrtcInfoReply) -> Reply {
    Reply::RandrGetCrtcInfo(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::SetCrtcConfigReply> for Reply {
  fn from(reply: randr::SetCrtcConfigReply) -> Reply {
    Reply::RandrSetCrtcConfig(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetCrtcGammaSizeReply> for Reply {
  fn from(reply: randr::GetCrtcGammaSizeReply) -> Reply {
    Reply::RandrGetCrtcGammaSize(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetCrtcGammaReply> for Reply {
  fn from(reply: randr::GetCrtcGammaReply) -> Reply {
    Reply::RandrGetCrtcGamma(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetScreenResourcesCurrentReply> for Reply {
  fn from(reply: randr::GetScreenResourcesCurrentReply) -> Reply {
    Reply::RandrGetScreenResourcesCurrent(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetCrtcTransformReply> for Reply {
  fn from(reply: randr::GetCrtcTransformReply) -> Reply {
    Reply::RandrGetCrtcTransform(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetPanningReply> for Reply {
  fn from(reply: randr::GetPanningReply) -> Reply {
    Reply::RandrGetPanning(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::SetPanningReply> for Reply {
  fn from(reply: randr::SetPanningReply) -> Reply {
    Reply::RandrSetPanning(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetOutputPrimaryReply> for Reply {
  fn from(reply: randr::GetOutputPrimaryReply) -> Reply {
    Reply::RandrGetOutputPrimary(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetProvidersReply> for Reply {
  fn from(reply: randr::GetProvidersReply) -> Reply {
    Reply::RandrGetProviders(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetProviderInfoReply> for Reply {
  fn from(reply: randr::GetProviderInfoReply) -> Reply {
    Reply::RandrGetProviderInfo(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::ListProviderPropertiesReply> for Reply {
  fn from(reply: randr::ListProviderPropertiesReply) -> Reply {
    Reply::RandrListProviderProperties(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::QueryProviderPropertyReply> for Reply {
  fn from(reply: randr::QueryProviderPropertyReply) -> Reply {
    Reply::RandrQueryProviderProperty(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetProviderPropertyReply> for Reply {
  fn from(reply: randr::GetProviderPropertyReply) -> Reply {
    Reply::RandrGetProviderProperty(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::GetMonitorsReply> for Reply {
  fn from(reply: randr::GetMonitorsReply) -> Reply {
    Reply::RandrGetMonitors(reply)
  }
}
#[cfg(feature = "randr")]
impl From<randr::CreateLeaseReply> for Reply {
  fn from(reply: randr::CreateLeaseReply) -> Reply {
    Reply::RandrCreateLease(reply)
  }
}
#[cfg(feature = "record")]
impl From<record::QueryVersionReply> for Reply {
  fn from(reply: record::QueryVersionReply) -> Reply {
    Reply::RecordQueryVersion(reply)
  }
}
#[cfg(feature = "record")]
impl From<record::GetContextReply> for Reply {
  fn from(reply: record::GetContextReply) -> Reply {
    Reply::RecordGetContext(reply)
  }
}
#[cfg(feature = "record")]
impl From<record::EnableContextReply> for Reply {
  fn from(reply: record::EnableContextReply) -> Reply {
    Reply::RecordEnableContext(reply)
  }
}
#[cfg(feature = "render")]
impl From<render::QueryVersionReply> for Reply {
  fn from(reply: render::QueryVersionReply) -> Reply {
    Reply::RenderQueryVersion(reply)
  }
}
#[cfg(feature = "render")]
impl From<render::QueryPictFormatsReply> for Reply {
  fn from(reply: render::QueryPictFormatsReply) -> Reply {
    Reply::RenderQueryPictFormats(reply)
  }
}
#[cfg(feature = "render")]
impl From<render::QueryPictIndexValuesReply> for Reply {
  fn from(reply: render::QueryPictIndexValuesReply) -> Reply {
    Reply::RenderQueryPictIndexValues(reply)
  }
}
#[cfg(feature = "render")]
impl From<render::QueryFiltersReply> for Reply {
  fn from(reply: render::QueryFiltersReply) -> Reply {
    Reply::RenderQueryFilters(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryVersionReply> for Reply {
  fn from(reply: res::QueryVersionReply) -> Reply {
    Reply::ResQueryVersion(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryClientsReply> for Reply {
  fn from(reply: res::QueryClientsReply) -> Reply {
    Reply::ResQueryClients(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryClientResourcesReply> for Reply {
  fn from(reply: res::QueryClientResourcesReply) -> Reply {
    Reply::ResQueryClientResources(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryClientPixmapBytesReply> for Reply {
  fn from(reply: res::QueryClientPixmapBytesReply) -> Reply {
    Reply::ResQueryClientPixmapBytes(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryClientIdsReply> for Reply {
  fn from(reply: res::QueryClientIdsReply) -> Reply {
    Reply::ResQueryClientIds(reply)
  }
}
#[cfg(feature = "res")]
impl From<res::QueryResourceBytesReply> for Reply {
  fn from(reply: res::QueryResourceBytesReply) -> Reply {
    Reply::ResQueryResourceBytes(reply)
  }
}
#[cfg(feature = "screensaver")]
impl From<screensaver::QueryVersionReply> for Reply {
  fn from(reply: screensaver::QueryVersionReply) -> Reply {
    Reply::ScreensaverQueryVersion(reply)
  }
}
#[cfg(feature = "screensaver")]
impl From<screensaver::QueryInfoReply> for Reply {
  fn from(reply: screensaver::QueryInfoReply) -> Reply {
    Reply::ScreensaverQueryInfo(reply)
  }
}
#[cfg(feature = "shape")]
impl From<shape::QueryVersionReply> for Reply {
  fn from(reply: shape::QueryVersionReply) -> Reply {
    Reply::ShapeQueryVersion(reply)
  }
}
#[cfg(feature = "shape")]
impl From<shape::QueryExtentsReply> for Reply {
  fn from(reply: shape::QueryExtentsReply) -> Reply {
    Reply::ShapeQueryExtents(reply)
  }
}
#[cfg(feature = "shape")]
impl From<shape::InputSelectedReply> for Reply {
  fn from(reply: shape::InputSelectedReply) -> Reply {
    Reply::ShapeInputSelected(reply)
  }
}
#[cfg(feature = "shape")]
impl From<shape::GetRectanglesReply> for Reply {
  fn from(reply: shape::GetRectanglesReply) -> Reply {
    Reply::ShapeGetRectangles(reply)
  }
}
#[cfg(feature = "shm")]
impl From<shm::QueryVersionReply> for Reply {
  fn from(reply: shm::QueryVersionReply) -> Reply {
    Reply::ShmQueryVersion(reply)
  }
}
#[cfg(feature = "shm")]
impl From<shm::GetImageReply> for Reply {
  fn from(reply: shm::GetImageReply) -> Reply {
    Reply::ShmGetImage(reply)
  }
}
#[cfg(feature = "shm")]
impl From<shm::CreateSegmentReply> for Reply {
  fn from(reply: shm::CreateSegmentReply) -> Reply {
    Reply::ShmCreateSegment(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::InitializeReply> for Reply {
  fn from(reply: sync::InitializeReply) -> Reply {
    Reply::SyncInitialize(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::ListSystemCountersReply> for Reply {
  fn from(reply: sync::ListSystemCountersReply) -> Reply {
    Reply::SyncListSystemCounters(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::QueryCounterReply> for Reply {
  fn from(reply: sync::QueryCounterReply) -> Reply {
    Reply::SyncQueryCounter(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::QueryAlarmReply> for Reply {
  fn from(reply: sync::QueryAlarmReply) -> Reply {
    Reply::SyncQueryAlarm(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::GetPriorityReply> for Reply {
  fn from(reply: sync::GetPriorityReply) -> Reply {
    Reply::SyncGetPriority(reply)
  }
}
#[cfg(feature = "sync")]
impl From<sync::QueryFenceReply> for Reply {
  fn from(reply: sync::QueryFenceReply) -> Reply {
    Reply::SyncQueryFence(reply)
  }
}
impl From<xc_misc::GetVersionReply> for Reply {
  fn from(reply: xc_misc::GetVersionReply) -> Reply {
    Reply::XcMiscGetVersion(reply)
  }
}
impl From<xc_misc::GetXIDRangeReply> for Reply {
  fn from(reply: xc_misc::GetXIDRangeReply) -> Reply {
    Reply::XcMiscGetXIDRange(reply)
  }
}
impl From<xc_misc::GetXIDListReply> for Reply {
  fn from(reply: xc_misc::GetXIDListReply) -> Reply {
    Reply::XcMiscGetXIDList(reply)
  }
}
#[cfg(feature = "xevie")]
impl From<xevie::QueryVersionReply> for Reply {
  fn from(reply: xevie::QueryVersionReply) -> Reply {
    Reply::XevieQueryVersion(reply)
  }
}
#[cfg(feature = "xevie")]
impl From<xevie::StartReply> for Reply {
  fn from(reply: xevie::StartReply) -> Reply {
    Reply::XevieStart(reply)
  }
}
#[cfg(feature = "xevie")]
impl From<xevie::EndReply> for Reply {
  fn from(reply: xevie::EndReply) -> Reply {
    Reply::XevieEnd(reply)
  }
}
#[cfg(feature = "xevie")]
impl From<xevie::SendReply> for Reply {
  fn from(reply: xevie::SendReply) -> Reply {
    Reply::XevieSend(reply)
  }
}
#[cfg(feature = "xevie")]
impl From<xevie::SelectInputReply> for Reply {
  fn from(reply: xevie::SelectInputReply) -> Reply {
    Reply::XevieSelectInput(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::QueryVersionReply> for Reply {
  fn from(reply: xf86dri::QueryVersionReply) -> Reply {
    Reply::Xf86driQueryVersion(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::QueryDirectRenderingCapableReply> for Reply {
  fn from(reply: xf86dri::QueryDirectRenderingCapableReply) -> Reply {
    Reply::Xf86driQueryDirectRenderingCapable(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::OpenConnectionReply> for Reply {
  fn from(reply: xf86dri::OpenConnectionReply) -> Reply {
    Reply::Xf86driOpenConnection(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::GetClientDriverNameReply> for Reply {
  fn from(reply: xf86dri::GetClientDriverNameReply) -> Reply {
    Reply::Xf86driGetClientDriverName(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::CreateContextReply> for Reply {
  fn from(reply: xf86dri::CreateContextReply) -> Reply {
    Reply::Xf86driCreateContext(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::CreateDrawableReply> for Reply {
  fn from(reply: xf86dri::CreateDrawableReply) -> Reply {
    Reply::Xf86driCreateDrawable(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::GetDrawableInfoReply> for Reply {
  fn from(reply: xf86dri::GetDrawableInfoReply) -> Reply {
    Reply::Xf86driGetDrawableInfo(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::GetDeviceInfoReply> for Reply {
  fn from(reply: xf86dri::GetDeviceInfoReply) -> Reply {
    Reply::Xf86driGetDeviceInfo(reply)
  }
}
#[cfg(feature = "xf86dri")]
impl From<xf86dri::AuthConnectionReply> for Reply {
  fn from(reply: xf86dri::AuthConnectionReply) -> Reply {
    Reply::Xf86driAuthConnection(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::QueryVersionReply> for Reply {
  fn from(reply: xf86vidmode::QueryVersionReply) -> Reply {
    Reply::Xf86vidmodeQueryVersion(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetModeLineReply> for Reply {
  fn from(reply: xf86vidmode::GetModeLineReply) -> Reply {
    Reply::Xf86vidmodeGetModeLine(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetMonitorReply> for Reply {
  fn from(reply: xf86vidmode::GetMonitorReply) -> Reply {
    Reply::Xf86vidmodeGetMonitor(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetAllModeLinesReply> for Reply {
  fn from(reply: xf86vidmode::GetAllModeLinesReply) -> Reply {
    Reply::Xf86vidmodeGetAllModeLines(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::ValidateModeLineReply> for Reply {
  fn from(reply: xf86vidmode::ValidateModeLineReply) -> Reply {
    Reply::Xf86vidmodeValidateModeLine(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetViewPortReply> for Reply {
  fn from(reply: xf86vidmode::GetViewPortReply) -> Reply {
    Reply::Xf86vidmodeGetViewPort(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetDotClocksReply> for Reply {
  fn from(reply: xf86vidmode::GetDotClocksReply) -> Reply {
    Reply::Xf86vidmodeGetDotClocks(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetGammaReply> for Reply {
  fn from(reply: xf86vidmode::GetGammaReply) -> Reply {
    Reply::Xf86vidmodeGetGamma(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetGammaRampReply> for Reply {
  fn from(reply: xf86vidmode::GetGammaRampReply) -> Reply {
    Reply::Xf86vidmodeGetGammaRamp(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetGammaRampSizeReply> for Reply {
  fn from(reply: xf86vidmode::GetGammaRampSizeReply) -> Reply {
    Reply::Xf86vidmodeGetGammaRampSize(reply)
  }
}
#[cfg(feature = "xf86vidmode")]
impl From<xf86vidmode::GetPermissionsReply> for Reply {
  fn from(reply: xf86vidmode::GetPermissionsReply) -> Reply {
    Reply::Xf86vidmodeGetPermissions(reply)
  }
}
#[cfg(feature = "xfixes")]
impl From<xfixes::QueryVersionReply> for Reply {
  fn from(reply: xfixes::QueryVersionReply) -> Reply {
    Reply::XfixesQueryVersion(reply)
  }
}
#[cfg(feature = "xfixes")]
impl From<xfixes::GetCursorImageReply> for Reply {
  fn from(reply: xfixes::GetCursorImageReply) -> Reply {
    Reply::XfixesGetCursorImage(reply)
  }
}
#[cfg(feature = "xfixes")]
impl From<xfixes::FetchRegionReply> for Reply {
  fn from(reply: xfixes::FetchRegionReply) -> Reply {
    Reply::XfixesFetchRegion(reply)
  }
}
#[cfg(feature = "xfixes")]
impl From<xfixes::GetCursorNameReply> for Reply {
  fn from(reply: xfixes::GetCursorNameReply) -> Reply {
    Reply::XfixesGetCursorName(reply)
  }
}
#[cfg(feature = "xfixes")]
impl From<xfixes::GetCursorImageAndNameReply> for Reply {
  fn from(reply: xfixes::GetCursorImageAndNameReply) -> Reply {
    Reply::XfixesGetCursorImageAndName(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::QueryVersionReply> for Reply {
  fn from(reply: xinerama::QueryVersionReply) -> Reply {
    Reply::XineramaQueryVersion(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::GetStateReply> for Reply {
  fn from(reply: xinerama::GetStateReply) -> Reply {
    Reply::XineramaGetState(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::GetScreenCountReply> for Reply {
  fn from(reply: xinerama::GetScreenCountReply) -> Reply {
    Reply::XineramaGetScreenCount(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::GetScreenSizeReply> for Reply {
  fn from(reply: xinerama::GetScreenSizeReply) -> Reply {
    Reply::XineramaGetScreenSize(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::IsActiveReply> for Reply {
  fn from(reply: xinerama::IsActiveReply) -> Reply {
    Reply::XineramaIsActive(reply)
  }
}
#[cfg(feature = "xinerama")]
impl From<xinerama::QueryScreensReply> for Reply {
  fn from(reply: xinerama::QueryScreensReply) -> Reply {
    Reply::XineramaQueryScreens(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetExtensionVersionReply> for Reply {
  fn from(reply: xinput::GetExtensionVersionReply) -> Reply {
    Reply::XinputGetExtensionVersion(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::ListInputDevicesReply> for Reply {
  fn from(reply: xinput::ListInputDevicesReply) -> Reply {
    Reply::XinputListInputDevices(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::OpenDeviceReply> for Reply {
  fn from(reply: xinput::OpenDeviceReply) -> Reply {
    Reply::XinputOpenDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::SetDeviceModeReply> for Reply {
  fn from(reply: xinput::SetDeviceModeReply) -> Reply {
    Reply::XinputSetDeviceMode(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetSelectedExtensionEventsReply> for Reply {
  fn from(reply: xinput::GetSelectedExtensionEventsReply) -> Reply {
    Reply::XinputGetSelectedExtensionEvents(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceDontPropagateListReply> for Reply {
  fn from(reply: xinput::GetDeviceDontPropagateListReply) -> Reply {
    Reply::XinputGetDeviceDontPropagateList(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceMotionEventsReply> for Reply {
  fn from(reply: xinput::GetDeviceMotionEventsReply) -> Reply {
    Reply::XinputGetDeviceMotionEvents(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::ChangeKeyboardDeviceReply> for Reply {
  fn from(reply: xinput::ChangeKeyboardDeviceReply) -> Reply {
    Reply::XinputChangeKeyboardDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::ChangePointerDeviceReply> for Reply {
  fn from(reply: xinput::ChangePointerDeviceReply) -> Reply {
    Reply::XinputChangePointerDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GrabDeviceReply> for Reply {
  fn from(reply: xinput::GrabDeviceReply) -> Reply {
    Reply::XinputGrabDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceFocusReply> for Reply {
  fn from(reply: xinput::GetDeviceFocusReply) -> Reply {
    Reply::XinputGetDeviceFocus(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetFeedbackControlReply> for Reply {
  fn from(reply: xinput::GetFeedbackControlReply) -> Reply {
    Reply::XinputGetFeedbackControl(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceKeyMappingReply> for Reply {
  fn from(reply: xinput::GetDeviceKeyMappingReply) -> Reply {
    Reply::XinputGetDeviceKeyMapping(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceModifierMappingReply> for Reply {
  fn from(reply: xinput::GetDeviceModifierMappingReply) -> Reply {
    Reply::XinputGetDeviceModifierMapping(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::SetDeviceModifierMappingReply> for Reply {
  fn from(reply: xinput::SetDeviceModifierMappingReply) -> Reply {
    Reply::XinputSetDeviceModifierMapping(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceButtonMappingReply> for Reply {
  fn from(reply: xinput::GetDeviceButtonMappingReply) -> Reply {
    Reply::XinputGetDeviceButtonMapping(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::SetDeviceButtonMappingReply> for Reply {
  fn from(reply: xinput::SetDeviceButtonMappingReply) -> Reply {
    Reply::XinputSetDeviceButtonMapping(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::QueryDeviceStateReply> for Reply {
  fn from(reply: xinput::QueryDeviceStateReply) -> Reply {
    Reply::XinputQueryDeviceState(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::SetDeviceValuatorsReply> for Reply {
  fn from(reply: xinput::SetDeviceValuatorsReply) -> Reply {
    Reply::XinputSetDeviceValuators(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDeviceControlReply> for Reply {
  fn from(reply: xinput::GetDeviceControlReply) -> Reply {
    Reply::XinputGetDeviceControl(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::ChangeDeviceControlReply> for Reply {
  fn from(reply: xinput::ChangeDeviceControlReply) -> Reply {
    Reply::XinputChangeDeviceControl(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::ListDevicePropertiesReply> for Reply {
  fn from(reply: xinput::ListDevicePropertiesReply) -> Reply {
    Reply::XinputListDeviceProperties(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::GetDevicePropertyReply> for Reply {
  fn from(reply: xinput::GetDevicePropertyReply) -> Reply {
    Reply::XinputGetDeviceProperty(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIQueryPointerReply> for Reply {
  fn from(reply: xinput::XIQueryPointerReply) -> Reply {
    Reply::XinputXIQueryPointer(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIGetClientPointerReply> for Reply {
  fn from(reply: xinput::XIGetClientPointerReply) -> Reply {
    Reply::XinputXIGetClientPointer(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIQueryVersionReply> for Reply {
  fn from(reply: xinput::XIQueryVersionReply) -> Reply {
    Reply::XinputXIQueryVersion(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIQueryDeviceReply> for Reply {
  fn from(reply: xinput::XIQueryDeviceReply) -> Reply {
    Reply::XinputXIQueryDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIGetFocusReply> for Reply {
  fn from(reply: xinput::XIGetFocusReply) -> Reply {
    Reply::XinputXIGetFocus(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIGrabDeviceReply> for Reply {
  fn from(reply: xinput::XIGrabDeviceReply) -> Reply {
    Reply::XinputXIGrabDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIPassiveGrabDeviceReply> for Reply {
  fn from(reply: xinput::XIPassiveGrabDeviceReply) -> Reply {
    Reply::XinputXIPassiveGrabDevice(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIListPropertiesReply> for Reply {
  fn from(reply: xinput::XIListPropertiesReply) -> Reply {
    Reply::XinputXIListProperties(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIGetPropertyReply> for Reply {
  fn from(reply: xinput::XIGetPropertyReply) -> Reply {
    Reply::XinputXIGetProperty(reply)
  }
}
#[cfg(feature = "xinput")]
impl From<xinput::XIGetSelectedEventsReply> for Reply {
  fn from(reply: xinput::XIGetSelectedEventsReply) -> Reply {
    Reply::XinputXIGetSelectedEvents(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::UseExtensionReply> for Reply {
  fn from(reply: xkb::UseExtensionReply) -> Reply {
    Reply::XkbUseExtension(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetStateReply> for Reply {
  fn from(reply: xkb::GetStateReply) -> Reply {
    Reply::XkbGetState(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetControlsReply> for Reply {
  fn from(reply: xkb::GetControlsReply) -> Reply {
    Reply::XkbGetControls(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetMapReply> for Reply {
  fn from(reply: xkb::GetMapReply) -> Reply {
    Reply::XkbGetMap(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetCompatMapReply> for Reply {
  fn from(reply: xkb::GetCompatMapReply) -> Reply {
    Reply::XkbGetCompatMap(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetIndicatorStateReply> for Reply {
  fn from(reply: xkb::GetIndicatorStateReply) -> Reply {
    Reply::XkbGetIndicatorState(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetIndicatorMapReply> for Reply {
  fn from(reply: xkb::GetIndicatorMapReply) -> Reply {
    Reply::XkbGetIndicatorMap(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetNamedIndicatorReply> for Reply {
  fn from(reply: xkb::GetNamedIndicatorReply) -> Reply {
    Reply::XkbGetNamedIndicator(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetNamesReply> for Reply {
  fn from(reply: xkb::GetNamesReply) -> Reply {
    Reply::XkbGetNames(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::PerClientFlagsReply> for Reply {
  fn from(reply: xkb::PerClientFlagsReply) -> Reply {
    Reply::XkbPerClientFlags(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::ListComponentsReply> for Reply {
  fn from(reply: xkb::ListComponentsReply) -> Reply {
    Reply::XkbListComponents(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetKbdByNameReply> for Reply {
  fn from(reply: xkb::GetKbdByNameReply) -> Reply {
    Reply::XkbGetKbdByName(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::GetDeviceInfoReply> for Reply {
  fn from(reply: xkb::GetDeviceInfoReply) -> Reply {
    Reply::XkbGetDeviceInfo(reply)
  }
}
#[cfg(feature = "xkb")]
impl From<xkb::SetDebuggingFlagsReply> for Reply {
  fn from(reply: xkb::SetDebuggingFlagsReply) -> Reply {
    Reply::XkbSetDebuggingFlags(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintQueryVersionReply> for Reply {
  fn from(reply: xprint::PrintQueryVersionReply) -> Reply {
    Reply::XprintPrintQueryVersion(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetPrinterListReply> for Reply {
  fn from(reply: xprint::PrintGetPrinterListReply) -> Reply {
    Reply::XprintPrintGetPrinterList(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetContextReply> for Reply {
  fn from(reply: xprint::PrintGetContextReply) -> Reply {
    Reply::XprintPrintGetContext(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetScreenOfContextReply> for Reply {
  fn from(reply: xprint::PrintGetScreenOfContextReply) -> Reply {
    Reply::XprintPrintGetScreenOfContext(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetDocumentDataReply> for Reply {
  fn from(reply: xprint::PrintGetDocumentDataReply) -> Reply {
    Reply::XprintPrintGetDocumentData(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintInputSelectedReply> for Reply {
  fn from(reply: xprint::PrintInputSelectedReply) -> Reply {
    Reply::XprintPrintInputSelected(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetAttributesReply> for Reply {
  fn from(reply: xprint::PrintGetAttributesReply) -> Reply {
    Reply::XprintPrintGetAttributes(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetOneAttributesReply> for Reply {
  fn from(reply: xprint::PrintGetOneAttributesReply) -> Reply {
    Reply::XprintPrintGetOneAttributes(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetPageDimensionsReply> for Reply {
  fn from(reply: xprint::PrintGetPageDimensionsReply) -> Reply {
    Reply::XprintPrintGetPageDimensions(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintQueryScreensReply> for Reply {
  fn from(reply: xprint::PrintQueryScreensReply) -> Reply {
    Reply::XprintPrintQueryScreens(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintSetImageResolutionReply> for Reply {
  fn from(reply: xprint::PrintSetImageResolutionReply) -> Reply {
    Reply::XprintPrintSetImageResolution(reply)
  }
}
#[cfg(feature = "xprint")]
impl From<xprint::PrintGetImageResolutionReply> for Reply {
  fn from(reply: xprint::PrintGetImageResolutionReply) -> Reply {
    Reply::XprintPrintGetImageResolution(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::QueryVersionReply> for Reply {
  fn from(reply: xselinux::QueryVersionReply) -> Reply {
    Reply::XselinuxQueryVersion(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetDeviceCreateContextReply> for Reply {
  fn from(reply: xselinux::GetDeviceCreateContextReply) -> Reply {
    Reply::XselinuxGetDeviceCreateContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetDeviceContextReply> for Reply {
  fn from(reply: xselinux::GetDeviceContextReply) -> Reply {
    Reply::XselinuxGetDeviceContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetWindowCreateContextReply> for Reply {
  fn from(reply: xselinux::GetWindowCreateContextReply) -> Reply {
    Reply::XselinuxGetWindowCreateContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetWindowContextReply> for Reply {
  fn from(reply: xselinux::GetWindowContextReply) -> Reply {
    Reply::XselinuxGetWindowContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetPropertyCreateContextReply> for Reply {
  fn from(reply: xselinux::GetPropertyCreateContextReply) -> Reply {
    Reply::XselinuxGetPropertyCreateContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetPropertyUseContextReply> for Reply {
  fn from(reply: xselinux::GetPropertyUseContextReply) -> Reply {
    Reply::XselinuxGetPropertyUseContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetPropertyContextReply> for Reply {
  fn from(reply: xselinux::GetPropertyContextReply) -> Reply {
    Reply::XselinuxGetPropertyContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetPropertyDataContextReply> for Reply {
  fn from(reply: xselinux::GetPropertyDataContextReply) -> Reply {
    Reply::XselinuxGetPropertyDataContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::ListPropertiesReply> for Reply {
  fn from(reply: xselinux::ListPropertiesReply) -> Reply {
    Reply::XselinuxListProperties(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetSelectionCreateContextReply> for Reply {
  fn from(reply: xselinux::GetSelectionCreateContextReply) -> Reply {
    Reply::XselinuxGetSelectionCreateContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetSelectionUseContextReply> for Reply {
  fn from(reply: xselinux::GetSelectionUseContextReply) -> Reply {
    Reply::XselinuxGetSelectionUseContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetSelectionContextReply> for Reply {
  fn from(reply: xselinux::GetSelectionContextReply) -> Reply {
    Reply::XselinuxGetSelectionContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetSelectionDataContextReply> for Reply {
  fn from(reply: xselinux::GetSelectionDataContextReply) -> Reply {
    Reply::XselinuxGetSelectionDataContext(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::ListSelectionsReply> for Reply {
  fn from(reply: xselinux::ListSelectionsReply) -> Reply {
    Reply::XselinuxListSelections(reply)
  }
}
#[cfg(feature = "xselinux")]
impl From<xselinux::GetClientContextReply> for Reply {
  fn from(reply: xselinux::GetClientContextReply) -> Reply {
    Reply::XselinuxGetClientContext(reply)
  }
}
#[cfg(feature = "xtest")]
impl From<xtest::GetVersionReply> for Reply {
  fn from(reply: xtest::GetVersionReply) -> Reply {
    Reply::XtestGetVersion(reply)
  }
}
#[cfg(feature = "xtest")]
impl From<xtest::CompareCursorReply> for Reply {
  fn from(reply: xtest::CompareCursorReply) -> Reply {
    Reply::XtestCompareCursor(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryExtensionReply> for Reply {
  fn from(reply: xv::QueryExtensionReply) -> Reply {
    Reply::XvQueryExtension(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryAdaptorsReply> for Reply {
  fn from(reply: xv::QueryAdaptorsReply) -> Reply {
    Reply::XvQueryAdaptors(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryEncodingsReply> for Reply {
  fn from(reply: xv::QueryEncodingsReply) -> Reply {
    Reply::XvQueryEncodings(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::GrabPortReply> for Reply {
  fn from(reply: xv::GrabPortReply) -> Reply {
    Reply::XvGrabPort(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryBestSizeReply> for Reply {
  fn from(reply: xv::QueryBestSizeReply) -> Reply {
    Reply::XvQueryBestSize(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::GetPortAttributeReply> for Reply {
  fn from(reply: xv::GetPortAttributeReply) -> Reply {
    Reply::XvGetPortAttribute(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryPortAttributesReply> for Reply {
  fn from(reply: xv::QueryPortAttributesReply) -> Reply {
    Reply::XvQueryPortAttributes(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::ListImageFormatsReply> for Reply {
  fn from(reply: xv::ListImageFormatsReply) -> Reply {
    Reply::XvListImageFormats(reply)
  }
}
#[cfg(feature = "xv")]
impl From<xv::QueryImageAttributesReply> for Reply {
  fn from(reply: xv::QueryImageAttributesReply) -> Reply {
    Reply::XvQueryImageAttributes(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::QueryVersionReply> for Reply {
  fn from(reply: xvmc::QueryVersionReply) -> Reply {
    Reply::XvmcQueryVersion(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::ListSurfaceTypesReply> for Reply {
  fn from(reply: xvmc::ListSurfaceTypesReply) -> Reply {
    Reply::XvmcListSurfaceTypes(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::CreateContextReply> for Reply {
  fn from(reply: xvmc::CreateContextReply) -> Reply {
    Reply::XvmcCreateContext(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::CreateSurfaceReply> for Reply {
  fn from(reply: xvmc::CreateSurfaceReply) -> Reply {
    Reply::XvmcCreateSurface(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::CreateSubpictureReply> for Reply {
  fn from(reply: xvmc::CreateSubpictureReply) -> Reply {
    Reply::XvmcCreateSubpicture(reply)
  }
}
#[cfg(feature = "xvmc")]
impl From<xvmc::ListSubpictureTypesReply> for Reply {
  fn from(reply: xvmc::ListSubpictureTypesReply) -> Reply {
    Reply::XvmcListSubpictureTypes(reply)
  }
}

/// Enumeration of all possible X11 errors.
#[derive(Debug, Clone)]
pub enum Error {
    Unknown(Vec<u8>),
    Access(xproto::AccessError),
    Alloc(xproto::AllocError),
    Atom(xproto::AtomError),
    Colormap(xproto::ColormapError),
    Cursor(xproto::CursorError),
    Drawable(xproto::DrawableError),
    Font(xproto::FontError),
    GContext(xproto::GContextError),
    IDChoice(xproto::IDChoiceError),
    Implementation(xproto::ImplementationError),
    Length(xproto::LengthError),
    Match(xproto::MatchError),
    Name(xproto::NameError),
    Pixmap(xproto::PixmapError),
    Request(xproto::RequestError),
    Value(xproto::ValueError),
    Window(xproto::WindowError),
    #[cfg(feature = "damage")]
    DamageBadDamage(damage::BadDamageError),
    #[cfg(feature = "glx")]
    GlxBadContext(glx::BadContextError),
    #[cfg(feature = "glx")]
    GlxBadContextState(glx::BadContextStateError),
    #[cfg(feature = "glx")]
    GlxBadContextTag(glx::BadContextTagError),
    #[cfg(feature = "glx")]
    GlxBadCurrentDrawable(glx::BadCurrentDrawableError),
    #[cfg(feature = "glx")]
    GlxBadCurrentWindow(glx::BadCurrentWindowError),
    #[cfg(feature = "glx")]
    GlxBadDrawable(glx::BadDrawableError),
    #[cfg(feature = "glx")]
    GlxBadFBConfig(glx::BadFBConfigError),
    #[cfg(feature = "glx")]
    GlxBadLargeRequest(glx::BadLargeRequestError),
    #[cfg(feature = "glx")]
    GlxBadPbuffer(glx::BadPbufferError),
    #[cfg(feature = "glx")]
    GlxBadPixmap(glx::BadPixmapError),
    #[cfg(feature = "glx")]
    GlxBadRenderRequest(glx::BadRenderRequestError),
    #[cfg(feature = "glx")]
    GlxBadWindow(glx::BadWindowError),
    #[cfg(feature = "glx")]
    GlxGLXBadProfileARB(glx::GLXBadProfileARBError),
    #[cfg(feature = "glx")]
    GlxUnsupportedPrivateRequest(glx::UnsupportedPrivateRequestError),
    #[cfg(feature = "randr")]
    RandrBadCrtc(randr::BadCrtcError),
    #[cfg(feature = "randr")]
    RandrBadMode(randr::BadModeError),
    #[cfg(feature = "randr")]
    RandrBadOutput(randr::BadOutputError),
    #[cfg(feature = "randr")]
    RandrBadProvider(randr::BadProviderError),
    #[cfg(feature = "record")]
    RecordBadContext(record::BadContextError),
    #[cfg(feature = "render")]
    RenderGlyph(render::GlyphError),
    #[cfg(feature = "render")]
    RenderGlyphSet(render::GlyphSetError),
    #[cfg(feature = "render")]
    RenderPictFormat(render::PictFormatError),
    #[cfg(feature = "render")]
    RenderPictOp(render::PictOpError),
    #[cfg(feature = "render")]
    RenderPicture(render::PictureError),
    #[cfg(feature = "shm")]
    ShmBadSeg(shm::BadSegError),
    #[cfg(feature = "sync")]
    SyncAlarm(sync::AlarmError),
    #[cfg(feature = "sync")]
    SyncCounter(sync::CounterError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadClock(xf86vidmode::BadClockError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadHTimings(xf86vidmode::BadHTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeBadVTimings(xf86vidmode::BadVTimingsError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeClientNotLocal(xf86vidmode::ClientNotLocalError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeExtensionDisabled(xf86vidmode::ExtensionDisabledError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeModeUnsuitable(xf86vidmode::ModeUnsuitableError),
    #[cfg(feature = "xf86vidmode")]
    Xf86vidmodeZoomLocked(xf86vidmode::ZoomLockedError),
    #[cfg(feature = "xfixes")]
    XfixesBadRegion(xfixes::BadRegionError),
    #[cfg(feature = "xinput")]
    XinputClass(xinput::ClassError),
    #[cfg(feature = "xinput")]
    XinputDevice(xinput::DeviceError),
    #[cfg(feature = "xinput")]
    XinputDeviceBusy(xinput::DeviceBusyError),
    #[cfg(feature = "xinput")]
    XinputEvent(xinput::EventError),
    #[cfg(feature = "xinput")]
    XinputMode(xinput::ModeError),
    #[cfg(feature = "xkb")]
    XkbKeyboard(xkb::KeyboardError),
    #[cfg(feature = "xprint")]
    XprintBadContext(xprint::BadContextError),
    #[cfg(feature = "xprint")]
    XprintBadSequence(xprint::BadSequenceError),
    #[cfg(feature = "xv")]
    XvBadControl(xv::BadControlError),
    #[cfg(feature = "xv")]
    XvBadEncoding(xv::BadEncodingError),
    #[cfg(feature = "xv")]
    XvBadPort(xv::BadPortError),
}

impl Error {
    /// Parse a generic X11 error into a concrete error type.
    #[allow(clippy::cognitive_complexity, clippy::match_single_binding)]
    pub fn parse(
        error: &[u8],
        ext_info_provider: &dyn ExtInfoProvider,
    ) -> Result<Self, ParseError> {
        let error_code = error_code(error)?;

        // Check if this is a core protocol error
        match error_code {
            xproto::ACCESS_ERROR => return Ok(Self::Access(error.try_into()?)),
            xproto::ALLOC_ERROR => return Ok(Self::Alloc(error.try_into()?)),
            xproto::ATOM_ERROR => return Ok(Self::Atom(error.try_into()?)),
            xproto::COLORMAP_ERROR => return Ok(Self::Colormap(error.try_into()?)),
            xproto::CURSOR_ERROR => return Ok(Self::Cursor(error.try_into()?)),
            xproto::DRAWABLE_ERROR => return Ok(Self::Drawable(error.try_into()?)),
            xproto::FONT_ERROR => return Ok(Self::Font(error.try_into()?)),
            xproto::G_CONTEXT_ERROR => return Ok(Self::GContext(error.try_into()?)),
            xproto::ID_CHOICE_ERROR => return Ok(Self::IDChoice(error.try_into()?)),
            xproto::IMPLEMENTATION_ERROR => return Ok(Self::Implementation(error.try_into()?)),
            xproto::LENGTH_ERROR => return Ok(Self::Length(error.try_into()?)),
            xproto::MATCH_ERROR => return Ok(Self::Match(error.try_into()?)),
            xproto::NAME_ERROR => return Ok(Self::Name(error.try_into()?)),
            xproto::PIXMAP_ERROR => return Ok(Self::Pixmap(error.try_into()?)),
            xproto::REQUEST_ERROR => return Ok(Self::Request(error.try_into()?)),
            xproto::VALUE_ERROR => return Ok(Self::Value(error.try_into()?)),
            xproto::WINDOW_ERROR => return Ok(Self::Window(error.try_into()?)),
            _ => {}
        }

        // Find the extension that this error could belong to
        let ext_info = ext_info_provider.get_from_error_code(error_code);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    damage::BAD_DAMAGE_ERROR => Ok(Self::DamageBadDamage(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    glx::BAD_CONTEXT_ERROR => Ok(Self::GlxBadContext(error.try_into()?)),
                    glx::BAD_CONTEXT_STATE_ERROR => Ok(Self::GlxBadContextState(error.try_into()?)),
                    glx::BAD_CONTEXT_TAG_ERROR => Ok(Self::GlxBadContextTag(error.try_into()?)),
                    glx::BAD_CURRENT_DRAWABLE_ERROR => Ok(Self::GlxBadCurrentDrawable(error.try_into()?)),
                    glx::BAD_CURRENT_WINDOW_ERROR => Ok(Self::GlxBadCurrentWindow(error.try_into()?)),
                    glx::BAD_DRAWABLE_ERROR => Ok(Self::GlxBadDrawable(error.try_into()?)),
                    glx::BAD_FB_CONFIG_ERROR => Ok(Self::GlxBadFBConfig(error.try_into()?)),
                    glx::BAD_LARGE_REQUEST_ERROR => Ok(Self::GlxBadLargeRequest(error.try_into()?)),
                    glx::BAD_PBUFFER_ERROR => Ok(Self::GlxBadPbuffer(error.try_into()?)),
                    glx::BAD_PIXMAP_ERROR => Ok(Self::GlxBadPixmap(error.try_into()?)),
                    glx::BAD_RENDER_REQUEST_ERROR => Ok(Self::GlxBadRenderRequest(error.try_into()?)),
                    glx::BAD_WINDOW_ERROR => Ok(Self::GlxBadWindow(error.try_into()?)),
                    glx::GLX_BAD_PROFILE_ARB_ERROR => Ok(Self::GlxGLXBadProfileARB(error.try_into()?)),
                    glx::UNSUPPORTED_PRIVATE_REQUEST_ERROR => Ok(Self::GlxUnsupportedPrivateRequest(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    randr::BAD_CRTC_ERROR => Ok(Self::RandrBadCrtc(error.try_into()?)),
                    randr::BAD_MODE_ERROR => Ok(Self::RandrBadMode(error.try_into()?)),
                    randr::BAD_OUTPUT_ERROR => Ok(Self::RandrBadOutput(error.try_into()?)),
                    randr::BAD_PROVIDER_ERROR => Ok(Self::RandrBadProvider(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "record")]
            Some((record::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    record::BAD_CONTEXT_ERROR => Ok(Self::RecordBadContext(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "render")]
            Some((render::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    render::GLYPH_ERROR => Ok(Self::RenderGlyph(error.try_into()?)),
                    render::GLYPH_SET_ERROR => Ok(Self::RenderGlyphSet(error.try_into()?)),
                    render::PICT_FORMAT_ERROR => Ok(Self::RenderPictFormat(error.try_into()?)),
                    render::PICT_OP_ERROR => Ok(Self::RenderPictOp(error.try_into()?)),
                    render::PICTURE_ERROR => Ok(Self::RenderPicture(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    shm::BAD_SEG_ERROR => Ok(Self::ShmBadSeg(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    sync::ALARM_ERROR => Ok(Self::SyncAlarm(error.try_into()?)),
                    sync::COUNTER_ERROR => Ok(Self::SyncCounter(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xf86vidmode")]
            Some((xf86vidmode::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xf86vidmode::BAD_CLOCK_ERROR => Ok(Self::Xf86vidmodeBadClock(error.try_into()?)),
                    xf86vidmode::BAD_H_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadHTimings(error.try_into()?)),
                    xf86vidmode::BAD_V_TIMINGS_ERROR => Ok(Self::Xf86vidmodeBadVTimings(error.try_into()?)),
                    xf86vidmode::CLIENT_NOT_LOCAL_ERROR => Ok(Self::Xf86vidmodeClientNotLocal(error.try_into()?)),
                    xf86vidmode::EXTENSION_DISABLED_ERROR => Ok(Self::Xf86vidmodeExtensionDisabled(error.try_into()?)),
                    xf86vidmode::MODE_UNSUITABLE_ERROR => Ok(Self::Xf86vidmodeModeUnsuitable(error.try_into()?)),
                    xf86vidmode::ZOOM_LOCKED_ERROR => Ok(Self::Xf86vidmodeZoomLocked(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xfixes::BAD_REGION_ERROR => Ok(Self::XfixesBadRegion(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xinput::CLASS_ERROR => Ok(Self::XinputClass(error.try_into()?)),
                    xinput::DEVICE_ERROR => Ok(Self::XinputDevice(error.try_into()?)),
                    xinput::DEVICE_BUSY_ERROR => Ok(Self::XinputDeviceBusy(error.try_into()?)),
                    xinput::EVENT_ERROR => Ok(Self::XinputEvent(error.try_into()?)),
                    xinput::MODE_ERROR => Ok(Self::XinputMode(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xkb::KEYBOARD_ERROR => Ok(Self::XkbKeyboard(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xprint::BAD_CONTEXT_ERROR => Ok(Self::XprintBadContext(error.try_into()?)),
                    xprint::BAD_SEQUENCE_ERROR => Ok(Self::XprintBadSequence(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, ext_info)) => {
                match error_code - ext_info.first_error {
                    xv::BAD_CONTROL_ERROR => Ok(Self::XvBadControl(error.try_into()?)),
                    xv::BAD_ENCODING_ERROR => Ok(Self::XvBadEncoding(error.try_into()?)),
                    xv::BAD_PORT_ERROR => Ok(Self::XvBadPort(error.try_into()?)),
                    _ => Ok(Self::Unknown(error.to_vec())),
                }
            }
            _ => Ok(Self::Unknown(error.to_vec())),
        }
    }

    /// Get the sequence number contained in this X11 error
    pub fn wire_sequence_number(&self) -> u16 {
        match self {
            Error::Unknown(value) => sequence_number(value).unwrap(),
            Error::Access(value) => value.sequence,
            Error::Alloc(value) => value.sequence,
            Error::Atom(value) => value.sequence,
            Error::Colormap(value) => value.sequence,
            Error::Cursor(value) => value.sequence,
            Error::Drawable(value) => value.sequence,
            Error::Font(value) => value.sequence,
            Error::GContext(value) => value.sequence,
            Error::IDChoice(value) => value.sequence,
            Error::Implementation(value) => value.sequence,
            Error::Length(value) => value.sequence,
            Error::Match(value) => value.sequence,
            Error::Name(value) => value.sequence,
            Error::Pixmap(value) => value.sequence,
            Error::Request(value) => value.sequence,
            Error::Value(value) => value.sequence,
            Error::Window(value) => value.sequence,
            #[cfg(feature = "damage")]
            Error::DamageBadDamage(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadContext(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadContextState(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadContextTag(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentDrawable(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentWindow(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadDrawable(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadFBConfig(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadLargeRequest(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadPbuffer(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadPixmap(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadRenderRequest(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxBadWindow(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxGLXBadProfileARB(value) => value.sequence,
            #[cfg(feature = "glx")]
            Error::GlxUnsupportedPrivateRequest(value) => value.sequence,
            #[cfg(feature = "randr")]
            Error::RandrBadCrtc(value) => value.sequence,
            #[cfg(feature = "randr")]
            Error::RandrBadMode(value) => value.sequence,
            #[cfg(feature = "randr")]
            Error::RandrBadOutput(value) => value.sequence,
            #[cfg(feature = "randr")]
            Error::RandrBadProvider(value) => value.sequence,
            #[cfg(feature = "record")]
            Error::RecordBadContext(value) => value.sequence,
            #[cfg(feature = "render")]
            Error::RenderGlyph(value) => value.sequence,
            #[cfg(feature = "render")]
            Error::RenderGlyphSet(value) => value.sequence,
            #[cfg(feature = "render")]
            Error::RenderPictFormat(value) => value.sequence,
            #[cfg(feature = "render")]
            Error::RenderPictOp(value) => value.sequence,
            #[cfg(feature = "render")]
            Error::RenderPicture(value) => value.sequence,
            #[cfg(feature = "shm")]
            Error::ShmBadSeg(value) => value.sequence,
            #[cfg(feature = "sync")]
            Error::SyncAlarm(value) => value.sequence,
            #[cfg(feature = "sync")]
            Error::SyncCounter(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadClock(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadHTimings(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadVTimings(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeClientNotLocal(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeExtensionDisabled(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeModeUnsuitable(value) => value.sequence,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeZoomLocked(value) => value.sequence,
            #[cfg(feature = "xfixes")]
            Error::XfixesBadRegion(value) => value.sequence,
            #[cfg(feature = "xinput")]
            Error::XinputClass(value) => value.sequence,
            #[cfg(feature = "xinput")]
            Error::XinputDevice(value) => value.sequence,
            #[cfg(feature = "xinput")]
            Error::XinputDeviceBusy(value) => value.sequence,
            #[cfg(feature = "xinput")]
            Error::XinputEvent(value) => value.sequence,
            #[cfg(feature = "xinput")]
            Error::XinputMode(value) => value.sequence,
            #[cfg(feature = "xkb")]
            Error::XkbKeyboard(value) => value.sequence,
            #[cfg(feature = "xprint")]
            Error::XprintBadContext(value) => value.sequence,
            #[cfg(feature = "xprint")]
            Error::XprintBadSequence(value) => value.sequence,
            #[cfg(feature = "xv")]
            Error::XvBadControl(value) => value.sequence,
            #[cfg(feature = "xv")]
            Error::XvBadEncoding(value) => value.sequence,
            #[cfg(feature = "xv")]
            Error::XvBadPort(value) => value.sequence,
        }
    }

    /// Get the error code of this X11 error
    pub fn error_code(&self) -> u8 {
        match self {
            Error::Unknown(value) => error_code(value).unwrap(),
            Error::Access(value) => value.error_code,
            Error::Alloc(value) => value.error_code,
            Error::Atom(value) => value.error_code,
            Error::Colormap(value) => value.error_code,
            Error::Cursor(value) => value.error_code,
            Error::Drawable(value) => value.error_code,
            Error::Font(value) => value.error_code,
            Error::GContext(value) => value.error_code,
            Error::IDChoice(value) => value.error_code,
            Error::Implementation(value) => value.error_code,
            Error::Length(value) => value.error_code,
            Error::Match(value) => value.error_code,
            Error::Name(value) => value.error_code,
            Error::Pixmap(value) => value.error_code,
            Error::Request(value) => value.error_code,
            Error::Value(value) => value.error_code,
            Error::Window(value) => value.error_code,
            #[cfg(feature = "damage")]
            Error::DamageBadDamage(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadContext(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadContextState(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadContextTag(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentDrawable(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentWindow(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadDrawable(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadFBConfig(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadLargeRequest(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadPbuffer(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadPixmap(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadRenderRequest(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxBadWindow(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxGLXBadProfileARB(value) => value.error_code,
            #[cfg(feature = "glx")]
            Error::GlxUnsupportedPrivateRequest(value) => value.error_code,
            #[cfg(feature = "randr")]
            Error::RandrBadCrtc(value) => value.error_code,
            #[cfg(feature = "randr")]
            Error::RandrBadMode(value) => value.error_code,
            #[cfg(feature = "randr")]
            Error::RandrBadOutput(value) => value.error_code,
            #[cfg(feature = "randr")]
            Error::RandrBadProvider(value) => value.error_code,
            #[cfg(feature = "record")]
            Error::RecordBadContext(value) => value.error_code,
            #[cfg(feature = "render")]
            Error::RenderGlyph(value) => value.error_code,
            #[cfg(feature = "render")]
            Error::RenderGlyphSet(value) => value.error_code,
            #[cfg(feature = "render")]
            Error::RenderPictFormat(value) => value.error_code,
            #[cfg(feature = "render")]
            Error::RenderPictOp(value) => value.error_code,
            #[cfg(feature = "render")]
            Error::RenderPicture(value) => value.error_code,
            #[cfg(feature = "shm")]
            Error::ShmBadSeg(value) => value.error_code,
            #[cfg(feature = "sync")]
            Error::SyncAlarm(value) => value.error_code,
            #[cfg(feature = "sync")]
            Error::SyncCounter(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadClock(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadHTimings(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadVTimings(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeClientNotLocal(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeExtensionDisabled(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeModeUnsuitable(value) => value.error_code,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeZoomLocked(value) => value.error_code,
            #[cfg(feature = "xfixes")]
            Error::XfixesBadRegion(value) => value.error_code,
            #[cfg(feature = "xinput")]
            Error::XinputClass(value) => value.error_code,
            #[cfg(feature = "xinput")]
            Error::XinputDevice(value) => value.error_code,
            #[cfg(feature = "xinput")]
            Error::XinputDeviceBusy(value) => value.error_code,
            #[cfg(feature = "xinput")]
            Error::XinputEvent(value) => value.error_code,
            #[cfg(feature = "xinput")]
            Error::XinputMode(value) => value.error_code,
            #[cfg(feature = "xkb")]
            Error::XkbKeyboard(value) => value.error_code,
            #[cfg(feature = "xprint")]
            Error::XprintBadContext(value) => value.error_code,
            #[cfg(feature = "xprint")]
            Error::XprintBadSequence(value) => value.error_code,
            #[cfg(feature = "xv")]
            Error::XvBadControl(value) => value.error_code,
            #[cfg(feature = "xv")]
            Error::XvBadEncoding(value) => value.error_code,
            #[cfg(feature = "xv")]
            Error::XvBadPort(value) => value.error_code,
        }
    }

    /// Get the response type of this X11 error
    ///
    /// This is not `pub` because it should always be `0` for errors.
    fn raw_response_type(&self) -> u8 {
        match self {
            Error::Unknown(value) => response_type(value).unwrap(),
            _ => 0
        }
    }
}

/// Enumeration of all possible X11 events.
#[derive(Debug, Clone)]
pub enum Event {
    Unknown(Vec<u8>),
    Error(Error),
    ButtonPress(xproto::ButtonPressEvent),
    ButtonRelease(xproto::ButtonReleaseEvent),
    CirculateNotify(xproto::CirculateNotifyEvent),
    CirculateRequest(xproto::CirculateRequestEvent),
    ClientMessage(xproto::ClientMessageEvent),
    ColormapNotify(xproto::ColormapNotifyEvent),
    ConfigureNotify(xproto::ConfigureNotifyEvent),
    ConfigureRequest(xproto::ConfigureRequestEvent),
    CreateNotify(xproto::CreateNotifyEvent),
    DestroyNotify(xproto::DestroyNotifyEvent),
    EnterNotify(xproto::EnterNotifyEvent),
    Expose(xproto::ExposeEvent),
    FocusIn(xproto::FocusInEvent),
    FocusOut(xproto::FocusOutEvent),
    GeGeneric(xproto::GeGenericEvent),
    GraphicsExposure(xproto::GraphicsExposureEvent),
    GravityNotify(xproto::GravityNotifyEvent),
    KeyPress(xproto::KeyPressEvent),
    KeyRelease(xproto::KeyReleaseEvent),
    KeymapNotify(xproto::KeymapNotifyEvent),
    LeaveNotify(xproto::LeaveNotifyEvent),
    MapNotify(xproto::MapNotifyEvent),
    MapRequest(xproto::MapRequestEvent),
    MappingNotify(xproto::MappingNotifyEvent),
    MotionNotify(xproto::MotionNotifyEvent),
    NoExposure(xproto::NoExposureEvent),
    PropertyNotify(xproto::PropertyNotifyEvent),
    ReparentNotify(xproto::ReparentNotifyEvent),
    ResizeRequest(xproto::ResizeRequestEvent),
    SelectionClear(xproto::SelectionClearEvent),
    SelectionNotify(xproto::SelectionNotifyEvent),
    SelectionRequest(xproto::SelectionRequestEvent),
    UnmapNotify(xproto::UnmapNotifyEvent),
    VisibilityNotify(xproto::VisibilityNotifyEvent),
    #[cfg(feature = "damage")]
    DamageNotify(damage::NotifyEvent),
    #[cfg(feature = "dri2")]
    Dri2BufferSwapComplete(dri2::BufferSwapCompleteEvent),
    #[cfg(feature = "dri2")]
    Dri2InvalidateBuffers(dri2::InvalidateBuffersEvent),
    #[cfg(feature = "glx")]
    GlxBufferSwapComplete(glx::BufferSwapCompleteEvent),
    #[cfg(feature = "glx")]
    GlxPbufferClobber(glx::PbufferClobberEvent),
    #[cfg(feature = "present")]
    PresentCompleteNotify(present::CompleteNotifyEvent),
    #[cfg(feature = "present")]
    PresentConfigureNotify(present::ConfigureNotifyEvent),
    #[cfg(feature = "present")]
    PresentGeneric(present::GenericEvent),
    #[cfg(feature = "present")]
    PresentIdleNotify(present::IdleNotifyEvent),
    #[cfg(feature = "present")]
    PresentRedirectNotify(present::RedirectNotifyEvent),
    #[cfg(feature = "randr")]
    RandrNotify(randr::NotifyEvent),
    #[cfg(feature = "randr")]
    RandrScreenChangeNotify(randr::ScreenChangeNotifyEvent),
    #[cfg(feature = "screensaver")]
    ScreensaverNotify(screensaver::NotifyEvent),
    #[cfg(feature = "shape")]
    ShapeNotify(shape::NotifyEvent),
    #[cfg(feature = "shm")]
    ShmCompletion(shm::CompletionEvent),
    #[cfg(feature = "sync")]
    SyncAlarmNotify(sync::AlarmNotifyEvent),
    #[cfg(feature = "sync")]
    SyncCounterNotify(sync::CounterNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesCursorNotify(xfixes::CursorNotifyEvent),
    #[cfg(feature = "xfixes")]
    XfixesSelectionNotify(xfixes::SelectionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierHit(xinput::BarrierHitEvent),
    #[cfg(feature = "xinput")]
    XinputBarrierLeave(xinput::BarrierLeaveEvent),
    #[cfg(feature = "xinput")]
    XinputButtonPress(xinput::ButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputButtonRelease(xinput::ButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputChangeDeviceNotify(xinput::ChangeDeviceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonPress(xinput::DeviceButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonRelease(xinput::DeviceButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceButtonStateNotify(xinput::DeviceButtonStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceChanged(xinput::DeviceChangedEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusIn(xinput::DeviceFocusInEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceFocusOut(xinput::DeviceFocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyPress(xinput::DeviceKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyRelease(xinput::DeviceKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceKeyStateNotify(xinput::DeviceKeyStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMappingNotify(xinput::DeviceMappingNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceMotionNotify(xinput::DeviceMotionNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePresenceNotify(xinput::DevicePresenceNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDevicePropertyNotify(xinput::DevicePropertyNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceStateNotify(xinput::DeviceStateNotifyEvent),
    #[cfg(feature = "xinput")]
    XinputDeviceValuator(xinput::DeviceValuatorEvent),
    #[cfg(feature = "xinput")]
    XinputEnter(xinput::EnterEvent),
    #[cfg(feature = "xinput")]
    XinputFocusIn(xinput::FocusInEvent),
    #[cfg(feature = "xinput")]
    XinputFocusOut(xinput::FocusOutEvent),
    #[cfg(feature = "xinput")]
    XinputHierarchy(xinput::HierarchyEvent),
    #[cfg(feature = "xinput")]
    XinputKeyPress(xinput::KeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputKeyRelease(xinput::KeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputLeave(xinput::LeaveEvent),
    #[cfg(feature = "xinput")]
    XinputMotion(xinput::MotionEvent),
    #[cfg(feature = "xinput")]
    XinputProperty(xinput::PropertyEvent),
    #[cfg(feature = "xinput")]
    XinputProximityIn(xinput::ProximityInEvent),
    #[cfg(feature = "xinput")]
    XinputProximityOut(xinput::ProximityOutEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonPress(xinput::RawButtonPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawButtonRelease(xinput::RawButtonReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyPress(xinput::RawKeyPressEvent),
    #[cfg(feature = "xinput")]
    XinputRawKeyRelease(xinput::RawKeyReleaseEvent),
    #[cfg(feature = "xinput")]
    XinputRawMotion(xinput::RawMotionEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchBegin(xinput::RawTouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchEnd(xinput::RawTouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputRawTouchUpdate(xinput::RawTouchUpdateEvent),
    #[cfg(feature = "xinput")]
    XinputTouchBegin(xinput::TouchBeginEvent),
    #[cfg(feature = "xinput")]
    XinputTouchEnd(xinput::TouchEndEvent),
    #[cfg(feature = "xinput")]
    XinputTouchOwnership(xinput::TouchOwnershipEvent),
    #[cfg(feature = "xinput")]
    XinputTouchUpdate(xinput::TouchUpdateEvent),
    #[cfg(feature = "xkb")]
    XkbAccessXNotify(xkb::AccessXNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbActionMessage(xkb::ActionMessageEvent),
    #[cfg(feature = "xkb")]
    XkbBellNotify(xkb::BellNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbCompatMapNotify(xkb::CompatMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbControlsNotify(xkb::ControlsNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbExtensionDeviceNotify(xkb::ExtensionDeviceNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorMapNotify(xkb::IndicatorMapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbIndicatorStateNotify(xkb::IndicatorStateNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbMapNotify(xkb::MapNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNamesNotify(xkb::NamesNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbNewKeyboardNotify(xkb::NewKeyboardNotifyEvent),
    #[cfg(feature = "xkb")]
    XkbStateNotify(xkb::StateNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintAttributNotify(xprint::AttributNotifyEvent),
    #[cfg(feature = "xprint")]
    XprintNotify(xprint::NotifyEvent),
    #[cfg(feature = "xv")]
    XvPortNotify(xv::PortNotifyEvent),
    #[cfg(feature = "xv")]
    XvVideoNotify(xv::VideoNotifyEvent),
}

impl Event {
    /// Parse a generic X11 event into a concrete event type.
    #[allow(clippy::cognitive_complexity, clippy::match_single_binding)]
    pub fn parse(
        event: &[u8],
        ext_info_provider: &dyn ExtInfoProvider,
    ) -> Result<Self, ParseError> {
        let event_code = response_type(event)?;

        // Check if this is a core protocol event or error, or from the generic event extension
        match event_code {
            0 => return Ok(Self::Error(Error::parse(event, ext_info_provider)?)),
            xproto::BUTTON_PRESS_EVENT => return xproto::ButtonPressEvent::ugly_hack(event),
            xproto::BUTTON_RELEASE_EVENT => return xproto::ButtonReleaseEvent::ugly_hack(event),
            xproto::CIRCULATE_NOTIFY_EVENT => return xproto::CirculateNotifyEvent::ugly_hack(event),
            xproto::CIRCULATE_REQUEST_EVENT => return xproto::CirculateRequestEvent::ugly_hack(event),
            xproto::CLIENT_MESSAGE_EVENT => return xproto::ClientMessageEvent::ugly_hack(event),
            xproto::COLORMAP_NOTIFY_EVENT => return xproto::ColormapNotifyEvent::ugly_hack(event),
            xproto::CONFIGURE_NOTIFY_EVENT => return xproto::ConfigureNotifyEvent::ugly_hack(event),
            xproto::CONFIGURE_REQUEST_EVENT => return xproto::ConfigureRequestEvent::ugly_hack(event),
            xproto::CREATE_NOTIFY_EVENT => return xproto::CreateNotifyEvent::ugly_hack(event),
            xproto::DESTROY_NOTIFY_EVENT => return xproto::DestroyNotifyEvent::ugly_hack(event),
            xproto::ENTER_NOTIFY_EVENT => return xproto::EnterNotifyEvent::ugly_hack(event),
            xproto::EXPOSE_EVENT => return xproto::ExposeEvent::ugly_hack(event),
            xproto::FOCUS_IN_EVENT => return xproto::FocusInEvent::ugly_hack(event),
            xproto::FOCUS_OUT_EVENT => return xproto::FocusOutEvent::ugly_hack(event),
            xproto::GRAPHICS_EXPOSURE_EVENT => return xproto::GraphicsExposureEvent::ugly_hack(event),
            xproto::GRAVITY_NOTIFY_EVENT => return xproto::GravityNotifyEvent::ugly_hack(event),
            xproto::KEY_PRESS_EVENT => return xproto::KeyPressEvent::ugly_hack(event),
            xproto::KEY_RELEASE_EVENT => return xproto::KeyReleaseEvent::ugly_hack(event),
            xproto::KEYMAP_NOTIFY_EVENT => return xproto::KeymapNotifyEvent::ugly_hack(event),
            xproto::LEAVE_NOTIFY_EVENT => return xproto::LeaveNotifyEvent::ugly_hack(event),
            xproto::MAP_NOTIFY_EVENT => return xproto::MapNotifyEvent::ugly_hack(event),
            xproto::MAP_REQUEST_EVENT => return xproto::MapRequestEvent::ugly_hack(event),
            xproto::MAPPING_NOTIFY_EVENT => return xproto::MappingNotifyEvent::ugly_hack(event),
            xproto::MOTION_NOTIFY_EVENT => return xproto::MotionNotifyEvent::ugly_hack(event),
            xproto::NO_EXPOSURE_EVENT => return xproto::NoExposureEvent::ugly_hack(event),
            xproto::PROPERTY_NOTIFY_EVENT => return xproto::PropertyNotifyEvent::ugly_hack(event),
            xproto::REPARENT_NOTIFY_EVENT => return xproto::ReparentNotifyEvent::ugly_hack(event),
            xproto::RESIZE_REQUEST_EVENT => return xproto::ResizeRequestEvent::ugly_hack(event),
            xproto::SELECTION_CLEAR_EVENT => return xproto::SelectionClearEvent::ugly_hack(event),
            xproto::SELECTION_NOTIFY_EVENT => return xproto::SelectionNotifyEvent::ugly_hack(event),
            xproto::SELECTION_REQUEST_EVENT => return xproto::SelectionRequestEvent::ugly_hack(event),
            xproto::UNMAP_NOTIFY_EVENT => return xproto::UnmapNotifyEvent::ugly_hack(event),
            xproto::VISIBILITY_NOTIFY_EVENT => return xproto::VisibilityNotifyEvent::ugly_hack(event),
            xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, ext_info_provider),
            _ => {}
        }
        // Find the extension that this event could belong to
        let ext_info = ext_info_provider.get_from_event_code(event_code);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    damage::NOTIFY_EVENT => damage::NotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "dri2")]
            Some((dri2::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    dri2::BUFFER_SWAP_COMPLETE_EVENT => dri2::BufferSwapCompleteEvent::ugly_hack(event),
                    dri2::INVALIDATE_BUFFERS_EVENT => dri2::InvalidateBuffersEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    glx::BUFFER_SWAP_COMPLETE_EVENT => glx::BufferSwapCompleteEvent::ugly_hack(event),
                    glx::PBUFFER_CLOBBER_EVENT => glx::PbufferClobberEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "present")]
            Some((present::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    present::GENERIC_EVENT => present::GenericEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    randr::NOTIFY_EVENT => randr::NotifyEvent::ugly_hack(event),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => randr::ScreenChangeNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "screensaver")]
            Some((screensaver::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    screensaver::NOTIFY_EVENT => screensaver::NotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shape")]
            Some((shape::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    shape::NOTIFY_EVENT => shape::NotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    shm::COMPLETION_EVENT => shm::CompletionEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    sync::ALARM_NOTIFY_EVENT => sync::AlarmNotifyEvent::ugly_hack(event),
                    sync::COUNTER_NOTIFY_EVENT => sync::CounterNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => xfixes::CursorNotifyEvent::ugly_hack(event),
                    xfixes::SELECTION_NOTIFY_EVENT => xfixes::SelectionNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => xinput::ChangeDeviceNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => xinput::DeviceButtonPressEvent::ugly_hack(event),
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => xinput::DeviceButtonReleaseEvent::ugly_hack(event),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => xinput::DeviceButtonStateNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_FOCUS_IN_EVENT => xinput::DeviceFocusInEvent::ugly_hack(event),
                    xinput::DEVICE_FOCUS_OUT_EVENT => xinput::DeviceFocusOutEvent::ugly_hack(event),
                    xinput::DEVICE_KEY_PRESS_EVENT => xinput::DeviceKeyPressEvent::ugly_hack(event),
                    xinput::DEVICE_KEY_RELEASE_EVENT => xinput::DeviceKeyReleaseEvent::ugly_hack(event),
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => xinput::DeviceKeyStateNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => xinput::DeviceMappingNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => xinput::DeviceMotionNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => xinput::DevicePresenceNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => xinput::DevicePropertyNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => xinput::DeviceStateNotifyEvent::ugly_hack(event),
                    xinput::DEVICE_VALUATOR_EVENT => xinput::DeviceValuatorEvent::ugly_hack(event),
                    xinput::PROXIMITY_IN_EVENT => xinput::ProximityInEvent::ugly_hack(event),
                    xinput::PROXIMITY_OUT_EVENT => xinput::ProximityOutEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, ext_info)) => {
                if event_code != ext_info.first_event {
                    return Ok(Self::Unknown(event.to_vec()));
                }
                match *event.get(1).ok_or(ParseError::ParseError)? {
                    xkb::ACCESS_X_NOTIFY_EVENT => xkb::AccessXNotifyEvent::ugly_hack(event),
                    xkb::ACTION_MESSAGE_EVENT => xkb::ActionMessageEvent::ugly_hack(event),
                    xkb::BELL_NOTIFY_EVENT => xkb::BellNotifyEvent::ugly_hack(event),
                    xkb::COMPAT_MAP_NOTIFY_EVENT => xkb::CompatMapNotifyEvent::ugly_hack(event),
                    xkb::CONTROLS_NOTIFY_EVENT => xkb::ControlsNotifyEvent::ugly_hack(event),
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => xkb::ExtensionDeviceNotifyEvent::ugly_hack(event),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => xkb::IndicatorMapNotifyEvent::ugly_hack(event),
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => xkb::IndicatorStateNotifyEvent::ugly_hack(event),
                    xkb::MAP_NOTIFY_EVENT => xkb::MapNotifyEvent::ugly_hack(event),
                    xkb::NAMES_NOTIFY_EVENT => xkb::NamesNotifyEvent::ugly_hack(event),
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => xkb::NewKeyboardNotifyEvent::ugly_hack(event),
                    xkb::STATE_NOTIFY_EVENT => xkb::StateNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => xprint::AttributNotifyEvent::ugly_hack(event),
                    xprint::NOTIFY_EVENT => xprint::NotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xv::PORT_NOTIFY_EVENT => xv::PortNotifyEvent::ugly_hack(event),
                    xv::VIDEO_NOTIFY_EVENT => xv::VideoNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            _ => Ok(Self::Unknown(event.to_vec())),
        }
    }

    #[allow(clippy::match_single_binding)]
    fn from_generic_event(
        event: &[u8],
        ext_info_provider: &dyn ExtInfoProvider,
    ) -> Result<Self, ParseError> {
        let ge_event = xproto::GeGenericEvent::try_from(event)?;
        let ext_name = ext_info_provider
            .get_from_major_opcode(ge_event.extension)
            .map(|(name, _)| name);
        match ext_name {
            #[cfg(feature = "present")]
            Some(present::X11_EXTENSION_NAME) => {
                match ge_event.event_type {
                    present::COMPLETE_NOTIFY_EVENT => present::CompleteNotifyEvent::ugly_hack(event),
                    present::CONFIGURE_NOTIFY_EVENT => present::ConfigureNotifyEvent::ugly_hack(event),
                    present::IDLE_NOTIFY_EVENT => present::IdleNotifyEvent::ugly_hack(event),
                    present::REDIRECT_NOTIFY_EVENT => present::RedirectNotifyEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some(xinput::X11_EXTENSION_NAME) => {
                match ge_event.event_type {
                    xinput::BARRIER_HIT_EVENT => xinput::BarrierHitEvent::ugly_hack(event),
                    xinput::BARRIER_LEAVE_EVENT => xinput::BarrierLeaveEvent::ugly_hack(event),
                    xinput::BUTTON_PRESS_EVENT => xinput::ButtonPressEvent::ugly_hack(event),
                    xinput::BUTTON_RELEASE_EVENT => xinput::ButtonReleaseEvent::ugly_hack(event),
                    xinput::DEVICE_CHANGED_EVENT => xinput::DeviceChangedEvent::ugly_hack(event),
                    xinput::ENTER_EVENT => xinput::EnterEvent::ugly_hack(event),
                    xinput::FOCUS_IN_EVENT => xinput::FocusInEvent::ugly_hack(event),
                    xinput::FOCUS_OUT_EVENT => xinput::FocusOutEvent::ugly_hack(event),
                    xinput::HIERARCHY_EVENT => xinput::HierarchyEvent::ugly_hack(event),
                    xinput::KEY_PRESS_EVENT => xinput::KeyPressEvent::ugly_hack(event),
                    xinput::KEY_RELEASE_EVENT => xinput::KeyReleaseEvent::ugly_hack(event),
                    xinput::LEAVE_EVENT => xinput::LeaveEvent::ugly_hack(event),
                    xinput::MOTION_EVENT => xinput::MotionEvent::ugly_hack(event),
                    xinput::PROPERTY_EVENT => xinput::PropertyEvent::ugly_hack(event),
                    xinput::RAW_BUTTON_PRESS_EVENT => xinput::RawButtonPressEvent::ugly_hack(event),
                    xinput::RAW_BUTTON_RELEASE_EVENT => xinput::RawButtonReleaseEvent::ugly_hack(event),
                    xinput::RAW_KEY_PRESS_EVENT => xinput::RawKeyPressEvent::ugly_hack(event),
                    xinput::RAW_KEY_RELEASE_EVENT => xinput::RawKeyReleaseEvent::ugly_hack(event),
                    xinput::RAW_MOTION_EVENT => xinput::RawMotionEvent::ugly_hack(event),
                    xinput::RAW_TOUCH_BEGIN_EVENT => xinput::RawTouchBeginEvent::ugly_hack(event),
                    xinput::RAW_TOUCH_END_EVENT => xinput::RawTouchEndEvent::ugly_hack(event),
                    xinput::RAW_TOUCH_UPDATE_EVENT => xinput::RawTouchUpdateEvent::ugly_hack(event),
                    xinput::TOUCH_BEGIN_EVENT => xinput::TouchBeginEvent::ugly_hack(event),
                    xinput::TOUCH_END_EVENT => xinput::TouchEndEvent::ugly_hack(event),
                    xinput::TOUCH_OWNERSHIP_EVENT => xinput::TouchOwnershipEvent::ugly_hack(event),
                    xinput::TOUCH_UPDATE_EVENT => xinput::TouchUpdateEvent::ugly_hack(event),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            _ => Ok(Self::Unknown(event.to_vec())),
        }
    }

    /// Get the sequence number contained in this X11 event
    pub fn wire_sequence_number(&self) -> Option<u16> {
        match self {
            Event::Unknown(value) => sequence_number(value).ok(),
            Event::Error(value) => Some(value.wire_sequence_number()),
            Event::ButtonPress(value) => Some(value.sequence),
            Event::ButtonRelease(value) => Some(value.sequence),
            Event::CirculateNotify(value) => Some(value.sequence),
            Event::CirculateRequest(value) => Some(value.sequence),
            Event::ClientMessage(value) => Some(value.sequence),
            Event::ColormapNotify(value) => Some(value.sequence),
            Event::ConfigureNotify(value) => Some(value.sequence),
            Event::ConfigureRequest(value) => Some(value.sequence),
            Event::CreateNotify(value) => Some(value.sequence),
            Event::DestroyNotify(value) => Some(value.sequence),
            Event::EnterNotify(value) => Some(value.sequence),
            Event::Expose(value) => Some(value.sequence),
            Event::FocusIn(value) => Some(value.sequence),
            Event::FocusOut(value) => Some(value.sequence),
            Event::GeGeneric(value) => Some(value.sequence),
            Event::GraphicsExposure(value) => Some(value.sequence),
            Event::GravityNotify(value) => Some(value.sequence),
            Event::KeyPress(value) => Some(value.sequence),
            Event::KeyRelease(value) => Some(value.sequence),
            Event::KeymapNotify(_) => None,
            Event::LeaveNotify(value) => Some(value.sequence),
            Event::MapNotify(value) => Some(value.sequence),
            Event::MapRequest(value) => Some(value.sequence),
            Event::MappingNotify(value) => Some(value.sequence),
            Event::MotionNotify(value) => Some(value.sequence),
            Event::NoExposure(value) => Some(value.sequence),
            Event::PropertyNotify(value) => Some(value.sequence),
            Event::ReparentNotify(value) => Some(value.sequence),
            Event::ResizeRequest(value) => Some(value.sequence),
            Event::SelectionClear(value) => Some(value.sequence),
            Event::SelectionNotify(value) => Some(value.sequence),
            Event::SelectionRequest(value) => Some(value.sequence),
            Event::UnmapNotify(value) => Some(value.sequence),
            Event::VisibilityNotify(value) => Some(value.sequence),
            #[cfg(feature = "damage")]
            Event::DamageNotify(value) => Some(value.sequence),
            #[cfg(feature = "dri2")]
            Event::Dri2BufferSwapComplete(value) => Some(value.sequence),
            #[cfg(feature = "dri2")]
            Event::Dri2InvalidateBuffers(value) => Some(value.sequence),
            #[cfg(feature = "glx")]
            Event::GlxBufferSwapComplete(value) => Some(value.sequence),
            #[cfg(feature = "glx")]
            Event::GlxPbufferClobber(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentCompleteNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentConfigureNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentGeneric(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentIdleNotify(value) => Some(value.sequence),
            #[cfg(feature = "present")]
            Event::PresentRedirectNotify(value) => Some(value.sequence),
            #[cfg(feature = "randr")]
            Event::RandrNotify(value) => Some(value.sequence),
            #[cfg(feature = "randr")]
            Event::RandrScreenChangeNotify(value) => Some(value.sequence),
            #[cfg(feature = "screensaver")]
            Event::ScreensaverNotify(value) => Some(value.sequence),
            #[cfg(feature = "shape")]
            Event::ShapeNotify(value) => Some(value.sequence),
            #[cfg(feature = "shm")]
            Event::ShmCompletion(value) => Some(value.sequence),
            #[cfg(feature = "sync")]
            Event::SyncAlarmNotify(value) => Some(value.sequence),
            #[cfg(feature = "sync")]
            Event::SyncCounterNotify(value) => Some(value.sequence),
            #[cfg(feature = "xfixes")]
            Event::XfixesCursorNotify(value) => Some(value.sequence),
            #[cfg(feature = "xfixes")]
            Event::XfixesSelectionNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputBarrierHit(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputBarrierLeave(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputChangeDeviceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceChanged(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMappingNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMotionNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDevicePresenceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDevicePropertyNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputDeviceValuator(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputEnter(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputFocusIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputFocusOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputHierarchy(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputLeave(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputMotion(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProperty(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProximityIn(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputProximityOut(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyPress(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyRelease(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawMotion(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchBegin(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchEnd(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchUpdate(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchBegin(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchEnd(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchOwnership(value) => Some(value.sequence),
            #[cfg(feature = "xinput")]
            Event::XinputTouchUpdate(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbAccessXNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbActionMessage(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbBellNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbCompatMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbControlsNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbExtensionDeviceNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbMapNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbNamesNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbNewKeyboardNotify(value) => Some(value.sequence),
            #[cfg(feature = "xkb")]
            Event::XkbStateNotify(value) => Some(value.sequence),
            #[cfg(feature = "xprint")]
            Event::XprintAttributNotify(value) => Some(value.sequence),
            #[cfg(feature = "xprint")]
            Event::XprintNotify(value) => Some(value.sequence),
            #[cfg(feature = "xv")]
            Event::XvPortNotify(value) => Some(value.sequence),
            #[cfg(feature = "xv")]
            Event::XvVideoNotify(value) => Some(value.sequence),
        }
    }

    /// Get the raw response type of this X11 event
    ///
    /// Response types have seven bits in X11. The eight bit indicates whether
    /// the packet was generated through the `SendEvent` request. This function
    /// returns all eight bits.
    ///
    /// See also the `response_type()`, `server_generated()` and `sent_event()` methods.
    pub fn raw_response_type(&self) -> u8 {
        match self {
            Event::Unknown(value) => response_type(value).unwrap(),
            Event::Error(value) => value.raw_response_type(),
            Event::ButtonPress(value) => value.response_type,
            Event::ButtonRelease(value) => value.response_type,
            Event::CirculateNotify(value) => value.response_type,
            Event::CirculateRequest(value) => value.response_type,
            Event::ClientMessage(value) => value.response_type,
            Event::ColormapNotify(value) => value.response_type,
            Event::ConfigureNotify(value) => value.response_type,
            Event::ConfigureRequest(value) => value.response_type,
            Event::CreateNotify(value) => value.response_type,
            Event::DestroyNotify(value) => value.response_type,
            Event::EnterNotify(value) => value.response_type,
            Event::Expose(value) => value.response_type,
            Event::FocusIn(value) => value.response_type,
            Event::FocusOut(value) => value.response_type,
            Event::GeGeneric(value) => value.response_type,
            Event::GraphicsExposure(value) => value.response_type,
            Event::GravityNotify(value) => value.response_type,
            Event::KeyPress(value) => value.response_type,
            Event::KeyRelease(value) => value.response_type,
            Event::KeymapNotify(value) => value.response_type,
            Event::LeaveNotify(value) => value.response_type,
            Event::MapNotify(value) => value.response_type,
            Event::MapRequest(value) => value.response_type,
            Event::MappingNotify(value) => value.response_type,
            Event::MotionNotify(value) => value.response_type,
            Event::NoExposure(value) => value.response_type,
            Event::PropertyNotify(value) => value.response_type,
            Event::ReparentNotify(value) => value.response_type,
            Event::ResizeRequest(value) => value.response_type,
            Event::SelectionClear(value) => value.response_type,
            Event::SelectionNotify(value) => value.response_type,
            Event::SelectionRequest(value) => value.response_type,
            Event::UnmapNotify(value) => value.response_type,
            Event::VisibilityNotify(value) => value.response_type,
            #[cfg(feature = "damage")]
            Event::DamageNotify(value) => value.response_type,
            #[cfg(feature = "dri2")]
            Event::Dri2BufferSwapComplete(value) => value.response_type,
            #[cfg(feature = "dri2")]
            Event::Dri2InvalidateBuffers(value) => value.response_type,
            #[cfg(feature = "glx")]
            Event::GlxBufferSwapComplete(value) => value.response_type,
            #[cfg(feature = "glx")]
            Event::GlxPbufferClobber(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentCompleteNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentConfigureNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentGeneric(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentIdleNotify(value) => value.response_type,
            #[cfg(feature = "present")]
            Event::PresentRedirectNotify(value) => value.response_type,
            #[cfg(feature = "randr")]
            Event::RandrNotify(value) => value.response_type,
            #[cfg(feature = "randr")]
            Event::RandrScreenChangeNotify(value) => value.response_type,
            #[cfg(feature = "screensaver")]
            Event::ScreensaverNotify(value) => value.response_type,
            #[cfg(feature = "shape")]
            Event::ShapeNotify(value) => value.response_type,
            #[cfg(feature = "shm")]
            Event::ShmCompletion(value) => value.response_type,
            #[cfg(feature = "sync")]
            Event::SyncAlarmNotify(value) => value.response_type,
            #[cfg(feature = "sync")]
            Event::SyncCounterNotify(value) => value.response_type,
            #[cfg(feature = "xfixes")]
            Event::XfixesCursorNotify(value) => value.response_type,
            #[cfg(feature = "xfixes")]
            Event::XfixesSelectionNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputBarrierHit(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputBarrierLeave(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputChangeDeviceNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceButtonStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceChanged(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceFocusOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceKeyStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMappingNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceMotionNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDevicePresenceNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDevicePropertyNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceStateNotify(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputDeviceValuator(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputEnter(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputFocusIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputFocusOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputHierarchy(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputLeave(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputMotion(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProperty(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProximityIn(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputProximityOut(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawButtonRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyPress(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawKeyRelease(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawMotion(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchBegin(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchEnd(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputRawTouchUpdate(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchBegin(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchEnd(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchOwnership(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Event::XinputTouchUpdate(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbAccessXNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbActionMessage(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbBellNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbCompatMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbControlsNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbExtensionDeviceNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbIndicatorStateNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbMapNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbNamesNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbNewKeyboardNotify(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Event::XkbStateNotify(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Event::XprintAttributNotify(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Event::XprintNotify(value) => value.response_type,
            #[cfg(feature = "xv")]
            Event::XvPortNotify(value) => value.response_type,
            #[cfg(feature = "xv")]
            Event::XvVideoNotify(value) => value.response_type,
        }
    }

    /// Get the response type of this X11 event
    pub fn response_type(&self) -> u8 {
        self.raw_response_type() & 0x7f
    }

    /// Was this event generated by the X11 server?
    ///
    /// If this function returns true, then this event comes from the X11 server.
    /// Otherwise, it was sent from another client via the `SendEvent` request.
    pub fn server_generated(&self) -> bool {
        self.raw_response_type() & 0x80 == 0
    }

    /// Was this event generated by another X11 client?
    ///
    /// If this function returns true, then this event comes from another client via
    /// the `SendEvent` request. Otherwise, it was generated by the X11 server.
    pub fn sent_event(&self) -> bool {
        self.raw_response_type() & 0x80 != 0
    }
}

/// Get the response type out of the raw bytes of an X11 error or event.
fn response_type(raw_bytes: &[u8]) -> Result<u8, ParseError> {
    raw_bytes.get(0)
        .map(|x| x & 0x7f)
        .ok_or(ParseError::ParseError)
}

/// Get the error code out of the raw bytes of an X11 error.
fn error_code(raw_bytes: &[u8]) -> Result<u8, ParseError> {
    raw_bytes.get(1)
        .copied()
        .ok_or(ParseError::ParseError)
}

/// Get the sequence number out of an X11 packet.
fn sequence_number(raw_bytes: &[u8]) -> Result<u16, ParseError> {
    raw_bytes.get(2..4)
        .map(|b| u16::from_ne_bytes(b.try_into().unwrap()))
        .ok_or(ParseError::ParseError)
}

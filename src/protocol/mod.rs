// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

//! Bindings to the X11 protocol.
//!
//! Each sub-module of this module corresponds to one X11 extension. It contains all the
//! definitions from that extension. The core X11 protocol is in [`xproto`](xproto/index.html).

use std::convert::{TryFrom, TryInto};
use crate::errors::ParseError;
use crate::utils::RawFdContainer;
use crate::x11_utils::{parse_request_header, BigRequests, ExtInfoProvider, RequestHeader};

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
    Unknown(RequestHeader, &'input [u8]),
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
        request: &'input [u8],
        fds: &mut Vec<RawFdContainer>,
        big_requests_enabled: BigRequests,
        ext_info_provider: &dyn ExtInfoProvider,
    ) -> Result<Self, ParseError> {
        // Might not be used if none of the extensions that use FD passing is enabled
        // The `allow` is not in the function argument because it is not stable in Rust 1.37
        #[allow(unused_variables)]
        let fds = fds;
        let (header, remaining) = parse_request_header(request, big_requests_enabled)?;
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
        Ok(Request::Unknown(header, remaining))
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
            Error::Access(value) => value.response_type,
            Error::Alloc(value) => value.response_type,
            Error::Atom(value) => value.response_type,
            Error::Colormap(value) => value.response_type,
            Error::Cursor(value) => value.response_type,
            Error::Drawable(value) => value.response_type,
            Error::Font(value) => value.response_type,
            Error::GContext(value) => value.response_type,
            Error::IDChoice(value) => value.response_type,
            Error::Implementation(value) => value.response_type,
            Error::Length(value) => value.response_type,
            Error::Match(value) => value.response_type,
            Error::Name(value) => value.response_type,
            Error::Pixmap(value) => value.response_type,
            Error::Request(value) => value.response_type,
            Error::Value(value) => value.response_type,
            Error::Window(value) => value.response_type,
            #[cfg(feature = "damage")]
            Error::DamageBadDamage(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadContext(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadContextState(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadContextTag(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentDrawable(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadCurrentWindow(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadDrawable(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadFBConfig(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadLargeRequest(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadPbuffer(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadPixmap(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadRenderRequest(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxBadWindow(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxGLXBadProfileARB(value) => value.response_type,
            #[cfg(feature = "glx")]
            Error::GlxUnsupportedPrivateRequest(value) => value.response_type,
            #[cfg(feature = "randr")]
            Error::RandrBadCrtc(value) => value.response_type,
            #[cfg(feature = "randr")]
            Error::RandrBadMode(value) => value.response_type,
            #[cfg(feature = "randr")]
            Error::RandrBadOutput(value) => value.response_type,
            #[cfg(feature = "randr")]
            Error::RandrBadProvider(value) => value.response_type,
            #[cfg(feature = "record")]
            Error::RecordBadContext(value) => value.response_type,
            #[cfg(feature = "render")]
            Error::RenderGlyph(value) => value.response_type,
            #[cfg(feature = "render")]
            Error::RenderGlyphSet(value) => value.response_type,
            #[cfg(feature = "render")]
            Error::RenderPictFormat(value) => value.response_type,
            #[cfg(feature = "render")]
            Error::RenderPictOp(value) => value.response_type,
            #[cfg(feature = "render")]
            Error::RenderPicture(value) => value.response_type,
            #[cfg(feature = "shm")]
            Error::ShmBadSeg(value) => value.response_type,
            #[cfg(feature = "sync")]
            Error::SyncAlarm(value) => value.response_type,
            #[cfg(feature = "sync")]
            Error::SyncCounter(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadClock(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadHTimings(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeBadVTimings(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeClientNotLocal(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeExtensionDisabled(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeModeUnsuitable(value) => value.response_type,
            #[cfg(feature = "xf86vidmode")]
            Error::Xf86vidmodeZoomLocked(value) => value.response_type,
            #[cfg(feature = "xfixes")]
            Error::XfixesBadRegion(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Error::XinputClass(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Error::XinputDevice(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Error::XinputDeviceBusy(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Error::XinputEvent(value) => value.response_type,
            #[cfg(feature = "xinput")]
            Error::XinputMode(value) => value.response_type,
            #[cfg(feature = "xkb")]
            Error::XkbKeyboard(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Error::XprintBadContext(value) => value.response_type,
            #[cfg(feature = "xprint")]
            Error::XprintBadSequence(value) => value.response_type,
            #[cfg(feature = "xv")]
            Error::XvBadControl(value) => value.response_type,
            #[cfg(feature = "xv")]
            Error::XvBadEncoding(value) => value.response_type,
            #[cfg(feature = "xv")]
            Error::XvBadPort(value) => value.response_type,
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
            xproto::BUTTON_PRESS_EVENT => return Ok(Self::ButtonPress(event.try_into()?)),
            xproto::BUTTON_RELEASE_EVENT => return Ok(Self::ButtonRelease(event.try_into()?)),
            xproto::CIRCULATE_NOTIFY_EVENT => return Ok(Self::CirculateNotify(event.try_into()?)),
            xproto::CIRCULATE_REQUEST_EVENT => return Ok(Self::CirculateRequest(event.try_into()?)),
            xproto::CLIENT_MESSAGE_EVENT => return Ok(Self::ClientMessage(event.try_into()?)),
            xproto::COLORMAP_NOTIFY_EVENT => return Ok(Self::ColormapNotify(event.try_into()?)),
            xproto::CONFIGURE_NOTIFY_EVENT => return Ok(Self::ConfigureNotify(event.try_into()?)),
            xproto::CONFIGURE_REQUEST_EVENT => return Ok(Self::ConfigureRequest(event.try_into()?)),
            xproto::CREATE_NOTIFY_EVENT => return Ok(Self::CreateNotify(event.try_into()?)),
            xproto::DESTROY_NOTIFY_EVENT => return Ok(Self::DestroyNotify(event.try_into()?)),
            xproto::ENTER_NOTIFY_EVENT => return Ok(Self::EnterNotify(event.try_into()?)),
            xproto::EXPOSE_EVENT => return Ok(Self::Expose(event.try_into()?)),
            xproto::FOCUS_IN_EVENT => return Ok(Self::FocusIn(event.try_into()?)),
            xproto::FOCUS_OUT_EVENT => return Ok(Self::FocusOut(event.try_into()?)),
            xproto::GRAPHICS_EXPOSURE_EVENT => return Ok(Self::GraphicsExposure(event.try_into()?)),
            xproto::GRAVITY_NOTIFY_EVENT => return Ok(Self::GravityNotify(event.try_into()?)),
            xproto::KEY_PRESS_EVENT => return Ok(Self::KeyPress(event.try_into()?)),
            xproto::KEY_RELEASE_EVENT => return Ok(Self::KeyRelease(event.try_into()?)),
            xproto::KEYMAP_NOTIFY_EVENT => return Ok(Self::KeymapNotify(event.try_into()?)),
            xproto::LEAVE_NOTIFY_EVENT => return Ok(Self::LeaveNotify(event.try_into()?)),
            xproto::MAP_NOTIFY_EVENT => return Ok(Self::MapNotify(event.try_into()?)),
            xproto::MAP_REQUEST_EVENT => return Ok(Self::MapRequest(event.try_into()?)),
            xproto::MAPPING_NOTIFY_EVENT => return Ok(Self::MappingNotify(event.try_into()?)),
            xproto::MOTION_NOTIFY_EVENT => return Ok(Self::MotionNotify(event.try_into()?)),
            xproto::NO_EXPOSURE_EVENT => return Ok(Self::NoExposure(event.try_into()?)),
            xproto::PROPERTY_NOTIFY_EVENT => return Ok(Self::PropertyNotify(event.try_into()?)),
            xproto::REPARENT_NOTIFY_EVENT => return Ok(Self::ReparentNotify(event.try_into()?)),
            xproto::RESIZE_REQUEST_EVENT => return Ok(Self::ResizeRequest(event.try_into()?)),
            xproto::SELECTION_CLEAR_EVENT => return Ok(Self::SelectionClear(event.try_into()?)),
            xproto::SELECTION_NOTIFY_EVENT => return Ok(Self::SelectionNotify(event.try_into()?)),
            xproto::SELECTION_REQUEST_EVENT => return Ok(Self::SelectionRequest(event.try_into()?)),
            xproto::UNMAP_NOTIFY_EVENT => return Ok(Self::UnmapNotify(event.try_into()?)),
            xproto::VISIBILITY_NOTIFY_EVENT => return Ok(Self::VisibilityNotify(event.try_into()?)),
            xproto::GE_GENERIC_EVENT => return Self::from_generic_event(event, ext_info_provider),
            _ => {}
        }
        // Find the extension that this event could belong to
        let ext_info = ext_info_provider.get_from_event_code(event_code);
        match ext_info {
            #[cfg(feature = "damage")]
            Some((damage::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    damage::NOTIFY_EVENT => Ok(Self::DamageNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "dri2")]
            Some((dri2::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    dri2::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::Dri2BufferSwapComplete(event.try_into()?)),
                    dri2::INVALIDATE_BUFFERS_EVENT => Ok(Self::Dri2InvalidateBuffers(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "glx")]
            Some((glx::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    glx::BUFFER_SWAP_COMPLETE_EVENT => Ok(Self::GlxBufferSwapComplete(event.try_into()?)),
                    glx::PBUFFER_CLOBBER_EVENT => Ok(Self::GlxPbufferClobber(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "present")]
            Some((present::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    present::GENERIC_EVENT => Ok(Self::PresentGeneric(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "randr")]
            Some((randr::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    randr::NOTIFY_EVENT => Ok(Self::RandrNotify(event.try_into()?)),
                    randr::SCREEN_CHANGE_NOTIFY_EVENT => Ok(Self::RandrScreenChangeNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "screensaver")]
            Some((screensaver::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    screensaver::NOTIFY_EVENT => Ok(Self::ScreensaverNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shape")]
            Some((shape::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    shape::NOTIFY_EVENT => Ok(Self::ShapeNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "shm")]
            Some((shm::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    shm::COMPLETION_EVENT => Ok(Self::ShmCompletion(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "sync")]
            Some((sync::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    sync::ALARM_NOTIFY_EVENT => Ok(Self::SyncAlarmNotify(event.try_into()?)),
                    sync::COUNTER_NOTIFY_EVENT => Ok(Self::SyncCounterNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xfixes")]
            Some((xfixes::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xfixes::CURSOR_NOTIFY_EVENT => Ok(Self::XfixesCursorNotify(event.try_into()?)),
                    xfixes::SELECTION_NOTIFY_EVENT => Ok(Self::XfixesSelectionNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some((xinput::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xinput::CHANGE_DEVICE_NOTIFY_EVENT => Ok(Self::XinputChangeDeviceNotify(event.try_into()?)),
                    xinput::DEVICE_BUTTON_PRESS_EVENT => Ok(Self::XinputDeviceButtonPress(event.try_into()?)),
                    xinput::DEVICE_BUTTON_RELEASE_EVENT => Ok(Self::XinputDeviceButtonRelease(event.try_into()?)),
                    xinput::DEVICE_BUTTON_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceButtonStateNotify(event.try_into()?)),
                    xinput::DEVICE_FOCUS_IN_EVENT => Ok(Self::XinputDeviceFocusIn(event.try_into()?)),
                    xinput::DEVICE_FOCUS_OUT_EVENT => Ok(Self::XinputDeviceFocusOut(event.try_into()?)),
                    xinput::DEVICE_KEY_PRESS_EVENT => Ok(Self::XinputDeviceKeyPress(event.try_into()?)),
                    xinput::DEVICE_KEY_RELEASE_EVENT => Ok(Self::XinputDeviceKeyRelease(event.try_into()?)),
                    xinput::DEVICE_KEY_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceKeyStateNotify(event.try_into()?)),
                    xinput::DEVICE_MAPPING_NOTIFY_EVENT => Ok(Self::XinputDeviceMappingNotify(event.try_into()?)),
                    xinput::DEVICE_MOTION_NOTIFY_EVENT => Ok(Self::XinputDeviceMotionNotify(event.try_into()?)),
                    xinput::DEVICE_PRESENCE_NOTIFY_EVENT => Ok(Self::XinputDevicePresenceNotify(event.try_into()?)),
                    xinput::DEVICE_PROPERTY_NOTIFY_EVENT => Ok(Self::XinputDevicePropertyNotify(event.try_into()?)),
                    xinput::DEVICE_STATE_NOTIFY_EVENT => Ok(Self::XinputDeviceStateNotify(event.try_into()?)),
                    xinput::DEVICE_VALUATOR_EVENT => Ok(Self::XinputDeviceValuator(event.try_into()?)),
                    xinput::PROXIMITY_IN_EVENT => Ok(Self::XinputProximityIn(event.try_into()?)),
                    xinput::PROXIMITY_OUT_EVENT => Ok(Self::XinputProximityOut(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xkb")]
            Some((xkb::X11_EXTENSION_NAME, ext_info)) => {
                if event_code != ext_info.first_event {
                    return Ok(Self::Unknown(event.to_vec()));
                }
                match *event.get(1).ok_or(ParseError::ParseError)? {
                    xkb::ACCESS_X_NOTIFY_EVENT => Ok(Self::XkbAccessXNotify(event.try_into()?)),
                    xkb::ACTION_MESSAGE_EVENT => Ok(Self::XkbActionMessage(event.try_into()?)),
                    xkb::BELL_NOTIFY_EVENT => Ok(Self::XkbBellNotify(event.try_into()?)),
                    xkb::COMPAT_MAP_NOTIFY_EVENT => Ok(Self::XkbCompatMapNotify(event.try_into()?)),
                    xkb::CONTROLS_NOTIFY_EVENT => Ok(Self::XkbControlsNotify(event.try_into()?)),
                    xkb::EXTENSION_DEVICE_NOTIFY_EVENT => Ok(Self::XkbExtensionDeviceNotify(event.try_into()?)),
                    xkb::INDICATOR_MAP_NOTIFY_EVENT => Ok(Self::XkbIndicatorMapNotify(event.try_into()?)),
                    xkb::INDICATOR_STATE_NOTIFY_EVENT => Ok(Self::XkbIndicatorStateNotify(event.try_into()?)),
                    xkb::MAP_NOTIFY_EVENT => Ok(Self::XkbMapNotify(event.try_into()?)),
                    xkb::NAMES_NOTIFY_EVENT => Ok(Self::XkbNamesNotify(event.try_into()?)),
                    xkb::NEW_KEYBOARD_NOTIFY_EVENT => Ok(Self::XkbNewKeyboardNotify(event.try_into()?)),
                    xkb::STATE_NOTIFY_EVENT => Ok(Self::XkbStateNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xprint")]
            Some((xprint::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xprint::ATTRIBUT_NOTIFY_EVENT => Ok(Self::XprintAttributNotify(event.try_into()?)),
                    xprint::NOTIFY_EVENT => Ok(Self::XprintNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xv")]
            Some((xv::X11_EXTENSION_NAME, ext_info)) => {
                match event_code - ext_info.first_event {
                    xv::PORT_NOTIFY_EVENT => Ok(Self::XvPortNotify(event.try_into()?)),
                    xv::VIDEO_NOTIFY_EVENT => Ok(Self::XvVideoNotify(event.try_into()?)),
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
                    present::COMPLETE_NOTIFY_EVENT => Ok(Self::PresentCompleteNotify(event.try_into()?)),
                    present::CONFIGURE_NOTIFY_EVENT => Ok(Self::PresentConfigureNotify(event.try_into()?)),
                    present::IDLE_NOTIFY_EVENT => Ok(Self::PresentIdleNotify(event.try_into()?)),
                    present::REDIRECT_NOTIFY_EVENT => Ok(Self::PresentRedirectNotify(event.try_into()?)),
                    _ => Ok(Self::Unknown(event.to_vec())),
                }
            }
            #[cfg(feature = "xinput")]
            Some(xinput::X11_EXTENSION_NAME) => {
                match ge_event.event_type {
                    xinput::BARRIER_HIT_EVENT => Ok(Self::XinputBarrierHit(event.try_into()?)),
                    xinput::BARRIER_LEAVE_EVENT => Ok(Self::XinputBarrierLeave(event.try_into()?)),
                    xinput::BUTTON_PRESS_EVENT => Ok(Self::XinputButtonPress(event.try_into()?)),
                    xinput::BUTTON_RELEASE_EVENT => Ok(Self::XinputButtonRelease(event.try_into()?)),
                    xinput::DEVICE_CHANGED_EVENT => Ok(Self::XinputDeviceChanged(event.try_into()?)),
                    xinput::ENTER_EVENT => Ok(Self::XinputEnter(event.try_into()?)),
                    xinput::FOCUS_IN_EVENT => Ok(Self::XinputFocusIn(event.try_into()?)),
                    xinput::FOCUS_OUT_EVENT => Ok(Self::XinputFocusOut(event.try_into()?)),
                    xinput::HIERARCHY_EVENT => Ok(Self::XinputHierarchy(event.try_into()?)),
                    xinput::KEY_PRESS_EVENT => Ok(Self::XinputKeyPress(event.try_into()?)),
                    xinput::KEY_RELEASE_EVENT => Ok(Self::XinputKeyRelease(event.try_into()?)),
                    xinput::LEAVE_EVENT => Ok(Self::XinputLeave(event.try_into()?)),
                    xinput::MOTION_EVENT => Ok(Self::XinputMotion(event.try_into()?)),
                    xinput::PROPERTY_EVENT => Ok(Self::XinputProperty(event.try_into()?)),
                    xinput::RAW_BUTTON_PRESS_EVENT => Ok(Self::XinputRawButtonPress(event.try_into()?)),
                    xinput::RAW_BUTTON_RELEASE_EVENT => Ok(Self::XinputRawButtonRelease(event.try_into()?)),
                    xinput::RAW_KEY_PRESS_EVENT => Ok(Self::XinputRawKeyPress(event.try_into()?)),
                    xinput::RAW_KEY_RELEASE_EVENT => Ok(Self::XinputRawKeyRelease(event.try_into()?)),
                    xinput::RAW_MOTION_EVENT => Ok(Self::XinputRawMotion(event.try_into()?)),
                    xinput::RAW_TOUCH_BEGIN_EVENT => Ok(Self::XinputRawTouchBegin(event.try_into()?)),
                    xinput::RAW_TOUCH_END_EVENT => Ok(Self::XinputRawTouchEnd(event.try_into()?)),
                    xinput::RAW_TOUCH_UPDATE_EVENT => Ok(Self::XinputRawTouchUpdate(event.try_into()?)),
                    xinput::TOUCH_BEGIN_EVENT => Ok(Self::XinputTouchBegin(event.try_into()?)),
                    xinput::TOUCH_END_EVENT => Ok(Self::XinputTouchEnd(event.try_into()?)),
                    xinput::TOUCH_OWNERSHIP_EVENT => Ok(Self::XinputTouchOwnership(event.try_into()?)),
                    xinput::TOUCH_UPDATE_EVENT => Ok(Self::XinputTouchUpdate(event.try_into()?)),
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

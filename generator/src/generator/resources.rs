use super::{CreateInfo, ResourceInfo};

pub(super) fn for_extension(extension: &str) -> &'static [ResourceInfo<'static>] {
    EXTENSION_RESOURCES
        .iter()
        .find(|(ext, _)| extension == *ext)
        .map(|(_, info)| *info)
        .unwrap_or(&[])
}

const EXTENSION_RESOURCES: [(&str, &[ResourceInfo<'_>]); 1] = [(
    "xproto",
    &[
        ResourceInfo {
            resource_name: "Pixmap",
            create_requests: &[CreateInfo {
                request_name: "CreatePixmap",
                created_argument: "pid",
            }],
            free_request: "FreePixmap",
        },
        ResourceInfo {
            resource_name: "Window",
            create_requests: &[CreateInfo {
                request_name: "CreateWindow",
                created_argument: "wid",
            }],
            free_request: "DestroyWindow",
        },
        ResourceInfo {
            resource_name: "Font",
            create_requests: &[CreateInfo {
                request_name: "OpenFont",
                created_argument: "fid",
            }],
            free_request: "CloseFont",
        },
        ResourceInfo {
            resource_name: "Gcontext",
            create_requests: &[CreateInfo {
                request_name: "CreateGC",
                created_argument: "cid",
            }],
            free_request: "FreeGC",
        },
        ResourceInfo {
            resource_name: "Colormap",
            create_requests: &[CreateInfo {
                request_name: "CreateColormap",
                created_argument: "mid",
            }],
            free_request: "FreeColormap",
        },
        ResourceInfo {
            resource_name: "Cursor",
            create_requests: &[
                CreateInfo {
                    request_name: "CreateCursor",
                    created_argument: "cid",
                },
                CreateInfo {
                    request_name: "CreateGlyphCursor",
                    created_argument: "cid",
                },
            ],
            free_request: "FreeCursor",
        },
    ],
)];

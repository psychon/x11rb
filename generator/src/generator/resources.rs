use super::{CreateInfo, ResourceInfo};

pub(super) fn for_extension(extension: &str) -> &'static [ResourceInfo<'static>] {
    EXTENSION_RESOURCES
        .iter()
        .find(|(ext, _)| extension == *ext)
        .map(|(_, info)| *info)
        .unwrap_or(&[])
}

const EXTENSION_RESOURCES: [(&str, &[ResourceInfo<'_>]); 4] = [
    (
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
    ),
    (
        "damage",
        &[ResourceInfo {
            resource_name: "Damage",
            create_requests: &[CreateInfo {
                request_name: "Create",
                created_argument: "damage",
            }],
            free_request: "Destroy",
        }],
    ),
    (
        "record",
        &[ResourceInfo {
            resource_name: "Context",
            create_requests: &[CreateInfo {
                request_name: "CreateContext",
                created_argument: "context",
            }],
            free_request: "FreeContext",
        }],
    ),
    (
        "xfixes",
        &[ResourceInfo {
            resource_name: "Region",
            create_requests: &[
                CreateInfo {
                    request_name: "CreateRegion",
                    created_argument: "region",
                },
                CreateInfo {
                    request_name: "CreateRegionFromBitmap",
                    created_argument: "region",
                },
                CreateInfo {
                    request_name: "CreateRegionFromWindow",
                    created_argument: "region",
                },
                CreateInfo {
                    request_name: "CreateRegionFromGC",
                    created_argument: "region",
                },
                CreateInfo {
                    request_name: "CreateRegionFromPicture",
                    created_argument: "region",
                },
                CreateInfo {
                    request_name: "composite:CreateRegionFromBorderClip",
                    created_argument: "region",
                },
            ],
            free_request: "DestroyRegion",
        }],
    ),
];

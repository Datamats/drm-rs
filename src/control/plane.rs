//! # Plane
//!
//! A plane is an object you can attach framebuffers to for use in displays.

use control::{self, crtc, framebuffer, ResourceHandle, ResourceInfo};
use result::*;
use ffi;
use {iRect, uRect};

/// A [`ResourceHandle`] for a plane.
///
/// Like all control resources, every planehas a unique `Handle` associated with
/// it. This `Handle` can be used to acquire information about the plane
/// (see [`plane::Info`]) or change the plane's state.
///
/// These can be retrieved by using [`PlaneResourceHandles::planes`].
///
/// [`ResourceHandle`]: ResourceHandle.t.html
/// [`plane::Info`]: Info.t.html
/// [`PlaneResourceHandles::planes`]: PlaneResourceHandles.t.html#method.planes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, From, Into)]
pub struct Handle(pub control::RawHandle);
impl ResourceHandle for Handle {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
/// The `ResourceInfo` on a plane.
pub struct Info {
    handle: Handle,
    crtc: control::crtc::Handle,
    fb: control::framebuffer::Handle,
    // TODO: count_formats,
    // TODO: possible_crtcs
    gamma_length: u32,
    // TODO: formats
}

impl control::property::LoadProperties for Handle {
    const TYPE: u32 = ffi::DRM_MODE_OBJECT_PLANE;
}

impl ResourceInfo for Info {
    type Handle = Handle;

    fn load_from_device<T>(device: &T, handle: Self::Handle) -> Result<Self>
    where
        T: control::Device,
    {
        let plane = {
            let mut raw: ffi::drm_mode_get_plane = Default::default();
            raw.plane_id = handle.into();
            unsafe {
                try!(ffi::ioctl_mode_getplane(device.as_raw_fd(), &mut raw));
            }

            Self {
                handle: handle,
                crtc: control::crtc::Handle::from(raw.crtc_id),
                fb: control::framebuffer::Handle::from(raw.fb_id),
                gamma_length: raw.gamma_size,
            }
        };

        Ok(plane)
    }

    fn handle(&self) -> Self::Handle {
        self.handle
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum PresentFlag {
    TopField = (0 << 1),
    BottomField = (1 << 1),
}

/// Attaches a framebuffer to a CRTC's plane for hardware-composing
pub fn set<T>(
    plane: Handle,
    device: &T,
    crtc: crtc::Handle,
    framebuffer: framebuffer::Handle,
    flags: PresentFlag,
    crtc_rect: iRect,
    src_rect: uRect,
) -> Result<()>
where
    T: control::Device,
{
    let mut raw: ffi::drm_mode_set_plane = Default::default();

    raw.plane_id = plane.into();
    raw.crtc_id = crtc.into();
    raw.fb_id = framebuffer.into();
    raw.flags = flags as u32;
    raw.crtc_x = (crtc_rect.0).0;
    raw.crtc_y = (crtc_rect.0).1;
    raw.crtc_w = (crtc_rect.1).0;
    raw.crtc_h = (crtc_rect.1).1;
    raw.src_x = (src_rect.0).0;
    raw.src_y = (src_rect.0).1;
    raw.src_w = (src_rect.1).0;
    raw.src_h = (src_rect.1).1;

    unsafe {
        ffi::ioctl_mode_setplane(device.as_raw_fd(), &mut raw)?;
    }

    Ok(())
}

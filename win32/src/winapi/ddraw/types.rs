//! Types defined in the DirectDraw API.

use crate::winapi::types::*;
use bitflags::bitflags;
use memory::Pod;

// TODO: maybe make some shared const fn for errors that sets high bit?
// TODO: share constants with winapi ERROR type?
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum DD {
    OK = 0,
    E_NOINTERFACE = 0x80004002,
    ERR_GENERIC = 0x80004005,
}

impl crate::winapi::stack_args::ABIReturn for DD {
    fn into_abireturn(self) -> u64 {
        self as u64
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DDSCAPS2 {
    pub dwCaps: DDSCAPS,
    dwCaps2: DWORD,
    dwCaps3: DWORD,
    dwCaps4: DWORD,
}
unsafe impl memory::Pod for DDSCAPS2 {}

bitflags! {
    #[derive(Default)]
    pub struct DDSCAPS: u32 {
        const ALPHA = 0x00000002;
        const BACKBUFFER = 0x00000004;
        const COMPLEX = 0x00000008;
        const FLIP = 0x00000010;
        const FRONTBUFFER = 0x00000020;
        const OFFSCREENPLAIN = 0x00000040;
        const OVERLAY = 0x00000080;
        const PALETTE = 0x00000100;
        const PRIMARYSURFACE = 0x00000200;
        const PRIMARYSURFACELEFT = 0x00000400;
        const SYSTEMMEMORY = 0x00000800;
        const TEXTURE = 0x00001000;
        const _3DDEVICE = 0x00002000;
        const VIDEOMEMORY = 0x00004000;
        const VISIBLE = 0x00008000;
        const WRITEONLY = 0x00010000;
        const ZBUFFER = 0x00020000;
        const OWNDC = 0x00040000;
        const LIVEVIDEO = 0x00080000;
        const HWCODEC = 0x00100000;
        const MODEX = 0x00200000;
        const MIPMAP = 0x00400000;
        const ALLOCONLOAD = 0x04000000;
        const VIDEOPORT = 0x08000000;
        const LOCALVIDMEM = 0x10000000;
        const NONLOCALVIDMEM = 0x20000000;
        const STANDARDVGAMODE = 0x40000000;
    }
}
unsafe impl memory::Pod for DDSCAPS {}

bitflags! {
    #[derive(Default)]
    pub struct DDSD: u32 {
        const CAPS = 0x00000001;
        const HEIGHT = 0x00000002;
        const WIDTH = 0x00000004;
        const PITCH = 0x00000008;
        const BACKBUFFERCOUNT = 0x00000020;
        const ZBUFFERBITDEPTH = 0x00000040;
        const ALPHABITDEPTH = 0x00000080;
        const LPSURFACE = 0x00000800;
        const PIXELFORMAT = 0x00001000;
        const CKDESTOVERLAY = 0x00002000;
        const CKDESTBLT = 0x00004000;
        const CKSRCOVERLAY= 0x00008000;
        const CKSRCBLT = 0x00010000;
        const MIPMAPCOUNT = 0x00020000;
        const REFRESHRATE = 0x00040000;
        const LINEARSIZE = 0x00080000;
        const TEXTURESTAGE = 0x00100000;
        const FVF = 0x00200000;
        const SRCVBHANDLE = 0x00400000;
        const DEPTH = 0x00800000;
    }
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct DDPCAPS: u32 {
        const _4BIT = 0x00000001;
        const _8BITENTRIES = 0x00000002;
        const _8BIT = 0x00000004;
        const INITIALIZE = 0x00000008;
        const PRIMARYSURFACE = 0x00000010;
        const PRIMARYSURFACELEFT = 0x00000020;
        const ALLOW256 = 0x00000040;
        const VSYNC = 0x00000080;
        const _1BIT = 0x00000100;
        const _2BIT = 0x00000200;
        const ALPHA =  0x00000400;
    }
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct DDLOCK: u32 {
        const SURFACEMEMORYPTR= 0x00000000;
        const WAIT = 0x00000001;
        const EVENT = 0x00000002;
        const READONLY = 0x00000010;
        const WRITEONLY = 0x00000020;
        const NOSYSLOCK = 0x00000800;
        const NOOVERWRITE = 0x00001000;
        const DISCARDCONTENTS = 0x00002000;
        const OKTOSWAP = 0x00002000;
        const DONOTWAIT = 0x00004000;
        const HASVOLUMETEXTUREBOXRECT = 0x00008000;
        const NODIRTYUPDATE = 0x00010000;
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct DDCOLORKEY {
    dwColorSpaceLowValue: DWORD,
    dwColorSpaceHighValue: DWORD,
}
unsafe impl memory::Pod for DDCOLORKEY {}

#[repr(C)]
#[derive(Clone, Default)]
pub struct DDSURFACEDESC {
    pub dwSize: DWORD,
    pub dwFlags: DDSD,
    pub dwHeight: DWORD,
    pub dwWidth: DWORD,

    pub lPitch_dwLinearSize: DWORD,
    pub dwBackBufferCount: DWORD,
    pub dwMipMapCount_dwZBufferBitDepth_dwRefreshRate: DWORD,
    pub dwAlphaBitDepth: DWORD,
    pub dwReserved: DWORD,
    pub lpSurface: DWORD,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub ddpfPixelFormat: DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
unsafe impl memory::Pod for DDSURFACEDESC {}

/// Custom implementation of Debug that only displays the fields that have been marked present in the flags member.
impl std::fmt::Debug for DDSURFACEDESC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st = f.debug_struct("DDSURFACEDESC");
        st.field("dwSize", &self.dwSize);
        st.field("dwFlags", &self.dwFlags);
        if self.dwFlags.contains(DDSD::HEIGHT) {
            st.field("dwHeight", &self.dwHeight);
        }
        if self.dwFlags.contains(DDSD::WIDTH) {
            st.field("dwWidth", &self.dwWidth);
        }
        if self.dwFlags.contains(DDSD::PITCH) {
            st.field("lPitch", &self.lPitch_dwLinearSize);
        }
        if self.dwFlags.contains(DDSD::LINEARSIZE) {
            st.field("dwLinearSize", &self.lPitch_dwLinearSize);
        }
        if self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            st.field("dwBackBufferCount", &self.dwBackBufferCount);
        }
        if self.dwFlags.contains(DDSD::MIPMAPCOUNT) {
            st.field(
                "dwMipMapCount_dwZBufferBitDepth_dwRefreshRate",
                &self.dwMipMapCount_dwZBufferBitDepth_dwRefreshRate,
            );
        }
        if self.dwFlags.contains(DDSD::ALPHABITDEPTH) {
            st.field("dwAlphaBitDepth", &self.dwAlphaBitDepth);
        }
        if self.dwFlags.contains(DDSD::LPSURFACE) {
            st.field("lpSurface", &self.lpSurface);
        }
        if self.dwFlags.contains(DDSD::CKDESTOVERLAY) {
            st.field("ddckCKDestOverlay", &self.ddckCKDestOverlay);
        }
        if self.dwFlags.contains(DDSD::CKDESTBLT) {
            st.field("ddckCKDestBlt", &self.ddckCKDestBlt);
        }
        if self.dwFlags.contains(DDSD::CKSRCOVERLAY) {
            st.field("ddckCKSrcOverlay", &self.ddckCKSrcOverlay);
        }
        if self.dwFlags.contains(DDSD::CKSRCBLT) {
            st.field("ddckCKSrcBlt", &self.ddckCKSrcBlt);
        }
        if self.dwFlags.contains(DDSD::PIXELFORMAT) {
            st.field("ddpfPixelFormat", &self.ddpfPixelFormat);
        }
        if self.dwFlags.contains(DDSD::CAPS) {
            st.field("ddsCaps", &self.ddsCaps);
        }
        st.finish()
    }
}

impl DDSURFACEDESC {
    pub fn caps(&self) -> Option<&DDSCAPS> {
        if !self.dwFlags.contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }
    pub fn back_buffer_count(&self) -> Option<u32> {
        if !self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount)
    }

    pub fn from_desc2(desc2: &DDSURFACEDESC2) -> DDSURFACEDESC {
        DDSURFACEDESC {
            dwSize: std::mem::size_of::<DDSURFACEDESC>() as u32,
            dwFlags: desc2.dwFlags,
            dwHeight: desc2.dwHeight,
            dwWidth: desc2.dwWidth,

            lPitch_dwLinearSize: desc2.lPitch_dwLinearSize,
            dwBackBufferCount: desc2.dwBackBufferCount_dwDepth,
            dwMipMapCount_dwZBufferBitDepth_dwRefreshRate: Default::default(),
            dwAlphaBitDepth: Default::default(),
            dwReserved: Default::default(),
            lpSurface: desc2.lpSurface,
            ddckCKDestOverlay: Default::default(),
            ddckCKDestBlt: Default::default(),
            ddckCKSrcOverlay: Default::default(),
            ddckCKSrcBlt: Default::default(),
            ddpfPixelFormat: desc2.ddpfPixelFormat.clone(),
            ddsCaps: desc2.ddsCaps.dwCaps,
        }
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct DDSURFACEDESC2 {
    pub dwSize: DWORD,
    pub dwFlags: DDSD,
    pub dwHeight: DWORD,
    pub dwWidth: DWORD,

    pub lPitch_dwLinearSize: DWORD,
    pub dwBackBufferCount_dwDepth: DWORD,
    pub dwMipMapCount_dwRefreshRate_dwSrcVBHandle: DWORD,

    pub dwAlphaBitDepth: DWORD,
    pub dwReserved: DWORD,
    pub lpSurface: DWORD,
    pub ddckCKDestOverlay_dwEmptyFaceColor: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,

    pub ddpfPixelFormat: DDPIXELFORMAT,
    // TODO: dwFVF
    pub ddsCaps: DDSCAPS2,
    pub dwTextureStage: DWORD,
}
unsafe impl memory::Pod for DDSURFACEDESC2 {}

impl Default for DDSURFACEDESC2 {
    fn default() -> Self {
        let mut desc: Self = DDSURFACEDESC2::zeroed();
        desc.dwSize = std::mem::size_of::<Self>() as u32;
        desc
    }
}

/// Custom implementation of Debug that only displays the fields that have been marked present in the flags member.
impl std::fmt::Debug for DDSURFACEDESC2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut st = f.debug_struct("DDSURFACEDESC2");
        st.field("dwSize", &self.dwSize);
        st.field("dwFlags", &self.dwFlags);
        if self.dwFlags.contains(DDSD::HEIGHT) {
            st.field("dwHeight", &self.dwHeight);
        }
        if self.dwFlags.contains(DDSD::WIDTH) {
            st.field("dwWidth", &self.dwWidth);
        }
        if self.dwFlags.contains(DDSD::PITCH) {
            st.field("lPitch_dwLinearSize", &self.lPitch_dwLinearSize);
        }
        if self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            st.field("dwBackBufferCount_dwDepth", &self.dwBackBufferCount_dwDepth);
        }
        if self.dwFlags.contains(DDSD::MIPMAPCOUNT) {
            st.field(
                "dwMipMapCount_dwRefreshRate_dwSrcVBHandle",
                &self.dwMipMapCount_dwRefreshRate_dwSrcVBHandle,
            );
        }
        if self.dwFlags.contains(DDSD::ALPHABITDEPTH) {
            st.field("dwAlphaBitDepth", &self.dwAlphaBitDepth);
        }
        if self.dwFlags.contains(DDSD::LPSURFACE) {
            st.field("lpSurface", &self.lpSurface);
        }
        if self.dwFlags.contains(DDSD::CKDESTOVERLAY) {
            st.field(
                "ddckCKDestOverlay_dwEmptyFaceColor",
                &self.ddckCKDestOverlay_dwEmptyFaceColor,
            );
        }
        if self.dwFlags.contains(DDSD::CKDESTBLT) {
            st.field("ddckCKDestBlt", &self.ddckCKDestBlt);
        }
        if self.dwFlags.contains(DDSD::CKSRCOVERLAY) {
            st.field("ddckCKSrcOverlay", &self.ddckCKSrcOverlay);
        }
        if self.dwFlags.contains(DDSD::CKSRCBLT) {
            st.field("ddckCKSrcBlt", &self.ddckCKSrcBlt);
        }
        if self.dwFlags.contains(DDSD::PIXELFORMAT) {
            st.field("ddpfPixelFormat", &self.ddpfPixelFormat);
        }
        if self.dwFlags.contains(DDSD::CAPS) {
            st.field("ddsCaps", &self.ddsCaps);
        }
        if self.dwFlags.contains(DDSD::TEXTURESTAGE) {
            st.field("dwTextureStage", &self.dwTextureStage);
        }
        st.finish()
    }
}

impl DDSURFACEDESC2 {
    pub fn back_buffer_count(&self) -> Option<u32> {
        if !self.dwFlags.contains(DDSD::BACKBUFFERCOUNT) {
            return None;
        }
        Some(self.dwBackBufferCount_dwDepth)
    }
    pub fn caps(&self) -> Option<&DDSCAPS2> {
        if !self.dwFlags.contains(DDSD::CAPS) {
            return None;
        }
        Some(&self.ddsCaps)
    }

    pub fn from_desc(desc: &DDSURFACEDESC) -> DDSURFACEDESC2 {
        DDSURFACEDESC2 {
            dwSize: std::mem::size_of::<DDSURFACEDESC2>() as u32,
            dwFlags: desc.dwFlags,
            dwHeight: desc.dwHeight,
            dwWidth: desc.dwWidth,
            lPitch_dwLinearSize: desc.lPitch_dwLinearSize,
            dwBackBufferCount_dwDepth: desc.dwBackBufferCount,
            dwMipMapCount_dwRefreshRate_dwSrcVBHandle: Default::default(),
            dwAlphaBitDepth: Default::default(),
            dwReserved: Default::default(),
            lpSurface: desc.lpSurface,
            ddckCKDestOverlay_dwEmptyFaceColor: Default::default(),
            ddckCKDestBlt: Default::default(),
            ddckCKSrcOverlay: Default::default(),
            ddckCKSrcBlt: Default::default(),
            ddpfPixelFormat: Default::default(),
            ddsCaps: DDSCAPS2 {
                dwCaps: desc.ddsCaps,
                dwCaps2: Default::default(),
                dwCaps3: Default::default(),
                dwCaps4: Default::default(),
            },
            dwTextureStage: Default::default(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct DDPIXELFORMAT {
    pub dwSize: DWORD,
    pub dwFlags: DWORD,
    pub dwFourCC: DWORD,
    pub dwRGBBitCount: DWORD,
    pub dwRBitMask: DWORD,
    pub dwGBitMask: DWORD,
    pub dwBBitMask: DWORD,
    pub dwRGBAlphaBitMask: DWORD,
}
unsafe impl memory::Pod for DDPIXELFORMAT {}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct DDBLT: u32 {
        const ALPHADEST                = 0x00000001;
        const ALPHADESTCONSTOVERRIDE   = 0x00000002;
        const ALPHADESTNEG             = 0x00000004;
        const ALPHADESTSURFACEOVERRIDE = 0x00000008;
        const ALPHAEDGEBLEND           = 0x00000010;
        const ALPHASRC                 = 0x00000020;
        const ALPHASRCCONSTOVERRIDE    = 0x00000040;
        const ALPHASRCNEG              = 0x00000080;
        const ALPHASRCSURFACEOVERRIDE  = 0x00000100;
        const ASYNC                    = 0x00000200;
        const COLORFILL                = 0x00000400;
        const DDFX                     = 0x00000800;
        const DDROPS                   = 0x00001000;
        const KEYDEST                  = 0x00002000;
        const KEYDESTOVERRIDE          = 0x00004000;
        const KEYSRC                   = 0x00008000;
        const KEYSRCOVERRIDE           = 0x00010000;
        const ROP                      = 0x00020000;
        const ROTATIONANGLE            = 0x00040000;
        const ZBUFFER                  = 0x00080000;
        const ZBUFFERDESTCONSTOVERRIDE = 0x00100000;
        const ZBUFFERDESTOVERRIDE      = 0x00200000;
        const ZBUFFERSRCCONSTOVERRIDE  = 0x00400000;
        const ZBUFFERSRCOVERRIDE       = 0x00800000;
        const WAIT                     = 0x01000000;
        const DEPTHFILL                = 0x02000000;
        const DONOTWAIT                = 0x08000000;
  }
}

bitflags! {
    #[derive(win32_derive::TryFromBitflags)]
    pub struct DDBLTFAST: u32 {
        // const NOCOLORKEY   = 0x00000000;
        const SRCCOLORKEY  = 0x00000001;
        const DESTCOLORKEY = 0x00000002;
        const WAIT         = 0x00000010;
        const DONOTWAIT    = 0x00000020;
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct DDBLTFX {
    pub dwSize: u32,
    pub dwDDFX: DDBLTFXT,
    pub dwROP: u32,
    pub dwDDROP: u32,
    pub dwRotationAngle: u32,
    pub dwZBufferOpCode: u32,
    pub dwZBufferLow: u32,
    pub dwZBufferHigh: u32,
    pub dwZBufferBaseDest: u32,
    pub dwZDestConstBitDepth: u32,
    pub zDest: u32,
    pub dwZSrcConstBitDepth: u32,
    pub zSrc: u32,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub alphaDest: u32,
    pub dwAlphaSrcConstBitDepth: u32,
    pub alphaSrc: u32,
    pub dwFillColor: u32,
    pub ddckDestColorkey: DDCOLORKEY,
    pub ddckSrcColorkey: DDCOLORKEY,
}
unsafe impl memory::Pod for DDBLTFX {}

bitflags! {
    pub struct DDBLTFXT: u32 {
        const ARITHSTRETCHY   = 0x001;
        const MIRRORLEFTRIGHT = 0x002;
        const MIRRORUPDOWN    = 0x004;
        const NOTEARING       = 0x008;
        const ROTATE180       = 0x010;
        const ROTATE270       = 0x020;
        const ROTATE90        = 0x040;
        const ZBUFFERRANGE    = 0x080;
        const ZBUFFERBASEDEST = 0x100;
    }
}

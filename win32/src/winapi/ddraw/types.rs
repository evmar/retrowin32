//! Types defined in the DirectDraw API.

use crate::winapi::types::*;
use bitflags::bitflags;

#[repr(C)]
#[derive(Debug, Default)]
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
impl TryFrom<u32> for DDPCAPS {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        DDPCAPS::from_bits(value).ok_or(value)
    }
}

bitflags! {
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
impl TryFrom<u32> for DDLOCK {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        DDLOCK::from_bits(value).ok_or(value)
    }
}

#[repr(C)]
#[derive(Debug, Default)]
pub struct DDCOLORKEY {
    dwColorSpaceLowValue: DWORD,
    dwColorSpaceHighValue: DWORD,
}
unsafe impl memory::Pod for DDCOLORKEY {}

#[repr(C)]
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
            ddpfPixelFormat: Default::default(),
            ddsCaps: desc2.ddsCaps.dwCaps,
        }
    }
}

#[repr(C)]
#[derive(Debug, Default)]
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
#[derive(Debug, Default)]
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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct PALETTEENTRY {
    pub peRed: u8,
    pub peGreen: u8,
    pub peBlue: u8,
    pub peFlags: u8,
}
unsafe impl memory::Pod for PALETTEENTRY {}

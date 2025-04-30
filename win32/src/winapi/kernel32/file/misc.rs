use memory::str16::Str16;

use crate::{Machine, System};

#[win32_derive::dllexport]
pub fn GetDiskFreeSpaceA(
    machine: &mut Machine,
    lpRootPathName: Option<&str>,
    lpSectorsPerCluster: Option<&mut u32>,
    lpBytesPerSector: Option<&mut u32>,
    lpNumberOfFreeClusters: Option<&mut u32>,
    lpTotalNumberOfClusters: Option<&mut u32>,
) -> bool {
    let sector_size = 512;
    let cluster_size = 4 << 10; // 4kb
    let free_space = 4 << 20; // 4mb
    let total_space = 64 << 20; // 64mb

    if let Some(sectors_per_cluster) = lpSectorsPerCluster {
        *sectors_per_cluster = cluster_size / sector_size;
    }
    if let Some(bytes_per_sector) = lpBytesPerSector {
        *bytes_per_sector = sector_size;
    }
    if let Some(number_of_free_clusters) = lpNumberOfFreeClusters {
        *number_of_free_clusters = free_space / cluster_size;
    }
    if let Some(total_number_of_clusters) = lpTotalNumberOfClusters {
        *total_number_of_clusters = total_space / cluster_size;
    }
    true
}

#[win32_derive::dllexport]
pub fn GetDriveTypeW(sys: &dyn System, lpRootPathName: Option<&Str16>) -> u32 {
    todo!()
}

#[win32_derive::dllexport]
pub fn GetDriveTypeA(sys: &dyn System, lpRootPathName: Option<&str>) -> u32 {
    const DRIVE_FIXED: u32 = 3; // hard drive
    DRIVE_FIXED
}

#[win32_derive::dllexport]
pub fn GetLogicalDrives(sys: &dyn System) -> u32 {
    let mut drives = 0;
    drives |= 1 << 2; // C:
    drives
}

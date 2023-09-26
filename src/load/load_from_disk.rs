use std::sync::Arc;
use std::{os::unix::prelude::MetadataExt, time::SystemTime};

use walkdir::WalkDir;

use crate::protos::configuration::ApplicationConfiguration;
use crate::{lru::disk_lru, proxy::proxy_factory};

#[derive(Debug)]
pub struct ScanResult {
    key: String,
    size: u64,
    atime: SystemTime,
}

pub fn scan_dir(dir: &String) -> Vec<ScanResult> {
    let mut scan_results = vec![];
    for entry in WalkDir::new(&dir).into_iter().filter_map(|e| e.ok()) {
        let metadata = entry.metadata().unwrap();
        if metadata.is_file() {
            scan_results.push(ScanResult {
                key: entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                size: metadata.size(),
                atime: metadata.modified().unwrap(),
            });
            log::debug!("scan file with the result {:?}", scan_results.last())
        }
    }

    scan_results.sort_by(|l, r| l.atime.cmp(&r.atime));
    scan_results
}

pub async fn create_and_add_to_lru(
    scan_results: Vec<ScanResult>,
    app_config: &ApplicationConfiguration,
) -> disk_lru::DiskLRU {
    let proxy = proxy_factory::create_proxy(&app_config).await;
    let disk_lru = disk_lru::DiskLRU::new(
        app_config.data_directory_path.clone(),
        app_config.max_size,
        Arc::new(proxy),
    );
    for scan_result in scan_results {
        disk_lru.add_file_from_scan(scan_result.key, scan_result.size);
    }
    disk_lru
}

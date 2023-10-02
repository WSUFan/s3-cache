use actix_files::NamedFile;

use actix_web::error::Error;
use lru::LruCache;
use sha256::digest;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::proxy::proxy::StorageProxy;

pub struct LRUItem {
    size: u64,
}

pub struct DiskLRU {
    dir: Arc<String>,
    curr_size: Arc<Mutex<u64>>,
    max_size: u64,
    storage_proxy: Arc<Box<dyn StorageProxy>>,
    pub lru: Arc<Mutex<LruCache<String, LRUItem>>>,
}

impl Clone for DiskLRU {
    fn clone(&self) -> Self {
        Self {
            dir: self.dir.clone(),
            max_size: self.max_size,
            lru: self.lru.clone(),
            curr_size: self.curr_size.clone(),
            storage_proxy: self.storage_proxy.clone(),
        }
    }
}

fn sha256_of(data: &String) -> String {
    digest(data)
}

impl DiskLRU {
    pub fn new(dir: String, max_size: u64, proxy: Arc<Box<dyn StorageProxy>>) -> DiskLRU {
        DiskLRU {
            dir: Arc::new(dir),
            curr_size: Arc::new(Mutex::new(0)),
            lru: Arc::new(Mutex::new(LruCache::unbounded())),
            storage_proxy: proxy.clone(),
            max_size: max_size,
        }
    }

    fn generate_absolute_path(&self, path: &String) -> PathBuf {
        let p = Path::new(self.dir.as_str());
        p.join(path)
    }

    fn inc_size(&self, size: u64) {
        let mut curr_size = self.curr_size.lock().unwrap();
        *curr_size += size;
    }

    fn dec_size(&self, size: u64) {
        let mut curr_size = self.curr_size.lock().unwrap();
        *curr_size -= size;
    }

    pub async fn get_file(&self, path: String) -> actix_web::Result<NamedFile> {
        let hash_key = sha256_of(&path);
        let file_path = self.generate_absolute_path(&hash_key);

        log::info!("get file {:?} from {}", file_path, path);

        match NamedFile::open_async(file_path).await {
            Ok(v) => {
                self.move_file_to_front(hash_key);
                Ok(v)
            }
            Err(err) => Err(Error::from(err)),
        }
    }

    async fn reserve_size(&self, size: u64) -> Result<(), std::io::Error> {
        loop {
            let curr_size = *self.curr_size.lock().unwrap();
            if curr_size + size > self.max_size {
                match self.lru.lock().unwrap().pop_lru() {
                    Some((key, item)) => {
                        let file_path = self.generate_absolute_path(&key);
                        tokio::fs::remove_file(file_path).await?;
                        self.dec_size(item.size);
                    }
                    None => {}
                }
            } else {
                break;
            }
        }
        Ok(())
    }

    pub fn add_file_from_scan(&self, path: String, size: u64) {
        self.lru
            .clone()
            .lock()
            .unwrap()
            .put(path, LRUItem { size: size });
    }

    pub async fn add_file(&self, path: String) -> actix_web::Result<()> {
        let hash_key = sha256_of(&path);
        let tmp_path = path;
        let new_path = self.generate_absolute_path(&hash_key);

        let metadata = tokio::fs::metadata(&tmp_path).await?;

        let file_size = metadata.len();

        self.reserve_size(file_size).await?;
        tokio::fs::rename(&tmp_path, &new_path).await?;

        log::info!("rename file from {} to {:?}", tmp_path, new_path);

        {
            self.lru
                .clone()
                .lock()
                .unwrap()
                .put(hash_key.clone(), LRUItem { size: file_size });
        }

        self.inc_size(file_size);
        self.storage_proxy.put_object(&Path::new(&hash_key)).await;

        Ok(())
    }

    pub fn move_file_to_front(&self, path: String) {
        let lru = self.lru.clone();
        let mut lru = lru.lock().unwrap();
        lru.get(&path).or_else(|| {
            log::warn!("file {} does not exit in the LRU", path);
            None
        });
    }
}

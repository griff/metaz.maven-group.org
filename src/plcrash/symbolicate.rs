use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

use addr2line::{self, BufferMapping, Options};
use owning_ref::OwningHandle;
use lru::LruCache;

use ::VersionInfo;

struct Cache {
    versions: HashMap<String, VersionInfo>,
    lru: LruCache<String, Symbolicate>,
    disk_cache: PathBuf,
}

impl Cache {
    fn get(&mut self, version: &String) -> Result<Option<&Symbolicate>, io::Error> {
        if self.lru.contains(version) {
            return Ok(self.lru.get(version));
        }

        if let Some(info) = self.versions.get(version) {
            if let &Some(ref dsym) = &info.dsym {
                let cache_path = self.disk_cache.join(format!("{}.zip", version));
                if !cache_path.exists() {
                    let file = File::create(&cache_path)?;
                }
                let symbolicate = Symbolicate::new(&cache_path)?;
                self.lru.put(version.clone(), symbolicate);
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }

        Ok(self.lru.get(version))
    }
}

struct Symbolicate {
    files: HashMap<OsString,Lookup>,
}

impl Symbolicate {
    fn new<P:AsRef<Path>>(path: P) -> io::Result<Symbolicate> {
        Ok(Symbolicate {
            files: HashMap::new(),
        })
    }
}

pub struct Lookup {
    inner: OwningHandle<Vec<u8>, Box<BufferMapping<'static>>>,
}

impl Lookup {
    pub fn new(buf: Vec<u8>, opts: Options) -> addr2line::Result<Lookup> {
        OwningHandle::try_new(buf, |buf| -> addr2line::Result<_> {
            let bytes = unsafe { &*buf };
            opts.build_from_buffer(bytes)
                .map(|bm| Box::new(bm))
        }).map(|oh| Lookup { inner: oh })
    }
}

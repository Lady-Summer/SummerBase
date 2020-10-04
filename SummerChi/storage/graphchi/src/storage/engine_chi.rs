use std::collections::BinaryHeap;
use crate::interfaces::Edge;
use std::cmp::Ordering;
use tokio::io::AsyncBufRead;
use crate::storage::{ShardId, VertexId};

const ROOT_PATH: String = String::from("/data/summerchi");
const ENTER: u8 = '\n' as u8;

mod io {
    use tokio::stream::StreamExt;
    use std::io::{Read, Seek, Write, Result};
    use std::error::Error;
    use std::fs::{File, OpenOptions, Metadata};
    use std::borrow::{Borrow, BorrowMut};
    use std::path::Path;
    use std::iter::Map;
    use crate::storage::engine_chi::ENTER;

    pub struct FileInputStream<'a> {
        file: &'a File,
    }

    impl <'a> StreamExt for FileInputStream<'a> {
        fn map<T, F>(self, f: F) -> Map<Self, F> where
            F: FnMut(Self::Item) -> T,
            Self: Sized, {
            unimplemented!()
        }
    }

    impl <'a> FileInputStream<'a> {

        /// Read all lines and convert it
        pub fn read_to_end(&mut self) -> Result<&Vec<Vec<u8>>> {
            let size = self.file.metadata().map(|x| x.len()).unwrap() as usize;
            let mut buf: Vec<u8> = Vec::with_capacity(size);
            let ref mut result: Vec<Vec<u8>> = Vec::new();
            match tokio::fs::asyncify(move || self.file.read_to_end(buf.as_mut())).await {
                Ok(_) => {
                    let mut foo: Vec<u8> = Vec::new();
                    buf.iter().for_each(move |x| {
                        if x.eq(ENTER.borrow()) {
                            result.push(foo.clone());
                            foo.clear();
                        } else { foo.push(x.clone()); }
                    });
                    Ok(result)
                },
                Err(e) => Err(e)
            }
        }

        pub fn metadata(&self) -> Metadata {
            self.file.metadata().unwrap()
        }

        /// Read bytes to nth line
        pub fn read_to_nth_line(&mut self, n: &usize) -> Result<&Vec<Vec<u8>>> {
        }

        pub fn new<P: AsRef<Path>>(path: &P) -> FileInputStream {
            FileInputStream {
                file: File::open(path).unwrap().borrow(),
            }
        }
    }

    pub struct FileOutputStream<'a> {
        file: &'a File,
        buffer: Vec<u8>,
        buf_size: usize
    }

    impl <'a> Write for FileOutputStream<'a> {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            unimplemented!()
        }

        fn flush(&mut self) -> Result<()> {
            unimplemented!()
        }
    }

    impl <'a> FileOutputStream<'a> {

        pub fn new<P: AsRef<Path>>(path: &P, capacity: &usize) -> Self {
            let ref file = Self::create(path).unwrap();
            FileOutputStream {
                file,
                buffer: Vec::with_capacity(capacity.clone()),
                buf_size: 0
            }
        }

        /// Create a file, will not truncate if exist
        fn create<P: AsRef<Path>>(path: &P) -> std::io::Result<File> {
            OpenOptions::new().write(true).truncate(false).create(true).open(path)
        }

    }
}

#[derive(PartialOrd, PartialEq)]
struct Unit(ShardId, VertexId, dyn Edge);

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

struct EdgeBuffer  {
    buffer: BinaryHeap<Unit>
}

pub mod engine_core {
    use std::error::Error;
    use crate::storage::*;
    use std::io::Result;
    use std::path::Path;
    use crate::storage::engine_chi::ROOT_PATH;
    use std::borrow::Borrow;
    use std::collections::BinaryHeap;
    use crate::interfaces::Edge;

    pub struct ChiEdge<'a> {
        weight: f64,
        src: &'a u64,
        dest: &'a u64,
        id: u64,
        shard_id: &'a usize
    }

    impl Edge for ChiEdge {
        fn get_weight(&self) -> f64 {
            self.weight
        }

        fn set_id(&mut self, edge_id: &usize) {
            self.id = edge_id.clone() as u64
        }

        fn edge_id(&self) -> u64 {
            self.id.clone()
        }

        fn shard_id(&self) -> usize {
            self.shard_id.clone()
        }

        fn get_src(&self) -> u64 {
            self.src.clone()
        }

        fn get_dest(&self) -> u64 {
            self.dest.clone()
        }

        fn set_src(&mut self, src_id: &u64) {
            self.src = src_id.borrow()
        }

        fn from(shard_id: &usize, edge_id: &usize, src: &u64, dest: &u64) -> ChiEdge {
            ChiEdge {
                weight: 1,
                src,
                dest,
                id: edge_id.clone() as u64,
                shard_id
            }
        }
    }

    pub struct SummerChiEngine {
        edge_buffer: super::EdgeBuffer
    }

    impl Storage for SummerChiEngine {

        fn get_interval(&self, interval_id: &usize)
                        -> Result<(Vec<AdjacentShard>, EdgeDataShard)>
        {
            let path = Path::new(&(ROOT_PATH + "/interval_" + stringify!(interval_id)));
            let ref mut in_stream = super::io::FileInputStream::new(path);
            match in_stream.read_to_end() {
                Ok(content) => {
                    // TODO Push in a MinHeap called edge_arr compared with src id in each iteration
                    let adj_shard_arr = content.iter().map(|x| {
                        match Self::deserialize_line(x) {
                            Ok(adj_shard) => adj_shard.borrow(),
                            Err(e) => panic!("Error when deserialize interval {:?} data", interval_id)
                        }
                    }).collect();
                    Ok((adj_shard_arr, EdgeDataShard()))
                },
                Err(e) => Err(e)
            }
        }

        fn get_interval_num(&self) -> Result<usize>
        {
            unimplemented!()
        }

        fn update_interval(&self, adj_shard: AdjacentShard, edge_shard: EdgeDataShard)
                           -> Result<()>
        {
            unimplemented!()
        }
    }

    impl SummerChiEngine {
        fn deserialize_line(line: &Vec<u8>) -> Result<AdjacentShard>{
        }
    }
}

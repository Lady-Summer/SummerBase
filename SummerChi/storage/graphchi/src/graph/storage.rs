use std::io::Result;
use crate::graph::core::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tokio::macros::support::Future;

pub trait Storage {

    /// load interval with interval id from disk
    /// PSW can call this directly to load one interval.
    fn get_interval(
        &self,
        interval_id: &usize
    ) -> Result<(Vec<AdjacentShard>, EdgeDataShard)>;

    ///
    fn get_shard_num (
        &self,
        interval_id: &usize
    ) -> Option<usize>;

    /// Flush all updated interval data into disk
    fn update_interval(
        &self,
        interval_id: &usize,
        adj_shard: &AdjacentShard,
        edge_shard: &EdgeDataShard
    ) -> Result<()>;
}

mod io {
    use std::io::{Read, Write, Result};
    use std::borrow::Borrow;
    use std::path::Path;
    use crate::graph as Graph;
    use Graph::core::*;
    use tokio::fs::File;
    use tokio::prelude::{AsyncBufRead, AsyncWrite};

    pub type Stream = Vec<u8>;

    /// A common type for file output stream.
    pub trait InputStream: AsyncBufRead + Iterator {

        /// Read a line of a file input stream and pc which indicates the offset of the line end.
        /// While read success, return an Option with Stream type data. If catch EOF, return None type.
        ///
        /// # Error
        ///
        fn readline(&mut self) -> Result<Option<Stream>>;

        /// Read lines in a range and move pc to the end.
        /// If end exceeds the number of file's line, it will only read to the end.
        /// If start < 0, then read from the first line in default. If start=end, read start'th line.
        /// # Error
        /// If start > end or end < 0
        fn readline_range(&mut self, start: u64, end: u64) -> Result<Vec<Stream>>;

        /// Read lines to the end of a file
        /// # Error
        ///
        fn read_all_lines(&mut self) -> Result<Vec<Stream>>;

        /// Open a file stream in path
        /// Return a tokio type file
        /// # Error
        ///
        fn open<P: AsRef<Path>>(
            path: P
        ) -> Result<File>;

        /// Return true:  pc in the end of the stream.
        /// Return false: pc is not in the end
        fn is_end(&self) -> bool;

        fn is_empty(&self) -> bool;

        // The number of the line
        fn len(&self) -> u64;

    }


    /// It is a general type of Stream Transformer. It is mainly used to transform each line of a input stream in a
    /// specific type by a given decoder
    pub trait StreamTransformer {
        fn decode_all<U, F>(
            input_stream: Vec<&Stream>,
            decoder: &F
        ) -> Vec<U> where F: Fn(&Stream) -> U, U: Clone {
            input_stream.iter().map(
                |x| Self::decode_stream(*x, decoder)
            ).collect()
        }

        fn decode_stream<U, F>(
            stream: &Stream,
            decoder: &F
        ) -> U where F: Fn(&Stream) -> U {
            decoder(stream)
        }

        fn encode_to_stream<U, F>(
            data: &U,
            encoder: &F
        ) -> Stream where F: Fn(&U) -> Stream {
            encoder(data)
        }

        fn encode_all<U, F> (
            stream: Vec<&U>,
            encoder: &F
        ) -> Vec<Stream> where F: Fn(&U) -> Stream {
            stream.iter().map(
                |x| Self::encode_to_stream(*x, encoder)
            ).collect()
        }
    }

    pub trait OutputStream: AsyncWrite {

        /// Write a line into file directly
        /// # Error
        ///
        fn write_line(&self, line: Stream) -> Result<()>;

        // Write a line into buffer. If buffer is full, then we first flush all data from buffer
        // into disk then write new data in buffer.
        fn write_line_with_buffer(
            &self,
            buf: &mut [Stream],
            line: Stream
        ) -> Result<()>;

        /// Flush all data from buffer into disk
        fn flush(&self, buf: &mut [Stream]) -> Result<()>;

        /// Open a file output stream
        /// If the file is not exist, create it. If it does, we do not truncate it,
        /// only add new data.
        /// # Error
        /// If file fails to be opened/created, return
        fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<File>;
    }
}

#[derive(PartialEq, PartialOrd ,Ord, Eq, Clone)]
struct Unit(ShardId, IntervalId, Edge<f64>);
//
// impl <'a> PartialEq for Unit<'a> {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 == other.0 && self.1 == other.1 && self.2 == other.2
//     }
// }
//
// impl <'a> PartialOrd for Unit<'a> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         unimplemented!()
//     }
// }

#[derive(Clone)]
struct EdgeBuffer {
    buffer: BinaryHeap<Unit>
}

pub mod storage_core {
    use std::io::{Result, Read, SeekFrom};
    use std::path::Path;
    use std::borrow::{Borrow, BorrowMut};
    use std::collections::BinaryHeap;
    use super::super as Graph;
    use Graph::core::*;
    use Graph::storage::EdgeBuffer;
    use super::io::*;
    use tokio::fs::File;
    use crate::error::IntervalLoadError;
    use Graph::storage::Unit;
    use std::error::Error;
    use super::super::super::utils::*;
    use std::sync::Arc;
    use std::sync::atomic::AtomicPtr;
    use tokio::io::util::async_read_ext::AsyncReadExt;
    use tokio::prelude::{AsyncBufRead, AsyncRead};
    use std::task::{Context, Waker};
    use tokio::macros::support::{Pin, Poll};
    use std::ops::AddAssign;
    use tokio::future::poll_fn;
    use tokio::io::AsyncSeek;

    mod codec {
        use std::io::Result;
        use crate::graph::storage::io::*;

        pub trait Codec<R: InputStream, W: OutputStream, U> {
            fn encode(data: Vec<U>, level: i32) -> Result<&'static [Stream]>;

            fn decode(steam: Stream) -> Result<U>;
        }

    }

    #[derive(Clone, Hash, PartialOrd, PartialEq)]
    struct Metadata {
        line_num: u64,
        interval_id: u64,
        shard_size: u64,
        shard_num: u64
    }

    struct GraphChiInputStream {
        file: Arc<File>,
        pc: Arc<u64>,
    }

    impl AsyncBufRead for GraphChiInputStream {

        /// Read from where pc points
        fn poll_fill_buf(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>
        ) -> Poll<Result<&[u8]>> {
            unimplemented!()
        }

        fn consume(self: Pin<&mut Self>, amt: usize) {
            unimplemented!()
        }
    }

    impl AsyncRead for GraphChiInputStream {
        fn poll_read (
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut [u8]
        ) -> Poll<Result<usize>> {
            unimplemented!()
        }
    }

    impl AsyncSeek for GraphChiInputStream {

        fn start_seek(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            position: SeekFrom
        ) -> Poll<Result<()>> {
            unimplemented!()
        }

        fn poll_complete(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>
        ) -> Poll<Result<u64>> {
            unimplemented!()
        }
    }

    impl InputStream for GraphChiInputStream {
        fn readline(&mut self) -> Result<Option<Stream>> {
            self.file.seek(SeekFrom::Start(self.pc.into_u64())).await?;
            let mut flag = vec![0u8; 4];
            self.file.read_exact(&mut flag).await?;
            self.pc.add_assign(4);
            let length = into_usize(flag);
            /// Multithread scenario analysis:
            /// Two threads A and B concurrently call readline() of a single input stream reference.
            ///
            poll_fn(move |ctx|
                self.poll_in_buf( ctx, length)
            ).await.map (|x|  {
                    self.pc.add_assign(x.len() as u64);
                    if x.len() > 0 { Some(Stream::from(buf)) } else { None }
                }
            )
        }

        fn readline_range (
            &mut self,
            start: u64,
            end: u64
        ) -> Result<Vec<Stream>> {
            unimplemented!()
        }

        fn read_all_lines(&mut self) -> Result<Vec<Stream>> {
            unimplemented!()
        }

        fn open<P: AsRef<Path>> (
            path: P
        ) -> Result<File>{
            let std_file = std::fs::File::open(path.as_ref());
            match std_file {
                Ok(f) => Ok(File::from_std(f)),
                Err(e) => Err(tokio::io::Error::new(e.kind(), e))
            }
        }

        fn is_end(&self) -> bool {
            unimplemented!()
        }

        fn is_empty(&self) -> bool {
            unimplemented!()
        }

        fn len(&self) -> u64 {
            unimplemented!()
        }

    }

    impl Iterator for GraphChiInputStream {
        type Item = Stream;

        fn next(&mut self) -> Option<Self::Item> {
            match self.readline() {
                Ok(line) => line,
                Err(e) => None
            }
        }
    }

    impl GraphChiInputStream {
        pub fn new(file: File) -> GraphChiInputStream {
            GraphChiInputStream {
                file: Arc::new(file),
                pc: Arc::new(1),
            }
        }

        fn poll_in_buf(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            amt: usize
        ) -> Poll<Result<&[u8]>> {
            self.consume(amt);
            self.poll_fill_buf(cx)
        }
    }

    #[derive(Clone)]
    pub struct GraphChiStorage {
        edge_buffer: super::EdgeBuffer,
        root_dir: String,
        prefix_interval: String,
        interval_metadata_buf: Vec<Metadata>,
        gid: u32
    }

    impl GraphChiStorage {

        pub async fn get_interval (
            &self,
            interval_id: &usize
        ) -> Result<(Vec<AdjacentShard>, EdgeDataShard)> {
            let path = Path::new (
                root_path_for_interval(self.gid.borrow(), interval_id).as_str()
            );
            match GraphChiInputStream::open(path) {
                Ok(s) =>
                    self.extract_adj_and_edge_shard(s),
                Err(e) => {
                    Err(
                        IntervalLoadError::new(
                            "Error occurs when loading Interval".to_string().borrow(),
                            std::io::Error::new(e.kind(), e)
                        ).into()
                    )
                }
            }
        }

        pub fn get_shard_num (
            &self,
            interval_id: &usize
        ) -> Option<usize> {
            match self.interval_metadata_buf.
                binary_search_by_key(
                    interval_id,
                    |x| x.interval_id as usize
                ) {
                Ok(pos) => match self.interval_metadata_buf.get(pos) {
                    Some(metadata) =>
                        Some(metadata.shard_num as usize),
                    None => None
                }
                Err(e) => None
            }
        }

        pub async fn update_interval (
            &self,
            interval_id: &usize,
            adj_shard: &AdjacentShard,
            edge_shard: &EdgeDataShard
        ) -> Result<()> {
            // TODO
            unimplemented!()
        }

        pub fn new(
            gid: &u32,
            root_dir: &String,
            prefix_interval: &String
        ) -> Self {
            GraphChiStorage {
                edge_buffer: EdgeBuffer { buffer: BinaryHeap::<Unit>::new() },
                root_dir: root_dir.clone(),
                prefix_interval: prefix_interval.clone(),
                interval_metadata_buf: vec![],
                gid: gid.clone()
            }
        }

        fn process_line(
            x: &Stream,
            adj_shard: &mut Vec<AdjacentShard>,
            edge_arr: &mut Vec<Edge<f64>>
        ) {
            // TODO byte parser
        }

        fn extract_adj_and_edge_shard (
            &self,
            file: File
        ) -> Result<(Vec<AdjacentShard>, EdgeDataShard)> {
            let ref mut adj_shard = vec![];
            let ref mut edge_arr = vec![];
            let ref mut file_stream = GraphChiInputStream::new(file);
            file_stream.for_each(
                |x|
                    Self::process_line(
                        &x,
                        adj_shard,
                        edge_arr
                    )
            );
            Ok(
                (
                    adj_shard.to_vec(),
                    EdgeDataShard { 0: edge_arr.to_vec() }
                )
            )
        }
    }
}


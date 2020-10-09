use std::io::Result;
use crate::graph::core::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub trait Storage {

    /// load interval with interval id from disk
    /// PSW can call this directly to load one interval.
    fn get_interval(
        &self,
        interval_id: &usize
    ) -> Result<(Vec<AdjacentShard>, EdgeDataShard)>;

    ///
    fn get_shard_num(&self, interval_id: &usize) -> Option<usize>;

    /// Flush all updated interval data into disk
    fn update_interval(
        &self,
        adj_shard: AdjacentShard,
        edge_shard: EdgeDataShard
    ) -> Result<()>;
}

mod io {
    use std::io::{Read, Write, Result};
    use std::borrow::Borrow;
    use std::path::Path;
    use crate::graph as Graph;
    use Graph::core::*;

    pub type Stream = Vec<u8>;

    /// A common type for file output stream.
    pub trait InputStream: Read {

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
        /// # Error
        ///
        fn open<P: AsRef<Path>>(path: P) -> Result<dyn InputStream>;

        /// Return true:  pc in the end of the stream.
        /// Return false: pc is not in the end
        fn is_end(&self) -> bool;

        fn is_empty(&self) -> bool;

        // The number of the line
        fn len(&self) -> u64;

    }

    impl Iterator for dyn InputStream {
        type Item = Stream;

        fn next(&mut self) -> Option<Self::Item> {
            match self.readline() {
                Ok(line) => line,
                Err(e) => None
            }
        }
    }


    /// It is a general type of Stream Transformer. It is mainly used to transform each line of a input stream in a
    /// specific type by a given decoder
    pub trait StreamTransformer {

        fn decode_all_lines<U, F>(
            &self,
            input_stream: &mut dyn InputStream,
            decoder: F
        ) -> &Vec<U> where F: Fn(&Stream) -> U {
            let ref mut result = vec![];
            while !input_stream.is_end() {
                match input_stream.readline() {
                    Ok(stream) => match stream {
                        Some(line) => result.push(decoder(line.borrow())),
                        None => break
                    },
                    Err(e) => panic!(e)
                }
            }
            result
        }

        fn decode_lines_range<U, F>(
            &self,
            input_stream: &mut dyn InputStream,
            decoder: F) -> &Vec<U>
            where F: Fn(&Stream) -> U {
        }

        fn decode_stream<U, F>(
            stream: &Stream,
            decoder: F
        ) -> U where F: Fn(&Stream) -> U {
            decoder(stream)
        }

        fn encode_to_stream<U, F>(
            data: &U,
            encoder: F
        ) -> Stream where F: Fn(&U) -> Stream {
            encoder(data)
        }
    }

    pub trait OutputStream: Write {

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
        fn open<P: AsRef<Path>>(&mut self, path: &P) -> Result<()>;
    }

    impl Write for dyn OutputStream {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            unimplemented!()
        }

        fn flush(&mut self) -> Result<()> {
            unimplemented!()
        }
    }
}

#[derive(PartialOrd, PartialEq)]
struct Unit<'a>(ShardId, IntervalId, Edge<'a, f64>);
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

struct EdgeBuffer<'a>  {
    buffer: BinaryHeap<Unit<'a>>
}

pub mod storage_core {
    use std::io::Result;
    use std::path::Path;
    use std::borrow::Borrow;
    use std::collections::BinaryHeap;
    use crate::graph as Graph;
    use Graph::core::*;
    use Graph::storage::EdgeBuffer;
    use super::io::*;
    use tokio::fs::File;
    use crate::error::IntervalLoadError;

    mod codec {
        use std::io::Result;
        use crate::graph::storage::io::*;

        pub trait Codec<R: InputStream, W: OutputStream, U> {
            fn encode(data: Vec<U>, level: i32) -> Result<&'static [Stream]>;

            fn decode(steam: Stream) -> Result<U>;
        }

    }

    #[derive(Hash, PartialOrd, PartialEq)]
    struct Metadata {
        line_num: u64,
        interval_id: u64,
        shard_size: u64,
        shard_num: u64
    }

    struct GraphChiInputStream<'a> {
        file: &'a File,
        pc: u64
    }

    impl <'a> InputStream for GraphChiInputStream<'a> {
        fn readline(&mut self) -> Result<Option<Stream>> {
            unimplemented!()
        }

        fn readline_range(
            &mut self,
            start: u64,
            end: u64
        ) -> Result<Vec<Stream>> {
            unimplemented!()
        }

        fn read_all_lines(&mut self) -> Result<Vec<Stream>> {
            unimplemented!()
        }

        fn open<P: AsRef<Path>>(path: P) -> Result<dyn InputStream> {
            unimplemented!()
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

    pub struct GraphChiStorage<'a> {
        edge_buffer: super::EdgeBuffer<'a>,
        root_dir: String,
        prefix_interval: String,
        interval_metadata_buf: Vec<Metadata>
    }

    impl <'a> super::Storage for GraphChiStorage<'a> {

        fn get_interval(
            &self,
            interval_id: &usize
        ) -> Result<(Vec<AdjacentShard>, EdgeDataShard<'a>)> {
            let path = Path::new(&(Graph::constants::ROOT_PATH + "/interval_" + stringify!(interval_id)));
            let ref mut in_stream = GraphChiInputStream::open(path);
            let ref mut adj_shard = vec![];
            let ref mut edge_arr = vec![];
            match in_stream {
                Ok(s) => {
                    s.for_each(
                        move |x|
                            Self::process_line(
                                x, adj_shard,
                                edge_arr
                            )
                    );
                    Ok(
                        (
                            adj_shard.into_vec(),
                            EdgeDataShard { 0: edge_arr.iter().map(|x| x).collect() }
                        )
                    )
                },
                Err(e) => {
                    Err(
                        IntervalLoadError::new(
                            "Error occurs when loading Interval".to_string().borrow(),
                            e
                        ).into()
                    )
                }
            }
        }

        fn get_shard_num(
            &self,
            interval_id: &usize
        ) -> Option<usize> {
            match self.interval_metadata_buf.
                binary_search_by_key(
                    interval_id.into_u64().borrow(),
                    |x| x.interval_id
                ) {
                Ok(pos) => match self.interval_metadata_buf.get(pos) {
                    Some(metadata) =>
                        Some(metadata.shard_num.borrow() as usize),
                    None => None
                }
                Err(e) => None
            }
        }

        fn update_interval(
            &self,
            adj_shard: AdjacentShard,
            edge_shard: EdgeDataShard<'a>
        ) -> Result<()> {
            // TODO
            unimplemented!()
        }
    }

    impl <'a> GraphChiStorage<'a> {

        pub fn new(
            root_dir: &String,
            prefix_interval: &String
        ) -> Self {
            GraphChiStorage {
                edge_buffer: EdgeBuffer { buffer: BinaryHeap::new() },
                root_dir: root_dir.clone(),
                prefix_interval: prefix_interval.clone(),
                interval_metadata_buf: vec![]
            }
        }

        pub fn process_line(
            x: &dyn InputStream,
            adj_shard: &mut Vec<AdjacentShard>,
            edge_arr: &mut Vec<Edge<'a, f64>>) {
            // TODO
        }
    }
}

// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// rustpkg utilities having to do with local and remote paths

use std::clone::Clone;
use std::hash::Streaming;
use std::hash;
use std::option::Some;
use std::path::Path;
use std::rt::io::Writer;

/// Wrappers to prevent local and remote paths from getting confused
/// (These will go away after #6407)
pub struct RemotePath (Path);

impl Clone for RemotePath {
    fn clone(&self) -> RemotePath {
        RemotePath((**self).clone())
    }
}

pub struct LocalPath (Path);

impl Clone for LocalPath {
    fn clone(&self) -> LocalPath {
        LocalPath((**self).clone())
    }
}


// normalize should be the only way to construct a LocalPath
// (though this isn't enforced)
/// Replace all occurrences of '-' in the stem part of path with '_'
/// This is because we treat rust-foo-bar-quux and rust_foo_bar_quux
/// as the same name
pub fn normalize(p_: RemotePath) -> LocalPath {
    let RemotePath(p) = p_;
    match p.filestem() {
        None => LocalPath(p),
        Some(st) => {
            let replaced = st.replace("-", "_");
            if replaced != st {
                LocalPath(p.with_filestem(replaced))
            }
            else {
                LocalPath(p)
            }
        }
    }
}

pub fn write<W: Writer>(writer: &mut W, string: &str) {
    writer.write(string.as_bytes());
}

pub fn hash(data: ~str) -> ~str {
    let hasher = &mut hash::default_state();
    write(hasher, data);
    hasher.result_str()
}

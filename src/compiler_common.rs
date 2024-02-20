// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Shared with build.rs and compiler.rs

pub struct DummyLoader;

impl rquickjs::loader::Loader for DummyLoader {
    fn load(
        &mut self,
        _ctx: &rquickjs::Ctx<'_>,
        name: &str,
    ) -> rquickjs::Result<rquickjs::module::ModuleData> {
        Ok(rquickjs::module::ModuleData::source(name, ""))
    }
}

pub struct DummyResolver;

impl rquickjs::loader::Resolver for DummyResolver {
    fn resolve(
        &mut self,
        _ctx: &rquickjs::Ctx<'_>,
        _base: &str,
        name: &str,
    ) -> rquickjs::Result<String> {
        Ok(name.into())
    }
}

pub fn human_file_size(size: usize) -> String {
    let fsize = size as f64;
    let i = if size == 0 {
        0
    } else {
        (fsize.log2() / 1024f64.log2()).floor() as i32
    };
    let size = fsize / 1024f64.powi(i);
    let units = ["B", "kB", "MB", "GB", "TB", "PB"];
    format!("{:.3} {}", size, units[i as usize])
}

// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::{Func, Opt},
    Ctx, Result, Value,
};

use crate::{
    http::{url::URL, url_search_params::URLSearchParams},
    module::export_default,
    utils::result::ResultExt,
};
pub struct UrlModule;

static ERROR_MESSAGE: &str = "Not a valid URL";

fn url<'js>(ctx: Ctx<'js>, input: Value<'js>, base: Opt<Value<'js>>) -> Result<URL<'js>> {
    URL::new(ctx.clone(), input, base).or_throw_msg(&ctx, ERROR_MESSAGE)
}

fn can_parse(value: Value<'_>) -> bool {
    URL::can_parse(value)
}

fn url_search_params<'js>(ctx: Ctx<'js>, init: Opt<Value<'js>>) -> Result<URLSearchParams> {
    URLSearchParams::new(ctx.clone(), init)
}

impl ModuleDef for UrlModule {
    fn declare(declare: &mut Declarations) -> Result<()> {
        declare.declare("URL")?;
        declare.declare("canParse")?;
        declare.declare("URLSearchParams")?;
        declare.declare("default")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            default.set("URL", Func::from(url))?;
            default.set("canParse", Func::from(can_parse))?;
            default.set("URLSearchParams", Func::from(url_search_params))?;
            Ok(())
        })
    }
}

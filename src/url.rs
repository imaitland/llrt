// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use once_cell::sync::Lazy;
use ring::rand::SecureRandom;
use rquickjs::{
    module::{Declarations, Exports, ModuleDef},
    prelude::{Func, Opt},
    Ctx, Function, IntoJs, Object, Result, TypedArray, Value,
};
use url::Url;
use uuid::Uuid;
use uuid_simd::UuidExt;

use crate::{
    crypto::SYSTEM_RANDOM,
    encoding::encoder::bytes_to_hex,
    module::export_default,
    utils::{
        object::{get_bytes, get_bytes_offset_length},
        result::ResultExt,
    },
};

pub struct UrlModule;

static ERROR_MESSAGE: &str = "Not a valid URL";

static NODE_ID: Lazy<[u8; 6]> = Lazy::new(|| {
    let mut bytes = [0; 6];
    SYSTEM_RANDOM.fill(&mut bytes).unwrap();
    bytes
});

// keep
struct UrlObject<'js> {
    obj: Object<'js>,
    has_value: bool,
}
// keep
impl<'js> UrlObject<'js> {
    fn new(ctx: Ctx<'js>) -> Result<Self> {
        Ok(Self {
            obj: Object::new(ctx)?,
            has_value: false,
        })
    }

    fn into_value(self, ctx: &Ctx<'js>) -> Result<Value<'js>> {
        if self.has_value {
            return Ok(self.obj.into_value());
        }
        "".into_js(ctx)
    }
}

fn from_value<'js>(ctx: &Ctx<'js>, value: Value<'js>) -> Result<Uuid> {
    if value.is_string() {
        Uuid::try_parse(&value.as_string().unwrap().to_string()?)
    } else {
        Uuid::from_slice(&get_bytes(ctx, value)?)
    }
    .or_throw_msg(ctx, ERROR_MESSAGE)
}

fn uuidv1() -> String {
    Uuid::now_v1(&NODE_ID).format_hyphenated().to_string()
}

fn uuidv3<'js>(ctx: Ctx<'js>, name: String, namespace: Value<'js>) -> Result<String> {
    let uuid = Uuid::new_v3(&from_value(&ctx, namespace)?, name.as_bytes())
        .format_hyphenated()
        .to_string();
    Ok(uuid)
}

fn uuidv5<'js>(ctx: Ctx<'js>, name: String, namespace: Value<'js>) -> Result<String> {
    let uuid = Uuid::new_v5(&from_value(&ctx, namespace)?, name.as_bytes())
        .format_hyphenated()
        .to_string();
    Ok(uuid)
}

pub fn uuidv4() -> String {
    Uuid::new_v4().format_hyphenated().to_string()
}

// keep
fn echo(ctx: Ctx<'_>, value: String) -> String {
    value
}
//keep
fn parse(ctx: Ctx<'_>, value: String) -> Object<'_> {
    let url = Url::parse(&value)
        .or_throw_msg(&ctx, ERROR_MESSAGE)
        .unwrap();
    let obj = UrlObject::new(ctx.clone())
        .or_throw_msg(&ctx, ERROR_MESSAGE)
        .unwrap();
    let _ = obj.obj.set("hostname", url.host_str());
    obj.obj
}

fn stringify<'js>(ctx: Ctx<'js>, value: Value<'js>, offset: Opt<u8>) -> Result<String> {
    let value = get_bytes_offset_length(&ctx, value, offset.0.map(|o| o.into()), None)?;
    let value = bytes_to_hex(&value);

    let uuid = Uuid::try_parse_ascii(&value)
        .or_throw_msg(&ctx, ERROR_MESSAGE)?
        .as_hyphenated()
        .to_string();

    Ok(uuid)
}

fn validate(value: String) -> bool {
    Uuid::parse_str(&value).is_ok()
}

fn version(ctx: Ctx<'_>, value: String) -> Result<u8> {
    let uuid = Uuid::parse_str(&value).or_throw_msg(&ctx, ERROR_MESSAGE)?;
    Ok(uuid.get_version().map(|v| v as u8).unwrap_or(0))
}

impl ModuleDef for UrlModule {
    fn declare(declare: &mut Declarations) -> Result<()> {
        declare.declare("v1")?;
        declare.declare("v3")?;
        declare.declare("v4")?;
        declare.declare("v5")?;
        declare.declare("parse")?;
        declare.declare("echo")?;
        declare.declare("validate")?;
        declare.declare("stringify")?;
        declare.declare("version")?;
        declare.declare("NIL")?;
        declare.declare("default")?;

        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &mut Exports<'js>) -> Result<()> {
        export_default(ctx, exports, |default| {
            let dns_namespace = Uuid::NAMESPACE_DNS.format_hyphenated().to_string();
            let url_namespace = Uuid::NAMESPACE_URL.format_hyphenated().to_string();

            let v3_func = Function::new(ctx.clone(), uuidv3)?;
            let v3_object = v3_func.as_object().unwrap();

            let v5_func = Function::new(ctx.clone(), uuidv5)?;
            let v5_object = v5_func.as_object().unwrap();

            v3_object.set("DNS", dns_namespace.clone())?;
            v3_object.set("URL", url_namespace.clone())?;
            v5_object.set("DNS", dns_namespace)?;
            v5_object.set("URL", url_namespace)?;

            default.set("v1", Func::from(uuidv1))?;
            default.set("v3", v3_func)?;
            default.set("v4", Func::from(uuidv4))?;
            default.set("v5", v5_func)?;
            default.set("NIL", "00000000-0000-0000-0000-000000000000")?;
            default.set("parse", Func::from(parse))?;
            default.set("echo", Func::from(echo))?;
            default.set("stringify", Func::from(stringify))?;
            default.set("validate", Func::from(validate))?;
            default.set("version", Func::from(version))?;
            Ok(())
        })
    }
}

use std::error::Error;
use std::num::ParseFloatError;

use super::traits::FromStrBasedOnCtx;
use crate::context::Context;

pub type Number = f64;

impl FromStrBasedOnCtx for Number {
    fn from_str_based_on(s: &str, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        match s {
            "$time" => Ok(ctx.current_time),
            s => s.parse().map_err(|e: ParseFloatError| e.into())
        }
    }
}


use std::error::Error;
use core::slice::Iter;
use std::convert::{TryFrom, TryInto};
use crate::PartsSetting;
use crate::context::Context;


// FromStrBasedOnCtxを実装すれば "...".from_str_based_on(ctx)で文字列を任意の型にパースすることができるようになる

pub trait FromStrBasedOnCtx where Self: Sized { fn from_str_based_on(s: &str, ctx: &Context) -> Result<Self, Box<dyn Error>>; }
pub trait ParsableBasedOnCtx<To: FromStrBasedOnCtx> { fn parse_based_on(&self, ctx: &Context) -> Result<To, Box<dyn Error>>; }
impl<To: FromStrBasedOnCtx> ParsableBasedOnCtx<To> for &str {
    fn parse_based_on(&self, ctx: &Context) -> Result<To, Box<dyn Error>> {
        To::from_str_based_on(self, ctx)
    }
}


// visualize用要素に変換するためのクラスのトレイト
// Visualizableはそれ単体で変換可能で、VisualizableByはVisualizableなFromをTryFromで変換して生成する
// set_by_param_name_and_word以外はデフォルト実装があるので基本はこれだけ実装すれば良い

pub trait Visualizable<'a>
where
    Self: Default
{
    fn set_by_param_name_and_word(&mut self, param_name: &'a str, word: &'a str, ctx: &Context) -> Result<(), Box<dyn Error>>;

    fn default_by_setting(setting: &'a PartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        let mut parts = Self::default();
        for (param_name, word) in &setting.default_values {
            parts.set_by_param_name_and_word(param_name.as_str(), word.as_str(), ctx)?;
        }
        Ok(parts)
    }

    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a PartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        let mut parts = Self::default_by_setting(setting, ctx)?;
        for params_name in setting.input_params.iter() {
            let next_word = *words_iter.next().ok_or(format!("words iter don't have more words. required: {}", params_name))?;
            parts.set_by_param_name_and_word(params_name.as_str(), next_word, ctx)?;
        }
        Ok(parts)
    }
}

pub trait VisualizableFrom<'a, From>
where
    From: Visualizable<'a> + Default,
    Self: TryFrom<From>,
    <Self as TryFrom<From>>::Error: Into<Box<dyn Error>>
{
    fn extract_diff_from(&self, other: &Self) -> Self;

    fn from_words_and_setting(words_iter: &mut Iter<&'a str>, setting: &'a PartsSetting, ctx: &Context) -> Result<Self, Box<dyn Error>> {
        From::from_words_and_setting(words_iter, setting, ctx)?.try_into().map_err(|e: <Self as TryFrom<From>>::Error| e.into())
    }
}

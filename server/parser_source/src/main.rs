mod context;
mod setting;
mod consts;
mod utils;
mod parts;

use std::fs::{read_to_string, File};
use std::error::Error;
use std::io::Write;

use docopt::Docopt;
use serde::{ Deserialize };

use context::Context;
use setting::{ load_settings, Setting };
use parts::rect::{ Rect };
use parts::traits::*;
use parts::charts::Charts;
use crate::parts::circle::Circle;
use crate::parts::shapes::Canvases;
use std::collections::HashMap;
use crate::parts::path::Path;


/*
{
  'charts': {
    'canvasID': [
        { "name": "", "data": [[x,y], [x,y], ], "color": "#000000" }
        { "name": "", "data": [y,y,y,y ], "color": "#000000" }
        ...
    ]
  }
  'shapes': {
    'canvasID': {
      "initial": {
        "time": 0.,
        "shapes": [{}, ...]
      }
      "final": {
        "time": 0.,
        "shapes": [{}, ...]
      }
      "transitions": [
        {
          "time": num,
          "next": [{ "ID": "", ... }],
          "prev": [{ "ID": "", ... }],
        }
      ]
    }
  }
}
*/


const USAGE: &str = r"
Parser

Usage:
  ./parser [--input=<in> --settings=<settings> --output=<out>]
  ./parser (-h | --help)

Options:
  -h --help                  Show this screen.
  -i --input=<in>            input file path    [default: ./input.txt]
  -o --output=<out>          output destination [default: ./output.json]
  -s --settings=<settings>   settings path      [default: ./settings.json]
";

#[derive(Debug, Deserialize)]
struct Args {
    flag_input: String,
    flag_output: String,
    flag_settings: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    println!("{:?}", args);

    let content_path = args.flag_input;
    let output_dest = args.flag_output;
    let settings_path = args.flag_settings.as_str();

    let content = read_to_string(content_path)?;
    let words = content.split_whitespace().collect::<Vec<_>>();

    let settings = load_settings(settings_path)?;

    // println!("settings: {:?}", settings);

    println!("\n ======================= \n");

    let mut ctx = Context::default();
    let mut words_iter = words.iter();
    let mut charts = Charts::default();
    let mut canvases = Canvases::default();

    while let Some(&shape) = words_iter.next() {
        match shape {
            "update" => {
                let next_time: f64 = words_iter.next().ok_or("words iter don't have more words. required: time")?.parse_based_on(&ctx)?;
                if next_time.is_finite() {
                    ctx.update_time(next_time);
                    Ok(())
                } else {
                    Err(format!("this number is invalid: {}", next_time))
                }
            }
            "chart" => {
                charts.add_datum_by_words_and_setting(&mut words_iter, settings.get("chart").unwrap(), &ctx)?;
                Ok(())
            }
            "rect" => {
                let rect = Rect::from_words_and_setting(&mut words_iter, settings.get("rect").unwrap(), &ctx)?;
                canvases.add_rect(rect, &ctx);
                Ok(())
            },
            "point" => {
                let circle = Circle::from_words_and_setting(&mut words_iter, settings.get("point").unwrap(), &ctx)?;
                canvases.add_circle(circle, &ctx);
                Ok(())
            }
            "path" => {
                let path = Path::from_words_and_setting(&mut words_iter, settings.get("path").unwrap(), &ctx)?;
                canvases.add_path(path, &ctx);
                Ok(())
            }
            cmd => { Err(format!("this command is not exists: {}", cmd)) }
        }?;
    }

    println!("\n ======================= \n");

    let mut file = File::create(output_dest)?;
    let mut result = HashMap::new();
    result.insert("charts", serde_json::to_value(charts.create_map())?);
    result.insert("shapes", serde_json::to_value(canvases.create_map())?);
    writeln!(file, "{}", serde_json::to_string(&result)?)?;
    file.flush()?;

    Ok(())
}

mod context;
mod setting;
mod consts;
mod utils;
mod vis_comps;

use std::fs::{read_to_string, File};
use std::error::Error;
use std::io::Write;

use docopt::Docopt;
use serde::{ Deserialize };

use context::Context;
use setting::{load_settings, GraphicPartsSetting};
use vis_comps::vis_comps_creator::VisCompsCreator;
use vis_comps::traits::ParsableBasedOnCtx;


/*
{
  $groupID: {
    camera?: { x,y,w,h }
    charts?: {
      $lineID: {
        { "name": "", "data": [[x,y], [x,y], ], "color": "#000000" }
        { "name": "", "data": [y,y,y,y ], "color": "#000000" }
        ...
      }
      ...
    },
    graphic?: {
      "initial": {
        "time": num,
        "elems": [{}, ...]
      }
      "final": {
        "time": num,
        "elems": [{}, ...]
      }
      "transitions": [
        {
          "time": num,
          "next": [{ "elemID": "", ... }],
          "prev": [{ "elemID": "", ... }],
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

    let settings = load_settings(settings_path)?;

    let content = settings.initial_text + " " + read_to_string(content_path)?.as_str();
    let words = content.split_whitespace().collect::<Vec<_>>();

    println!("\n ===== now parsing ===== \n");

    let mut ctx = Context::default();
    let mut words_iter = words.iter();
    let mut vis_comps_creator = VisCompsCreator::default();

    while let Some(&command) = words_iter.next() {
        match command {
            "update" => {
                let next_time: f64 = words_iter.next().ok_or("words iter don't have more words. required: time")?.parse_based_on(&ctx)?;
                if next_time.is_finite() { ctx.update_time(next_time); Ok(()) }
                else { Err(format!("this number is invalid: {}", next_time).into()) }
            }
            command => {
                if let Some(setting) = settings.command_to_setting.get(command) {
                    match setting.use_elem.as_str() {
                        "chart"  => { vis_comps_creator.add_line_datum_from(&mut words_iter, setting, &ctx) }
                        "camera" => { vis_comps_creator.add_camera_from    (&mut words_iter, setting, &ctx) }
                        "circle" => { vis_comps_creator.add_circle_from    (&mut words_iter, setting, &ctx) }
                        "path"   => { vis_comps_creator.add_path_from      (&mut words_iter, setting, &ctx) }
                        "rect"   => { vis_comps_creator.add_rect_from      (&mut words_iter, setting, &ctx) }
                        parts_name => { Err(format!("this parts is not exists: {}", parts_name).into()) }
                    }
                } else { Err(format!("this command is not exists: {}", command).into()) }
            }
        }?;
    }

    println!("\n ====== completed ====== \n");

    let mut file = File::create(output_dest)?;
    writeln!(file, "{}", vis_comps_creator.create_json_string()?)?;
    file.flush()?;

    Ok(())
}

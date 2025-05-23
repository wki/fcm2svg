use std::{fs, path::PathBuf};
use std::any::Any;
use std::io::Error;
use clap::{Parser};
use fcmlib;
use fcmlib::{Outline, PathShape};

#[derive(Debug)]
#[derive(Parser)]
struct Cli {
    /// Specifies the input file
    #[arg(short, long, value_name = "FILE")]
    input_file: PathBuf,

    /// Specifies the output file
    #[arg(short, long, value_name = "FILE")]
    output_file: PathBuf,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

struct Pt {
    x: f64,
    y: f64
}

impl Pt {
    fn new(x: i32, y: i32, transform: (f32,f32,f32,f32,f32,f32)) -> Self {
        let f_x = f64::from(x);
        let f_y = f64::from(y);
        let (t0,t1,t2,t3,t4,t5) = transform;
        let t0 = f64::from(t0);
        let t1 = f64::from(t1);
        let t2 = f64::from(t2);
        let t3 = f64::from(t3);
        let t4 = f64::from(t4);
        let t5 = f64::from(t5);
        let t_x = (f_x * t0 + f_y * t1 + t4) / 35.277777;
        let t_y = (f_x * t2 + f_y * t3 + t5) / 35.277777;

        Pt {x: t_x, y: t_y}
    }

    fn to_string(&self) -> String {
        format!("{:.3},{:.3}", self.x, self.y)
    }
}

fn main() {
    let cli = Cli::parse();

    let fcm = fcmlib::FcmFile::from_file(cli.input_file.as_path());
    // dbg!(&fcm);
    // dbg!(&fcm.unwrap().piece_table.pieces);
    for (i, piece) in &fcm.unwrap().piece_table.pieces {
        // dbg!(&piece);
        let transform = piece.transform.unwrap();
        for path in &piece.paths {
            // TODO: path.tool --> fill style
            match &path.shape {
                Some(PathShape{start, outlines}) => {
                    let mut svg_path : Vec<String> = vec![];
                    let pt = Pt::new(start.x, start.y, transform);
                    svg_path.push(format!("M{}", pt.to_string()));
                    for outline in outlines {
                        match outline {
                            Outline::Line(line) => {
                                for segment in line {
                                    let lineto = Pt::new(segment.end.x, segment.end.y, transform);
                                    svg_path.push(format!("L{}", lineto.to_string()));
                                }
                            }
                            Outline::Bezier(bezier) => {
                                for segment in bezier {
                                    let cp1 = Pt::new(segment.control1.x, segment.control1.y, transform);
                                    let cp2 = Pt::new(segment.control2.x, segment.control2.y, transform);
                                    let curveto = Pt::new(segment.end.x, segment.end.y, transform);
                                    svg_path.push(format!("C{} {} {}", cp1.to_string(), cp2.to_string(), curveto.to_string()));
                                }
                            }
                        }
                    }
                    let path_str = svg_path.join("");
                    println!("<path d=\"{}\" style=\"TODO\"/>", path_str);
                }
                None => {}
            }
        }
    }
    dbg!(&cli);

    // let result = load(&cli.input_file)
    //     .and_then(convert)
    //     .and_then(|c| save(&cli.output_file, c));
    // 
    // match result {
    //     Ok(_) => println!("Done"),
    //     Err(e) => println!("Could not process, error occured: '{}'", e),
    // }
}

fn load(file: &PathBuf) -> Result<String, Error> {
    fs::read_to_string(file)
}

fn convert(content: String) -> Result<String, Error> {
    // TODO: convert to SVG
    Ok(content + " / converted")
}

fn save(file: &PathBuf, content: String) -> Result<(), Error> {
    fs::write(file, content)
}

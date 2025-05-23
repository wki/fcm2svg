use std::{fmt, fs, path::PathBuf};
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

impl fmt::Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn main() {
    let cli = Cli::parse();

    let mut xml_paths: Vec<String> = vec![];
    let fcm = fcmlib::FcmFile::from_file(cli.input_file.as_path());
    for (_, piece) in &fcm.unwrap().piece_table.pieces {
        let transform = piece.transform.unwrap();
        for path in &piece.paths {
            let mut svg_path: Vec<String> = vec![];
            // TODO: path.tool --> fill style
            match &path.shape {
                Some(PathShape { start, outlines }) => {
                    let pt = Pt::new(start.x, start.y, transform);
                    svg_path.push(format!("M{pt}"));
                    for outline in outlines {
                        match outline {
                            Outline::Line(line) => {
                                for segment in line {
                                    let lineto = Pt::new(segment.end.x, segment.end.y, transform);
                                    svg_path.push(format!("L{lineto}"));
                                }
                            }
                            Outline::Bezier(bezier) => {
                                for segment in bezier {
                                    let cp1 = Pt::new(segment.control1.x, segment.control1.y, transform);
                                    let cp2 = Pt::new(segment.control2.x, segment.control2.y, transform);
                                    let curveto = Pt::new(segment.end.x, segment.end.y, transform);
                                    svg_path.push(format!("C{cp1} {cp2} {curveto}"));
                                }
                            }
                        }
                    }
                }
                None => {}
            }
            let xml_path = svg_path.join(" ");
            xml_paths.push(format!("  <path d=\"{}\" style=\"stroke:black;stroke-width:0.24px;\"/>", xml_path));
        }
    }

    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n");
    xml.push_str("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n");
    xml.push_str("<svg width=\"842px\" height=\"596px\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" xml:space=\"preserve\" xmlns:serif=\"http://www.serif.com/\" style=\"fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round;stroke-linejoin:round;stroke-miterlimit:1.5;\">\n");

    for xml_path in &xml_paths {
        xml.push_str(&format!("  {xml_path}\n"));
    }

    xml.push_str("</svg>\n");

    fs::write(&cli.output_file, &xml).ok();
}

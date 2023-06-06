
use chrono::prelude::*;

#[cfg(target_arch = "wasm32")]
static REPLAY: &'static [u8] = include_bytes!("/home/tommy/dev/bar-spring/bld-profile-gcc64/demos/md.gz");

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    current_frame: f64,
    #[serde(skip)]
    replay: RecoilReplay,
    #[serde(skip)]
    texture: Option<egui::TextureHandle>,
    #[serde(skip)]
    speed: f64,
    #[serde(skip)]
    last_update: DateTime<Utc>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            current_frame: 0.0,
            replay: RecoilReplay::new(),
            texture: None,
            speed: 30.0,
            last_update: Utc::now(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

use egui::pos2;
use egui::Vec2;
use egui::Sense;

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label, current_frame , replay, texture, speed, last_update } = self;

        let ms_diff = Utc::now().signed_duration_since(*last_update).num_milliseconds() as f64;
        *current_frame = (*current_frame + (ms_diff * (*speed) / 1000.0)).clamp(0.0, (replay.frames.len()-1) as f64);

        //*current_frame = std::cmp::min(next_frame, 

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        /*
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            //ui.add(egui::Slider::new(current_frame, 0.0..=10.0).text("current_frame").step_by(1.0));
            //if ui.button("Increment").clicked() {
            //    *current_frame += 1.0;
            //}

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });
        */

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("BAR replay");
            /*
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
            */

            let texture: &egui::TextureHandle = texture.get_or_insert_with(|| {
                ui.ctx().load_texture(
                    "my-image",
                    replay.image.clone(), // FIXME do not copy, move?
                    Default::default()
                ) }
                );
            ui.heading("PIC:");
            ui.add(egui::Slider::new(speed, 0.0..=120.0).integer().step_by(1.0 as f64).text("speed"));
            //ui.add(egui::Image::new(texture, texture.size_vec2()));
            //ui.image(texture, texture.size_vec2());

            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );


            let (_, painter) = ui.allocate_painter(texture.size_vec2(), Sense::hover());

            let texture_size = egui::Rect::from_min_max(pos2(0.0, 0.0), texture.size_vec2().to_pos2());
            
            let ref cur = replay.frames[*current_frame as usize];
            let beginning = cur.unit_data_pos as usize;
            let end = beginning + cur.units as usize;
            //println!("cur frame: {}", current_frame);
            painter.image(texture.id(), texture_size, egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), egui::Color32::WHITE);
            replay.unit_data[beginning..end].iter().for_each(|unit| {
                let a = unit.pos_x;
                let b = unit.pos_y;
                //println!("drawing: {} {}", a, b);
                let color = if unit.team == 1 { egui::Color32::RED } else { egui::Color32::BLUE };
                painter.circle_filled(pos2(unit.pos_x as f32, unit.pos_y as f32), 5.0, color);
            });
            ui.expand_to_include_rect(painter.clip_rect());

            /*
            let first_frame = replay.frames.first().map_or(0, |e| {e.num} );
            let last_frame = replay.frames.last().map_or(0, |e| {e.num} );
            let diff = if replay.frames.is_empty() { 0 } else { (last_frame - first_frame) / replay.frames.len() as i32 };
            */
            ui.spacing_mut().slider_width = texture_size.width();
            ui.add(egui::Slider::new(current_frame, 0.0..=(replay.frames.len()-1) as f64).integer().step_by(1.0 as f64)); // text("current_frame").
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }

        *last_update = Utc::now();
        ctx.request_repaint();
    }
}

use std::fs;
use image;
use image::ImageFormat;
use bincode::deserialize;
// use std::io::Cursor;

struct RecoilReplay {
    //data : Vec<u8>,
    frames : Vec<MatchFrame>,
    unit_data : Vec<UnitData>,
    image : egui::ColorImage,
}


#[repr(C, packed)]
#[derive(serde::Deserialize)]
struct MatchFrame {
    num: i32,
    units: u32,
    #[serde(skip)]
    unit_data_pos : u32,
}

#[repr(C, packed)]
#[derive(serde::Deserialize)]
struct UnitData {
	id : u8,
	utype : u8,
	team : u8,
	//uint8_t player=0;
	// bool new_unit=false;
	pos_x: u16,
	pos_y: u16,
}

#[repr(C, packed)]
#[derive(serde::Deserialize)]
struct ImageData {
    dim_x : u16,
    dim_y : u16,
    width : u16,
    height : u16,
    #[serde(skip)]
    data : Vec<u8>,
}

/*
impl<'de> Deserialize<'de> for ImageData {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        // ...deserialize implementation.
        deserializer.deserialize
        
    }
}*/

impl RecoilReplay {
    fn new() -> Self {
        let data = read_demo();

        let it = &data[..];
        println!("file size = {}", it.len());
        let (image, it) = Self::parse_image(it);
        let (frames, unit_data) = Self::parse_units(it);
        
        RecoilReplay { 
            //data,
            frames,
            unit_data,
            image,
        }
    }

    // TODO use std::io::cursor instead of slice?
    fn parse_image(mut it: &[u8]) -> (egui::ColorImage, &[u8]) {
        let secret_message = "Long Live Coil".as_bytes();

        let secret = it.take(..secret_message.len()).unwrap();
        assert_eq!(secret, secret_message);


        let ptr = it.take(..8).unwrap(); 
        let mut image_data : ImageData = deserialize(&ptr).unwrap();
        //let dim_x = u16::from_ne_bytes(ptr.try_into().unwrap()) as usize;
        //let ptr = it.take(..2).unwrap(); 
        //let dim_y = u16::from_ne_bytes(ptr.try_into().unwrap()) as usize;
        ////let dim = unsafe { std::mem::transmute::<&[u8], &[u16]>(nums) };
        let img_size = image_data.width as usize * image_data.height as usize * 4;

        let dim_x = image_data.dim_x as u32;
        let dim_y = image_data.dim_y as u32;

        let mut width = image_data.width as u32;
        let mut height = image_data.height as u32;

        println!("dim {} {} img size: {}x{} {}", dim_x,dim_y,width, height, img_size);

        let img_data = it.take(..img_size).unwrap();
        let img_vec = img_data.to_vec();

        println!("img vec len {}", img_vec.len());
        assert_eq!(img_vec.len(), img_size);
        //let image = image::load_from_memory(img_data).unwrap();
        //let i = image::load_from_memory_with_format(img_data, ImageFormat::Bmp).unwrap();

        let i = image::RgbaImage::from_vec(width, height, img_vec).unwrap();
        let img = image::DynamicImage::ImageRgba8(i);
        /*
        if dim_x > dim_y {
            height = height * dim_y/dim_x;
        } else if dim_y > dim_x {
            width = width * dim_x/dim_y;
        }
        let img = img.resize_exact(width, height,  image::imageops::FilterType::Nearest);
        */
        let img = img.resize_exact(dim_x, dim_y,  image::imageops::FilterType::Nearest);
        let width = dim_x;
        let height = dim_y;

        let buf = img.to_rgba8();
        let pixels = buf.as_flat_samples();
        let img_data = pixels.as_slice();
        let img = egui::ColorImage::from_rgba_unmultiplied([width as usize, height as usize], img_data);

        (img, it)

    }

    fn parse_units(mut it: &[u8]) -> (Vec<MatchFrame>, Vec<UnitData>) {
        println!("parse units size: {}", it.len());

        let mut frames : Vec<MatchFrame> = Default::default();
        let mut unit_data : Vec<UnitData> = Default::default();

        let mut unit_data_pos : u32 = 0;
        loop {
            if it.is_empty() {
                break;
            }

            let ptr = it.take(..8).unwrap(); 
            let mut mf : MatchFrame = deserialize(&ptr).unwrap();
            mf.unit_data_pos = unit_data_pos;

            let a = mf.num;
            let b = mf.units;
            let c = mf.unit_data_pos;

            for _ in 0..mf.units {
                let ptr = it.take(.. std::mem::size_of::<UnitData>()).unwrap(); 
                let ud : UnitData = deserialize(&ptr).unwrap();
                let a = ud.pos_x;
                let b = ud.pos_y;
                // println!("pos {} {}", a, b);
                unit_data.push( ud);
            }

            unit_data_pos += mf.units;
            //println!("frame f={} u={} c={}", a, b, c);
            frames.push(mf);
        }
        assert_eq!(it.len(), 0);
        (frames, unit_data)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn read_demo() -> Vec<u8> {
    let path = "/home/tommy/dev/bar-spring/bld-profile-gcc64/demos/md";
    let data = fs::read(path).unwrap();
    return data;
}


#[cfg(target_arch = "wasm32")]
fn read_demo() -> Vec<u8> {
    use flate2::read::GzDecoder;
    use std::io::Read;
    println!("load demo");
    let mut d = GzDecoder::new(REPLAY);
    let mut data = Vec::<u8>::new();
    let res = d.read_to_end(&mut data);
    assert_eq!(res.is_ok(), true);
    println!("loaded");
    return data;
}

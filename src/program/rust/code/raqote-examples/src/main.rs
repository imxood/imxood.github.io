use raqote::*;
use ttf_parser::GlyphId;

fn main() {
    let gradient = Source::Solid(SolidSource::from_unpremultiplied_argb(255, 255, 255, 0));
    let mut dt = DrawTarget::new(400, 400);

    // 加载字体
    let face =
        ttf_parser::Face::from_slice(include_bytes!("../../fonts/DroidSansFallbackFull.ttf"), 0)
            .map_err(|e| format!("{:?}", &e))
            .unwrap();

    // 解析字形
    let glyph_id = face.glyph_index('你').unwrap();
    let mut outline_builder = OutlineBuilder::new();
    let rect = face.outline_glyph(glyph_id, &mut outline_builder).unwrap();
    let path = outline_builder.build();

    dt.fill(&path, &gradient, &DrawOptions::new());

    // 解析字形
    let glyph_id = face.glyph_index('。').unwrap();
    let mut outline_builder = OutlineBuilder::new();
    let rect = face.outline_glyph(glyph_id, &mut outline_builder).unwrap();
    let path = outline_builder
        .build()
        .transform(&Transform::identity().post_translate(Vector::new(200.0, 0.0)));

    dt.fill(&path, &gradient, &DrawOptions::new());

    dt.write_png("example.png").unwrap();
}

struct OutlineBuilder {
    builder: PathBuilder,
}

impl OutlineBuilder {
    pub fn new() -> Self {
        Self {
            builder: PathBuilder::new(),
        }
    }
    pub fn build(self) -> Path {
        self.builder.finish()
    }
}

impl ttf_parser::OutlineBuilder for OutlineBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        println!("M {} {} ", x, y);
        self.builder.move_to(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        println!("L {} {} ", x, y);
        self.builder.line_to(x, y);
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        println!("Q {} {} {} {} ", x1, y1, x, y);
        self.builder.quad_to(x1, y1, x, y);
    }

    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        println!("C {} {} {} {} {} {} ", x1, y1, x2, y2, x, y);
        self.builder.cubic_to(x1, y1, x2, y2, x, y);
    }

    fn close(&mut self) {
        println!("Z ");
        self.builder.close();
    }
}

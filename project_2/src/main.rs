
use renderer::Drawable;
use scene::Scene;
use gui::manager::Manager as GUI;

fn main() {
    println!("Hello, world!");
}


struct MainScene {
    gui: GUI
}

impl Scene for MainScene {
    fn handle_event(e: scene::Event) -> Result<(), scene::Error> {
        todo!()
    }
}

impl Drawable for MainScene {
    fn draw(&self, renderer: &mut dyn renderer::Renderer) {
        todo!()
    }
}


use string_fixed::StringFixed;

pub trait TraitSceneData {
    fn initialize(&mut self);
    fn activate(&mut self);
    fn deactivate(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn audio(&mut self);
}

pub struct Scene<D: TraitSceneData> {
    name: StringFixed,
    pub data: D,
}

impl<D: TraitSceneData> Scene<D> {
    pub fn initialize(&mut self, name: &str) {
        self.name.from_string(name);
        self.data.initialize();
    }
    pub fn activate(&mut self) {
        self.data.activate();
    }
    pub fn deactivate(&mut self) {
        self.data.deactivate();
    }
    pub fn update(&mut self) {
        self.data.update();
    }
    pub fn render(&mut self) {
        self.data.render();
    }
    pub fn audio(&mut self) {
        self.data.audio();
    }
}

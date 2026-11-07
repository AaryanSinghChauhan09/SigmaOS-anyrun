//! anyrun model — GTK4/Quickshell-style Wayland launcher.

#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub plugins: Vec<Plugin>,
    pub active_plugin: String,
}

impl Default for Model {
    fn default() -> Self { Self {
        plugins: vec![
            Plugin { name: "applications".into(), enabled: true },
            Plugin { name: "window_switcher".into(), enabled: true },
            Plugin { name: "clipboard".into(), enabled: true },
        ],
        active_plugin: "applications".into(),
    } }
}

impl Model {
    pub fn new() -> Self { Self::default() }
    pub fn list(&self) -> &[Plugin] { &self.plugins }
    pub fn switch(&mut self, name: &str) -> bool {
        if self.plugins.iter().any(|p| p.name == name) { self.active_plugin = name.into(); true } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn switch() {
        let mut a = Model::new();
        assert_eq!(a.list().len(), 3);
        assert!(a.switch("clipboard"));
        assert_eq!(a.active_plugin, "clipboard");
    }
}

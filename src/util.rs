//! spice model — SPICE protocol for VM display.

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub port: u16,
    pub connected: bool,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub channels: Vec<Channel>,
}

impl Default for Model {
    fn default() -> Self { Self { channels: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_channel(&mut self, name: &str, port: u16) {
        self.channels.push(Channel { name: name.into(), port, connected: false });
    }

    pub fn connect(&mut self, name: &str) -> bool {
        if let Some(c) = self.channels.iter_mut().find(|c| c.name == name) {
            c.connected = true;
            true
        } else { false }
    }

    pub fn disconnect(&mut self, name: &str) -> bool {
        if let Some(c) = self.channels.iter_mut().find(|c| c.name == name) {
            c.connected = false;
            true
        } else { false }
    }

    pub fn list(&self) -> &[Channel] { &self.channels }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channel_connect() {
        let mut s = Model::new();
        s.add_channel("main", 5900);
        assert!(s.connect("main"));
        assert!(s.disconnect("main"));
    }
}

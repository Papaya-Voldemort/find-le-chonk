use std::collections::HashSet;
use crate::story::Scene;
use std::io::Error;
use cliclack::select;

pub struct Game {
    pub birds_found: u8,
    pub visited: HashSet<Scene>,
}

impl Game {
    pub(crate) fn new() -> Self {
        Game { birds_found: 0, visited: HashSet::new() }
    }

    pub(crate) fn visited(&mut self, place: Scene) -> Result<(), Error> {
        self.visited.insert(place);
        Ok(())
    }

    fn remaining(&self) -> Vec<Scene> {
        Scene::EXPLORABLE
            .into_iter()
            .filter(|s| !self.visited.contains(s))
            .collect()
    }

    pub(crate) fn where_next(&mut self) -> Result<Scene, Error> {
        let options = self.remaining();
        if options.is_empty() {
            return Ok(Scene::Ending); // all searched → ending unlocks itself
        }

        let mut prompt = select("Where do you look for Le Chonk?");
        for scene in options {
            prompt = prompt.item(scene, scene.label(), scene.hint());
        }
        Ok(prompt.interact()?)
    }

    pub(crate) fn found_bird(&mut self) -> Result<(), Error>{
        self.birds_found += 1;

        Ok(())
    }

    pub(crate) fn main_story(&mut self) -> Result<(), Error> {
        let start = self.opening()?;
        match start {
            Scene::Backyard => self.backyard()?,
            Scene::Laundry => self.laundry()?,
            Scene::Garage => self.garage()?,
            Scene::Window => self.window()?,
            _ => ()
        }

        // add ascii Le Chonk later

        Ok(())
    }
}

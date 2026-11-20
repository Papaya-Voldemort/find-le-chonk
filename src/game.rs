use crate::story::Scene;
use cliclack::select;
use rand::rng;
use rand::seq::IndexedRandom;
use std::collections::HashSet;
use std::io::Error;

pub struct Game {
    pub birds_found: u8,
    pub visited: HashSet<Scene>,
    unlocked: Vec<Scene>,
    pub cat_location: Scene,
}

impl Game {
    pub(crate) fn new() -> Self {
        let scene = [
            Scene::Backyard,
            Scene::Laundry,
            Scene::Garage,
            Scene::Porch,
            Scene::Fridge,
            Scene::Tree,
        ]
        .choose(&mut rng())
        .copied()
        .unwrap();

        Game {
            birds_found: 0,
            visited: HashSet::new(),
            unlocked: vec![],
            cat_location: scene,
        }
    }

    pub(crate) fn visited(&mut self, place: Scene) -> Result<(), Error> {
        self.visited.insert(place);
        Ok(())
    }

    pub(crate) fn unlock(&mut self, scenes: impl IntoIterator<Item = Scene>) -> bool {
        let before = self.unlocked.len();
        for scene in scenes {
            if !self.unlocked.contains(&scene) {
                self.unlocked.push(scene);
            }
        }
        self.unlocked.len() > before
    }

    fn remaining(&self) -> Vec<Scene> {
        self.unlocked
            .iter()
            .copied()
            .filter(|s| !self.visited.contains(s))
            .collect()
    }

    pub(crate) fn where_next(&mut self) -> Result<Scene, Error> {
        let options = self.remaining();
        if options.is_empty() {
            return Ok(Scene::Ending);
        }

        let mut prompt = select("Where do you look for Le Chonk?");
        for scene in options {
            prompt = prompt.item(scene, scene.label(), scene.hint());
        }
        Ok(prompt.interact()?)
    }

    pub(crate) fn found_bird(&mut self) -> Result<(), Error> {
        self.birds_found += 1;

        Ok(())
    }

    pub(crate) fn main_story(&mut self) -> Result<(), Error> {
        let mut scene = self.opening()?;

        loop {
            scene = match scene {
                Scene::Opening => self.opening()?,
                Scene::Backyard => self.backyard()?,
                Scene::Laundry => self.laundry()?,
                Scene::Garage => self.garage()?,
                Scene::Window => self.window()?,
                Scene::Fridge => self.fridge()?,
                Scene::Tree => self.tree()?,
                Scene::Porch => self.porch()?,
                Scene::Ending => break,
            };
        }

        self.ending()?;

        // add ascii Le Chonk later

        Ok(())
    }
}

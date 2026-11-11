use crate::game::Game;
use crate::story::Scene::{Backyard, Garage, Laundry, Opening, Window};
use crate::utils::{blank_rail, cat, dot_with_wait, say, sleep};
use cliclack::intro;
use std::io::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scene {
    Opening,
    Backyard,
    Laundry,
    Garage,
    Window,
    Ending,
}

impl Scene {
    pub(crate) const EXPLORABLE: [Scene; 4] = [Backyard, Laundry, Garage, Window];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Backyard => "Check the backyard",
            Laundry => "Ransack the laundry room",
            Garage => "Peek in the garage",
            Window => "Look out the window",
            _ => unreachable!(),
        }
    }

    pub(crate) fn hint(self) -> String {
        match self {
            Backyard => format!(
                "Maybe {} is in the garden? Larry did say he planted catnip...",
                cat()
            ),
            Laundry => format!("{} has been known to like the dryer...", cat()),
            Garage => format!("Garages have boxes, Cats like boxes, {} is a cat...", cat()),
            Window => "What is that thing on the window...".to_string(),
            _ => unreachable!(),
        }
    }
}

impl Game {
    pub fn opening(&mut self) -> Result<Scene, Error> {
        self.visited(Opening)?;
        intro("A MORNING WITH LE CHONK")?;

        say(format!("Hello, {}!", cat()));
        sleep(750);

        dot_with_wait(3, 500);

        println!("Wait! you're not {}!", cat());
        blank_rail();
        sleep(800);

        say(format!(
            "You look down at what you thought was a cat, instead you see a fluffy grey throw pillow whose large silhouette matched the large feline one to one.\
        Your uncle Larry had trusted you to watch the company cat, appropriately named {} (fat cat), while they were away on the annual business retreat.\
        Usually {} just sat on this rug most of the day, occasionally going and eating his \"snack\". However this morning, the last morning until Larry got back, he clearly was not going to be content.",
            cat(),
            cat()
        ));
        blank_rail();

        Ok(self.where_next()?)
    }

    pub(crate) fn backyard(&mut self) -> Result<(), Error> {
        self.visited(Backyard)?;

        Ok(())
    }

    pub(crate) fn laundry(&mut self) -> Result<(), Error> {
        self.visited(Laundry)?;

        Ok(())
    }

    pub(crate) fn garage(&mut self) -> Result<(), Error> {
        self.visited(Garage)?;

        Ok(())
    }

    pub(crate) fn window(&mut self) -> Result<(), Error> {
        self.visited(Window)?;

        Ok(())
    }
}

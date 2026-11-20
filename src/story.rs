use crate::game::Game;
use crate::story::Scene::{
    Backyard, Ending, Fridge, Garage, Laundry, Opening, Porch, Tree, Window,
};
use crate::utils::{bird, blank_rail, cat, dot_with_wait, say, sleep};
use cliclack::{confirm, intro};
use std::io::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Scene {
    Opening,
    Backyard,
    Laundry,
    Garage,
    Window,
    Porch,
    Fridge,
    Tree,
    Ending,
}

impl Scene {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Opening => "",
            Backyard => "Check the backyard",
            Laundry => "Ransack the laundry room",
            Garage => "Peek in the garage",
            Window => "Look out the window",
            Fridge => "Open the fridge",
            Tree => "Investigate the tree",
            Porch => "Check the front porch",
            Ending => "",
        }
    }

    pub(crate) fn hint(self) -> String {
        match self {
            Opening => String::new(),
            Backyard => format!(
                "Maybe {} is in the garden? Larry did say he planted catnip...",
                cat()
            ),
            Laundry => format!("{} has been known to like the dryer...", cat()),
            Garage => format!("Garages have boxes. Cats like boxes. {} is a cat...", cat()),
            Window => "What is that thing on the window...".to_string(),
            Fridge => format!(
                "The fridge is purring. Fridges don't purr. {} purrs...",
                cat()
            ),
            Tree => format!(
                "The neighborhood birds have unionized and refuse to talk. This feels {} related...",
                cat()
            ),
            Porch => format!("{} is known to sunbathe on the porch", cat()),
            Ending => String::new(),
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

        println!("Wait! You're not {}!", cat());
        blank_rail();
        sleep(800);

        say(format!(
            "You look down at what you thought was a cat, instead you see a fluffy grey throw pillow whose large silhouette matched the large feline one to one. \
        Your uncle Larry had trusted you to watch the company cat, appropriately named {} (fat cat), while they were away on the annual business retreat. \
        Usually {} just sat on this rug most of the day, occasionally going and eating his \"snack\". However this morning, the last morning until Larry got back, he clearly was not going to be content.",
            cat(),
            cat()
        ));
        blank_rail();

        self.unlock([Backyard, Laundry, Garage, Window]);

        Ok(self.where_next()?)
    }

    pub(crate) fn backyard(&mut self) -> Result<Scene, Error> {
        self.visited(Backyard)?;

        say("You slide open the glass door and step out into the dewy morning.");
        sleep(500);

        say(format!(
            "Remembering what Uncle Larry said, you head straight for the garden bed where he planted the fresh catnip. If {} is anywhere out here, he'll be in that patch.",
            cat()
        ));
        blank_rail();
        sleep(800);

        if self.cat_location == Backyard {
            say("A big orange tail poked out of the shrubs and waved at you brightly.");
            sleep(500);

            blank_rail();
            say(format!(
                "There you are {}! I have been looking all over for you!",
                cat()
            ));

            return Ok(Ending);
        } else {
            say("Wait... the shrubs right next to the catnip are rustling violently!");
            dot_with_wait(3, 500);

            println!();
            blank_rail();

            let choice = confirm("Do you want to look in the bush?").interact()?;

            if choice {
                say(format!(
                    "You crouch down and part the thick leaves, fully expecting to see the chubby body of {}...",
                    cat()
                ));
                sleep(750);

                dot_with_wait(3, 500);
                println!();
                blank_rail();

                say(format!(
                    "Instead, a startled {} hops out of the foliage, aggressively pecking at a stray catnip leaf before looking at you!",
                    bird()
                ));

                blank_rail();
                say(format!("You found a {}! {} would be proud!", bird(), cat()));

                self.found_bird()?;
                sleep(600);

                say(format!(
                    "Keep collecting {}s, you're at {}/?.",
                    bird(),
                    self.birds_found
                ));

                blank_rail();
                sleep(500);
            } else {
                say("It's probably nothing, plus that bush scares me a bit.");

                blank_rail();
                sleep(500);
            }

            say(format!(
                "No sign of {} here, unless he somehow learned how to fly. Though looking closely at the dirt, you do spot a trail of suspiciously deep, heavy paw prints leading back toward the house...",
                cat()
            ));

            blank_rail();
            sleep(400);
        }

        self.unlock([Tree, Porch]);

        Ok(self.where_next()?)
    }

    pub(crate) fn laundry(&mut self) -> Result<Scene, Error> {
        self.visited(Laundry)?;

        say(
            "You push open the laundry room door and flick on the switch. The bulb hums to life revealing the bland room.",
        );
        sleep(500);

        say(format!(
            "There in the corner sits the dryer, warm and gently tumbling. Uncle Larry always said {} likes it in here because it's \"warm like a fresh loaf of bread.\"",
            cat()
        ));
        blank_rail();
        sleep(800);

        if self.cat_location == Laundry {
            say(
                "You lean into the dryer's little round glass door, and two big eyes lean back out of the dark.",
            );
            sleep(500);

            blank_rail();
            say(format!("There he is! {} is", cat()));
            dot_with_wait(3, 500);
            say(" IN THE RUNNING DRYER!!");

            return Ok(Ending);
        } else {
            say("You pull the dryer door open. Empty. Just a single sock, and some dryer balls.");
            dot_with_wait(3, 500);

            println!();
            blank_rail();

            say(
                "But wait... the laundry basket on top of the dryer is wobbling. Dryers don't wobble. Baskets don't wobble.",
            );
            dot_with_wait(3, 400);

            blank_rail();

            let choice = confirm("Do you want to tip over the basket?").interact()?;

            if choice {
                say("You grab the basket and slowly tip it toward the floor...");
                sleep(750);

                dot_with_wait(3, 500);
                println!();
                blank_rail();

                say(format!(
                    "A {} bursts out in a puff of lint and fabric softener sheets, wearing one of Larry's shirts like a cape!",
                    bird()
                ));

                blank_rail();
                say(format!("You found a {}! {} would be proud!", bird(), cat()));

                self.found_bird()?;
                sleep(600);

                say(format!(
                    "Keep collecting {}s, you're at {}/?.",
                    bird(),
                    self.birds_found
                ));

                blank_rail();
                sleep(500);
            } else {
                say(
                    "Whatever's in there can keep the basket. You've seen enough horror movies to know how this goes.",
                );
                blank_rail();
                sleep(500);
            }

            say(format!(
                "No sign of {} in here. On your way out, your stomach growls loud enough to startle you. You skipped breakfast, and the kitchen is just down the hall...",
                cat()
            ));

            blank_rail();
            sleep(400);
        }

        self.unlock([Fridge]);

        Ok(self.where_next()?)
    }

    pub(crate) fn garage(&mut self) -> Result<Scene, Error> {
        self.visited(Garage)?;

        say("You walk into the two-car garage. One side is empty, the other is full of boxes.");
        sleep(500);

        say(format!("{} could be in any of those boxes...", cat()));
        sleep(500);

        if self.cat_location == Garage {
            say(format!(
                "Suddenly {}'s head pokes out from a stack of 8 boxes.",
                cat()
            ));
            sleep(500);

            return Ok(Ending);
        }

        say("Oh?");
        dot_with_wait(3, 500);

        say("One of the boxes are shaking...");
        sleep(300);

        let choice = confirm("Do you want to look in the box?").interact()?;

        if choice {
            say("You walk over and look in the box.");
            sleep(500);

            say(format!("Oh! Its a {}!", bird()));
            sleep(500);

            blank_rail();
            say(format!("You found a {}! {} would be proud!", bird(), cat()));

            self.found_bird()?;
            sleep(600);

            say(format!(
                "Keep collecting {}s, you're at {}/?.",
                bird(),
                self.birds_found
            ));

            blank_rail();
            sleep(500);
        }

        say(format!(
            "Well no {} must not be here. Time to head back in.",
            cat()
        ));

        Ok(self.where_next()?)
    }

    pub(crate) fn window(&mut self) -> Result<Scene, Error> {
        self.visited(Window)?;

        if [Backyard, Garage, Laundry]
            .iter()
            .any(|place| self.visited.contains(place))
        {
            say("You don't see anything");

            dot_with_wait(3, 500);

            println!();
            blank_rail();

            say("Maybe you should have come by earlier.");

            blank_rail();

            sleep(200);

            Ok(self.where_next()?)
        } else {
            say("You head over to the window, something seems off...");

            sleep(500);

            say(
                "The normally pristine beige curtains have a grey shadow behind them, and it's moving!",
            );

            dot_with_wait(3, 500);

            sleep(500);

            println!();
            blank_rail();

            say(format!("OH! It's a {}!", bird()));

            self.found_bird()?;

            sleep(600);

            say(format!(
                "You found {}/? {}s, {} would be proud! Keep finding birds to unlock a secret!",
                self.birds_found,
                bird(),
                cat()
            ));

            blank_rail();

            sleep(200);

            Ok(self.where_next()?)
        }
    }

    pub(crate) fn fridge(&mut self) -> Result<Scene, Error> {
        self.visited(Fridge)?;

        say("prrrrr...prrrrr");
        dot_with_wait(3, 500);

        say("Huh? Is that coming from the fridge!");

        sleep(500);

        say("You walk over to the fridge and crack open the door.");
        blank_rail();
        sleep(800);

        if self.cat_location == Fridge {
            say(format!(
                "A wave of cold air hits you, and there on the middle shelf, wedged between the leftover lasagna and a jar of pickles, sits {}.",
                cat()
            ));
            sleep(500);

            blank_rail();
            say(format!("{}! How did you even fit in there?!", cat()));
            sleep(500);

            say("He blinks slowly at you, a noodle hanging from his whisker.");

            return Ok(Ending);
        } else {
            say(
                "Cold air, the little fridge light, and a lot of Larry's leftovers. No cat. But the purring is still going, and it's definitely louder now.",
            );
            dot_with_wait(3, 500);

            println!();
            blank_rail();

            say("It's coming from the crisper drawer.");
            sleep(500);

            blank_rail();

            let choice = confirm("Do you want to open the crisper drawer?").interact()?;

            if choice {
                say("You grab the handle and slide the drawer open...");
                sleep(750);

                dot_with_wait(3, 500);
                println!();
                blank_rail();

                say(format!(
                    "A {} is nestled in the lettuce, chirping happily with a cherry tomato stuffed in its beak. Turns out, birds can purr. Or at least, this one can.",
                    bird()
                ));

                blank_rail();
                say(format!("You found a {}! {} would be proud!", bird(), cat()));

                self.found_bird()?;
                sleep(600);

                say(format!(
                    "Keep collecting {}s, you're at {}/?.",
                    bird(),
                    self.birds_found
                ));

                blank_rail();
                sleep(500);
            } else {
                say("You close the fridge and decide not to ask questions. Some drawers are better left shut.");

                blank_rail();
                sleep(500);
            }

            say(format!(
                "No sign of {} in here. You do grab a quick bite of lasagna on the way out. For the search, of course.",
                cat()
            ));

            blank_rail();
            sleep(400);
        }

        Ok(self.where_next()?)
    }

    pub(crate) fn tree(&mut self) -> Result<Scene, Error> {
        self.visited(Tree)?;

        say("You walk out front to check on the big oak tree.");
        sleep(500);

        say("The yard is dead silent. No chirping, no singing, not even a squawk.");
        blank_rail();
        sleep(800);

        if self.cat_location == Tree {
            say("You look up. Way up. About halfway up the trunk, a big orange blob is clinging to a branch that is bending much farther than a branch should.");
            sleep(500);

            blank_rail();
            say(format!("{}!! How did you get up there?!", cat()));
            sleep(500);

            say("He meows down at you, like this is somehow your fault.");

            return Ok(Ending);
        } else {
            say(
                "Dozens of birds are perched on the branches, all staring right at you. Not a single one of them is making a sound.",
            );
            dot_with_wait(3, 500);

            println!();
            blank_rail();

            say(
                "One of them is higher up than the rest, wearing a tiny bottle cap like a hard hat, and it keeps glancing at a hole in the trunk.",
            );
            sleep(500);

            blank_rail();

            let choice = confirm("Do you want to climb up and check the hole?").interact()?;

            if choice {
                say("You grab the lowest branch and haul yourself up, trying to ignore the hundred tiny eyes watching you...");
                sleep(750);

                dot_with_wait(3, 500);
                println!();
                blank_rail();

                say(format!(
                    "Inside the hole is a {} holding a scrap of cardboard that reads \"ON STRIKE.\" It hands you a feather and a pamphlet, then goes back to ignoring you.",
                    bird()
                ));

                blank_rail();
                say(format!("You found a {}! {} would be proud!", bird(), cat()));

                self.found_bird()?;
                sleep(600);

                say(format!(
                    "Keep collecting {}s, you're at {}/?.",
                    bird(),
                    self.birds_found
                ));

                blank_rail();
                sleep(500);
            } else {
                say("You decide not to cross the picket line. Climbing trees in front of an angry union seems like a bad idea.");

                blank_rail();
                sleep(500);
            }

            say(format!(
                "No sign of {} up here, and the birds aren't talking. You climb back down, feeling very judged.",
                cat()
            ));

            blank_rail();
            sleep(400);
        }

        Ok(self.where_next()?)
    }

    pub(crate) fn porch(&mut self) -> Result<Scene, Error> {
        self.visited(Porch)?;

        say("You open the front door and step out onto the porch. The morning sun is already warming the old wooden boards.");
        sleep(500);

        say(format!(
            "If there's one spot {} loves more than that rug, it's right here in the sun.",
            cat()
        ));
        blank_rail();
        sleep(800);

        if self.cat_location == Porch {
            say("There, in the exact middle of the sunniest patch of the porch, lies a giant orange loaf of cat, belly up and snoring.");
            sleep(500);

            blank_rail();
            say(format!(
                "{}! I've been looking everywhere for you!",
                cat()
            ));
            sleep(500);

            say("He doesn't even open his eyes. He just flops over to the other side.");

            return Ok(Ending);
        } else {
            say(
                "The sunny spot is empty, but the boards are still warm, with a big, cat-shaped dent in the welcome mat.",
            );
            dot_with_wait(3, 500);

            println!();
            blank_rail();

            say("Then you hear it: the mailbox is rattling, and the little red flag is wiggling up and down.");
            sleep(500);

            blank_rail();

            let choice = confirm("Do you want to open the mailbox?").interact()?;

            if choice {
                say("You walk down to the end of the path and slowly lift the mailbox lid...");
                sleep(750);

                dot_with_wait(3, 500);
                println!();
                blank_rail();

                say(format!(
                    "A {} pops out, scattering a pile of junk mail everywhere, and lands on your head like it owns the place!",
                    bird()
                ));

                blank_rail();
                say(format!("You found a {}! {} would be proud!", bird(), cat()));

                self.found_bird()?;
                sleep(600);

                say(format!(
                    "Keep collecting {}s, you're at {}/?.",
                    bird(),
                    self.birds_found
                ));

                blank_rail();
                sleep(500);
            } else {
                say("Probably just junk mail. Besides, the mailbox looks like it's in a mood.");

                blank_rail();
                sleep(500);
            }

            say(format!(
                "No sign of {} out here, just a warm spot where he used to be. He can't have gone far...",
                cat()
            ));

            blank_rail();
            sleep(400);
        }

        Ok(self.where_next()?)
    }
    pub(crate) fn ending(&mut self) -> Result<Scene, Error> {
        self.visited(Ending)?;

        say(format!(
            "Larry opens the door to see {} curled up on the couch next to you.",
            cat()
        ));
        sleep(500);

        say("\"Thanks so much for watching him! I hope he was not too much trouble!\"");
        sleep(500);

        say("You laugh \"Ofc, anytime\"");

        // Add ending logo

        Ok(self.where_next()?)
    }
}

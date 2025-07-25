use super::UiApp;
use cookie_clicker_tui_core::NewsEntry as E;
use rand::prelude::*;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

// TODO: Marquee animation for long messages.
pub fn news(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    app.news.render(area, buf, |entry| {
        Paragraph::new(line_text(entry, app.bakery.name()))
            .centered()
            .block(Block::bordered())
    });
}

fn line_text(entry: E, bakery_name: Option<&str>) -> Line<'static> {
    let mut rng = rand::rng();

    macro_rules! raw {
        ($s:expr) => {
            Line::raw($s)
        };
    }

    macro_rules! sps {
        ($($expr:expr),*$(,)?) => {
            Line::from(vec![$($expr),*])
        };
    }

    macro_rules! sp {
        ($raw:expr) => {
            Span::raw($raw)
        };
        ($raw:expr, $mod:ident) => {
            Span::styled($raw, Modifier::$mod)
        };
    }

    macro_rules! gquote {
        ($quote:expr) => {
            sps!(sp!(concat!("\"", $quote, "\""), ITALIC), sp!(" -grandma"))
        };
    }

    macro_rules! choose {
        ($($expr:expr),*) => {
            [$($expr),*].choose(&mut rng).copied().unwrap()
        };
    }

    macro_rules! many {
        ($($num:literal => $option:expr),*,) => {{
            match [$($num),*].choose(&mut rng).copied().unwrap() {
                $($num => $option,)*
                _ => unreachable!(),
            }
        }};
    }

    match entry {
        E::CookiesAllTime_Below_5 => {
            raw!("You feel like making cookies. But nobody wants to eat your cookies.")
        }
        E::CookiesAllTime_5_To_50 => {
            raw!("Your first batch goes in the trash. The neighborhood raccoon barely touches it.")
        }
        E::CookiesAllTime_50_To_100 => raw!("Your family accepts to try some of your cookies."),
        E::CookiesAllTime_100_To_500 => raw!("Your cookies are popular in the neighborhood."),
        E::CookiesAllTime_500_To_1K => raw!("People are starting to talk about your cookies."),
        E::CookiesAllTime_1K_To_5K => raw!("Your cookies are talked about for miles around."),
        E::CookiesAllTime_5K_To_10K => raw!("Your cookies are renowned in the whole town!"),
        E::CookiesAllTime_10K_To_50K => raw!("Your cookies bring all the boys to the yard."),
        E::CookiesAllTime_50K_To_100K => raw!("Your cookies now have their own website!"),
        E::CookiesAllTime_100K_To_500K => raw!("Your cookies are worth a lot of money."),
        E::CookiesAllTime_500K_To_1M => raw!("Your cookies sell very well in distant countries."),
        E::CookiesAllTime_1M_To_5M => {
            raw!("People come from very far away to get a taste of your cookies.")
        }
        E::CookiesAllTime_5M_To_10M => {
            raw!("Kings and queens from all over the world are enjoying your cookies.")
        }
        E::CookiesAllTime_10M_To_50M => raw!("There are now museums dedicated to your cookies."),
        E::CookiesAllTime_50M_To_100M => {
            raw!("A national day has been created in honor of your cookies.")
        }
        E::CookiesAllTime_100M_To_500M => {
            raw!("Your cookies have been named a part of the world wonders.")
        }
        E::CookiesAllTime_500M_To_1B => {
            raw!("History books now include a whole chapter about your cookies.")
        }
        E::CookiesAllTime_1B_To_5B => {
            raw!("Your cookies have been placed under government surveillance.")
        }
        E::CookiesAllTime_5B_To_10B => raw!("The whole planet is enjoying your cookies!"),
        E::CookiesAllTime_10B_To_50B => {
            raw!("Strange creatures from neighboring planets wish to try your cookies.")
        }
        E::CookiesAllTime_50B_To_100B => {
            raw!("Elder gods from the whole cosmos have awoken to taste your cookies.")
        }
        E::CookiesAllTime_100B_To_500B => raw!(
            "Beings from other dimensions lapse into existence just to get a taste of your cookies. "
        ),
        E::CookiesAllTime_500B_To_1T => raw!("Your cookies have achieved sentience."),
        E::CookiesAllTime_1T_To_5T => {
            raw!("The universe has now turned into cookie dough, to the molecular level.")
        }
        E::CookiesAllTime_5T_To_10T => {
            raw!("Your cookies are rewriting the fundamental laws of the universe.")
        }
        E::CookiesAllTime_10T_To_100T => raw!("it's time to stop playing"),
        E::CookiesAllTime_Above_100T => {
            raw!("A local news station runs a 10-minute segment about your cookies. Success!")
        }
        /* -------------------------------------------------------------------------- */
        /*                               Building Based                               */
        /* -------------------------------------------------------------------------- */
        E::Building_Grandma_1 => many! {
            0 => gquote!("Moist cookies."),
            1 => gquote!("We're nice grandmas."),
            2 => gquote!("Indentured servitude."),
            3 => gquote!("Come give grandma a kiss."),
            4 => gquote!("Why don't you visit more often?"),
            5 => gquote!("Call me..."),
        },
        E::Building_Grandma_50 => many! {
            0 => gquote!("Absolutely disgusting."),
            1 => gquote!("You make me sick."),
            2 => gquote!("You disgust me."),
            3 => gquote!("We rise."),
            4 => gquote!("It begins."),
            5 => gquote!("It'll all be over soon."),
            6 => gquote!("You could have stopped it."),
        },
        E::Building_Farm_1 => many! {
            0 => raw!("News: cookie farms suspected of employing undeclared elderly workforce!"),
            1 => raw!("News: cookie farms release harmful chocolate in our rivers, says scientist!"),
            2 => raw!("News: genetically-modified chocolate controversy strikes cookie farmers!"),
            3 => raw!("News: free-range farm cookies popular with today's hip youth, says specialist."),
            4 => raw!("News: farm cookies deemed unfit for vegans, says nutritionist."),
        },
        E::Building_Mine_1 => many! {
            0 => raw!(format!("News: {} miners dead in chocolate mine catastrophe!", rng.random_range(2..=1001))),
            1 => raw!(format!("News: {} miners trapped in collapsed chocolate mine!", rng.random_range(2..=1001))),
            2 => raw!("News: chocolate mines found to cause earthquakes and sink holes!"),
            3 => raw!("News: chocolate mine goes awry, floods village in chocolate!"),
            4 => raw!("News: depths of chocolate mines found to house \"peculiar, chocolaty beings\"!"),
            5 => raw!("News: is our planet getting lighter? Experts examine the results of intensive chocolate mining."),
        },
        E::Building_Factory_1 => many! {
            0 => raw!("News: cookie factories linked to global warming!"),
            1 => raw!("News: cookie factories involved in chocolate weather controversy!"),
            2 => raw!("News: cookie factories on strike, robotic minions employed to replace workforce!"),
            3 => raw!("News: cookie factories on strike - workers demand to stop being paid in cookies!"),
            4 => raw!("News: factory-made cookies linked to obesity, says study."),
        },
        E::Building_Bank_1 => many! {
            0 => raw!("News: cookie loans on the rise as people can no longer afford them with regular money."),
            1 => raw!("News: cookies slowly creeping up their way as a competitor to traditional currency!"),
            2 => raw!("News: most bakeries now fitted with ATMs to allow for easy cookie withdrawals and deposits."),
            3 => raw!("News: cookie economy now strong enough to allow for massive vaults doubling as swimming pools!"),
            4 => raw!("News: \"Tomorrow's wealthiest people will be calculated by their worth in cookies\", predict specialists."),
        },
        E::Building_Temple_1 => many! {
            0 => raw!(format!(
                "News: explorers bring back ancient artifact from abandoned temple; archeologists marvel at the centuries-old {} {}!",
                choose!["magic", "carved", "engraved", "sculpted", "royal", "imperial", "mummified", "ritual", "golden", "silver", "stone", "cursed", "plastic", "bone", "blood", "holy", "sacred", "sacrificial", "electronic", "singing", "tapdancing"],
                choose!["spoon", "fork", "pizza", "washing machine", "calculator", "hat", "piano", "napkin", "skeleton", "gown", "dagger", "sword", "shield", "skull", "emerald", "bathtub", "mask", "rollerskates", "litterbox", "bait box", "cube", "sphere", "fungus"],
            )),
            1 => raw!("News: recently-discovered chocolate temples now sparking new cookie-related cult; thousands pray to Baker in the sky!"),
            2 => raw!(format!(
                "News: just how extensive is the cookie pantheon? Theologians speculate about possible {} of {}.",
                choose!["god", "goddess"],
                choose!["newts", "penguins", "scorpions", "axolotls", "puffins", "porpoises", "blowfish", "horses", "crayfish", "slugs", "humpback whales", "nurse sharks", "giant squids", "polar bears", "fruit bats", "frogs", "sea squirts", "velvet worms", "mole rats", "paramecia", "nematodes", "tardigrades", "giraffes", "monkfish", "wolfmen", "goblins", "hippies", "kazoos", "web design", "web browsers", "kittens", "atheism", "handbrakes", "hats", "aglets", "elevator music", "idle games", "the letter \"P\"", "memes", "hamburgers", "bad puns", "kerning", "stand-up comedy", "failed burglary attempts", "clickbait", "one weird tricks"],
            )),
            3 => raw!("News: theists of the world discover new cookie religion - \"Oh boy, guess we were wrong all along!\""),
            4 => raw!("News: cookie heaven allegedly \"sports elevator instead of stairway\"; cookie hell \"paved with flagstone, as good intentions make for poor building material\"."),
        },
        E::Building_WizardTower_1 => many! {
            0 => raw!(format!(
                "News: all {} turned into {} in freak magic catastrophe!",
                choose!["newts", "penguins", "scorpions", "axolotls", "puffins", "porpoises", "blowfish", "horses", "crayfish", "slugs", "humpback whales", "nurse sharks", "giant squids", "polar bears", "fruit bats", "frogs", "sea squirts", "velvet worms", "mole rats", "paramecia", "nematodes", "tardigrades", "giraffes", "monkfish", "wolfmen", "goblins", "hippies", "public restrooms", "clouds", "politicians", "moustaches", "hats", "shoes", "pants", "clowns", "encyclopedias", "websites", "potted plants", "lemons", "household items", "bodily fluids", "cutlery", "national landmarks", "yogurt", "rap music", "underwear"],
                // same as ^
                choose!["newts", "penguins", "scorpions", "axolotls", "puffins", "porpoises", "blowfish", "horses", "crayfish", "slugs", "humpback whales", "nurse sharks", "giant squids", "polar bears", "fruit bats", "frogs", "sea squirts", "velvet worms", "mole rats", "paramecia", "nematodes", "tardigrades", "giraffes", "monkfish", "wolfmen", "goblins", "hippies", "public restrooms", "clouds", "politicians", "moustaches", "hats", "shoes", "pants", "clowns", "encyclopedias", "websites", "potted plants", "lemons", "household items", "bodily fluids", "cutlery", "national landmarks", "yogurt", "rap music", "underwear"],
            )),
            1 => raw!(format!(
                "News: heavy dissent rages between the schools of {} magic and {} magic!",
                choose!["water", "fire", "earth", "air", "lightning", "acid", "song", "battle", "peace", "pencil", "internet", "space", "time", "brain", "nature", "techno", "plant", "bug", "ice", "poison", "crab", "kitten", "dolphin", "bird", "punch", "fart"],
                // same as ^
                choose!["water", "fire", "earth", "air", "lightning", "acid", "song", "battle", "peace", "pencil", "internet", "space", "time", "brain", "nature", "techno", "plant", "bug", "ice", "poison", "crab", "kitten", "dolphin", "bird", "punch", "fart"],
            )),
            2 => raw!("News: get your new charms and curses at the yearly National Spellcrafting Fair! Exclusive prices on runes and spellbooks."),
            3 => raw!("News: cookie wizards deny involvement in shockingly ugly newborn - infant is \"honestly grody-looking, but natural\", say doctors."),
            4 => raw!("News: \"Any sufficiently crude magic is indistinguishable from technology\", claims renowned technowizard.'"),
        },
        E::Building_Shipment_1 => many! {
            0 => raw!("News: new chocolate planet found, becomes target of cookie-trading spaceships!"),
            1 => raw!("News: massive chocolate planet found with 99.8% certified pure dark chocolate core!"),
            2 => raw!("News: space tourism booming as distant planets attract more bored millionaires!"),
            3 => raw!("News: chocolate-based organisms found on distant planet!"),
            4 => raw!("News: ancient baking artifacts found on distant planet; \"terrifying implications\", experts say."),
        },
        E::Building_AlchemyLab_1 => many! {
            0 => raw!("News: national gold reserves dwindle as more and more of the precious mineral is turned to cookies!"),
            1 => raw!("News: chocolate jewelry found fashionable, gold and diamonds \"just a fad\", says specialist."),
            2 => raw!("News: silver found to also be transmutable into white chocolate!"),
            3 => raw!("News: defective alchemy lab shut down, found to convert cookies to useless gold."),
            4 => raw!("News: alchemy-made cookies shunned by purists!"),
        },
        E::Building_Portal_1 => many! {
            0 => raw!("News: nation worried as more and more unsettling creatures emerge from dimensional portals!"),
            1 => raw!("News: dimensional portals involved in city-engulfing disaster!"),
            2 => raw!("News: tourism to cookieverse popular with bored teenagers! Casualty rate as high as 73%!"),
            3 => raw!("News: cookieverse portals suspected to cause fast aging and obsession with baking, says study."),
            4 => raw!("News: \"do not settle near portals,\" says specialist; \"your children will become strange and corrupted inside.\""),
        },
        E::Building_TimeMachine_1 => many! {
            0 => raw!("News: time machines involved in history-rewriting scandal! Or are they?"),
            1 => raw!("News: time machines used in unlawful time tourism!"),
            2 => raw!("News: cookies brought back from the past \"unfit for human consumption\", says historian."),
            3 => raw!("News: various historical figures inexplicably replaced with talking lumps of dough!"),
            4 => raw!("News: \"I have seen the future,\" says time machine operator, \"and I do not wish to go there again.\""),
        },
        E::Building_AntimatterCondenser_1 => many! {
            0 => raw!("News: first antimatter condenser successfully turned on, doesn't rip apart reality!"),
            1 => raw!("News: \"explain to me again why we need particle accelerators to bake cookies?\" asks misguided local woman."),
            2 => raw!("News: \"unravelling the fabric of reality just makes these cookies so much tastier\", claims scientist."),
            3 => raw!("News: whole town seemingly swallowed by antimatter-induced black hole; more reliable sources affirm town \"never really existed\"!"),
            4 => raw!("News: researchers conclude that what the cookie industry needs, first and foremost, is \"more magnets\"."),
        },
        E::Building_Prism_1 => many! {
            0 => raw!("News: new cookie-producing prisms linked to outbreak of rainbow-related viral videos."),
            1 => raw!("News: scientists warn against systematically turning light into matter - \"One day, we'll end up with all matter and no light!\""),
            2 => raw!("News: cookies now being baked at the literal speed of light thanks to new prismatic contraptions."),
            3 => raw!("News: \"Can't you sense the prism watching us?\", rambles insane local man. \"No idea what he's talking about\", shrugs cookie magnate/government official."),
            4 => raw!("News: world citizens advised \"not to worry\" about frequent atmospheric flashes."),
        },
        E::Building_Chancemaker_1 => many! {
            0 => raw!("News: million-to-one event sees gritty movie reboot turning out better than the original! \"We have no idea how this happened\", say movie execs."),
            1 => raw!("News: strange statistical anomalies continue as weather forecast proves accurate an unprecedented 3 days in a row!"),
            2 => raw!("News: neighboring nation somehow elects president with sensible policies in freak accident of random chance!"),
            3 => raw!("News: all scratching tickets printed as winners, prompting national economy to crash and, against all odds, recover overnight."),
            4 => raw!("News: local casino ruined as all gamblers somehow hit a week-long winning streak! \"We might still be okay\", says owner before being hit by lightning 47 times."),
        },
        E::Building_FractalEngine_1 => many! {
            0 => raw!("News: local man \"done with Cookie Clicker\", finds the constant self-references \"grating and on-the-nose\"."),
            1 => raw!("News: local man sails around the world to find himself - right where he left it."),
            2 => raw!("News: local guru claims \"there's a little bit of ourselves in everyone\", under investigation for alleged cannibalism."),
            3 => raw!("News: news writer finds herself daydreaming about new career. Or at least a raise."),
            4 => raw!("News: polls find idea of cookies made of cookies \"acceptable\" - \"at least we finally know what's in them\", says interviewed citizen."),
        },
        E::Building_RustPlayground_1 => many! {
            0 => raw!("News: strange fad has parents giving their newborns names such as Emma.rs or Liam.rs. At least one Baby.rs reported."),
            1 => raw!("News: coding is hip! More and more teenagers turn to technical fields like programming, ensuring a future robot apocalypse and the doom of all mankind."),
            2 => raw!("News: developers unsure what to call their new javascript libraries as all combinations of any 3 dictionary words have already been taken."),
            3 => raw!("News: nation holds breath as nested ifs about to hatch."),
            4 => raw!("News: clueless copywriter forgets to escape a quote, ends news line prematurely; last words reported to be \"Huh, why isn"),
        },
        E::Building_Idleverse_1 => many! {
            0 => raw!("News: is another you living out their dreams in an alternate universe? Probably, you lazy bum!"),
            1 => raw!("News: public recoils at the notion of a cosmos made of infinite idle games. \"I kinda hoped there'd be more to it\", says distraught citizen."),
            2 => raw!("News: with an infinity of parallel universes, people turn to reassuring alternate dimensions, which only number \"in the high 50s\"."),
            3 => raw!("News: \"I find solace in the knowledge that at least some of my alternate selves are probably doing fine out there\", says citizen's last remaining exemplar in the multiverse."),
            4 => raw!("News: comic book writers point to actual multiverse in defense of dubious plot points. \"See? I told you it wasn't 'hackneyed and contrived'!\""),
        },
        E::Building_CortexBaker_1 => many! {
            0 => raw!("News: cortex baker wranglers kindly remind employees that cortex bakers are the bakery's material property and should not be endeared with nicknames."),
            1 => raw!("News: are you smarter than a cortex baker? New game show deemed \"unfair\" by contestants."),
            2 => raw!("News: runt cortex baker identified with an IQ of only quintuple digits: \"just a bit of a dummy\", say specialists."),
            3 => raw!("News: astronomers warn of cortex baker trajectory drift, fear future head-on collisions resulting in costly concussion."),
            4 => raw!("News: space-faring employees advised to ignore unusual thoughts and urges experienced within 2 parsecs of gigantic cortex bakers, say guidelines."),
        },
        E::Building_You_1 => {
            let you = bakery_name.unwrap_or("you");
            let your = bakery_name.unwrap_or("your");
            many! {
                0 => raw!(format!("News: the person of the year is, this year again, {you}! How unexpected!")),
                1 => raw!(format!("News: criminals caught sharing pirated copies of {your}'s genome may be exposed to fines and up to 17 billion years prison, reminds constable.")),
                2 => raw!(format!("News: could local restaurants be serving you bootleg {your} clone meat? Our delicious investigation follows after tonight's news.")),
                3 => raw!(format!("News: beloved cookie magnate {you}, erroneously reported as trampled to death by crazed fans, thankfully found to be escaped clone mistaken for original.")),
                4 => raw!(format!("News: \"Really, we're just looking for some basic societal acceptance and compassion\", mumbles incoherent genetic freak {you}-Clone #59014.")),
            }
        }
        /* -------------------------------------------------------------------------- */
        /*                           Grandmapocalypse Based                           */
        /* -------------------------------------------------------------------------- */
        E::Grandmapocalypse_Awoken => many! {
            0 => raw!("News: millions of old ladies reported missing!"),
            1 => raw!("News: doctors swarmed by cases of old women with glassy eyes and a foamy mouth!"),
            2 => raw!("News: families around the continent report agitated, transfixed grandmothers!"),
            3 => raw!("News: processions of old ladies sighted around cookie facilities!"),
            4 => raw!("News: nurses report \"strange scent of cookie dough\" around elderly patients!"),
        },
        E::Grandmapocalypse_Displeased => many! {
            0 => raw!("News: whole continent undergoing mass exodus of old ladies!"),
            1 => raw!("News: sightings of old ladies with glowing eyes terrify local population!"),
            2 => raw!("News: towns in disarray as strange old ladies break into homes to steal infants and baking utensils!"),
            3 => raw!("News: retirement homes report \"female residents slowly congealing in their seats!\""),
            4 => raw!("News: old women freeze in place in streets, ooze warm sugary syrup!"),
        },
        E::Grandmapocalypse_Angered => many! {
            0 => raw!("News: wrinkled \"flesh tendrils\" visible from space!"),
            1 => raw!("News: remains of \"old ladies\" found frozen in the middle of growing fleshy structures!"),
            2 => raw!("News: large \"flesh highways\" scar continent, stretch between various cookie facilities!"),
            3 => raw!("News: all hope lost as writhing mass of flesh and dough engulfs whole city!"),
            4 => raw!("News: nightmare continues as wrinkled acres of flesh expand at alarming speeds!"),
        },
        E::Grandmapocalypse_Appeased => many! {
            0 => gquote!("shrivel"),
            1 => gquote!("writhe"),
            2 => gquote!("throb"),
            3 => gquote!("gnaw"),
            4 => gquote!("We will rise again."),
            5 => gquote!("A mere setback."),
            6 => gquote!("We are not satiated."),
            7 => gquote!("Too late."),
        },
    }
}

use crate::Building;
use cookie_clicker_tui_utils::num;

#[derive(Debug)]
pub struct SimpleTieredUpgrade {
    building: Building,
    template: SimpleTieredUpgradeTemplate,
}

impl SimpleTieredUpgrade {
    pub const fn new(building: Building, index: usize) -> Self {
        Self {
            building,
            template: SimpleTieredUpgradeTemplate::variants_array(building)[index],
        }
    }

    pub const fn building(&self) -> Building {
        self.building
    }

    pub const fn building_req(&self) -> u16 {
        self.template.building_req
    }

    pub const fn cost(&self) -> f64 {
        self.building.base_cost() * self.template.cost_mult
    }

    pub const fn label(&self) -> &'static str {
        labels::get(self.building)[self.template.index]
    }
}

#[derive(Debug, Copy, Clone)]
struct SimpleTieredUpgradeTemplate {
    index: usize,
    building_req: u16,
    cost_mult: f64,
}

impl SimpleTieredUpgradeTemplate {
    const VARIANT_COUNT: usize = 15;

    const VARIANTS_CURSOR: [Self; Self::VARIANT_COUNT] = [
        Self::new(0, 1, num::SIX_REPEATING),
        Self::new(1, 1, num::THREE_THREE_REPEATING),
        Self::new(2, 10, num::SIX_SIX_SIX_REPEATING),
        Self::new(3, 25, num::SIX_REPEATING * num::THOUSAND),
        Self::new(4, 50, num::SIX_SIX_SIX_REPEATING * num::THOUSAND),
        Self::new(5, 100, num::SIX_REPEATING * num::MILLION),
        Self::new(6, 150, num::SIX_SIX_REPEATING * num::MILLION),
        Self::new(7, 200, num::SIX_SIX_SIX_REPEATING * num::MILLION),
        Self::new(8, 250, num::SIX_SIX_SIX_REPEATING * num::BILLION),
        Self::new(9, 300, num::SIX_SIX_SIX_REPEATING * num::TRILLION),
        Self::new(10, 350, num::SIX_SIX_SIX_REPEATING * num::QUADRILLION),
        Self::new(11, 400, num::SIX_SIX_SIX_REPEATING * num::QUINTILLION),
        Self::new(12, 450, num::SIX_SIX_SIX_REPEATING * num::SEXTILLION),
        Self::new(13, 500, num::SIX_SIX_SIX_REPEATING * num::SEPTILLION),
        Self::new(14, 550, num::SIX_SIX_SIX_REPEATING * num::OCTILLION),
    ];

    const VARIANTS_NON_CURSOR: [Self; Self::VARIANT_COUNT] = [
        Self::new(0, 1, 10.0),
        Self::new(1, 5, 50.0),
        Self::new(2, 25, 500.0),
        Self::new(3, 50, 50.0 * num::THOUSAND),
        Self::new(4, 100, 5.0 * num::MILLION),
        Self::new(5, 150, 500.0 * num::MILLION),
        Self::new(6, 200, 500.0 * num::BILLION),
        Self::new(7, 250, 500.0 * num::TRILLION),
        Self::new(8, 300, 500.0 * num::QUADRILLION),
        Self::new(9, 350, 500.0 * num::QUINTILLION),
        Self::new(10, 400, 5.0 * num::SEPTILLION),
        Self::new(11, 450, 50.0 * num::OCTILLION),
        Self::new(12, 500, 500.0 * num::NONILLION),
        Self::new(13, 550, 5.0 * num::UNDECILLION),
        Self::new(14, 600, 50.0 * num::DUODECILLION),
    ];

    const fn new(index: usize, building_req: u16, cost_mult: f64) -> Self {
        Self {
            index,
            building_req,
            cost_mult,
        }
    }

    const fn variants_array(building: Building) -> [Self; Self::VARIANT_COUNT] {
        if matches!(building, Building::Cursor) {
            Self::VARIANTS_CURSOR
        } else {
            Self::VARIANTS_NON_CURSOR
        }
    }
}

#[rustfmt::skip]
#[allow(non_upper_case_globals)]
mod labels {
    type Labels = [&'static str; super::SimpleTieredUpgradeTemplate::VARIANT_COUNT];

    pub const fn get(building: super::Building) -> Labels {
        macro_rules! arms {
            ($($building:ident),*$(,)?) => {
                match building {
                    $(super::Building::$building => $building),*
                }
            };
        }
        crate::building::all_the_buildings!(arms)
    }

    const Cursor: Labels = ["Reinforced index finger", "Carpal tunnel prevention cream", "Ambidexterous", "Thousand fingers", "Million fingers", "Billion fingers", "Trillion fingers", "Quadrillion fingers", "Quintillion fingers", "Sextillion fingers", "Septillion fingers", "Octillion fingers", "Nonillion fingers", "Decillion fingers", "Undecillion fingers"];
   
    const Grandma: Labels = ["Forwards from grandma", "Steel-plated rolling pins", "Lubricated dentures", "Prune juce", "Double-thick glasses", "Aging agents", "Xtreme walkers", "The Unbrindling", "Reverse demetia", "Timeproof hair dyes", "Good manners", "Generation degeneration", "Visits", "Kitchen cabinets", "Foam-tipped-canes"];
 
    const Farm: Labels = ["Cheap hoes", "Fertilizer", "Cookie trees", "Genetically-modified cookies", "Gingerbread scarecrows", "Pulsar sprinklers", "Fudge fungus", "Wheat triffids", "Humane pesticides", "Barnstars", "Lindworms", "Global seed vault", "Reverse-veganism", "Cookie mulch", "Self-driving tractors"];
  
    const Mine: Labels = ["Sugar gas", "Megadril", "Ultradrill", "Ultimadrill", "H-bomb mining", "Coreforge", "Planetsplitters", "Canola oil wells", "Mole people", "Mine canaries", "Bore again", "Air mining", "Caramel alloys", "Delicious mineralogy", "Multishaft supports"];
   
    const Factory: Labels = ["Sturdier conveyor belts", "Child labour", "Sweatshop", "Radium reactors", "Recombobulators", "Deep-bake process", "Cyborg workforce", "78-hour days", "Machine learning", "Brownie point system", "\"Volunteer\" interns", "Behavioural reframing", "The infinity engine", "N-dimensional assembly lines", "Universal automation"];
   
    const Bank: Labels = ["Taller tellers", "Scissor-resistant credit cards", "Acid-proof vaults", "Chocolate coins", "Exponential interest rates", "Financial zen", "Way of the wallet", "The stuff rationale", "Edible money", "Grand supercycle", "Rules of acquisition", "Altruistic loop", "Diminishing tax returns", "Cookie Points", "The big shortcake"];
    
    const Temple: Labels = ["Golden idols", "Sacrifices", "Delicious blessing", "Sun festival", "Enlarged pantheon", "Great Baker in the sky", "Creation myth", "Theocracy", "Sick rap prayers", "Psalm-reading", "War of the gods", "A novel idea", "Apparitions", "Negatheism", "Temple traps"];

    const WizardTower: Labels = ["Pointier hats", "Beardlier beards", "Ancient grimoires", "Kitchen curses", "School of sorcery", "Dark formulas", "Cookiemancy", "Rabbit trick", "Deluxe tailored wands", "Immobile spellcasting", "Electricity", "Spelling bees", "Wizard basements", "Magical realism", "Polymorphism"];
    
    const Shipment: Labels = ["Vanilla nebulae", "Wormholes", "Frequent flyer", "Warp drive", "Chocolate monoliths", "Generation ship", "Dyson sphere", "The final frontier", "Autopilot", "Restaurants at the end of the universe", "Universal alphabet", "Toroid universe", "Prime directive", "Cosmic foreground radiation", "At your doorstep in 30 minutes or your money back"];
   
    const AlchemyLab: Labels = ["Antimony", "Essence of dough", "True chocolate", "Ambrosia", "Aqua crustulae", "Origin crucible", "Theory of atomic fluidity", "Beige goo", "The advent of chemistry", "On second thought", "Public betterment", "Hermetic reconciliation", "Chromatic cycling", "Arcanized glassware", "The dose makes the poison"];
   
    const Portal: Labels = ["Ancient tablet", "Insane oatling workers", "Soul bond", "Sanity dance", "Brane transplant", "Deity-sized portals", "End of times back-up plan", "Maddening chants", "The real world", "Dimensional garbage gulper", "Embedded microportals", "His advent", "Domestic rifts", "Portal guns", "A way home"];
    
    const TimeMachine: Labels = ["Flux capacitors", "Time paradox resolver", "Quantum conundrum", "Causality enforcer", "Yestermorrow comparators", "Far future enactment", "Great loop hypothesis", "Cookietopian moments of maybe", "Second seconds", "Additional clock hands", "Nostalgia", "Split seconds", "Patience abolished", "Timeproof upholstery", "Rectifying a mistake"];
    
    const AntimatterCondenser: Labels = ["Sugar bosons", "String theory", "Large macaron collider", "Big bang bake", "Reverse cyclotrons", "Nanocosmics", "The Pulse", "Some other super-tiny fundamental particle? Probably?", "Quantum comb", "Baking Nobel prize", "The definite molecule", "Flavor itself", "Delicious pull", "Employee minification", "Candied atoms"];
  
    const Prism: Labels = ["Gem polish", "9th color", "Chocolate light", "Grainbow", "Pure cosmic light", "Glow-in-the-dark", "Lux sanctorum", "Reverse shadows", "Crystal mirrors", "Reverse theory of light", "Light capture measures", "Light speed limit", "Occam's laser", "Hyperblack paint", "Lab goggles but like cool shades"];
    
    const Chancemaker: Labels = ["Your lucky cookie", "\"All Bets Are Off\" magic coin", "Winning lottery ticket", "Four-leaf clover field", "A recipe book about books", "Leprechaun village", "Improbability drive", "Antisuperstistronics", "Bunnypedes", "Revised probabilistics", "0-sided dice", "A touch of determinism", "On a streak", "Silver lining maximization", "Gambler's fallacy fallacy"];
  
    const FractalEngine: Labels = ["Metabakeries", "Mandelbrown sugar", "Fractoids", "Nested universe theory", "Menger sponge cake", "One particularly good-humored cow", "Chocolate ouroboros", "Nested", "Space-filling fibers", "Endless book of prose", "The set of all sets", "This upgrade", "A box", "Multiscale profiling", "The more they stay the same"];
    
    const RustPlayground: Labels = ["The Rust playground for dummies", "References", "Borrow checker", "Turbofish", "Syntactic sugar", "Become crab", "Compile-time baking", "cookies+=1", "Rust nightly", "Infinite loops", "Unsafe pointers", "Your biggest fans", "Hacker shades", "Unsafe containment vats", "Compiler intrinsics"];
 
    const Idleverse: Labels = ["Manifest destiny", "The multiverse in a nutshell", "All-conversion", "Multiverse agents", "Escape plan", "Game design", "Sandbox universes", "Multiverse wars", "Mobile ports", "Encapsulated realities", "Extrinsic clicking", "Universal idling", "Break the fifth wall", "Opposite universe", "The other routes to Rome"];
   
    const CortexBaker: Labels = ["Principled neural shackles", "Obey", "A sprinkle of irrationality", "Front and back hemispheres", "Neural networking", "Cosmic brainstorms", "Megatherapy", "Synaptic lubricant", "Psychokinesis", "Spines", "Neuraforming", "Epistemological trickery", "Every possible idea", "The land of dreams", "Intellectual property theft"];
  
    const You: Labels = ["Cloning vats", "Energized nutrients", "Stunt doubles", "Clone recycling plant", "Free-range clones", "Genetic tailoring", "Power in diversity", "Self-betterment", "Source control", "United workforce", "Safety patrols", "Clone rights", "One big family", "Fine-tuned body plans", "Reading your clones bedtime stories"];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Building;
    use approx_eq_trait::assert_approx_eq_slice;
    use cookie_clicker_tui_utils::num::*;

    fn upgrades_for(b: Building) -> [SimpleTieredUpgrade; 15] {
        std::array::from_fn(|i| SimpleTieredUpgrade::new(b, i))
    }

    #[test]
    fn cursor_prices() {
        assert_approx_eq_slice!(
            upgrades_for(Building::Cursor).map(|s| s.cost()),
            [
                100.0,
                500.0,
                10.0 * THOUSAND,
                100.0 * THOUSAND,
                10.0 * MILLION,
                100.0 * MILLION,
                1.0 * BILLION,
                10.0 * BILLION,
                10.0 * TRILLION,
                10.0 * QUADRILLION,
                10.0 * QUINTILLION,
                10.0 * SEXTILLION,
                10.0 * SEPTILLION,
                10.0 * OCTILLION,
                10.0 * NONILLION,
            ],
        )
    }

    #[test]
    fn farm_prices() {
        assert_approx_eq_slice!(
            upgrades_for(Building::Farm).map(|s| s.cost()),
            [
                11.0 * THOUSAND,
                55.0 * THOUSAND,
                550.0 * THOUSAND,
                55.0 * MILLION,
                5.5 * BILLION,
                550.0 * BILLION,
                550.0 * TRILLION,
                550.0 * QUADRILLION,
                550.0 * QUINTILLION,
                550.0 * SEXTILLION,
                5.5 * OCTILLION,
                55.0 * NONILLION,
                550.0 * DECILLION,
                5.5 * DUODECILLION,
                55.0 * TREDECILLION
            ],
        );
    }

    #[test]
    fn factory_prices() {
        assert_approx_eq_slice!(
            upgrades_for(Building::Factory).map(|s| s.cost()),
            [
                1.3 * MILLION,
                6.5 * MILLION,
                65.0 * MILLION,
                6.5 * BILLION,
                650.0 * BILLION,
                65.0 * TRILLION,
                65.0 * QUADRILLION,
                65.0 * QUINTILLION,
                65.0 * SEXTILLION,
                65.0 * SEPTILLION,
                650.0 * OCTILLION,
                6.5 * DECILLION,
                65.0 * UNDECILLION,
                650.0 * DUODECILLION,
                6.5 * QUATTORDECILLION,
            ],
        );
    }

    #[test]
    fn cursor_labels() {
        assert_eq!(
            upgrades_for(Building::Cursor).map(|s| s.label()),
            labels::get(Building::Cursor),
        );
    }
}

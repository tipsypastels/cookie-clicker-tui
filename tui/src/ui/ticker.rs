use super::UiApp;
use cookie_clicker_tui_core::TickerEntry;
use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph},
};

pub fn ticker(app: &mut UiApp, area: Rect, buf: &mut Buffer) {
    let ticker = app.core.ticker();
    let Some(entry) = ticker.entry() else {
        return;
    };

    let line = line_text(entry, ticker.seed1(), ticker.seed2());

    Paragraph::new(line)
        .centered()
        .block(Block::bordered())
        .render(area, buf);
}

fn line_text(entry: TickerEntry, seed1: u64, seed2: u64) -> Line<'static> {
    let _ = (seed1, seed2); // until needed;

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

    use TickerEntry as E;
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
        E::Grandmapocalypse_Awoken_1 => raw!("News: millions of old ladies reported missing!"),
        E::Grandmapocalypse_Awoken_2 => {
            raw!("News: doctors swarmed by cases of old women with glassy eyes and a foamy mouth!")
        }
        E::Grandmapocalypse_Awoken_3 => {
            raw!("News: families around the continent report agitated, transfixed grandmothers!")
        }
        E::Grandmapocalypse_Awoken_4 => {
            raw!("News: processions of old ladies sighted around cookie facilities!")
        }
        E::Grandmapocalypse_Awoken_5 => {
            raw!("News: nurses report \"strange scent of cookie dough\" around elderly patients!")
        }
        E::Grandmapocalypse_Displeased_1 => {
            raw!("News: whole continent undergoing mass exodus of old ladies!")
        }
        E::Grandmapocalypse_Displeased_2 => {
            raw!("News: sightings of old ladies with glowing eyes terrify local population!")
        }
        E::Grandmapocalypse_Displeased_3 => raw!(
            "News: towns in disarray as strange old ladies break into homes to steal infants and baking utensils!"
        ),
        E::Grandmapocalypse_Displeased_4 => raw!(
            "News: retirement homes report \"female residents slowly congealing in their seats!\""
        ),
        E::Grandmapocalypse_Displeased_5 => {
            raw!("News: old women freeze in place in streets, ooze warm sugary syrup!")
        }
        E::Grandmapocalypse_Angered_1 => {
            raw!("News: wrinkled \"flesh tendrils\" visible from space!")
        }
        E::Grandmapocalypse_Angered_2 => raw!(
            "News: remains of \"old ladies\" found frozen in the middle of growing fleshy structures!"
        ),
        E::Grandmapocalypse_Angered_3 => raw!(
            "News: large \"flesh highways\" scar continent, stretch between various cookie facilities!"
        ),
        E::Grandmapocalypse_Angered_4 => {
            raw!("News: all hope lost as writhing mass of flesh and dough engulfs whole city!")
        }
        E::Grandmapocalypse_Angered_5 => {
            raw!("News: nightmare continues as wrinkled acres of flesh expand at alarming speeds!")
        }
        E::Grandmapocalypse_Appeased_1 => sps!(sp!("\"shrivel\"", ITALIC), sp!(" -grandma")),
        E::Grandmapocalypse_Appeased_2 => sps!(sp!("\"writhe\"", ITALIC), sp!(" -grandma")),
        E::Grandmapocalypse_Appeased_3 => sps!(sp!("\"throb\"", ITALIC), sp!(" -grandma")),
        E::Grandmapocalypse_Appeased_4 => sps!(sp!("\"gnaw\"", ITALIC), sp!(" -grandma")),
        E::Grandmapocalypse_Appeased_5 => {
            sps!(sp!("\"We will rise again.\"", ITALIC), sp!(" -grandma"))
        }
        E::Grandmapocalypse_Appeased_6 => {
            sps!(sp!("\"A mere setback.\"", ITALIC), sp!(" -grandma"))
        }
        E::Grandmapocalypse_Appeased_7 => {
            sps!(sp!("\"We are not satiated.\"", ITALIC), sp!(" -grandma"))
        }
        E::Grandmapocalypse_Appeased_8 => sps!(sp!("\"Too late.\"", ITALIC), sp!(" -grandma")),
    }
}

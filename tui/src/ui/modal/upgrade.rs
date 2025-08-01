use super::{super::utils::num::PrintFloat, ModalImpl};
use cookie_clicker_tui_core::{Building, Upgrade, UpgradeEffectInfo};
use ratatui::{prelude::*, widgets::Paragraph};

pub fn upgrade(upgrade: Upgrade, area: Rect, buf: &mut Buffer) {
    let title = format!(" {} ", upgrade.name());
    let modal = ModalImpl {
        area,
        buf,
        title: title.into(),
        title_bottom: " Close <Esc> ".into(),
        screen_percent: (60, 31),
    };

    modal.render(|area, buf, block| {
        let mut lines = Vec::new();
        let info = upgrade.effect_info();

        print_info(info, &mut lines);
        Paragraph::new(lines).block(block).render(area, buf);
    });
}

pub fn print_info(info: UpgradeEffectInfo, lines: &mut Vec<Line>) {
    match info {
        UpgradeEffectInfo::Tiered(building) => {
            lines.push(line_2x_from_building(building));
        }
        UpgradeEffectInfo::ThousandFingers => lines.push(Line::from(vec![
            Span::raw("• clicking and cursors gain "),
            Span::styled("+0.1", Modifier::BOLD),
            Span::raw(" cookies per second from other buildings"),
        ])),
        UpgradeEffectInfo::ThousandFingersMult(mult) => lines.push(Line::from(vec![
            Span::raw("• multiplies the gain from "),
            Span::styled("thousand fingers", Modifier::BOLD),
            Span::raw(" by "),
            Span::styled(format!("{}", mult.print_float(0, 0)), Modifier::BOLD),
        ])),
        UpgradeEffectInfo::Grandma {
            building,
            num_req_for_1p,
        } => {
            lines.push(line_2x_from_building(Building::Grandma));
            lines.push(Line::from(vec![
                Span::raw("• "),
                Span::styled("+1%", Modifier::BOLD),
                Span::raw(" cookies per second from "),
                Span::styled(building.name_lower_plural(), Modifier::BOLD),
                Span::raw(" per "),
                Span::styled(
                    format!(
                        "{num_req_for_1p} {}",
                        Building::Grandma.name_lower_pluralized(num_req_for_1p as _)
                    ),
                    Modifier::BOLD,
                ),
            ]));
        }
        UpgradeEffectInfo::Kitten => {
            lines.push(Line::from(vec![
                Span::raw("• you gain more cookies per second the more "),
                Span::styled("milk", Modifier::BOLD),
                Span::raw(" you have"),
            ]));
        }
        UpgradeEffectInfo::Research { effect, warning } => {
            use cookie_clicker_tui_core::{
                UpgradeInfoEffectResearch as R, UpgradeInfoEffectResearchWarning as W,
            };

            match effect {
                R::StartAndGrandmaCpsMult(mult) => {
                    lines.push(Line::from(vec![
                        Span::raw("• "),
                        Span::styled(format!("{}x", mult.print_float(0, 0)), Modifier::BOLD),
                        Span::raw(" cookies per second from "),
                        Span::styled(Building::Grandma.name_lower_plural(), Modifier::BOLD),
                    ]));
                }
                R::CpsMultiplierPercent(p) => {
                    lines.push(Line::from(vec![
                        Span::raw("• "),
                        Span::styled(
                            format!("+{}%", (p * 100.0).print_float(0, 0)),
                            Modifier::BOLD,
                        ),
                        Span::raw(" cookies per second"),
                    ]));
                }
                R::GrandmaCpsDouble => {
                    lines.push(line_2x_from_building(Building::Grandma));
                }
                R::GrandmaGainsCpsPerBuilding(building, cps) => {
                    lines.push(Line::from(vec![
                        Span::raw("• "),
                        Span::styled(format!("+{}", cps.print_float(2, 0)), Modifier::BOLD),
                        Span::raw(" cookies per second from "),
                        Span::styled(Building::Grandma.name_lower_plural(), Modifier::BOLD),
                        Span::raw(" per "),
                        Span::styled(building.name_lower(), Modifier::BOLD),
                    ]));
                }
                R::ElderPledgesLastTwiceAsLong => {
                    lines.push(Line::from(vec![
                        Span::raw("• elder pledges last "),
                        Span::styled("twice", Modifier::BOLD),
                        Span::raw(" as long"),
                    ]));
                }
            }

            if let R::StartAndGrandmaCpsMult(_) = effect {
                lines.push(Line::styled(
                    "• regularly unlocks new upgrades",
                    Modifier::BOLD,
                ));
            }

            if let Some(warning) = warning.map(|warning| match warning {
                W::One => "the grandmas are growing restless",
                W::Two => "proceeding further may have unexpected results",
                W::Three => "this is a bad idea",
            }) {
                lines.push(Line::from(vec![
                    Span::raw("• "),
                    Span::styled(warning, Style::new().on_red()),
                ]));
            }
        }
        UpgradeEffectInfo::ElderPledge => {
            lines.push(Line::raw("• contains the wrath of the elders for a while"));
        }
        UpgradeEffectInfo::ElderCovenant { revoke: false } => {
            lines.push(Line::raw("• contains the wrath of the elders permanently"));
            lines.push(Line::from(vec![
                Span::raw("• reduces your cookies per second by "),
                Span::styled("5%", Modifier::BOLD),
            ]));
        }
        UpgradeEffectInfo::ElderCovenant { revoke: true } => {
            lines.push(Line::raw("• releases the wrath of the elders"));
            lines.push(Line::from(vec![
                Span::raw("• gives back your "),
                Span::styled("5%", Modifier::BOLD),
                Span::raw(" cookies per second"),
            ]));
        }
    }
}

fn line_2x_from_building(building: Building) -> Line<'static> {
    Line::from(vec![
        Span::raw("• "),
        Span::styled("2x", Modifier::BOLD),
        Span::raw(" cookies per second from "),
        Span::styled(building.name_lower_plural(), Modifier::BOLD),
    ])
}

use std::convert::TryFrom;
use std::fmt;

use chrono::{FixedOffset, Timelike, Utc};

pub struct Area {
    pub pref: String,
    pub city: String,
}

// はっきり変わる場合、時間ないかもしれない場合、時間外かもしれない場合、該当地域でない場合
pub enum HelpLineType {
    InService(AreaHelpLine),
    UnknownTime(AreaHelpLine),
    OutOfTime(AreaHelpLine),
    None,
}

// 午前nとか翌m時とか表示する用
#[derive(PartialEq, PartialOrd)]
struct MyTime(u8);
impl fmt::Display for MyTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                0..=12 => format!("午前{}時", self.0),
                13..=24 => format!("午後{}時", self.0 - 12),
                _ => format!("翌{}時", self.0 - 24),
            }
        )
    }
}

// 地域から窓口を取得する。
pub fn get_help_line(area: &Area) -> (HelpLineType, HelpLineType) {
    (
        get_help_line_from(adult, area),
        get_help_line_from(children, area),
    )
}
fn get_help_line_from(data: fn() -> Vec<AreaHelpLine>, area: &Area) -> HelpLineType {
    match data()
        .into_iter()
        // 対象のエリアがある場合
        .find(|help_line| {
            area.pref == help_line.pref && help_line.cities.contains(&area.city.as_str())
        })
        // 都道府県がやっているサービス
        .or_else(|| {
            data()
                .into_iter()
                .find(|help_line| area.pref == help_line.pref && help_line.cities.is_empty())
        }) {
        Some(help_line) => {
            return match &help_line.time {
                TimeType::Allday { .. } => {
                    if help_line.time.in_now() {
                        HelpLineType::InService(help_line)
                    } else {
                        HelpLineType::OutOfTime(help_line)
                    }
                }
                TimeType::WeekHoliday { .. } => HelpLineType::UnknownTime(help_line),
            };
        }
        None => {}
    };

    HelpLineType::None
}

// 平日と休日で時間が異なることがあるので
enum TimeType {
    Allday {
        from: MyTime,
        to: MyTime,
    },
    WeekHoliday {
        w_from: MyTime,
        w_to: MyTime,
        s_from: MyTime,
        s_to: MyTime,
        h_from: MyTime,
        h_to: MyTime,
    },
}
impl TimeType {
    pub fn in_now(&self) -> bool {
        match self {
            Self::Allday { from, to } => {
                let now = TryFrom::try_from(
                    Utc::now()
                        .with_timezone(&FixedOffset::east(9 * 3600))
                        .hour(),
                )
                .unwrap();
                if to.0 < 24 {
                    from.0 <= now && now < to.0
                } else {
                    from.0 <= now || now < to.0 - 24
                }
            }
            _ => false, // 今日が平日か休日かがわからないため、判定しない
        }
    }
}

pub struct AreaHelpLine {
    pref: &'static str,
    cities: Vec<&'static str>,
    hp: &'static str,
    url: &'static str,
    time: TimeType,
    phone: Vec<&'static str>,
}
impl fmt::Display for AreaHelpLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n\n{}\n\n{}",
            self.hp,
            self.url,
            match &self.time {
                TimeType::Allday { from, to } => {
                    format!("[全日]\n{}〜{}", from, to)
                }
                TimeType::WeekHoliday {
                    w_from,
                    w_to,
                    s_from,
                    s_to,
                    h_from,
                    h_to,
                } => {
                    format!(
                        "[平日]\n{}〜{}\n[土曜]\n{}〜{}\n[休日]\n{}〜{}",
                        w_from, w_to, s_from, s_to, h_from, h_to
                    )
                }
            },
            format!("[電話]\n{}\n", self.phone.join(" または "),),
        )
    }
}

fn adult() -> Vec<AreaHelpLine> {
    vec![
        AreaHelpLine {
            pref: "北海道",
            cities: vec!["札幌市", "石狩市", "新篠津村", "栗山町", "当別町", "南幌町"],
            hp: "救急安心センターさっぽろ",
            url: "https://www.city.sapporo.jp/hokenjo/qq7199/naiyou.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "011-272-7119"],
        },
        AreaHelpLine {
            pref: "北海道",
            cities: Vec::new(),
            hp: "北海道救急医療・広域災害情報システム(医療機関の案内のみ)",
            url: "https://www.qq.pref.hokkaido.jp/qq/qq01.asp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-20-8699", "011-221-8699"],
        },
        AreaHelpLine {
            pref: "青森県",
            cities: Vec::new(),
            hp: "あおもり医療情報ネットワーク",
            url: "https://www.qq.pref.aomori.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-733620"],
        },
    ]
}
fn children() -> Vec<AreaHelpLine> {
    vec![
        AreaHelpLine {
            pref: "北海道",
            cities: Vec::new(),
            hp: "北海道救急医療・広域災害情報システム(医療機関の案内のみ)",
            url: "https://www.qq.pref.hokkaido.jp/qq/qq01.asp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#8000", ""],
        },
        AreaHelpLine {
            pref: "青森県",
            cities: Vec::new(),
            hp: "青森県子ども医療でんわ相談",
            url: "https://www.pref.aomori.lg.jp/soshiki/kenko/iryo/kodomoqq.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(13),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "017－722－1152"],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mytime_0_24_in_now() {
        assert!(TimeType::Allday {
            from: MyTime(0),
            to: MyTime(24)
        }
        .in_now());
    }
    #[test]
    fn mytime_24_48_in_now() {
        assert!(TimeType::Allday {
            from: MyTime(24),
            to: MyTime(48)
        }
        .in_now());
    }
    #[test]
    fn mytime_week_hokiday() {
        assert_eq!(
            TimeType::WeekHoliday {
                w_from: MyTime(0),
                w_to: MyTime(24),
                s_from: MyTime(0),
                s_to: MyTime(24),
                h_from: MyTime(0),
                h_to: MyTime(24),
            }
            .in_now(),
            false
        );
    }
    #[test]
    fn get_help_line_ok() {
        assert!(match get_help_line(&Area {
            pref: "北海道".to_string(),
            city: "札幌市".to_string()
        }) {
            (HelpLineType::InService(..), HelpLineType::InService(..)) => true,
            _ => false,
        });
        assert!(match get_help_line(&Area {
            pref: "北海道".to_string(),
            city: "利尻町".to_string()
        }) {
            (HelpLineType::InService(..), HelpLineType::InService(..)) => true,
            _ => false,
        });
    }
    #[test]
    fn get_help_line_ng() {
        assert!(match get_help_line(&Area {
            pref: "東京都".to_string(),
            city: "東京市".to_string()
        }) {
            (HelpLineType::None, HelpLineType::None) => true,
            _ => false,
        });
    }
}

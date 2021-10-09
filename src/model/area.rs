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
#[derive(PartialEq, PartialOrd, Clone)]
pub struct MyTime(pub u8);
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

// 平日と休日で時間が異なることがあるので
pub enum TimeType {
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
    pub pref: &'static str,
    pub cities: Vec<&'static str>,
    pub hp: &'static str,
    pub url: &'static str,
    pub time: TimeType,
    pub phone: Vec<&'static str>,
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
                    format!(
                        "[全日]\n{}",
                        if from.0 == 0 && to.0 == 24 {
                            String::from("24時間")
                        } else {
                            format!("{}〜{}", from, to)
                        }
                    )
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
            format!("[電話]\n{}\n", self.phone.join(" または\n"),),
        )
    }
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
}

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
            format!("[電話]\n{}\n", self.phone.join(" または\n"),),
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
        AreaHelpLine {
            pref: "山形県",
            cities: Vec::new(),
            hp: "山形県救急電話相談（＃8000・＃8500） _ 山形県",
            url: "https://www.pref.yamagata.jp/090013/bosai/shobo/kyuukyuu/99tel.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#7119", "023-633-0799"],
        },
        AreaHelpLine {
            pref: "東京都",
            cities: vec![
                "千代田区",
                "中央区",
                "港区",
                "新宿区",
                "文京区",
                "台東区",
                "墨田区",
                "江東区",
                "品川区",
                "目黒区",
                "大田区",
                "世田谷区",
                "渋谷区",
                "中野区",
                "杉並区",
                "豊島区",
                "北区",
                "荒川区",
                "板橋区",
                "練馬区",
                "足立区",
                "葛飾区",
                "江戸川区",
            ],
            hp: "東京消防庁",
            url: "https://www.tfd.metro.tokyo.lg.jp/lfe/kyuu-adv/soudan-center.htm",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "03-3212-2323"],
        },
        AreaHelpLine {
            pref: "東京都",
            cities: vec![
                "千代田区",
                "中央区",
                "港区",
                "新宿区",
                "文京区",
                "台東区",
                "墨田区",
                "江東区",
                "品川区",
                "目黒区",
                "大田区",
                "世田谷区",
                "渋谷区",
                "中野区",
                "杉並区",
                "豊島区",
                "北区",
                "荒川区",
                "板橋区",
                "練馬区",
                "足立区",
                "葛飾区",
                "江戸川区",
            ],
            hp: "東京消防庁",
            url: "https://www.tfd.metro.tokyo.lg.jp/lfe/kyuu-adv/soudan-center.htm",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "042-521-2323"],
        },
        AreaHelpLine {
            pref: "埼玉県",
            cities: Vec::new(),
            hp: "埼玉県救急電話相談（お医者さんに行くべきか迷ったら、♯7119） - 埼玉県",
            url: "https://www.pref.saitama.lg.jp/a0703/20151214.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "048-824-4199"],
        },
        AreaHelpLine {
            pref: "茨城県",
            cities: Vec::new(),
            hp: "茨城県救急医療情報システム",
            url: "http://www.qq.pref.ibaraki.jp/?",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "050-5445-2856"],
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
        AreaHelpLine {
            pref: "岩手県",
            cities: Vec::new(),
            hp: "こどもの救急",
            url: "http://kodomo-qq.jp/index.php?pname=n8000%2Fp3",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(23),
            },
            phone: vec!["#8000", "019-605-9000"],
        },
        AreaHelpLine {
            pref: "秋田県",
            cities: Vec::new(),
            hp: "こども救急電話相談室について _ 美の国あきたネット",
            url: "https://www.pref.akita.lg.jp/pages/archive/2319",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "018-895-9900"],
        },
        AreaHelpLine {
            pref: "宮城県",
            cities: Vec::new(),
            hp: "宮城県こども夜間安心コール事業について - 宮城県公式ウェブサイト",
            url: "https://www.pref.miyagi.jp/soshiki/iryou/shouni04.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "022-212-9390"],
        },
        AreaHelpLine {
            pref: "山形県",
            cities: Vec::new(),
            hp: "山形県救急電話相談（＃8000・＃8500） _ 山形県",
            url: "https://www.pref.yamagata.jp/090013/bosai/shobo/kyuukyuu/99tel.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "023-633-0299"],
        },
        AreaHelpLine {
            pref: "福島県",
            cities: Vec::new(),
            hp: "子どもの救急について - 福島県ホームページ",
            url: "http://www.pref.fukushima.lg.jp/sec/21045c/iryou-kodomokyukyu.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "024-521-3790"],
        },
        AreaHelpLine {
            pref: "東京都",
            cities: Vec::new(),
            hp: "子供の健康相談室（小児救急相談）　東京都福祉保健局",
            url: "https://www.fukushihoken.metro.tokyo.lg.jp/kodomo/sodan/k_soudan.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(8),
                h_to: MyTime(32),
                h_from: MyTime(8),
                s_to: MyTime(32),
            },
            phone: vec!["#8000", "03－5285－8898"],
        },
        AreaHelpLine {
            pref: "神奈川県",
            cities: Vec::new(),
            hp: "かながわ小児救急ダイヤル - 神奈川県ホームページ",
            url: "http://www.pref.kanagawa.jp/docs/t3u/cnt/f952/",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "045-722-8000"],
        },
        AreaHelpLine {
            pref: "埼玉県",
            cities: Vec::new(),
            hp: "埼玉県救急電話相談（お医者さんに行くべきか迷ったら、♯7119） - 埼玉県",
            url: "https://www.pref.saitama.lg.jp/a0703/20151214.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#8000", "048-833-7911"],
        },
        AreaHelpLine {
            pref: "千葉県",
            cities: Vec::new(),
            hp: "こども急病電話相談（小児救急電話相談）について／千葉県",
            url: "http://www.pref.chiba.lg.jp/iryou/soudan/shouni.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(30),
            },
            phone: vec!["#8000", "043-242-9939"],
        },
        AreaHelpLine {
            pref: "茨城県",
            cities: Vec::new(),
            hp: "茨城県救急医療情報システム",
            url: "http://www.qq.pref.ibaraki.jp/?",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#8000", "050-5445-2856"],
        },
        AreaHelpLine {
            pref: "栃木県",
            cities: Vec::new(),
            hp: "栃木県／とちぎ子ども救急電話相談",
            url: "http://www.pref.tochigi.lg.jp/e02/advice/fukushi/iryou/totigikodomokyuukyuudennwasoudan.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "028-600-0099"],
        },
        AreaHelpLine {
            pref: "群馬県",
            cities: Vec::new(),
            hp: "群馬県 - 子ども医療電話相談（＃（シャープ）８０００）",
            url: "https://www.pref.gunma.jp/02/d1010002.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "03-6735-8835"],
        },
        AreaHelpLine {
            pref: "新潟県",
            cities: Vec::new(),
            hp: "夜間の小児救急医療電話相談（＃8000）を実施しています。 - 新潟県ホームページ",
            url: "https://www.pref.niigata.lg.jp/sec/chiikiiryo/1196180205728.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "025－288－2525"],
        },
        AreaHelpLine {
            pref: "富山県",
            cities: Vec::new(),
            hp: "富山県／子どもの救急について",
            url: "https://www.pref.toyama.jp/1204/kurashi/soudanshisetsu/madoguchi/kenkouiryou/kj00018760.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(33),
                s_from: MyTime(13),
                s_to: MyTime(33),
                h_from: MyTime(9),
                h_to: MyTime(33),
            },
            phone: vec!["#8000", "076-444-1099"],
        },
        AreaHelpLine {
            pref: "石川県",
            cities: Vec::new(),
            hp: "石川県／石川県内のこどもの救急",
            url: "https://www.pref.ishikawa.lg.jp/iryou/support/qqtel/index.html",
            time: TimeType::Allday {
                from: MyTime(18),
                to: MyTime(32),
            },
            phone: vec!["#8000", "076-238-0099"],
        },
        AreaHelpLine {
            pref: "福井県",
            cities: Vec::new(),
            hp: "＃８０００子ども救急医療電話相談事業について _ 福井県ホームページ",
            url: "https://www.pref.fukui.lg.jp/doc/iryou/iryoujouhou/8000.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(33),
                s_from: MyTime(19),
                s_to: MyTime(33),
                h_from: MyTime(9),
                h_to: MyTime(33),
            },
            phone: vec!["#8000", "0776-25-9955"],
        },
        AreaHelpLine {
            pref: "長野県",
            cities: Vec::new(),
            hp: "長野県小児救急電話相談について／長野県",
            url: "https://www.pref.nagano.lg.jp/hoken-shippei/sodan/shonikyukyu.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "0263-34-8000"],
        },
        AreaHelpLine {
            pref: "山梨県",
            cities: Vec::new(),
            hp: "山梨県／小児電話相談 ＃8000　／　小児救急医療の利用について",
            url: "https://www.pref.yamanashi.jp/imuka/shonikyukyu.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(31),
                s_from: MyTime(15),
                s_to: MyTime(31),
                h_from: MyTime(9),
                h_to: MyTime(31),
            },
            phone: vec!["#8000", "0776-25-9955"],
        },
        AreaHelpLine {
            pref: "静岡県",
            cities: Vec::new(),
            hp: "静岡県／静岡こども救急電話相談（#8000）",
            url: "http://www.pref.shizuoka.jp/kousei/ko-450/iryou/kodomokyuukyuudenwa.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(13),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "054-247-9910"],
        },
        AreaHelpLine {
            pref: "愛知県",
            cities: Vec::new(),
            hp: "子どもの救急｜あいち はぐみんNet",
            url: "https://www.pref.aichi.jp/kosodate/hagumin/emergency/index.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "052-962-9900"],
        },
        AreaHelpLine {
            pref: "岐阜県",
            cities: Vec::new(),
            hp: "ぎふ救急ネット",
            url: "http://www.qq.pref.gifu.lg.jp/qq21/WP0101/RP010101BL",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(8),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "058-240-4199"],
        },
        AreaHelpLine {
            pref: "三重県",
            cities: Vec::new(),
            hp: "三重県｜地域医療：みえ子ども医療ダイヤル（＃８０００）",
            url: "https://www.pref.mie.lg.jp/CHIIRYO/HP/2015030111.htm",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "059-232-9955"],
        },
        AreaHelpLine {
            pref: "滋賀県",
            cities: Vec::new(),
            hp: "小児救急電話相談 #8000 - 医療ネット滋賀",
            url: "http://www.shiga.iryo-navi.jp/qqport/kenmintop/other/fks020.php",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(9),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "077-524-7856"],
        },
        AreaHelpLine {
            pref: "京都府",
            cities: Vec::new(),
            hp: "小児救急電話相談（＃8000）／京都府ホームページ",
            url: "http://www.pref.kyoto.jp/iryo/8000.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(15),
                s_to: MyTime(32),
                h_from: MyTime(19),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "075-661-5596"],
        },
        AreaHelpLine {
            pref: "大阪府",
            cities: Vec::new(),
            hp: "大阪府／小児救急電話相談（＃８０００）について",
            url: "https://www.pref.osaka.lg.jp/iryo/syouni-qq/syouni_qq_tel.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "06-6765-3650"],
        },
        AreaHelpLine {
            pref: "兵庫県",
            cities: Vec::new(),
            hp: "兵庫県／子ども医療電話相談(#8000)の電話番号について",
            url: "http://web.pref.hyogo.lg.jp/kf15/hw11_000000013.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "078-304-8899"],
        },
        AreaHelpLine {
            pref: "奈良県",
            cities: Vec::new(),
            hp: "全国一充実した体制　～小児救急電話相談～_奈良県公式ホームページ",
            url: "https://www.pref.nara.jp/item/15976.htm",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(13),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "0742-20-8119"],
        },
        AreaHelpLine {
            pref: "和歌山県",
            cities: Vec::new(),
            hp: "子ども救急相談ダイヤル | 和歌山県",
            url: "https://www.pref.wakayama.lg.jp/prefg/050100/kodomodial/kodomokyukyu.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(33),
                s_from: MyTime(9),
                s_to: MyTime(33),
                h_from: MyTime(9),
                h_to: MyTime(33),
            },
            phone: vec!["#8000", "073-431-8000"],
        },
        AreaHelpLine {
            pref: "鳥取県",
            cities: Vec::new(),
            hp: "とっとり子ども救急ダイヤル(#8000)_とりネット_鳥取県公式サイト",
            url: "https://www.pref.tottori.lg.jp/dd.aspx?menuid=97931",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(8),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "03-6626-220"],
        },
        AreaHelpLine {
            pref: "島根県",
            cities: Vec::new(),
            hp: "島根県： 子ども医療電話相談（#8000）事業トップ",
            url: "https://www.pref.shimane.lg.jp/medical/kenko/iryo/shimaneno_iryo/8000/",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(33),
                s_from: MyTime(9),
                s_to: MyTime(33),
                h_from: MyTime(9),
                h_to: MyTime(33),
            },
            phone: vec!["#8000", "03-3478-1060"],
        },
        AreaHelpLine {
            pref: "岡山県",
            cities: Vec::new(),
            hp: "小児救急医療電話相談 - 岡山県ホームページ（医療推進課）",
            url: "https://www.pref.okayama.jp/page/detail-7000.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(18),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "086-801-0018"],
        },
        AreaHelpLine {
            pref: "広島県",
            cities: Vec::new(),
            hp: "小児救急医療電話相談　（こどもの救急電話相談） | 広島県",
            url: "https://www.pref.hiroshima.lg.jp/soshiki/54/syouniqqdenwa.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "082-505-1399"],
        },
        AreaHelpLine {
            pref: "山口県",
            cities: Vec::new(),
            hp: "小児救急医療電話相談・・ご案内｜山口県",
            url: "https://www.pref.yamaguchi.lg.jp/cms/a11700/shounikyuukyuu/201503250001.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "083-921-2755"],
        },
        AreaHelpLine {
            pref: "徳島県",
            cities: Vec::new(),
            hp: "徳島こども医療電話相談（＃8000）について｜徳島県ホームページ",
            url: "https://www.pref.tokushima.lg.jp/ippannokata/kenko/iryo/2012092100146/",
            time: TimeType::Allday {
                from: MyTime(18),
                to: MyTime(32),
            },
            phone: vec!["#8000", "088-621-2365"],
        },
        AreaHelpLine {
            pref: "香川県",
            cities: Vec::new(),
            hp: "小児救急電話相談 - 医療Netさぬき 広域災害・救急・周産期医療情報システム",
            url: "https://www.qq.pref.kagawa.lg.jp/ir37/qqport/kenmintop/other/fks510.php",
            time: TimeType::Allday {
                from: MyTime(18),
                to: MyTime(32),
            },
            phone: vec!["#8000", "087-823-1588"],
        },
        AreaHelpLine {
            pref: "愛媛県",
            cities: Vec::new(),
            hp: "愛媛県庁／小児救急医療情報",
            url: "https://www.pref.ehime.jp/h20150/kyukyu_syoni/syonikyukyu.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(13),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "089-913-2777"],
        },
        AreaHelpLine {
            pref: "高知県",
            cities: Vec::new(),
            hp: "こうちこども救急ダイヤルについて | 高知県庁ホームページ",
            url: "https://www.pref.kochi.lg.jp/soshiki/131301/syounidenwa.html",
            time: TimeType::Allday {
                from: MyTime(20),
                to: MyTime(25),
            },
            phone: vec!["#8000", "088-873-3090"],
        },
        AreaHelpLine {
            pref: "福岡県",
            cities: Vec::new(),
            hp: "子どもの急な病気に困ったら・・・小児救急医療電話相談（＃８０００） - 福岡県庁ホームページ",
            url: "https://www.pref.fukuoka.lg.jp/contents/8000syonidenwasoudan.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(31),
                s_from: MyTime(12),
                s_to: MyTime(31),
                h_from: MyTime(7),
                h_to: MyTime(31),
            },
            phone: vec!["#8000", "北九州地域 093-662-6700", "福岡地域 092-661-0771", "筑後地域 0942-37-6116", "筑豊地域 0948-23-8270"],
        },
        AreaHelpLine {
            pref: "佐賀県",
            cities: Vec::new(),
            hp: "小児救急医療電話相談事業（＃8000）を行っています _ 佐賀県",
            url: "https://www.pref.saga.lg.jp/kiji00334475/index.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "0952-24-2200"],
        },
        AreaHelpLine {
            pref: "長崎県",
            cities: Vec::new(),
            hp: "子ども医療電話相談 | 長崎県",
            url: "https://www.pref.nagasaki.jp/bunrui/hukushi-hoken/iryo/kensaku-iryo/shonikyukyu/",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(18),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "095-822-3308"],
        },
        AreaHelpLine {
            pref: "熊本県",
            cities: Vec::new(),
            hp: "熊本県子ども医療電話相談事業（＃8000事業） - 熊本県ホームページ",
            url: "https://www.pref.kumamoto.jp/soshiki/42/5938.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(15),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "096-364-9999"],
        },
        AreaHelpLine {
            pref: "大分県",
            cities: Vec::new(),
            hp: "大分県こども救急電話相談 - 大分県ホームページ",
            url: "https://www.pref.oita.jp/soshiki/12620/kodomodenwasoudan.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(19),
                s_to: MyTime(32),
                h_from: MyTime(9),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "097-503-8822"],
        },
        AreaHelpLine {
            pref: "宮崎県",
            cities: Vec::new(),
            hp: "宮崎県：子ども救急医療電話相談（＃8000）について",
            url: "http://www.pref.miyazaki.lg.jp/iryoyakumu/kenko/iryo/index-03.html",
            time: TimeType::Allday {
                from: MyTime(19),
                to: MyTime(32),
            },
            phone: vec!["#8000", "0985-35-8855"],
        },
        AreaHelpLine {
            pref: "鹿児島県",
            cities: Vec::new(),
            hp: "鹿児島県／小児救急電話相談（#8000）",
            url: "https://www.pref.kagoshima.jp/ae08/kenko-fukushi/kenko-iryo/kikan/chikiiryou/kyukyudenwa.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(19),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "099-254-1186"],
        },
        AreaHelpLine {
            pref: "沖縄県",
            cities: Vec::new(),
            hp: "小児救急電話相談(＃8000)の相談時間の延長について／沖縄県",
            url: "https://www.pref.okinawa.jp/site/hoken/iryoseisaku/iryo/8000.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(19),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#8000", "098-888-5230"],
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
            pref: "東京府".to_string(),
            city: "東京市".to_string()
        }) {
            (HelpLineType::None, HelpLineType::None) => true,
            _ => false,
        });
    }
}

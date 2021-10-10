use crate::model::area::{Area, AreaHelpLine, HelpLineType, MyTime, TimeType};

// 地域から窓口を取得する。
pub fn get_help_line(area: &Area) -> (HelpLineType, HelpLineType, HelpLineType) {
    (
        get_help_line_from(anai, area),
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

pub fn anai() -> Vec<AreaHelpLine> {
    vec![
        AreaHelpLine {
            pref: "北海道",
            cities: Vec::new(),
            hp: "北海道 救急医療・広域災害情報システム",
            url: "https://www.qq.pref.hokkaido.jp/qq/qq01.asp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-20-8699", "011-221-8699"],
        },
        AreaHelpLine {
            pref: "青森道",
            cities: Vec::new(),
            hp: "あおもり医療情報ネットワーク　青森県広域災害・救急医療情報システム",
            url: "https://www.qq.pref.aomori.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-733620"],
        },
        AreaHelpLine {
            pref: "岩手県",
            cities: Vec::new(),
            hp: "いわて医療ネット～岩手県医療機関検索サービス～",
            url: "http://www.med-info.pref.iwate.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["HPのみ"],
        },
        AreaHelpLine {
            pref: "宮城県",
            cities: Vec::new(),
            hp: "みやぎのお医者さんガイド",
            url: "http://medinf.mmic.or.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["HPのみ"],
        },
        AreaHelpLine {
            pref: "秋田県",
            cities: Vec::new(),
            hp: "あきた医療情報ガイド トップメニュー - 秋田県 災害・救急医療情報システム",
            url: "http://www.qq.pref.akita.lg.jp/qq05/WP0101/RP010101BL.do;jsessionid=1A22E05A8C7F68094742FC475C6B6628",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["HPのみ"],
        },
        AreaHelpLine {
            pref: "山形県",
            cities: Vec::new(),
            hp: "山形県医療機関情報ネットワーク",
            url: "https://www.pref.yamagata.jp/medical-net/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["HPのみ"],
        },
        AreaHelpLine {
            pref: "福島県",
            cities: Vec::new(),
            hp: "県民向けメニュー - ふくしま医療情報ネット",
            url: "http://www.ftmis.pref.fukushima.lg.jp/ap/qq/men/pwtpmenult01.aspx",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-963-990"],
        },
        AreaHelpLine {
            pref: "東京都",
            cities: Vec::new(),
            hp: "トップページ | 東京都医療機関・薬局案内サービス",
            url: "https://www.himawari.metro.tokyo.jp/qq13/qqport/tomintop/index.php",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "神奈川県",
            cities: Vec::new(),
            hp: "休日や夜間に急病になったとき｜相模原市",
            url: "https://www.city.sagamihara.kanagawa.jp/kurashi/kyubyo/1008444/index.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(17),
                w_to: MyTime(33),
                s_from: MyTime(13),
                s_to: MyTime(33),
                h_from: MyTime(9),
                h_to: MyTime(33),
            },
            phone: vec!["042-756-9000"],
        },
        AreaHelpLine {
            pref: "群馬県",
            cities: Vec::new(),
            hp: "群馬県統合型医療情報システム",
            url: "https://www.med.pref.gunma.jp/pb_md_telfax/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(リンク先を参照)"],
        },
        AreaHelpLine {
            pref: "山梨県",
            cities: Vec::new(),
            hp: "やまなし医療ネット - 山梨県救急医療情報センターの御案内",
            url: "https://www.yamanashi-iryo.net/qq19/QQ19TPMNLT/information.jsp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(リンク先を参照)"],
        },
        AreaHelpLine {
            pref: "長野県",
            cities: Vec::new(),
            hp: "長野県広域災害・救急医療情報システム「ながの医療情報Net」",
            url: "https://www.qq.pref.nagano.lg.jp/pb_top/pb_service_guide",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["050-3033-0665"],
        },
        AreaHelpLine {
            pref: "富山県",
            cities: Vec::new(),
            hp: "とやま医療情報ガイド",
            url: "https://www.qq.pref.toyama.jp/qq16/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "石川県",
            cities: Vec::new(),
            hp: "石川県医療・薬局機能情報提供システム",
            url: "http://i-search.pref.ishikawa.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "福井県",
            cities: Vec::new(),
            hp: "福井県広域災害・救急医療情報システム",
            url: "http://www.qq.pref.fukui.jp/qq18/qqport/kenmintop",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-987-199"],
        },
        AreaHelpLine {
            pref: "岐阜県",
            cities: Vec::new(),
            hp: "ぎふ救急ネット",
            url: "https://www.qq.pref.gifu.lg.jp/qq21/WP0101/RP010101BL",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "静岡県",
            cities: Vec::new(),
            hp: "医療ネットしずおか",
            url: "https://www.qq.pref.shizuoka.jp/qq22/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0800-222-1199"],
        },
        AreaHelpLine {
            pref: "京都府",
            cities: Vec::new(),
            hp: "府民向けTOPメニュー - 京都健康医療よろずネット",
            url: "http://www.mfis.pref.kyoto.lg.jp/ap/qq/men/pwtpmenult01.aspx",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["075-694-5499"],
        },
        AreaHelpLine {
            pref: "滋賀県",
            cities: Vec::new(),
            hp: "医療ネット滋賀",
            url: "http://www.shiga.iryo-navi.jp/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ?)"],
        },
        AreaHelpLine {
            pref: "大阪府",
            cities: Vec::new(),
            hp: "大阪府医療機関情報システム",
            url: "https://www.mfis.pref.osaka.jp/apqq/qq/men/pwtpmenult01.aspx",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["06-6693-1199"],
        },
        AreaHelpLine {
            pref: "奈良県",
            cities: Vec::new(),
            hp: "なら医療情報ネット",
            url: "https://www.qq.pref.nara.jp/qq29/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "鳥取県",
            cities: Vec::new(),
            hp: "TOP：とっとり医療情報ネット",
            url: "https://medinfo.pref.tottori.lg.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "岡山県",
            cities: Vec::new(),
            hp: "岡山県救急医療情報システム「おかやま医療情報ネット」",
            url: "https://www.qq.pref.okayama.jp/qq33/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "広島県",
            cities: Vec::new(),
            hp: "救急医療 Ｎｅｔ Ｈｉｒｏｓｈｉｍａ",
            url: "http://www.qq.pref.hiroshima.jp/qq34/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "山口県",
            cities: Vec::new(),
            hp: "やまぐち医療情報ネット",
            url: "https://www.qq.pref.yamaguchi.lg.jp/qq35/WP000/RP000001BL.do",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "愛媛県",
            cities: Vec::new(),
            hp: "えひめ医療情報ネット",
            url: "https://www.qq.pref.ehime.jp/qq38/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["0120-962-119"],
        },
        AreaHelpLine {
            pref: "高知県",
            cities: Vec::new(),
            hp: "高知県救急医療・広域災害情報システム「こうち医療ネット」",
            url: "https://www.kochi-iryo.net/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "福岡県",
            cities: Vec::new(),
            hp: "ふくおか医療情報ネット：福岡県の病院・診療所・歯科診療所・当番医の検索、医療情報の提供",
            url: "http://www.fmc.fukuoka.med.or.jp/qq/qq40gnmenult.asp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["092-415-3113"],
        },
        AreaHelpLine {
            pref: "佐賀県",
            cities: Vec::new(),
            hp: "佐賀県医療機関情報・救急医療情報システム「99さがネット」",
            url: "https://www.qq.pref.saga.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(大人の項目参照)"],
        },
        AreaHelpLine {
            pref: "長崎県",
            cities: Vec::new(),
            hp: "長崎県｜ながさき医療機関情報システム",
            url: "http://iryou.pref.nagasaki.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "熊本県",
            cities: Vec::new(),
            hp: "熊本県総合医療情報システム　くまもと医療ナビ Indexs",
            url: "http://mis.kumamoto.med.or.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "大分県",
            cities: Vec::new(),
            hp: "おおいた医療情報ほっとネット（病院・診療所・助産所・薬局案内）",
            url: "https://iryo-joho.pref.oita.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "宮崎県",
            cities: Vec::new(),
            hp: "宮崎県総合医療情報システム「みやざき医療ナビ」",
            url: "http://www.e-navi.pref.miyazaki.lg.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "鹿児島県",
            cities: Vec::new(),
            hp: "TOPページ | かごしま医療情報ネット",
            url: "http://iryo-info.pref.kagoshima.jp/qqport/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
        AreaHelpLine {
            pref: "沖縄県",
            cities: Vec::new(),
            hp: "沖縄県医療機関検索システム - Home",
            url: "http://imuutina.pref.okinawa.lg.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(HPのみ)"],
        },
    ]
}
pub fn adult() -> Vec<AreaHelpLine> {
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
            pref: "宮城県",
            cities: Vec::new(),
            hp: "おとな救急電話相談について - 宮城県公式ウェブサイト",
            url: "https://www.pref.miyagi.jp/soshiki/iryou/kyuukyuutel.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(14),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#7119", "022-706-7119"],
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
            phone: vec!["#8500", "023-633-0799"],
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
            hp: "東京消防庁(23区)",
            url: "https://www.tfd.metro.tokyo.lg.jp/lfe/kyuu-adv/soudan-center.htm",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "03-3212-2323"],
        },
        AreaHelpLine {
            pref: "東京都",
            cities: Vec::new(),
            hp: "東京消防庁(多摩地域)",
            url: "https://www.tfd.metro.tokyo.lg.jp/lfe/kyuu-adv/soudan-center.htm",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "042-521-2323"],
        },
        AreaHelpLine {
            pref: "神奈川都",
            cities: vec!["横浜市"],
            hp: "横浜市救急医療センター 救急電話相談",
            url: "https://www.yokohama-emc.jp/pc/syouni/syouni.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "045-232-7119"],
        },
        AreaHelpLine {
            pref: "神奈川都",
            cities: vec!["川崎市"],
            hp: "川崎市救急医療情報センター | 川崎市医師会",
            url: "http://www.kawasaki.kanagawa.med.or.jp/emergency/center",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["044-739-1919"],
        },
        AreaHelpLine {
            pref: "埼玉県",
            cities: Vec::new(),
            hp: "埼玉県救急医療情報システム",
            url: "https://99.pref.saitama.lg.jp/",
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
        AreaHelpLine {
            pref: "千葉県",
            cities: Vec::new(),
            hp: "ちば救急医療ネット",
            url: "http://www.qq.pref.chiba.lg.jp/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7009", " 03-6735-8305"],
        },
        AreaHelpLine {
            pref: "栃木県",
            cities: Vec::new(),
            hp: "栃木県／救急電話相談窓口について",
            url: "https://www.pref.tochigi.lg.jp/e02/welfare/iryou/kyuukyuu/kyukyudenwasoudan.html",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(22),
                s_from: MyTime(16),
                s_to: MyTime(22),
                h_from: MyTime(16),
                h_to: MyTime(22),
            },
            phone: vec!["#7111", "028-623-3344"],
        },
        AreaHelpLine {
            pref: "新潟県",
            cities: Vec::new(),
            hp: "新潟県ホームページ",
            url: "https://www.pref.niigata.lg.jp/sec/chiikiiryo/1356882439151.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "025－284－7119"],
        },
        AreaHelpLine {
            pref: "愛知県",
            cities: Vec::new(),
            hp: "救急医療情報センター電話番号案内 - あいち救急医療ガイド",
            url: "http://www.qq.pref.aichi.jp/es/qq/qq23tpdi_lt.asp",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(リンク先を参照)"],
        },
        AreaHelpLine {
            pref: "岐阜県",
            cities: Vec::new(),
            hp: "www.qq.pref.gifu.lg.jp - 岐阜医療情報ネット",
            url: "https://www.qq.pref.gifu.lg.jp/qq21/WP0003/RP000301BL",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(リンク先を参照)"],
        },
        AreaHelpLine {
            pref: "三重県",
            cities: Vec::new(),
            hp: "医療ネットみえ",
            url: "https://www.qq.pref.mie.lg.jp/qq24/qqport/kenmintop/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["059-229-1199"],
        },
        AreaHelpLine {
            pref: "京都府",
            cities: Vec::new(),
            hp: "救急安心センターきょうと（＃7119）／京都府ホームページ",
            url: "http://www.pref.kyoto.jp/iryo/7119.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "0570-00-7119"],
        },
        AreaHelpLine {
            pref: "大阪府",
            cities: Vec::new(),
            hp: "大阪市：突然の病気やケガで困ったら （…>相談・問合せ>相談窓口・電話番号一覧）",
            url: "https://www.city.osaka.lg.jp/shobo/page/0000052526.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "06-6582-7119"],
        },
        AreaHelpLine {
            pref: "兵庫県",
            cities: vec!["神戸市"],
            hp: "神戸市：電話で相談する",
            url: "https://www.city.kobe.lg.jp/a65055/bosai/kyukyuiryo/telephone.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "078-331-7119"],
        },
        AreaHelpLine {
            pref: "奈良県",
            cities: Vec::new(),
            hp: "奈良県救急安心センター（♯7119）ダイヤルについて_奈良県公式ホームページ",
            url: "https://www.pref.nara.jp/53886.htm",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "0744-20-0119"],
        },
        AreaHelpLine {
            pref: "和歌山県",
            cities: vec!["田辺市"],
            hp: "「救急受診ガイド」と「救急安心センターサービス」について｜田辺市",
            url: "http://www.city.tanabe.lg.jp/shoubo/gyouji/kinnkyuudohanntei.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "0739-22-0119"],
        },
        AreaHelpLine {
            pref: "和歌山県",
            cities: Vec::new(),
            hp: "トップページ - わかやま医療情報ネット",
            url: "https://www.wakayama.qq-net.jp/qq30/WP0101/RP010101BL.do",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["073-426-1199"],
        },
        AreaHelpLine {
            pref: "鳥取県",
            cities: Vec::new(),
            hp: "とっとりおとな救急ダイヤル（＃７１１９）_とりネット_鳥取県公式サイト",
            url: "https://www.pref.tottori.lg.jp/279398.htm",
            time: TimeType::WeekHoliday {
                w_from: MyTime(19),
                w_to: MyTime(32),
                s_from: MyTime(8),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#7111", "03-6667-3372"],
        },
        AreaHelpLine {
            pref: "広島県",
            cities: vec!["広島市", "呉市", "竹原市", "大竹市", "東広島市", "廿日市市", "安芸高田市", "江田島市", "府中町", "海田町", "熊野町", "坂町", "安芸太田町", "北広島町"],
            hp: "「救急相談センター広島広域都市圏」について - 広島市公式ホームページ",
            url: "https://www.city.hiroshima.lg.jp/site/holidaymedicalcare/14659.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "082-246-2000"],
        },
        AreaHelpLine {
            pref: "山口県",
            cities: vec!["岩国市", "和木町"],
            hp: "「救急相談センター広島広域都市圏」について - 広島市公式ホームページ",
            url: "https://www.city.hiroshima.lg.jp/site/holidaymedicalcare/14659.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "082-246-2000"],
        },
        AreaHelpLine {
            pref: "山口県",
            cities: vec!["下関市", "宇部市", "山口市", "防府市", "下松市", "光市", "長門市", "柳井市", "美祢市", "周南市", "山陽小野田市", "周防大島町", "上関町", "田布施町", "平生町"],
            hp: "#7119・救急医療電話相談について｜山口県",
            url: "https://www.pref.yamaguchi.lg.jp/cms/a11600/7119/201906200001.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "083-921-7119"],
        },
        AreaHelpLine {
            pref: "山口県",
            cities: vec!["萩市", "阿武町"],
            hp: "萩・阿武健康ダイヤル24（電話健康相談） - 萩市ホームページ",
            url: "https://www.city.hagi.lg.jp/soshiki/41/h33662.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec![ "0120-506-322"],
        },
        AreaHelpLine {
            pref: "徳島県",
            cities: Vec::new(),
            hp: "徳島救急医療電話相談（＃７１１９）について【令和元年12月1日相談受付開始】 | 安心とくしま",
            url: "https://anshin.pref.tokushima.jp/med/docs/2019111900060/",
            time: TimeType::WeekHoliday {
                w_from: MyTime(18),
                w_to: MyTime(32),
                s_from: MyTime(8),
                s_to: MyTime(32),
                h_from: MyTime(8),
                h_to: MyTime(32),
            },
            phone: vec!["#7111", "088-622-6530"],
        },
        AreaHelpLine {
            pref: "高知県",
            cities: Vec::new(),
            hp: "高知県救急医療・広域災害情報システム「こうち医療ネット」",
            url: "https://www.kochi-iryo.net/",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["088-825-1299"],
        },
        AreaHelpLine {
            pref: "福岡県",
            cities: Vec::new(),
            hp: "在宅患者救急時電話相談事業・医療機関案内",
            url: "http://www.fmc.fukuoka.med.or.jp/t7119/index.html",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["#7119", "092-471-0099"],
        },
        AreaHelpLine {
            pref: "佐賀県",
            cities: Vec::new(),
            hp: "佐賀県医療機関情報・救急医療情報システム「99さがネット」",
            url: "https://www.qq.pref.saga.jp/pb_top/top_medical_interrogating",
            time: TimeType::Allday {
                from: MyTime(0),
                to: MyTime(24),
            },
            phone: vec!["(リンク先参照)"],
        },
    ]
}
pub fn children() -> Vec<AreaHelpLine> {
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
            phone: vec!["#8000", "011-232-1599"],
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
    fn get_help_line_ok() {
        assert!(match get_help_line(&Area {
            pref: "北海道".to_string(),
            city: "札幌市".to_string()
        }) {
            (
                HelpLineType::InService(..),
                HelpLineType::InService(..),
                HelpLineType::InService(..),
            ) => true,
            _ => false,
        });
        assert!(match get_help_line(&Area {
            pref: "北海道".to_string(),
            city: "利尻町".to_string()
        }) {
            (HelpLineType::InService(..), HelpLineType::None, HelpLineType::InService(..)) => true,
            _ => false,
        });
    }
    #[test]
    fn get_help_line_ng() {
        assert!(match get_help_line(&Area {
            pref: "東京府".to_string(),
            city: "東京市".to_string()
        }) {
            (HelpLineType::None, HelpLineType::None, HelpLineType::None) => true,
            _ => false,
        });
    }
}

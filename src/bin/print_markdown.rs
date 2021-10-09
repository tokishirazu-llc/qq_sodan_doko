#![deny(warnings)]

use qq_sodan_doko::model::area::{AreaHelpLine, TimeType};
use qq_sodan_doko::repository::area::{adult, anai, children};

fn main() {
    /*
    vec![
        "北海道",
        "青森県",
        "岩手県",
        "宮城県",
        "秋田県",
        "山形県",
        "福島県",
        "茨城県",
        "栃木県",
        "群馬県",
        "埼玉県",
        "千葉県",
        "東京都",
        "神奈川県",
        "新潟県",
        "富山県",
        "石川県",
        "福井県",
        "山梨県",
        "長野県",
        "岐阜県",
        "静岡県",
        "愛知県",
        "三重県",
        "滋賀県",
        "京都府",
        "大阪府",
        "兵庫県",
        "奈良県",
        "和歌山県",
        "鳥取県",
        "島根県",
        "岡山県",
        "広島県",
        "山口県",
        "徳島県",
        "香川県",
        "愛媛県",
        "高知県",
        "福岡県",
        "佐賀県",
        "長崎県",
        "熊本県",
        "大分県",
        "宮崎県",
        "鹿児島県",
        "沖縄県",
    ].into_iter(|pref| {
        print!("{}");
        anai().filter(|area| pref == area.pref)
        print!("\b");
    })
    */

    print_markdown("医療機関を探す", anai);
    print_markdown("大人", adult);
    print_markdown("小児", children);
}

fn print_markdown(name: &str, data: fn() -> Vec<AreaHelpLine>) {
    println!("{}\n", name);
    println!("都道府県|市区町村|WEB|平日|土曜|日祝|電話");
    println!("---|---|---|---|---|---|---");
    data().into_iter().for_each(|help_line| {
        let (wf, wt, sf, st, hf, ht) = match help_line.time {
            TimeType::Allday { from, to } => (
                from.clone(),
                to.clone(),
                from.clone(),
                to.clone(),
                from.clone(),
                to.clone(),
            ),
            TimeType::WeekHoliday {
                w_from,
                w_to,
                s_from,
                s_to,
                h_from,
                h_to,
            } => (w_from, w_to, s_from, s_to, h_from, h_to),
        };
        println!(
            "{}|{}|[{}]({})|{}|{}|{}|{}",
            help_line.pref,
            help_line.cities.join("<br />"),
            "LINK",
            help_line.url,
            if wf.0 == 0 && wt.0 == 24 {
                String::from("24時間")
            } else {
                format!("{}から{}", wf, wt)
            },
            if sf.0 == 0 && st.0 == 24 {
                String::from("24時間")
            } else {
                format!("{}から{}", sf, st)
            },
            if hf.0 == 0 && ht.0 == 24 {
                String::from("24時間")
            } else {
                format!("{}から{}", hf, ht)
            },
            help_line.phone.join("<br />"),
        );
    })
}

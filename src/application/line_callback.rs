use log::error;

extern crate line_bot_sdk_rust as line;
use line::bot::LineBot;
use line::events::messages::MessageType;
use line::events::{Event, EventType};
use line::messages::{SendMessageType, TextMessage};

use crate::model::area::{get_help_line, Area, HelpLineType};
use crate::service::google_map_api::get_address_from_latlng;

pub fn main(bot: &LineBot, event: Event) {
    match event.r#type {
        EventType::FollowEvent(_follow) => {}
        EventType::PostBackEvent(_action) => {}
        EventType::MessageEvent(message) => match message.message.r#type {
            MessageType::LocationMessage(location) => {
                bot.reply_message(
                    &message.reply_token,
                    match get_address_from_latlng(location.latitude, location.longitude) {
                        Ok(area) => {
                            let (adult, children) = get_help_line_message(area);
                            vec![
                                SendMessageType::TextMessage(TextMessage {
                                    text: adult,
                                    emojis: None,
                                }),
                                SendMessageType::TextMessage(TextMessage {
                                    text: children,
                                    emojis: None,
                                }),
                            ]
                        }
                        Err(err) => {
                            error!("{}", err);
                            vec![SendMessageType::TextMessage(TextMessage {
                                text: format!("位置情報から住所を取得できませんでした"),
                                emojis: None,
                            })]
                        }
                    },
                )
                .unwrap();
            }
            _ => {
                bot.reply_message(
                    &message.reply_token,
                    vec![SendMessageType::TextMessage(TextMessage {
                        text: "リッチメニューから現在地を送信してください".to_string(),
                        emojis: None,
                    })],
                )
                .unwrap();
            }
        },
        _ => {}
    }
}

// 都道府県と市区町村から対応窓口のテキストを生成する
fn get_help_line_message(area: Area) -> (String, String) {
    let (adult, children) = get_help_line(&area);
    (
        format!(
            "{}{}は以下の窓口情報は以下の通りです。\n\n大人\n{}",
            area.pref,
            area.city,
            match adult {
                HelpLineType::InService(help_line) => {
                    format!("今相談できるようです。\n\n{}", help_line)
                }
                HelpLineType::UnknownTime(help_line) => {
                    format!("曜日によって対応時間が異なります。\n\n{}", help_line)
                }
                HelpLineType::OutOfTime(help_line) => {
                    format!("残念ながら時間外かもしれません\n\n{}", help_line)
                }
                HelpLineType::None => {
                    format!("残念ながら相談窓口はないようです。")
                }
            },
        ),
        format!(
            "小児\n{}",
            match children {
                HelpLineType::InService(help_line) => {
                    format!("今相談できるようです。\n\n{}", help_line)
                }
                HelpLineType::UnknownTime(help_line) => {
                    format!("曜日によって対応時間が異なります。\n\n{}", help_line)
                }
                HelpLineType::OutOfTime(help_line) => {
                    format!("残念ながら時間外かもしれません\n\n{}", help_line)
                }
                HelpLineType::None => {
                    format!("残念ながら相談窓口はないようです。")
                }
            },
        ),
    )
}

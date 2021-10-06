use log::error;

extern crate line_bot_sdk_rust as line;
use line::bot::LineBot;
use line::events::messages::MessageType;
use line::events::{Event, EventType};
use line::messages::{SendMessageType, TextMessage};

use crate::model::area::{get_help_line, HelpLineType};
use crate::service::google_map_api::get_address_from_latlng;

pub fn main(bot: &LineBot, event: Event) {
    match event.r#type {
        EventType::FollowEvent(_follow) => {}
        EventType::PostBackEvent(_action) => {}
        EventType::MessageEvent(message) => match message.message.r#type {
            MessageType::LocationMessage(location) => {
                bot.reply_message(
                    &message.reply_token,
                    vec![SendMessageType::TextMessage(TextMessage {
                        text: match get_address_from_latlng(location.latitude, location.longitude) {
                            Ok(area) => {
                                match get_help_line(&area) {
                                    HelpLineType::InService(help_line) => {
                                        format!("{}{}は以下の窓口に相談できるようです。\n\n{}", area.pref, area.city, help_line)
                                    },
                                    HelpLineType::UnknownTime(help_line) => {
                                        format!("{}{}は以下の窓口に相談できるようですが、曜日や時間によってはやっていないかもしれません\n\n{}", area.pref, area.city, help_line)
                                    },
                                    HelpLineType::OutOfTime(help_line) => {
                                        format!("{}{}は以下の窓口に相談できるようですが、残念ながら時間外かもしれません\n\n{}", area.pref, area.city, help_line)
                                    },
                                    HelpLineType::None => {
                                        format!("残念ながら{}{}に相談窓口はないようです。", area.pref, area.city)
                                    }
                                }
                            }
                            Err(err) => {
                                error!("{}", err);
                                format!("位置情報から住所を取得できませんでした")
                            }
                        },
                        emojis: None,
                    })],
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

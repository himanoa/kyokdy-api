use super::model::DraftSong;
use crate::domain::comment::{
    token::{Token, TokenKind},
    tokenize::tokenize_from_string,
};
use crate::domain::video::model::VideoId;

use anyhow::{anyhow, Result};

///
///
/// ```
/// use anyhow::Result;
/// use kyokdy_api::domain::song::parser::{parse_timestamp};
///
/// assert_eq!(parse_timestamp("1:00".to_string()).unwrap(), 60);
/// assert_eq!(parse_timestamp("1:01".to_string()).unwrap(), 61);
/// assert_eq!(parse_timestamp("1:11".to_string()).unwrap(), 71);
/// assert_eq!(parse_timestamp("2:11".to_string()).unwrap(), 131);
/// assert_eq!(parse_timestamp("2:11:01".to_string()).unwrap(), 7200 + 660 + 1);
/// assert_eq!(parse_timestamp("foo".to_string()).is_err(), true);
/// assert_eq!(parse_timestamp("11".to_string()).is_err(), true);
/// ```
///
///
pub fn parse_timestamp(s: String) -> Result<i64> {
    let timestamp = s.split(|c| c == ':').collect::<Vec<&str>>();

    match timestamp.len() {
        2 => {
            let minites: i64 = timestamp.get(0).unwrap().to_string().parse()?;
            let second: i64 = timestamp.get(1).unwrap().to_string().parse()?;

            Ok(minites * 60 + second)
        }
        3 => {
            let hour: i64 = timestamp.get(0).unwrap().to_string().parse()?;
            let minutes: i64 = timestamp.get(1).unwrap().to_string().parse()?;
            let second: i64 = timestamp.get(2).unwrap().to_string().parse()?;

            Ok(hour * 60 * 60 + minutes * 60 + second)
        }
        _ => Err(anyhow!("datetime parse failed.{:?}", s)),
    }
}

pub fn parse_song(tokens: Vec<Token>, video_id: &VideoId) -> Result<DraftSong> {
    let mut title: Vec<String> = vec![];
    let mut start_timestamp = None;
    let mut end_timestamp = None;
    let mut processed_tokens: Vec<Token> = vec![];

    for t in tokens.clone() {
        match t.kind {
            TokenKind::End => {
                break;
            }
            TokenKind::Timestamp if start_timestamp.is_none() => start_timestamp = Some(t.clone()),
            TokenKind::Timestamp if start_timestamp.is_some() => end_timestamp = Some(t.clone()),
            TokenKind::Separator if t.value == " " || t.value == "　" => {
                if let Some(l) = processed_tokens.last() {
                    if l.kind == TokenKind::Character {
                        title.push(t.value.clone());
                    }
                }
            }
            TokenKind::Separator => {
                println!("{:?}", t.value.clone());
            }
            TokenKind::Character => {
                title.push(t.value.clone());
            }
            _ => {
                break;
            }
        }
        processed_tokens.push(t)
    }

    let t = title.join("").trim().to_string();

    match (start_timestamp, end_timestamp, t.as_str()) {
        (_, _, "") => Err(anyhow!("Title is not found. {:?}", tokens)),
        (None, _, _) => Err(anyhow!("Start timestamp is not found. {:?}", tokens)),
        (Some(st), Some(et), _) => Ok(DraftSong {
            title: t,
            start_timestamp: parse_timestamp(st.value.clone())?,
            end_timestamp: Some(parse_timestamp(et.value.clone())?),
            video_id: video_id.clone(),
        }),
        (Some(st), None, _) => Ok(DraftSong {
            title: t,
            start_timestamp: parse_timestamp(st.value)?,
            end_timestamp: None,
            video_id: video_id.clone(),
        }),
    }
}

pub fn parse_songs(raw: String, video_id: VideoId) -> Vec<DraftSong> {
    tokenize_from_string(raw)
        .split(|t| {
            *t == Token {
                kind: TokenKind::End,
                value: "".to_string(),
            }
        })
        .flat_map(|ts| parse_song(ts.to_vec(), &video_id))
        .collect::<Vec<DraftSong>>()
}
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn parse_song_1() {
        let actual = parse_song(
            tokenize_from_string("開始 2:05~3:05\n".to_string()),
            &VideoId("foo".to_string()),
        )
        .unwrap();
        assert_eq!(
            actual,
            DraftSong {
                video_id: VideoId("foo".to_string()),
                title: "開始".to_string(),
                start_timestamp: 125,
                end_timestamp: Some(185)
            }
        )
    }

    #[test]
    fn parse_song_2() {
        let actual = parse_song(
            tokenize_from_string("開始 2:05\n".to_string()),
            &VideoId("foo".to_string()),
        )
        .unwrap();
        assert_eq!(
            actual,
            DraftSong {
                video_id: VideoId("foo".to_string()),
                title: "開始".to_string(),
                start_timestamp: 125,
                end_timestamp: None
            }
        )
    }

    #[test]
    fn parse_song_3() {
        let actual = parse_song(
            tokenize_from_string("2:05\n開始".to_string()),
            &VideoId("foo".to_string()),
        )
        .unwrap();
        assert_eq!(
            actual,
            DraftSong {
                video_id: VideoId("foo".to_string()),
                title: "開始".to_string(),
                start_timestamp: 125,
                end_timestamp: None
            }
        )
    }

    #[test]
    fn parse_song_4() {
        let actual = parse_song(
            tokenize_from_string("2:05 3:05\n開始".to_string()),
            &VideoId("foo".to_string()),
        )
        .unwrap();
        assert_eq!(
            actual,
            DraftSong {
                video_id: VideoId("foo".to_string()),
                title: "開始".to_string(),
                start_timestamp: 125,
                end_timestamp: Some(185)
            }
        )
    }

    #[test]
    fn parse_song_5() {
        let actual = parse_song(
            tokenize_from_string("2:05 \n開始".to_string()),
            &VideoId("foo".to_string()),
        )
        .unwrap();
        assert_eq!(
            actual,
            DraftSong {
                video_id: VideoId("foo".to_string()),
                title: "開始".to_string(),
                start_timestamp: 125,
                end_timestamp: None
            }
        )
    }
    #[test]
    fn parse_songs_1() {
        let video_id = VideoId("foo".to_string());
        let actual = parse_songs(
            "magnet 2:05\nBad Apple! 3:05 4:05\n8:05\n foo ".to_string(),
            video_id.clone(),
        );

        assert_eq!(
            actual,
            vec![
                DraftSong {
                    video_id: video_id.clone(),
                    title: "magnet".to_string(),
                    start_timestamp: 125,
                    end_timestamp: None
                },
                DraftSong {
                    video_id: video_id.clone(),
                    title: "Bad Apple!".to_string(),
                    start_timestamp: 185,
                    end_timestamp: Some(245)
                },
                DraftSong {
                    video_id: video_id.clone(),
                    title: "foo".to_string(),
                    start_timestamp: 485,
                    end_timestamp: None
                }
            ]
        )
    }
}

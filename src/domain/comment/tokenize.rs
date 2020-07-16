use super::token::{Token, TokenKind};
use std::iter::FromIterator;
use std::iter::Iterator;

const TS_SEPERATOR: &str = " 　~～";
struct Tokenizer {
    input: Vec<char>,
    current_position: usize,
}

pub fn tokenize_from_string(s: String) -> Vec<Token> {
    let mut tokenizer = Tokenizer::new(s.chars().collect::<Vec<char>>());
    let mut tokens = vec![];
    while let Some(t) = tokenizer.tokenize(&tokens) {
        tokens.push(t);
    }
    tokens
}

impl Tokenizer {
    fn new(input: Vec<char>) -> Self {
        Tokenizer {
            input,
            current_position: 0,
        }
    }

    fn tokenize(&mut self, acc: &Vec<Token>) -> Option<Token> {
        let mut ts = vec![*self.current()?];
        let mut token = None;

        if self.current()?.is_ascii_digit() {
            while let Some(i) = self.peak() {
                if i.is_numeric() || *i == ':' {
                    ts.push(*i);
                } else {
                    self.back();
                    break;
                }
            }
            token = String::from_iter(ts).parse::<String>().ok().and_then(|n| {
                Some(Token {
                    kind: TokenKind::Timestamp,
                    value: n,
                })
            })
        } else if TS_SEPERATOR
            .chars()
            .collect::<Vec<_>>()
            .contains(self.current()?)
        {
            token = Some(Token {
                kind: TokenKind::Separator,
                value: self.current()?.to_string(),
            })
        } else if *self.current()? == '\n' {
            if acc
                .iter()
                .rev()
                .take_while(|t| t.kind != TokenKind::End)
                .filter(|t| t.kind == TokenKind::Character)
                .collect::<Vec<&Token>>()
                .len()
                >= 1
            {
                token = Some(Token {
                    kind: TokenKind::End,
                    value: "".to_string(),
                });
            } else {
                token = Some(Token {
                    kind: TokenKind::Separator,
                    value: self.current()?.to_string(),
                })
            }
        } else {
            token = Some(Token {
                kind: TokenKind::Character,
                value: self.current()?.to_string(),
            });
        }

        self.next();
        return token;
    }

    fn next(&mut self) {
        self.current_position += 1;
    }

    fn back(&mut self) {
        self.current_position -= 1;
    }

    fn current(&mut self) -> Option<&char> {
        self.input.get(self.current_position)
    }

    fn peak(&mut self) -> Option<&char> {
        self.next();
        self.input.get(self.current_position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // とりあえずの最終目標は↓をきっちり分解できること
    // let PARSE_TARGET = "お歌配信乙のあ！\nどれもカッコよくて好きな曲じゃ✨\n千本桜を歌ってくれてありがとう‼️\n\n開始 2:05\nETERNAL BLAZE 4:51\nこのピアノでお前を8759632145回ぶん殴る 13:44\n吉原ラメント 21:11\n紅蓮華 28:15\nヨンジュウナナ 34:53\n千本桜 42:05\n↑リクエスト";

    #[test]
    fn test_tokenize_1() {
        assert_eq!(
            tokenize_from_string("開始 2:05\n".to_string()),
            vec![
                Token {
                    kind: TokenKind::Character,
                    value: "開".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "始".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "2:05".to_string()
                },
                Token {
                    kind: TokenKind::End,
                    value: "".to_string()
                },
            ]
        )
    }

    #[test]
    fn test_tokenize_2() {
        assert_eq!(
            tokenize_from_string("開始 2:05~3:05\n".to_string()),
            vec![
                Token {
                    kind: TokenKind::Character,
                    value: "開".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "始".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "2:05".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: "~".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "3:05".to_string()
                },
                Token {
                    kind: TokenKind::End,
                    value: "".to_string()
                },
            ]
        )
    }

    #[test]
    fn test_tokenize_3() {
        // 通った！
        assert_eq!(
            tokenize_from_string("2:05~3:05\n開始\n".to_string()),
            vec![
                Token {
                    kind: TokenKind::Timestamp,
                    value: "2:05".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: "~".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "3:05".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: "\n".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "開".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "始".to_string()
                },
                Token {
                    kind: TokenKind::End,
                    value: "".to_string()
                },
            ]
        )
    }

    #[test]
    fn test_tokenize_4() {
        assert_eq!(
            tokenize_from_string("Bad Apple! 3:05 4:05\n8:05\n foo".to_string()),
            vec![
                Token {
                    kind: TokenKind::Character,
                    value: "B".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "a".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "d".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "A".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "p".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "p".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "l".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "e".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "!".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "3:05".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "4:05".to_string()
                },
                Token {
                    kind: TokenKind::End,
                    value: "".to_string()
                },
                Token {
                    kind: TokenKind::Timestamp,
                    value: "8:05".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: "\n".to_string()
                },
                Token {
                    kind: TokenKind::Separator,
                    value: " ".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "f".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "o".to_string()
                },
                Token {
                    kind: TokenKind::Character,
                    value: "o".to_string()
                },
            ]
        )
    }
}

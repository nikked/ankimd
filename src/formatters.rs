use comrak::{markdown_to_html, ComrakOptions};
use regex::Regex;
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::error::AnkiCsvError;

pub fn format_front(
    front: &str,
    light_mode: bool,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> Result<String, AnkiCsvError> {
    // Skip chars '## '
    format_side(&front[3..], light_mode, &syntax_set, theme_set)
}

pub fn format_back(
    back: &str,
    light_mode: bool,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> Result<String, AnkiCsvError> {
    format_side(back, light_mode, &syntax_set, theme_set)
}

fn format_side(
    input_text: &str,
    light_mode: bool,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> Result<String, AnkiCsvError> {
    let mut result = "".to_string();

    let card_fragments: Vec<CardFragment> = split_input_to_card_fragments(input_text)?;

    for fragment in card_fragments {
        // Code blocks are converted to HTML with syntect (syntax highlighting
        if fragment.is_codeblock {
            result += &make_html_codeblock_str_with_syntect(
                &fragment.text,
                &fragment.codeblock_lang,
                light_mode,
                syntax_set,
                theme_set,
            );
        } else {
            // Markdown blocks are converted to HTML with comrak
            result += &markdown_to_html_with_comrak(&fragment.text)
        }
    }

    Ok(result)
}

#[derive(Debug)]
struct CardFragment {
    text: String,
    codeblock_lang: String,
    is_codeblock: bool,
}

const INVALID_CODEBLOCK: &str = "At2Y6mSB";

fn split_input_to_card_fragments(input: &str) -> Result<Vec<CardFragment>, AnkiCsvError> {
    let mut result = Vec::new();

    let mut is_codeblock = false;
    for codeblock_split in Regex::new(r"\n```|^```").unwrap().split(input) {
        let mut text = codeblock_split;

        // Check if code type is defined with code block
        // E.g.: ```rust, `python, ```markdown
        let mut codeblock_lang = "";
        if is_codeblock {
            codeblock_lang = match codeblock_split.split("\n").next() {
                Some(x) => x,
                _ => "",
            };

            // Remove first line of codeblock that includes code lang
            // E.g.: ```rust, `python, ```markdown
            let mut splitter = codeblock_split.splitn(2, '\n');
            let _ = splitter.next().unwrap_or(INVALID_CODEBLOCK);
            text = splitter.next().unwrap_or(INVALID_CODEBLOCK);

            if text == INVALID_CODEBLOCK {
                let err_message = "Invalid codeblock: ".to_owned() + codeblock_split;
                return Err(AnkiCsvError::AnkimdError(err_message));
            }
        }

        result.push(CardFragment {
            text: text.to_string(),
            is_codeblock: is_codeblock,
            codeblock_lang: codeblock_lang.to_string(),
        });

        // After regex split, every other fragment is a code block
        is_codeblock = !is_codeblock;
    }

    Ok(result)
}

fn markdown_to_html_with_comrak(input_markdown: &str) -> String {
    let mut html_string: String = markdown_to_html(&input_markdown, &ComrakOptions::default());

    html_string = str::replace(&html_string, "<pre", "<pre align=left ");
    html_string = str::replace(&html_string, "<ul", "<ul align=left ");
    html_string = str::replace(&html_string, "<ol", "<ol align=left ");

    html_string
}

fn make_html_codeblock_str_with_syntect(
    codeblock_str: &str,
    codeblock_lang: &str,
    light_mode: bool,
    syntax_set: &SyntaxSet,
    theme_set: &ThemeSet,
) -> String {
    let syntax = match syntax_set.find_syntax_by_token(codeblock_lang) {
        Some(x) => x,
        None => syntax_set.find_syntax_by_token("markdown").unwrap(),
    };

    let str_as_html = highlighted_html_for_string(
        codeblock_str,
        &syntax_set,
        syntax,
        &theme_set.themes["base16-eighties.dark"],
    );

    let align_to_left = str::replace(&str_as_html, "<pre", "<pre align=left ");
    align_to_left
}

#[cfg(test)]
mod test_split_input_to_card_fragments {
    use super::*;
    use failure::Error;

    #[test]
    fn test_rust_codeblock_found() -> Result<(), Error> {
        let input_str = "this is some text\n```rust\nlet x;\n```";
        let card_fragments = split_input_to_card_fragments(input_str)?;

        assert_eq!(card_fragments.len(), 3);

        assert_eq!(card_fragments[0].is_codeblock, false);
        assert_eq!(card_fragments[0].text, "this is some text");

        assert_eq!(card_fragments[1].is_codeblock, true);
        assert_eq!(card_fragments[1].text, "let x;");
        assert_eq!(card_fragments[1].codeblock_lang, "rust");

        assert_eq!(card_fragments[2].is_codeblock, false);
        assert_eq!(card_fragments[2].text, "");
        Ok(())
    }

    #[test]
    fn test_anon_codeblock_found() -> Result<(), Error> {
        let input_str = "this is some text\n```\nlet x;\n```";
        let card_fragments = split_input_to_card_fragments(input_str)?;

        assert_eq!(card_fragments.len(), 3);

        assert_eq!(card_fragments[0].is_codeblock, false);
        assert_eq!(card_fragments[0].text, "this is some text");

        assert_eq!(card_fragments[1].is_codeblock, true);
        assert_eq!(card_fragments[1].text, "let x;");
        assert_eq!(card_fragments[1].codeblock_lang, "");

        assert_eq!(card_fragments[2].is_codeblock, false);
        assert_eq!(card_fragments[2].text, "");
        Ok(())
    }

    #[test]
    fn test_raises_error_if_malformed_codeblock() -> Result<(), Error> {
        let input_str = "```let x;\n```";

        let s = match split_input_to_card_fragments(input_str) {
            Ok(_v) => "OK".to_string(),
            Err(e) => e.to_string(),
        };

        assert_eq!(s, "Error: \"Invalid codeblock: let x;\"");
        Ok(())
    }

    #[test]
    fn test_no_splits_done_if_no_code() -> Result<(), Error> {
        let input_str = "no codeblocks\njust a multiline string";
        let card_fragments = split_input_to_card_fragments(input_str)?;

        assert_eq!(card_fragments.len(), 1);

        assert_eq!(card_fragments[0].is_codeblock, false);
        assert_eq!(
            card_fragments[0].text,
            "no codeblocks\njust a multiline string"
        );

        Ok(())
    }
}

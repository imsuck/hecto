use std::cmp;

use crossterm::style::Stylize;
use unicode_segmentation::UnicodeSegmentation;

use crate::{highlighting, HighlightingOptions, SearchDirection};

#[derive(Default)]
pub struct Row {
    string: String,
    highlighting: Vec<highlighting::Type>,
    pub is_highlighted: bool,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: slice.to_owned(),
            highlighting: Vec::new(),
            is_highlighted: false,
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        let mut result = String::new();

        #[allow(clippy::integer_arithmetic)]
        for (index, grapheme) in self
            .string
            .graphemes(true)
            .enumerate()
            .skip(start)
            .take(end - start)
        {
            if let Some(c) = grapheme.chars().next() {
                let highlighting_type = self
                    .highlighting
                    .get(index)
                    .unwrap_or(&highlighting::Type::None);

                let highlight = format!("{}", c.with(highlighting_type.to_color()));
                result.push_str(&highlight);

                if c == '\t' {
                    result.push(' ');
                }
            }
        }

        result
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn insert(&mut self, at: usize, c: char) {
        if at >= self.len() {
            self.string.push(c);
            self.len += 1;
            return;
        }

        let mut result = String::new();
        let mut len = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            len += 1;

            if index == at {
                len += 1;
                result.push(c);
            }

            result.push_str(grapheme);
        }

        self.len = len;
        self.string = result;
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: usize) {
        if at >= self.len() {
            return;
        }

        let mut result = String::new();
        let mut len = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index != at {
                len += 1;
                result.push_str(grapheme);
            }
        }

        self.len = len;
        self.string = result;
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn append(&mut self, new: &Self) {
        self.string = format!("{}{}", self.string, new.string);
        self.len += new.len;
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn split(&mut self, at: usize) -> Self {
        let mut row = String::new();
        let mut len = 0;
        let mut splitted_row = String::new();
        let mut splitted_length = 0;

        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index < at {
                len += 1;
                row.push_str(grapheme);
            } else {
                splitted_length += 1;
                splitted_row.push_str(grapheme);
            }
        }

        self.string = row;
        self.len = len;
        self.is_highlighted = false;

        Self {
            string: splitted_row,
            highlighting: Vec::new(),
            is_highlighted: false,
            len: splitted_length,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.string.as_bytes()
    }

    pub fn find(&self, query: &str, at: usize, direction: SearchDirection) -> Option<usize> {
        if at > self.len || query.is_empty() {
            return None;
        }

        let start = if direction == SearchDirection::Forward {
            at
        } else {
            0
        };
        let end = if direction == SearchDirection::Forward {
            self.len
        } else {
            at
        };

        #[allow(clippy::integer_arithmetic)]
        let substring: String = self
            .string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect();

        let matching_byte_index = if direction == SearchDirection::Forward {
            substring.find(query)
        } else {
            substring.rfind(query)
        };

        if let Some(matching_byte_index) = matching_byte_index {
            for (grapheme_index, (byte_index, _)) in substring.grapheme_indices(true).enumerate() {
                if matching_byte_index == byte_index {
                    #[allow(clippy::integer_arithmetic)]
                    return Some(start + grapheme_index);
                }
            }
        }

        None
    }

    fn highlight_match(&mut self, word: &Option<String>) {
        if let Some(word) = word {
            if word.is_empty() {
                return;
            }

            let mut index = 0;
            while let Some(search_match) = self.find(word, index, SearchDirection::Forward) {
                if let Some(next_index) = search_match.checked_add(word.graphemes(true).count()) {
                    #[allow(clippy::indexing_slicing)]
                    for i in search_match..next_index {
                        self.highlighting[i] = highlighting::Type::Match;
                    }
                    index = next_index;
                } else {
                    break;
                }
            }
        }
    }

    #[allow(clippy::integer_arithmetic)]
    fn highlight_str(
        &mut self,
        index: &mut usize,
        substring: &str,
        chars: &[char],
        hl_type: highlighting::Type,
    ) -> bool {
        if substring.is_empty() {
            return false;
        }

        for (substring_index, c) in substring.chars().enumerate() {
            if let Some(next_char) = chars.get(index.saturating_add(substring_index)) {
                if *next_char != c {
                    return false;
                }
            } else {
                return false;
            }
        }

        for _ in 0..substring.len() {
            self.highlighting.push(hl_type);
            *index += 1;
        }

        true
    }

    fn highlight_keywords(
        &mut self,
        index: &mut usize,
        chars: &[char],
        keywords: &[String],
        hl_type: highlighting::Type,
    ) -> bool {
        if *index > 0 {
            #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
            let prev_char = chars[*index - 1];
            if !is_separator(prev_char) {
                return false;
            }
        }

        for word in keywords {
            if *index < chars.len().saturating_sub(word.len()) {
                #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
                let next_char = chars[*index + word.len()];

                if !is_separator(next_char) {
                    continue;
                }
            }

            if self.highlight_str(index, word, chars, hl_type) {
                return true;
            }
        }
        false
    }

    fn highlight_primary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.primary_keywords(),
            highlighting::Type::PrimaryKeywords,
        )
    }

    fn highlight_secondary_keywords(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        chars: &[char],
    ) -> bool {
        self.highlight_keywords(
            index,
            chars,
            opts.secondary_keywords(),
            highlighting::Type::SecondaryKeywords,
        )
    }

    #[allow(clippy::integer_arithmetic)]
    fn highlight_char(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.chars() && c == '\'' {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                let closing_index = if *next_char == '\\' {
                    index.saturating_add(3)
                } else {
                    index.saturating_add(2)
                };

                if let Some(closing_char) = chars.get(closing_index) {
                    if *closing_char == '\'' {
                        for _ in *index..=closing_index {
                            self.highlighting.push(highlighting::Type::Char);
                            *index += 1;
                        }
                        return true;
                    }
                }
            }
        }

        false
    }

    #[allow(clippy::integer_arithmetic)]
    fn highlight_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '/' {
                    for _ in *index..chars.len() {
                        self.highlighting.push(highlighting::Type::Comment);
                        *index += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    #[allow(
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::integer_arithmetic
    )]
    fn highlight_multiline_comment(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.comments() && c == '/' && *index < chars.len() {
            if let Some(next_char) = chars.get(index.saturating_add(1)) {
                if *next_char == '*' {
                    let closing_index =
                        if let Some(closing_index) = self.string[*index + 2..].find("*/") {
                            *index + closing_index + 4
                        } else {
                            chars.len()
                        };
                    for _ in *index..closing_index {
                        self.highlighting.push(highlighting::Type::MultilineComment);
                        *index += 1;
                    }

                    return true;
                }
            }
        }

        false
    }

    #[allow(clippy::integer_arithmetic)]
    fn highlight_string(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        // TODO: Extend for <lib> in C/C++
        if opts.strings() && c == '"' {
            loop {
                self.highlighting.push(highlighting::Type::String);
                *index += 1;

                if let Some(next_char) = chars.get(*index) {
                    if *next_char == '"' {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.highlighting.push(highlighting::Type::String);
            *index += 1;

            return true;
        }

        false
    }

    #[allow(clippy::integer_arithmetic)]
    fn highlight_number(
        &mut self,
        index: &mut usize,
        opts: &HighlightingOptions,
        c: char,
        chars: &[char],
    ) -> bool {
        if opts.numbers() && c.is_ascii_digit() {
            if *index > 0 {
                #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
                let prev_char = chars[*index - 1];
                if !is_separator(prev_char) {
                    return false;
                }
            }

            loop {
                self.highlighting.push(highlighting::Type::Number);
                *index += 1;
                if let Some(next_char) = chars.get(*index) {
                    if *next_char != '.' && !next_char.is_ascii_digit() {
                        break;
                    }
                } else {
                    break;
                }
            }
            return true;
        }

        false
    }

    #[allow(
        clippy::indexing_slicing,
        clippy::string_slice,
        clippy::integer_arithmetic
    )]
    pub fn highlight(
        &mut self,
        opts: &HighlightingOptions,
        word: &Option<String>,
        start_with_comment: bool,
    ) -> bool {
        let chars: Vec<char> = self.string.chars().collect();
        let mut index = 0;
        let mut in_ml_comment = start_with_comment;

        if self.is_highlighted && word.is_none() {
            if let Some(hl_type) = self.highlighting.last() {
                if *hl_type == highlighting::Type::MultilineComment
                    && self.string.len() > 1
                    && self.string[self.string.len() - 2..] == *"*/"
                {
                    return true;
                }
            }
            return false;
        }

        self.highlighting = Vec::new();

        if in_ml_comment {
            let closing_index = if let Some(closing_index) = self.string.find("*/") {
                closing_index + 2
            } else {
                chars.len()
            };

            for _ in 0..closing_index {
                self.highlighting.push(highlighting::Type::MultilineComment);
            }

            index = closing_index;
        }

        while let Some(c) = chars.get(index) {
            if self.highlight_multiline_comment(&mut index, opts, *c, &chars) {
                in_ml_comment = true;
                continue;
            }
            in_ml_comment = false;

            if self.highlight_char(&mut index, opts, *c, &chars)
                || self.highlight_comment(&mut index, opts, *c, &chars)
                || self.highlight_primary_keywords(&mut index, opts, &chars)
                || self.highlight_secondary_keywords(&mut index, opts, &chars)
                || self.highlight_string(&mut index, opts, *c, &chars)
                || self.highlight_number(&mut index, opts, *c, &chars)
            {
                continue;
            }
            self.highlighting.push(highlighting::Type::None);
            index += 1;
        }

        self.highlight_match(word);
        if in_ml_comment && &self.string[self.string.len().saturating_sub(2)..] != "*/" {
            return true;
        }

        self.is_highlighted = true;
        false
    }
}

fn is_separator(c: char) -> bool {
    c.is_ascii_punctuation() || c.is_ascii_whitespace()
}

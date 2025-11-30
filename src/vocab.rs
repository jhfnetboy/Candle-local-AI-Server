/// 词汇表模块 - Kokoro TTS
///
/// 字符 → Token ID 映射

use lazy_static::lazy_static;
use std::collections::HashMap;

fn build_vocab() -> HashMap<char, usize> {
    let pad = "$";
    let punctuation = r#";:,.!?¡¿—…"«»"" "#;
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let letters_ipa = "ɑɐɒæɓʙβɔɕçɗɖðʤəɘɚɛɜɝɞɟʄɡɠɢʛɦɧħɥʜɨɪʝɭɬɫɮʟɱɯɰŋɳɲɴøɵɸθœɶʘɹɺɾɻʀʁɽʂʃʈʧʉʊʋⱱʌɣɤʍχʎʏʑʐʒʔʡʕʢǀǁǂǃˈˌːˑʼʴʰʱʲʷˠˤ˞↓↑→↗↘'̩'ᵻ";

    let symbols: String = [pad, punctuation, letters, letters_ipa].concat();

    symbols
        .chars()
        .enumerate()
        .map(|(idx, c)| (c, idx))
        .collect()
}

lazy_static! {
    pub static ref VOCAB: HashMap<char, usize> = build_vocab();
}

/// 将音素字符串转换为 token IDs
pub fn tokenize(phonemes: &str) -> Vec<i64> {
    phonemes
        .chars()
        .filter_map(|c| VOCAB.get(&c))
        .map(|&idx| idx as i64)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vocab_size() {
        assert!(VOCAB.len() > 100);
        assert!(VOCAB.contains_key(&'$')); // pad token
        assert!(VOCAB.contains_key(&' ')); // space
        assert!(VOCAB.contains_key(&'ə')); // IPA
    }

    #[test]
    fn test_tokenize() {
        let phonemes = "həlˈoʊ";
        let tokens = tokenize(phonemes);
        assert!(tokens.len() > 0);
        assert!(tokens.iter().all(|&t| t >= 0));
    }
}

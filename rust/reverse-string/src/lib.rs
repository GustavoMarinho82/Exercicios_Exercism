use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    //chars() -> divide cada caracter da string e bota tudo em um iterador
    //rev() -> inverte a direcao do iterador
    //collect::<T> -> converte o iterador para o tipo T

    /* Codigo sem desafio extra
    input.chars().rev().collect::<String>()
    */

    //graphemes() -> basicamente, um chars() que implementa corretamente um agrupamento de grafemas (grapheme cluster)
    input.graphemes(true).rev().collect::<String>()
}

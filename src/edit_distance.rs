const MAX_STRING_LEN: usize = 256;

/// Calculate the Levenshtein distance between two strings.
pub fn levenshtein_distance(word: &str, word2: &str) -> usize {
    if word.is_empty() {
        return word2.len();
    }
    if word2.is_empty() {
        return word.len();
    }
    if word == word2 {
        return 0;
    }
    let m = word.len();
    let n = word2.len();

    let mut matrix = [[0; MAX_STRING_LEN + 1]; MAX_STRING_LEN + 1];

    (0..m).for_each(|i| {
        matrix[i][0] = i;
    });

    (0..n).for_each(|j| {
        matrix[0][j] = j;
    });

    let mut insertion: usize = 0;
    let mut deletion: usize = 0;
    let mut replacement: usize = 0;

    (1..=m).for_each(|i| {
        (1..=n).for_each(|j| {
            if word.chars().nth(i - 1).unwrap_or(' ') == word2.chars().nth(j - 1).unwrap_or(' ') {
                matrix[i][j] = matrix[i - 1][j - 1];
            } else {
                insertion = matrix[i][j - 1];
                deletion = matrix[i - 1][j];
                replacement = matrix[i - 1][j - 1];
                matrix[i][j] = 1 + insertion.min(deletion).min(replacement);
            }
        });
    });

    matrix[m][n]
}



#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_verify_no_overlapping_matches {
  ($count:expr, $peeked_val:ident) => {
    let count_matches = $count;
    if count_matches > 1 {
      return Err(
        ::parser_generator::error::ParserError::overlapping_token_matchers(format!(
          "{:?}",
          $peeked_val
        )),
      );
    }
  };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_verify_no_overlapping_matches {
  ($count:expr, $peeked_val:ident) => {};
}

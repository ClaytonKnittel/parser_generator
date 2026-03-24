use std::{borrow::Borrow, fmt::Debug, hash::Hash};

use crate::{
  error::LRTableResult,
  grammar::{Grammar, ProductionNode},
  indexed_grammar::{IndexedGrammar, IndexedProductionNode},
  lr_table::{Action, LRTable},
  vocabulary::AugmentedVocabToken,
};

pub struct Parser<T, L> {
  grammar: IndexedGrammar<T, L>,
  lr_table: LRTable<T>,
}

impl<T: Clone + Eq + Hash, L: Clone + Debug + Eq + Hash> Parser<T, L> {
  pub fn new(grammar: &Grammar<T, L>) -> LRTableResult<Self> {
    let grammar = IndexedGrammar::build(grammar)?;
    let lr_table = LRTable::build(&grammar)?;
    Ok(Self { grammar, lr_table })
  }
}

impl<T: Clone + Eq + Hash, L> Parser<T, L> {
  pub fn parse_stream<U: Borrow<T>>(&self, stream: impl IntoIterator<Item = U>) -> bool
  where
    T: Debug + ToString,
  {
    let mut states = vec![self.lr_table.root_state()];
    let mut nodes = Vec::<IndexedProductionNode>::new();
    let mut stream = stream.into_iter().peekable();

    while let Some(&state) = states.last() {
      let token = match stream.peek().map(|token| token.borrow().clone()) {
        Some(token) => token.into(),
        None => AugmentedVocabToken::EndOfStream,
      };

      println!("Stack: {:?}", states);
      println!("nodes: {:?}", nodes);
      println!("Token {:?}", token);

      let Ok(token) = self.grammar.vocab().augmented_token_to_id(&token) else {
        return false;
      };

      let Some(action) = self.lr_table.get_action(state, token) else {
        println!("No action found");
        return false;
      };

      match action {
        Action::Shift { next_state } => {
          println!("Shift {:?}", next_state);
          states.push(next_state);
          nodes.push(ProductionNode::Terminal(token));
          stream.next();
        }
        Action::Reduce { rule } => {
          println!("Reduce {:?}", rule);
          let grammar_rule = self.grammar.production_rule(rule);
          debug_assert!(grammar_rule.rules_excluding_epsilon().count() <= nodes.len());
          debug_assert!(
            grammar_rule
              .rules_excluding_epsilon()
              .zip(
                nodes
                  .iter()
                  .rev()
                  .take(grammar_rule.rules_excluding_epsilon().count())
                  .rev()
              )
              .all(|(grammar_node, stack_node)| { grammar_node == stack_node })
          );
          for _ in grammar_rule.rules_excluding_epsilon() {
            states.pop();
            nodes.pop();
          }
          debug_assert!(!states.is_empty());

          let label = *grammar_rule.symbol();
          nodes.push(ProductionNode::Production(label));

          match self.lr_table.get_goto(*states.last().unwrap(), label) {
            Some(goto) => {
              println!("Goto {:?}", goto);
              states.push(goto.state());
            }
            None => return false,
          }
        }
        Action::Accept => {
          println!("Accept!");
          return true;
        }
      }
    }

    // This is unreachable
    debug_assert!(false);
    false
  }
}

#[cfg(test)]
mod tests {
  use googletest::prelude::*;

  use crate::{grammar::Grammar, parse_simulator::Parser};

  #[gtest]
  fn test_trivial() {
    let grammar = Grammar::from_grammar_str(r#"S -> a"#).unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_false!(parser.parse_stream(b""));
    expect_true!(parser.parse_stream(b"a"));
    expect_false!(parser.parse_stream(b"aa"));
    expect_false!(parser.parse_stream(b"b"));
    expect_false!(parser.parse_stream(b"ba"));
  }

  #[gtest]
  fn test_nested_trivial() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_false!(parser.parse_stream(b""));
    expect_true!(parser.parse_stream(b"a"));
    expect_false!(parser.parse_stream(b"aa"));
    expect_false!(parser.parse_stream(b"b"));
    expect_false!(parser.parse_stream(b"ba"));
  }

  #[gtest]
  fn test_a_or_b() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a
         A -> b"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_false!(parser.parse_stream(b""));
    expect_true!(parser.parse_stream(b"a"));
    expect_false!(parser.parse_stream(b"aa"));
    expect_true!(parser.parse_stream(b"b"));
    expect_false!(parser.parse_stream(b"ba"));
  }

  #[gtest]
  fn test_even() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> A a a
         A -> !"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_true!(parser.parse_stream(b""));
    expect_false!(parser.parse_stream(b"a"));
    expect_true!(parser.parse_stream(b"aa"));
    expect_false!(parser.parse_stream(b"aaa"));
    expect_true!(parser.parse_stream(b"aaaa"));
    expect_false!(parser.parse_stream(b"aaaaa"));
  }

  #[gtest]
  fn test_equal_a_and_b() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a A b
         A -> !"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_true!(parser.parse_stream(b""));
    expect_true!(parser.parse_stream(b"ab"));
    expect_true!(parser.parse_stream(b"aabb"));
    expect_true!(parser.parse_stream(b"aaabbb"));
    expect_false!(parser.parse_stream(b"ba"));
    expect_false!(parser.parse_stream(b"a"));
    expect_false!(parser.parse_stream(b"b"));
    expect_false!(parser.parse_stream(b"bab"));
    expect_false!(parser.parse_stream(b"aba"));
    expect_false!(parser.parse_stream(b"abab"));
    expect_false!(parser.parse_stream(b"baba"));
  }

  #[gtest]
  fn test_sum_product() {
    let grammar = Grammar::from_grammar_str(
      r#"T -> S
         S -> S p P
         S -> P
         P -> P x V
         P -> V
         V -> a
         V -> b
         V -> c"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_true!(parser.parse_stream(b"a"));
    expect_true!(parser.parse_stream(b"apb"));
    expect_true!(parser.parse_stream(b"cxa"));
    expect_true!(parser.parse_stream(b"apbxc"));
    expect_true!(parser.parse_stream(b"apbpcpapbpc"));
    expect_true!(parser.parse_stream(b"axbxcxaxbxc"));
    expect_true!(parser.parse_stream(b"axbpcxaxbpc"));

    expect_false!(parser.parse_stream(b""));
    expect_false!(parser.parse_stream(b"ap"));
    expect_false!(parser.parse_stream(b"pb"));
    expect_false!(parser.parse_stream(b"cx"));
    expect_false!(parser.parse_stream(b"xa"));
    expect_false!(parser.parse_stream(b"ab"));
    expect_false!(parser.parse_stream(b"apbx"));
    expect_false!(parser.parse_stream(b"xapb"));
  }

  #[gtest]
  fn test_multiple_ways_to_resolve_same_rule() {
    let grammar = Grammar::from_grammar_str(
      r#"S -> A
         A -> a C
         A -> B
         B -> C
         B -> x b
         C -> x c"#,
    )
    .unwrap();
    let parser = Parser::new(&grammar).unwrap();

    expect_true!(parser.parse_stream(b"axc"));
    expect_true!(parser.parse_stream(b"xb"));
    expect_true!(parser.parse_stream(b"xc"));

    expect_false!(parser.parse_stream(b""));
    expect_false!(parser.parse_stream(b"axb"));
    expect_false!(parser.parse_stream(b"a"));
    expect_false!(parser.parse_stream(b"b"));
    expect_false!(parser.parse_stream(b"c"));
    expect_false!(parser.parse_stream(b"ax"));
    expect_false!(parser.parse_stream(b"ab"));
    expect_false!(parser.parse_stream(b"ac"));
  }
}

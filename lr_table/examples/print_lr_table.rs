use lr_table::{grammar::Grammar, indexed_grammar::IndexedGrammar, lr_table::LRTable};

fn main() {
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
  let indexed_grammar = IndexedGrammar::build(&grammar).unwrap();
  let lr_table = LRTable::build(&indexed_grammar).unwrap();

  println!("{lr_table}");
}

#![macro_use]
use middle::analysis::ast::*;
use ast::Expression::*;

pub trait ModuloSignedExt {
    fn modulo(&self, n: Self) -> Self;
}
macro_rules! modulo_signed_ext_impl {
    ($($t:ty)*) => ($(
        impl ModuloSignedExt for $t {
            #[inline]
            fn modulo(&self, n: Self) -> Self {
                (self % n + n) % n
            }
        }
    )*)
}
modulo_signed_ext_impl! { i8 i16 i32 i64 }

pub struct UselessChaining<'a: 'c, 'b: 'a, 'c>
{
  grammar: &'c AGrammar<'a, 'b>
}


impl <'a, 'b, 'c> UselessChaining<'a, 'b, 'c>
{
  pub fn analyse(grammar: AGrammar<'a, 'b>) -> Partial<AGrammar<'a, 'b>> {
      let mut cpt_not_not = 0;
      let mut cpt_and_and = 0;
      let mut cpt_not_and = 0;
      let mut cpt_and_not = 0;
      let mut cpt_oom_oom = 0;
      let mut cpt_oom_zom = 0;

      for expr in &grammar.exprs {

          match expr {
              &StrLiteral(_) => println!("\nStrLiteral"),
              &AnySingleChar => println!("AnySingleChar"),
              &CharacterClass(_) => println!("CharacterClass"),
              &NonTerminalSymbol(_) => println!("NonTerminalSymbol -> à évaluer en terminal symbol"),
              &Sequence(_) => println!("Sequence"),
              &Choice(_) => println!("Choice"),
              &ZeroOrMore(_) => println!("ZeroOrMore"),
              &OneOrMore(_) => println!("OneOrMore"),
              &ZeroOrOne(_) => println!("ZeroOrOne"),
              &NotPredicate(_) => println!("NotPredicate"),
              &AndPredicate(_) => println!("AndPredicate"),
              &SemanticAction(_, _) => println!("SemanticAction"),
              &TypeAscription(_, _) => println!("TypeAscription"),
              &SpannedExpr(_) => println!("SpannedExpr"),
          }

          match expr {
              &AndPredicate(_) => {
                  if cpt_and_and+1>=2 {
                      grammar.warn(format!("Detected useless chaining: &(&e) \nHelp: &(&e) ~ &e"));
                      cpt_and_and=0;
                  }
                  if cpt_not_and+1>=2 {
                      grammar.warn(format!("Detected useless chaining: !(&e) \nHelp: !(&e) ~ !e"));
                  }
                  cpt_and_and+=1;
                  cpt_and_not+=1;
                  cpt_not_not = 0;
                  cpt_not_and = 0;
                  cpt_oom_oom = 0;
                  cpt_oom_zom = 0;
              }
              &NotPredicate(_) => {
                  let mut new_not_cpt = cpt_not_not;
                  new_not_cpt+=1;


                  if (new_not_cpt.modulo(3))==0 {
                      grammar.warn(format!("Detected useless chaining : !(!(!(e)) \nHelp : !(!(!(e))) ~ !(e)"));
                     // cpt_not_not=1;
                  }



                  if (new_not_cpt.modulo(2))==0 {
                      grammar.warn(format!("Detected useless chaining: !(!e) \nHelp: !(!e) ~ &e"));
                      //cpt_not_not=0;
                  }

                  if cpt_and_not+1>=2 {
                      grammar.warn(format!("Detected useless chaining: &(!e) \nHelp: &(!e) ~ !e"));
                  }

                  cpt_not_not+=1;
                  cpt_not_and+=1;
                  cpt_and_and = 0;
                  cpt_and_not = 0;
                  cpt_oom_oom = 0;
                  cpt_oom_zom = 0;
              }
              &OneOrMore(_) => {
                  if cpt_oom_oom+1>=2 {
                      grammar.warn(format!("Detected useless chaining: (e+)+ \nHelp: (e+)+ ~ e+"));
                      cpt_oom_oom=0;
                  }
                  cpt_oom_oom+=1;
                  cpt_oom_zom+=1;
                  cpt_not_not = 0;
                  cpt_and_and = 0;
                  cpt_not_and = 0;
                  cpt_and_not = 0;
              }
              &ZeroOrMore(_) => {
                  if cpt_oom_zom+1>=2 {
                      grammar.warn(format!("Detected useless chaining: (e+)* \nHelp: (e+)* ~ e+"));
                  }
                  cpt_not_not = 0;
                  cpt_and_and = 0;
                  cpt_not_and = 0;
                  cpt_and_not = 0;
                  cpt_oom_oom = 0;
                  cpt_oom_zom = 0;
              }
              _ => {
                  cpt_not_not = 0;
                  cpt_and_and = 0;
                  cpt_not_and = 0;
                  cpt_and_not = 0;
                  cpt_oom_oom = 0;
                  cpt_oom_zom = 0;
              }
          }
      }
      Partial::Nothing
  }
}

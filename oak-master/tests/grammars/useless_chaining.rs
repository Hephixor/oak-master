// Copyright 2016 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! The types of the rules of this grammar must be valid (Bug #75).

pub use self::useless_chaining::*;

grammar! useless_chaining {

    #![show_api]

  test1 = !(!"a") // &"a" OK
  test2 = &(&"a") // &"a" OK
  test3 = !(&"a") // !"a" OK
  test4 = &(!"a") // !"a" OK
  //test5 = ("a"*)* // infinite loop -> deja detectee OK
  test6 = ("a"+)+ // "a"+ OK
  test7 = ("a"+)* // "a"+ OK
  //test8 = ("a"*)+ // infinite loop -> deja detectee OK
  //test9 = (!"a")* // infinite loop -> deja detectee OK
  test10 = !(!(!"a")) // !"a" -> warning pour signaler une optimisation possible en '!'
  //Besoin de reconnaissance de ce type de cas (probleme d'implementation actuelle : le cas 4 est inateignable si la règle du cas 2 est attrapee avant (et que le compteur est remis a zero)
  //Semble ne pas fonctionner avec modulo
  //Comme on ne va pas faire un compteur different pour chaque cas ..
  //Limitation de la déclaration des opérateurs de règle en 1 préfix / 1 suffixe ou récursion gauche et optimisation des opérateurs
  test11 = !(!(!(!"a")))

}

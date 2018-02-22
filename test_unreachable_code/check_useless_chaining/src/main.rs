#![feature(plugin)]
#![plugin(oak)]

extern crate oak_runtime;

grammar! useless_chaining {
    #![show_api]

    test1 = !(!"a") // &"a"
    test2 = &(&"a") // &"a"
    test3 = !(&"a") // !"a"
    test4 = &(!"a") // !"a"
    //test5 = ("a"*)* // infinite loop -> deja detectee
    test6 = ("a"+)+ // "a"+
    test7 = ("a"+)* // "a"+
    //test8 = ("a"*)+ // infinite loop -> deja detectee
    //test9 = (!"a")* // infinite loop -> deja detectee
    test10 = !(!(!"a")) // !"a" -> warning pour signaler une optimisation possible en '!'
    //Besoin de reconnaissance de ce type de cas (probleme d'implementation actuelle : le cas 4 est inateignable si la règle du cas 2 est attrapee avant (et que le compteur est remis a zero)
    //Semble ne pas fonctionner avec modulo
    //Comme on ne va pas faire un compteur different pour chaque cas ..
    //Limitation de la déclaration des opérateurs de règle en 1 préfix / 1 suffixe ou récursion gauche et optimisation des opérateurs
    test11 = !(!(!(!"a")))

}

fn main() {
    println!("Testing !(!\"a\") ok");
    println!("Testing &(&\"a\") ok");
    println!("Testing !(&\"a\") ok");
    println!("Testing &(!\"a\") ok");
    println!("Testing (\"a\"+)+ ok");
    println!("Testing (\"a\"+)* ok");
    println!("Testing !(!(!\"a\")) ok");
    println!("Testing !(!(!(!\"a\"))) fail");

}

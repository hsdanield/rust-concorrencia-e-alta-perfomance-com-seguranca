use chrono::prelude::*;
use chrono::Local;

fn print_today() {
    println!("\n#print_today");
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é: {}-{}-{}", year, month, day)
}

//Ponteiro para função - Parametro como função (na verdade é o ponteiro)
fn run_function(function: fn()) {
    function()
}

//Tipos de Dados em Rust
fn char_type() {
    println!("\n#char_type");

    let a: char = '\u{2764}';
    let b: char = '9';
    let c: char = '0';
    println!("{} é um dígito? {}", a, a.is_digit(10));
    println!("{} é um dígito? {}", b, b.is_digit(10));
    println!("{} é binário? {}", b, b.is_digit(2));
    println!("{} é um dígito? {}", c, c.is_digit(10));
    println!("{} é binário? {}", c, c.is_digit(2));
}

fn get_unicode_in_char(c: char) -> String {
    println!("\n#get_unicode_in_char");

    let repr: String = c.escape_unicode().collect();
    repr
}

fn char_is_alphabetic(c: char) -> bool {
    println!("\n#char_is_alphabetic");

    let check: bool = c.is_alphabetic();
    check
}

fn char_case(c: char) {
    println!("\n#char_case");

    println!("{} é Maiúsculo? -> {}", c, c.is_uppercase());
    println!("{} é Minúsculo? -> {}", c, c.is_lowercase());
    println!("{} é Espaço em branco? -> {}", c, c.is_whitespace());
    println!("{} é Alfanumérico? -> {}", c, c.is_alphanumeric());
    println!("{} é Numérico? -> {}", c, c.is_numeric());
}

fn print_bool(x: bool) {
    println!("\n#print_bool");

    if x {
        println!("X é true!");
    } else {
        println!("X é false!")
    }
}

fn bool_operators() {
    println!("\n#bool_operators");

    let x = true;
    let y = false;

    println!("true AND/&& false é {}", x && y);
    println!("true OR/|| false é {}", x || y);
    println!("not/! true é {}", !x);
}

fn int_declarations_max_value() {
    println!("\n#int_declarations_max_value");
    println!("i8 = {} a {}", i8::min_value(), i8::max_value());
    println!("i16= {} a {}", i16::min_value(), i16::max_value());
    println!("i32= {} a {}", i32::min_value(), i32::max_value());
    println!("i64= {} a {}", i64::min_value(), i64::max_value());
    println!("u8 = {} a {}", u8::min_value(), u8::max_value());
    println!("u16= {} a {}", u16::min_value(), u16::max_value());
    println!("u32= {} a {}", u16::min_value(), u32::max_value());
    println!("u64= {} a {}", u64::min_value(), u64::max_value());
}

fn count_one_and_zero(x: i8) {
    println!("\n#count_one");

    println!("O numero {} em binário é {:08b}", x, x);

    println!("Uns: {}", x.count_ones());
    println!("Zeros: {}", x.count_zeros());
}

fn float_methods_default(a: f32) {
    println!("\n#float_methods_default");

    println!("Numero: {}", a);
    println!("Piso: {}", a.floor());
    println!("Teto: {}", a.ceil());
    println!("Arredondado: {}", a.round());
    println!("Truncado: {}", a.trunc());
    println!("Fracionário: {}", a.fract());
    println!("É finito?: {}", a.is_finite());
    println!("É infinito?: {}", a.is_infinite());
    println!("É NaN?: {}", a.is_nan());
}

fn example_arrays() {
    println!("\n#example_arrays");

    let a: [i32; 5] = [0; 5];
    let b: [i32; 5] = [0, 0, 0, 0, 0];
    let mut c = [""; 3];

    println!("elementos de a {:?}", a);
    println!("elementos de b {:?}", b);

    println!("elementos de c {:?} (antes de alterar valores)", c);

    c[0] = "Item 0";
    c[1] = "Item 1";
    c[2] = "Item 2";
    println!("elementos de c {:?} (depos de alterar os valores)", c);

    let d: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
    let d_a: &[u8] = &d[..];
    let d_b: &[u8] = &d[3..5];

    println!("elementos de d {:?}", d);
    println!(
        "elementos de d_a (slice de todos os elementos de d): {:?} ",
        d_a
    );
    println!("elementos de d_b (slice dos elementos [3..5]): {:?}", d_b);

    println!("a tem {} elementos", a.len());
    println!("b tem {} elementos", b.len());
    println!("c tem {} elementos", c.len());
    println!("d_a tem {} elementos", d_a.len());
    println!("d_b tem {} elementos", d_b.len());

    let array: [char; 4] = ['a', 'b', 'c', 'd'];
    println!("Percorrendo o vetor: {:?}", array);
    for i in array {
        print!("i={} ", i);
    }
}

fn example_tuples() {
    println!("\n#example_tuples");

    let a: (char, &str) = ('a', "char");
    let b: (i32, &str) = (32, "integer");
    let c: (&str, &str) = ("Nome", "Daniel");
    let d: (&str, i32, char) = ("Ru", 5, 't');

    println!("a(completo): {:?} | a.0={}, a.1={}", a, a.0, a.1);
    println!("b(completo): {:?} | b.0={}, b.1={}", b, b.0, b.1);
    println!("c(completo): {:?} | c.0={}, c.1={}", c, c.0, c.1);
    println!(
        "d(completo): {:?} | d.0={}, d.1={}, d.2={}",
        d, d.0, d.1, d.2
    );

    println!("let destrutivo");
    let e: (&str, i32, char) = ("Ru", 5, 't');
    let (e0, e1, e2) = e;
    println!(
        "Expressão:
    let e: (&str, i32, char) = (Ru, 5, t);
    let (e0, e1, e2) = e;
    "
    );
    println!("e0={}, e1={}, e2={}", e0, e1, e2);

    println!("Array de Tuplas");
    let at: (i32, &str) = (1, "um");
    let bt: (i32, &str) = (2, "dois");
    let ct: (i32, &str) = (3, "tres");
    let arr_tp: [(i32, &str); 3] = [at, bt, ct];
    println!("{:?}", arr_tp)
}

fn enum_examples() {
    println!("\n#enum_examples");
    // Com o #[allow(dead_code)] ,
    // dizemos ao compilador: "Ei, Rust, tudo bem deixar esse código, que nunca
    // será usado, jogado aí".
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Race {
        Dwarf,
        Elf,
        Ent,
        Hobbit,
        Men,
    }
    // Com o #[allow(dead_code)] ,
    // dizemos ao compilador: "Ei, Rust, tudo bem deixar esse código, que nunca
    // será usado, jogado aí".
    #[allow(dead_code)]
    #[derive(Debug)]
    struct Creature {
        name: &'static str,
        gender: Race,
    }

    let elrond: Creature = Creature {
        name: "Elrond",
        gender: Race::Elf,
    };

    let treebeard: Creature = Creature {
        name: "Treebeard",
        gender: Race::Ent,
    };

    println!("{:#?}", elrond);
    println!("{:#?}", treebeard);
}

mod compute {

    // private function
    fn is_zero(number: i32) -> bool {
        if number == 0 {
            return true;
        };
        false
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }

    pub fn divide(a: i32, b: i32) -> i32 {
        if is_zero(b) {
            return 0;
        }
        a / b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }
}

mod example_if {
    // Com o #[allow(dead_code)] ,
    // dizemos ao compilador: "Ei, Rust, tudo bem deixar esse código, que nunca
    // será usado, jogado aí".
    #[allow(dead_code)]
    pub fn ex() {
        println!("\n#example_if.ex");

        let x = if 10 + 5 == 15 {
            "10 + 5 é 15"
        } else {
            "10 + 5 não é 15"
        };

        println!(
            "\
        let x = if 10 + 5 == 15 {{
            10 + 5 é 15
        }} else {{
            10 + 5 não é 15
        }};
    
    
        RESULTADO = {}",
            x
        );
    }

    // Retorno () é equivalente a retornar void
    pub fn check_grade(grade: f32) -> () {
        println!("\n#example_if.check_grade");

        println!("grade={}", grade);
        if grade >= 0.0 && grade <= 4.9 {
            println!("Não aprovado");
        } else if grade >= 4.9 && grade <= 6.0 {
            println!("De exame");
        } else if grade >= 6.0 && grade <= 10.0 {
            println!("Aprovado");
        } else {
            println!("Nota invalida");
        }
    }
}

mod example_match {

    pub fn check_grade(grade: f32) -> () {
        println!("\n#example_match.check_grade");
        match grade {
            0.0..=4.8 => println!("Não aprovado"),
            4.9..=5.9 => println!("De exame"),
            5.9..=10.0 => println!("Aprovado"),
            _ => println!("Nota invalida"),
        }
    }

    pub fn check_number(number: i16) -> () {
        println!("\n#example_match.check_number");

        println!("Numero: {}", number);
        match number {
            2 | 3 | 5 | 7 | 11 => println!("Primo"),
            _ => println!("Qualquer numero"),
        }
    }
}

mod enum_and_match {
    // Com o #[allow(dead_code)] ,
    // dizemos ao compilador: "Ei, Rust, tudo bem deixar esse código, que nunca
    // será usado, jogado aí".
    #[allow(dead_code)]
    pub enum Gender {
        Male,
        Female,
        Other,
    }

    pub fn check_gender(gender: Gender) -> () {
        println!("#enum_and_match.check_gender");

        match gender {
            Gender::Other => println!("Outro"),
            Gender::Male => println!("Homem"),
            Gender::Female => println!("Mulher"),
        }
    }
}

mod tuple_and_match {

    pub fn check_tuple(t: (i32, i32)) -> () {
        println!("\n#tuple_and_match.check_tuple");

        println!("Tuple: {:?}", t);
        match t {
            (x, 0) => println!("O Segundo Elemento é Zero"),
            (0, x) => println!("O Primeiro Elemento é Zero"),
            _ => println!("Nenhum zero"),
        }
    }
}

mod match_vinculation {
    pub fn is_vowel_or_consonant(c: char) -> char {
        println!("\n#match_vinculation.is_vowel_or_consonant");

        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => 'v',
            'A' | 'E' | 'I' | 'O' | 'U' => 'v',
            _ => 'c',
        }
    }
}

mod loops {
    pub fn loop_while(n: i32) {
        println!("#loop_while");

        let mut a = 0;
        while a < n {
            println!("a={}", a);
            a += 1;
        }
    }

    pub fn loop_loop(n: i32) {
        println!("#loop_loop");

        let mut a = 0;
        loop {
            if a == n - 1 {
                break;
            }

            a += 1;
            println!("a={}", a);
        }
    }

    pub fn loop_range(n: i32) {
        println!("#loop_range");


        println!("\nRange 1..5 (exclusivo, quando utilizamos ..)");
        println!("Repeticoes = {}", n);

        for i in 1..5 {
            println!("i={}", i);
        }

        println!("\nRange 1..=5 (inclusivo, quando utilizamos ..=)");
        println!("Repeticoes = {}", n);

        for i in 1..=5 {
            println!("i={}", i);
        }
    }
}

fn main() {
    let function: fn() = print_today; //aponta para função print_today
    run_function(function);

    let function2: fn() = char_type;
    run_function(function2);

    let a: char = '→';
    let unicode = get_unicode_in_char(a);
    println!("Codigo unicode: {}", unicode);

    println!("'a' é alphabetic ? {}", char_is_alphabetic('a'));
    println!("'→' é alphabetic ? {}", char_is_alphabetic('→'));

    let a: char = 'a';
    char_case(a);

    let x: bool = true;
    let y: bool = false;
    print_bool(x);
    print_bool(y);

    bool_operators();

    int_declarations_max_value();

    let a: i8 = 1;
    println!("a={}", a);
    count_one_and_zero(a);

    let a: f32 = 3.549236;
    float_methods_default(a);

    example_arrays();

    example_tuples();

    enum_examples();

    println!("\n#mod compute");
    let a: i32 = 10;
    let b: i32 = 4;
    println!("{} + {} = {}", a, b, compute::add(a, b));
    println!("{} - {} = {}", a, b, compute::subtract(a, b));
    println!("{} * {} = {}", a, b, compute::multiply(a, b));
    println!("{} / {} = {}", a, b, compute::divide(a, b));

    let grade_a = 0.0;
    let grade_b = 3.3;
    let grade_c = 5.5;
    let grade_d = 8.8;
    let grade_e = 10.10;

    println!("\nmod example if");
    example_if::check_grade(grade_a);
    example_if::check_grade(grade_b);
    example_if::check_grade(grade_c);
    example_if::check_grade(grade_d);
    example_if::check_grade(grade_e);

    println!("\nmod example match");
    example_match::check_grade(grade_a);
    example_match::check_grade(grade_b);
    example_match::check_grade(grade_c);
    example_match::check_grade(grade_d);
    example_match::check_grade(grade_e);

    let (number_a, number_b, number_c) = (0, 3, 8);
    example_match::check_number(number_a);
    example_match::check_number(number_b);
    example_match::check_number(number_c);

    println!("\nmod enum_and_match");
    let gender = enum_and_match::Gender::Male;
    enum_and_match::check_gender(gender);

    println!("\nmod tuple_and_match");
    tuple_and_match::check_tuple((0, 10));
    tuple_and_match::check_tuple((33, 0));
    tuple_and_match::check_tuple((8, 12));

    println!("\nmod match_vinculation");
    let name: &str = "Daniel";
    println!("name={}", name);
    for a in name.chars() {
        match match_vinculation::is_vowel_or_consonant(a) {
            r @ 'v' => println!("{}", r),
            r @ 'c' => println!("{}", r),
            _ => (),
        }
    }

    println!("\n mod loops");
    loops::loop_loop(5);
    loops::loop_while(5);
    loops::loop_range(5);
}


//############################CONCLUSÃO##################################333
//Neste capítulo, abordamos mais conceitualmente os tipos de
// dados básicos disponíveis no crate std do Rust, bem como outras
// características da linguagem, como comentários e módulos. Além
// disso, vimos os operadores condicionais e de repetição do Rust.
// Eles são a base do nosso sistema de decisão em fluxos de
// execução de um código Rust. Alguns deles são velhos conhecidos
// nossos, como o if , mas também nos deparamos com o poderoso
// match . No próximo capítulo, vamos nos aprofundar no conceito
// de traits.

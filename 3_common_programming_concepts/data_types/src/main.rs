fn main() {
    let integer: i32 = -42;
    println!(
        "{} is a 32-bit integer.  The \"i\" stands for \"integer\"  Negatives are welcome :-) \n",
        integer
    );

    let unsigned_integer: u32 = 42;
    println!(
        "{} is an unsigned integer. The \"u\" stands for \"unsigned integer\" -- meaning that there is no sign/symbol to indicate that it is negative.  Keep things positive! \n",
        unsigned_integer
    );

    let type_declared_in_integer_literal = 420u32;
    println!(
        "{} has its type declared in the literal expression.  Kind of how double-quotes around characters declare the type to be a string.  Cool! \n",
        type_declared_in_integer_literal
    );

    let uses_underscores_for_readability: u32 = 4_200;

    println!(
        "{} uses the underscore for readability.  It would be handwritten as \"4,200\", but we can't ust commas; underscore fills in nicely. \n",
        uses_underscores_for_readability
    )
}

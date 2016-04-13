
enum Answer {
    Higher,
    Lower,
    Bingo,
}

fn suggest_guess(prior_guess: u32, answer: Answer) {
    match answer {
        Answer::Higher => println!("maybe try {} next", prior_guess + 10),
        Answer::Lower  => println!("maybe try {} next", prior_guess - 1),
        Answer::Bingo  => println!("we won with {}!", prior_guess),
    }
}

struct GuessState {
    guess: u32,
    answer: Answer,
    low: u32,
    high: u32,
}

fn suggest_guess_smarter(s: GuessState) {
    match s {
        // First arm only fires on Bingo; it binds `p` to last guess.
        GuessState { answer: Answer::Bingo, guess: p, .. } => {
     // ~~~~~~~~~~   ~~~~~~~~~~~~~~~~~~~~~  ~~~~~~~~  ~~
     //     |                 |                 |     |
     //     |                 |                 |     Ignore remaining fields
     //     |                 |                 |
     //     |                 |      Copy value of field `guess` into local variable `p`
     //     |                 |
     //     |   Test that `answer field is equal to `Bingo`
     //     |
     //  Match against an instance of the struct `GuessState`

            println!("we won with {}!", p);
        }

        // Second arm fires if answer was too low or too high.
        // We want to find a new guess in the range (l..h), where:
        //
        // - If it was too low, then we want something higher, so we
        //   bind the guess to `l` and use our last high guess as `h`.
        // - If it was too high, then we want something lower; bind
        //   the guess to `h` and use our last low guess as `l`.
        GuessState { answer: Answer::Higher, low: _, guess: l, high: h } |
        GuessState { answer: Answer::Lower,  low: l, guess: h, high: _ } => {
     // ~~~~~~~~~~   ~~~~~~~~~~~~~~~~~~~~~   ~~~~~~  ~~~~~~~~  ~~~~~~~
     //     |                 |                 |        |        |
     //     |                 |                 |        |    Copy or ignore
     //     |                 |                 |        |    field `high`,
     //     |                 |                 |        |    as appropriate
     //     |                 |                 |        |
     //     |                 |                 |  Copy field `guess` into
     //     |                 |                 |  local variable `l` or `h`,
     //     |                 |                 |  as appropriate
     //     |                 |                 |
     //     |                 |    Copy value of field `low` into local
     //     |                 |    variable `l`, or ignore it, as appropriate
     //     |                 |
     //     |   Test that `answer field is equal
     //     |   to `Higher` or `Lower`, as appropriate
     //     |
     //  Match against an instance of the struct `GuessState`

            let mid = l + ((h - l) / 2);
            println!("lets try {} next", mid);
        }
    }
}
fn suggest_guess_fixed(prior_guess: u32, answer: Answer) {
    let next_guess = match answer {
        Answer::Higher => prior_guess + 10,
        Answer::Lower  => prior_guess - 1,
        Answer::Bingo  => {
            println!("we won with {}!", prior_guess);
            return;
        }
    };
    println!("maybe try {} next", next_guess);
}

enum BinaryTree {
    Leaf(i32),
    Node(Box<BinaryTree>, i32, Box<BinaryTree>)
}
fn tree_weight_v1(t: BinaryTree) -> i32 {
    match t {
        BinaryTree::Leaf(payload) => payload,
        BinaryTree::Node(left, payload, right) => {
            tree_weight_v1(*left) + payload + tree_weight_v1(*right)
        }
    }
}

/// Returns tree that Looks like:
///
///      +----(4)---+
///      |          |
///   +-(2)-+      [5]
///   |     |
///  [1]   [3]
///
fn sample_tree() -> BinaryTree {
    let l1 = Box::new(BinaryTree::Leaf(1));
    let l3 = Box::new(BinaryTree::Leaf(3));
    let n2 = Box::new(BinaryTree::Node(l1, 2, l3));
    let l6 = Box::new(BinaryTree::Leaf(6));
    let l7 = Box::new(BinaryTree::Leaf(7));
    let n5 = Box::new(BinaryTree::Node(l6,5,l7));

    BinaryTree::Node(n2, 4, n5)
}

fn num_to_ordinal(x: u32) -> String {
    let suffix;
    match (x % 10, x % 100) {
        (1, 1) | (1, 21...91) => {
            suffix = "st";
        }
        (2, 2) | (2, 22...92) => {
            suffix = "nd";
        }
        (3, 3) | (3, 23...93) => {
            suffix = "rd";
        }
        _                     => {
            suffix = "th";
        }
    }
    return format!("{}{}", x, suffix);
}

fn num_to_ordinal_expr(x: u32) -> String {
    format!("{}{}", x, match (x % 10, x % 100) {
        (1, 1) | (1, 21...91) => "st",
        (2, 2) | (2, 22...92) => "nd",
        (3, 3) | (3, 23...93) => "rd",
        _                     => "th"
    })
}
fn sometimes_initialize(input: i32) {
    let string: String; // a dynamically-constructed string value
    let borrowed: &str; // a reference to string data
    match input {
        0...100 => {
            // Construct a String on the fly...
            string = format!("input prints as {}", input);
            // ... and then borrow from inside it.
            borrowed = &string[6..];
        }
        _ => {
            // String literals are *already* borrowed references
            borrowed = "expected between 0 and 100";
        }
    }
    println!("borrowed: {}", borrowed);

    // Below would cause compile-time error if uncommented...

    // println!("string: {}", string);

    // ...namely: error: use of possibly uninitialized variable: `string`
}
fn tree_weight_v2(t: &BinaryTree) -> i32 {
    //               ^~~~~~~~~~~ The `&` means we are *borrowing* the tree
    match *t {
        BinaryTree::Leaf(payload) => payload,
        BinaryTree::Node(ref left, payload, ref right) => {
            tree_weight_v2(left) + payload + tree_weight_v2(right)
        }
    }
}
fn tree_grow(t: &mut BinaryTree) {
    //          ^~~~~~~~~~~~~~~ `&mut`: we have exclusive access to the tree
    match *t {
        BinaryTree::Leaf(ref mut payload) => *payload += 1,
        BinaryTree::Node(ref mut left, ref mut payload, ref mut right) => {
            tree_grow(left);
            *payload += 1;
            tree_grow(right);
        }
    }
}

#[test]
fn tree_demo_3() {
    let mut tree = sample_tree();
    tree_grow(&mut tree);
    assert_eq!(tree_weight_v2(&tree), (2 + 3 + 4) + 5 + 6+7+8);
}

#[test]
fn tree_demo_2() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v2(&tree), (1 + 2 + 3) + 4 + 5+6+7);
}

#[test]
fn demo_sometimes_initialize() {
    sometimes_initialize(23);  // this invocation will initialize `string`
    sometimes_initialize(123); // this one will not
}

#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(   0),    "0th");
    assert_eq!(num_to_ordinal(   1),    "1st");
    assert_eq!(num_to_ordinal(  12),   "12th");
    assert_eq!(num_to_ordinal(  22),   "22nd");
    assert_eq!(num_to_ordinal_expr(  43),   "43rd");
    assert_eq!(num_to_ordinal_expr(  67),   "67th");
    assert_eq!(num_to_ordinal_expr(1901), "1901st");
}

#[test]
fn tree_demo_1() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v1(tree), (1 + 2 + 3) + 4 + 5+6+7);
}

#[test]
fn demo_guess_fixed() {
    suggest_guess_fixed(10, Answer::Higher);
    suggest_guess_fixed(20, Answer::Lower);
    suggest_guess_fixed(19, Answer::Bingo);
}
#[test]
fn demo_suggest_guess() {
    suggest_guess(10, Answer::Higher);
    suggest_guess(20, Answer::Lower);
}
#[test]
fn demo_guess_state() {
    suggest_guess_smarter(GuessState {
        guess: 20, answer: Answer::Lower, low: 10, high: 1000
    });
}

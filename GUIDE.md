# Project: Snake in Rust

A guided project for learning Rust by building the classic Snake game with
the [`piston_window`](https://docs.rs/piston_window/0.117.0/piston_window/)
graphics library.

You get the structure: files, types, struct fields, constants and function
signatures. Every function body is a `todo!()`, and writing them is your job.
The 35 tests in `src/tests/` check your work. This guide tells you what each
function has to do and which Rust ideas you need for it. It does not give you
the code.

---

## Table of contents

1. [Learning objectives](#1-learning-objectives)
2. [Setup](#2-setup)
3. [How the project is organised](#3-how-the-project-is-organised)
4. [Working with the tests](#4-working-with-the-tests)
5. [The game world](#5-the-game-world)
6. [Phase 1 — Pixels and grid cells (`draw.rs`)](#phase-1--pixels-and-grid-cells-drawrs)
7. [Phase 2 — Directions (`snake.rs`)](#phase-2--directions-snakers)
8. [Phase 3 — Creating the snake](#phase-3--creating-the-snake)
9. [Phase 4 — Moving the snake](#phase-4--moving-the-snake)
10. [Phase 5 — Growing and overlapping](#phase-5--growing-and-overlapping)
11. [Phase 6 — Creating and restarting the game (`game.rs`)](#phase-6--creating-and-restarting-the-game-gamers)
12. [Phase 7 — Collisions, eating and food](#phase-7--collisions-eating-and-food)
13. [Phase 8 — Steps, keys and the clock](#phase-8--steps-keys-and-the-clock)
14. [Phase 9 — Drawing](#phase-9--drawing)
15. [Phase 10 — The game loop (`main.rs`)](#phase-10--the-game-loop-mainrs)
16. [Phase 11 — Clean up](#phase-11--clean-up)
17. [Stretch goals](#stretch-goals)
18. [Troubleshooting common compiler errors](#troubleshooting-common-compiler-errors)
19. [Progress checklist](#progress-checklist)

---

## 1. Learning objectives

By the end of this project you should be comfortable with:

- **Modules**: splitting a program across files with `mod` and `use`.
- **Structs and `impl` blocks**: constructors (`new`), methods, `&self` vs
  `&mut self`.
- **Enums and `match`**: exhaustive matching and catch-all patterns.
- **`Option<T>`**: representing "maybe a value" with `Some`, `None` and
  `if let`.
- **Ownership and borrowing**: when a value is moved, copied, cloned or
  borrowed.
- **Collections**: `LinkedList`, iterating by reference.
- **Primitive types and casting**: `i32`, `u32`, `f64` and `as`.
- **External crates**: adding dependencies in `Cargo.toml` and reading their
  docs.
- **Testing**: running `cargo test`, reading failures, and filtering tests.

Reading to keep open while you work:

| Topic | Link |
|---|---|
| The Rust Book (all chapters) | https://doc.rust-lang.org/book/ |
| Structs (ch. 5) | https://doc.rust-lang.org/book/ch05-00-structs.html |
| Enums and `match` (ch. 6) | https://doc.rust-lang.org/book/ch06-00-enums.html |
| Ownership (ch. 4) | https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html |
| Modules (ch. 7) | https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html |
| Tests (ch. 11) | https://doc.rust-lang.org/book/ch11-00-testing.html |
| Casting with `as` | https://doc.rust-lang.org/rust-by-example/types/cast.html |
| `LinkedList` | https://doc.rust-lang.org/std/collections/struct.LinkedList.html |
| `Option` | https://doc.rust-lang.org/std/option/enum.Option.html |
| `rand` 0.8 `Rng` trait | https://docs.rs/rand/0.8/rand/trait.Rng.html |
| `piston_window` 0.117 | https://docs.rs/piston_window/0.117.0/piston_window/ |

---

## 2. Setup

The dependencies are already in `Cargo.toml`:

```toml
[dependencies]
piston_window = "0.117.0"
rand = "0.8"
```

1. Run `cargo build`. The first build compiles the whole graphics stack and
   takes a couple of minutes. Later builds are fast.
2. Run `cargo test`. You should see **35 failed** tests, each one panicking
   with `not yet implemented: Phase N`. That is where you start.

> **Note on the original tutorial.** The reference code uses
> `extern crate piston_window;` at the top of `main.rs`. This project uses
> Rust **edition 2024**, where `extern crate` is no longer needed because
> dependencies in `Cargo.toml` are available automatically. If you look at
> the original code, you will see a few small differences like this one.

---

## 3. How the project is organised

```
src/
├── main.rs          the window and game loop          (Phase 10)
├── draw.rs          grid <-> pixel maths, drawing     (Phases 1, 9)
├── snake.rs         Direction, Block, Snake           (Phases 2-5, 9)
├── game.rs          Game: rules, food, timing         (Phases 6-9)
└── tests/
    ├── draw_tests.rs   3 tests   - do not edit
    ├── snake_tests.rs 15 tests   - do not edit
    └── game_tests.rs  17 tests   - do not edit
```

Dependencies between modules point one way:

```
main.rs ──> game.rs ──> snake.rs ──> draw.rs
   └────────────────────────────────────┘
```

`draw.rs` knows nothing about snakes, and `snake.rs` knows nothing about food
or walls. Think about why this layering is useful before you start.

### What you are given vs. what you write

| Given (do not change) | You write |
|---|---|
| Struct definitions and their fields | Every function body marked `todo!()` |
| Enum definitions and `#[derive]`s | |
| Constants (`BLOCK_SIZE`, colours, timings) | |
| Function names, parameters, return types | |
| The `len()` test helper on `Snake` | |
| Everything in `src/tests/` | |

You may add **private helper functions** if they make your code clearer. If
you change a given signature, the tests will stop compiling.

### About `#![allow(...)]` in `main.rs`

The first line of `main.rs` turns off warnings about unused variables, imports
and code. While everything is `todo!()` there would be dozens of them. In
Phase 11 you will delete that line and fix whatever the compiler reports.

---

## 4. Working with the tests

### Running tests

| Command | What it runs |
|---|---|
| `cargo test` | all 35 tests |
| `cargo test draw::` | only the `draw.rs` tests |
| `cargo test snake::` | only the `snake.rs` tests |
| `cargo test game::` | only the `game.rs` tests |
| `cargo test overlap` | every test whose name contains `overlap` |
| `cargo test -- --nocapture` | all tests, and shows your `println!` output |

The filter matches any part of a test's full name, for example
`snake::tests::overlap_tail_ignores_the_last_block`.

### Reading a failure

```
---- snake::tests::move_forward_without_direction_keeps_going stdout ----
assertion `left == right` failed
  left: [(4, 2), (3, 2), (2, 2)]
 right: [(5, 2), (4, 2), (3, 2)]
```

`left` is what your code produced. `right` is what the test expected. Open
the test file, find the test by name, and read it like a specification: it
shows the exact setup, the action, and the expected result.

### Why can the tests see private fields?

The tests read fields like `snake.body` and `game.food_x`, which are not
`pub`. That works because of this, at the bottom of each module:

```rust
#[cfg(test)]
#[path = "tests/snake_tests.rs"]
mod tests;
```

- `#[cfg(test)]` means "only compile this when running `cargo test`".
- `#[path = ...]` says where the module's file is.
- `mod tests;` makes `tests` a **child module** of `snake`. In Rust a child
  module can see its parent's private items, and `use super::*;` at the top
  of the test file brings them into scope.

**Question to think about:** an integration test in a top-level `tests/`
folder could *not* read `snake.body`. Why not?

### Which tests depend on what

Tests in later phases call functions from earlier phases. If an early
function panics with `todo!()`, every test that uses it fails too. Work
through the phases in order. "Everything is red" in Phase 7 usually means
something in Phase 3 or 4 is not finished.

---

## 5. The game world

The board is a grid of square **cells**. `(0, 0)` is the **top-left** corner,
`x` grows to the right, and `y` grows **downwards**:

```
        x →   0   1   2   3   4   5   6   7
      y ↓   ┌───┬───┬───┬───┬───┬───┬───┬───┐
        0   │ # │ # │ # │ # │ # │ # │ # │ # │   # = border wall
            ├───┼───┼───┼───┼───┼───┼───┼───┤
        1   │ # │   │   │   │   │   │   │ # │
            ├───┼───┼───┼───┼───┼───┼───┼───┤
        2   │ # │   │ t │ = │ H │   │   │ # │   H = head, t = tail end
            ├───┼───┼───┼───┼───┼───┼───┼───┤
        3   │ # │   │   │   │   │   │   │ # │
            ├───┼───┼───┼───┼───┼───┼───┼───┤
        4   │ # │   │   │   │   │   │ F │ # │   F = food
            ├───┼───┼───┼───┼───┼───┼───┼───┤
        5   │ # │ # │ # │ # │ # │ # │ # │ # │
            └───┴───┴───┴───┴───┴───┴───┴───┘
                   an 8 x 6 board (width = 8, height = 6)
```

Key facts you will need:

- `width` and `height` **include** the walls. The walls are the outermost
  ring of cells.
- The snake's body is stored **head first**: the front of the `LinkedList` is
  the head and the back is the tail end.
- One **move** means: a new block appears in front of the head, and the last
  block disappears. The length stays the same.
- **Growing** means: after a move, the block that disappeared comes back.

Before Phase 7, work out with pencil and paper: on a board with `width = 8`,
which `x` values are walls and which are playable? Write the answer in terms
of `width`, not as the number 8.

---

## Phase 1 — Pixels and grid cells (`draw.rs`)

**Functions:** `to_coord`, `to_coord_u32`
**Tests:** 3, run with `cargo test draw::`

### Goal

The game logic uses grid cells (`i32`). The graphics library uses pixels
(`f64`). Write the conversion.

### Concepts

- Rust **never** converts between number types automatically. `i32 * f64` is
  a compile error.
- The `as` keyword converts between primitive number types.
- A function returns the value of its last expression when that line has
  **no semicolon**.

### Questions to answer before coding

1. If one cell is `BLOCK_SIZE` pixels wide, where does cell `n` start?
2. Which value has to be converted so both sides of the multiplication have
   the same type?
3. `to_coord_u32` needs the same answer as a `u32`. Can you reuse
   `to_coord` instead of repeating the maths?

<details>
<summary>Hint: I get "mismatched types" or "cannot multiply"</summary>

Look at the types on each side of your operator. One side is `i32` and the
other is `f64`. Convert one of them with `as` so both match.
</details>

**Checkpoint:** `cargo test draw::` shows 3 passed.

---

## Phase 2 — Directions (`snake.rs`)

**Function:** `Direction::opposite`
**Tests:** 2, run with `cargo test snake::tests::opposite`

### Goal

Return the direction that points the other way.

### Concepts

- `match` must be **exhaustive**: every variant must be handled. Try leaving
  one out and read the compiler error, which is one of Rust's most helpful.
- The method takes `&self`, a *reference* to a `Direction`. Look into what
  matching on `self` vs. `*self` means.
- `Direction` derives `Copy`. Find out what that derive lets you do that you
  could not do otherwise.

### Questions

1. Why is `match` a better fit here than a chain of `if` statements?
2. `opposite_twice_is_original_direction` should pass automatically once the
   first test does. Why?

**Checkpoint:** `cargo test snake::tests::opposite` shows 2 passed.

---

## Phase 3 — Creating the snake

**Functions:** `Snake::new`, `head_position`, `head_direction`
**Tests:** 3, run with `cargo test new_snake`

### Goal

Build a three-block snake facing right that has not moved yet, plus two
small "getter" methods.

### Concepts

- Creating a struct: `Snake { direction: ..., body: ..., tail: ... }`.
- **Field init shorthand**: when a variable has the same name as a field
  (Rust Book §5.1).
- `LinkedList` methods: look up `new`, `push_back`, `push_front` and `front`
  in the docs.
- `front()` returns an `Option<&Block>`, not a `Block`. Why could it be
  empty? For now you can assume the snake always has at least one block. Look
  up how to get the value out of an `Option` when you are sure it is there.
- Returning a tuple: `(i32, i32)`.

### Questions

1. `Snake::new(x, y)` says the **tail end** is at `(x, y)`. Where are the
   other two blocks? Draw it.
2. The body is stored head first. In what order do you add the blocks, and
   with which push method?
3. What should `tail` be for a snake that has never moved?
4. `head_direction` returns `Direction` by value, not by reference. Why does
   that compile without a `.clone()`? (Hint: Phase 2.)

**Checkpoint:** `cargo test new_snake` shows 3 passed.

---

## Phase 4 — Moving the snake

**Functions:** `move_forward`, `next_head`
**Tests:** 6, run with `cargo test move_forward` and `cargo test next_head`

### Goal

`move_forward` really moves the snake. `next_head` only works out where the
head *would* go, and changes nothing.

### Concepts

- `&mut self` vs `&self`. Notice which one each method takes, and why.
- Matching on an `Option<Direction>`: `Some(d)` vs `None`. Also look at
  `if let Some(d) = dir { ... }` (Rust Book §6.3) as a shorter form.
- `match` is an **expression**: it can produce a value you store in a `let`.
- `pop_back()` also returns an `Option`.

### Plan for `move_forward` (in words)

1. Decide which direction the snake travels this move.
2. Find the current head position.
3. Work out the new head position from that direction. Remember which way
   `y` goes for `Up` (see [section 5](#5-the-game-world)).
4. Put a new block at the **head** end of the body.
5. Take a block off the **tail** end, and remember it in `self.tail`.

### Questions

1. `next_head` and `move_forward` both work out "the cell in front of the
   head". Can you avoid writing that logic twice? (You don't have to, but
   it's worth thinking about.)
2. `next_head` takes `&self`. What would the compiler say if you tried to
   change `self.direction` inside it? Try it, read the error, then undo it.
3. Why does `self.tail` hold an `Option<Block>` rather than a plain `Block`?

<details>
<summary>Hint: "cannot borrow `self.body` as mutable because it is also borrowed as immutable"</summary>

You are probably holding a reference into the list (from `front()`) while
also trying to push to it. Copy the numbers you need into local variables
first. `head_position()` already returns plain `i32`s, not references.
</details>

**Checkpoint:** `cargo test snake::` shows 11 of 15 passed.

---

## Phase 5 — Growing and overlapping

**Functions:** `restore_tail`, `overlap_tail`
**Tests:** 4, run with `cargo test restore_tail` and `cargo test overlap`

### Goal

Let the snake grow after eating, and check whether a cell is covered by the
snake.

### `restore_tail`: concepts

- `self.tail` is an `Option<Block>`, but `push_back` needs an owned `Block`.
- You cannot just move a value out of a field behind `&mut self`. Read the
  error the compiler gives you.
- Two common ways around this are `Option::clone` (copy it) and `Option::take`
  (move it out and leave `None` behind). Look both up. Either passes the
  tests. Which do you think makes more sense here, and why?

### `overlap_tail`: concepts

- Iterating over a collection **by reference**: `for block in &self.body`.
  What happens if you leave out the `&`?
- Returning early with `return` from inside a loop.
- Useful iterator tools: `.iter()`, `.take(n)`, `.enumerate()`, `.any(...)`.
  You don't need all of them. A plain `for` loop with a counter is fine too.

### Why skip the last block?

<details>
<summary>Think about it first, then open this</summary>

`overlap_tail` mostly answers "would the head crash into the body if it moved
here?" On the next move the last block disappears at the same moment the head
moves forward, so that cell is about to be empty. Moving the head into the
cell the tail is leaving is legal, so the last block must not count as a
collision.
</details>

### Questions

1. What is the length of the body? How many blocks should you check?
2. Which tests would catch you if you checked one block too many, or one too
   few?

**Checkpoint:** `cargo test snake::` shows 15 passed. `snake.rs` is done
except for drawing.

---

## Phase 6 — Creating and restarting the game (`game.rs`)

**Functions:** `Game::new`, `restart`
**Tests:** 2, run with `cargo test new_game` and `cargo test restart_resets`

### Goal

Set up the starting state, and be able to return to it.

### Concepts

- Struct construction again, this time with a field that is itself a struct
  (`Snake`).
- **Reassigning a field** of `self` in a `&mut self` method.

### Questions

1. The doc comment on `Game::new` lists the starting values. Which fields
   come from the parameters, and which are fixed?
2. `restart` and `new` describe the same starting state, except that
   `restart` keeps the board size. How can you keep them from drifting apart
   if you later change the starting food position? (One option: a private
   helper or constants. Another: think about whether `restart` can build on
   `new`. Is there a catch with the board size?)

**Checkpoint:** 2 more tests pass. Most `game::` tests will still fail
because they need Phase 7 and 8 functions.

---

## Phase 7 — Collisions, eating and food

**Functions:** `check_if_snake_alive`, `check_eating`, `add_food`
**Tests:** 6, run with `cargo test alive`, `cargo test dies`,
`cargo test check_eating` and `cargo test add_food`

### `check_if_snake_alive`

The snake dies if its **next** head position:

- overlaps its own body, or
- lands on any of the four walls.

Use the functions you wrote in Phase 4 and 5. You don't need to move the
snake to find out.

Questions:

1. Using your pencil-and-paper answer from section 5, write the condition
   for "inside the playable area" in terms of `width` and `height`. Check it
   against the left, right, top and bottom wall cases in
   `dies_when_hitting_any_wall`.
2. Rust's boolean operators are `&&`, `||` and `!`. Can the whole function be
   one expression?

### `check_eating`

If the head is on the food, the food is gone and the snake grows.

Questions:

1. Which `Snake` method makes it grow?
2. Should eating count if `food_exists` is `false` but the coordinates happen
   to match?

### `add_food`

Choose a random cell inside the walls that is not on the snake.

Concepts:

- `thread_rng()` gives you a random number generator. The `Rng` trait (already
  imported) gives it the method `gen_range`.
- `gen_range(a..b)` includes `a` but **excludes** `b`. This matters for your
  bounds.
- Traits: `gen_range` only works because `Rng` is imported. Try removing the
  import and read the error.
- A `while` loop that keeps trying until it finds a cell that is free.

Questions:

1. What range of `x` values should `gen_range` produce for food to always be
   inside the walls?
2. The test calls `add_food` 500 times on a tiny 8 x 6 board. Why a tiny
   board? Why does it also check that the food lands in **more than one**
   place?

**Checkpoint:** `cargo test game::` shows 8 of 17 passed.

---

## Phase 8 — Steps, keys and the clock

**Functions:** `update_snake`, `key_pressed`, `update`
**Tests:** 9, run with `cargo test update` and `cargo test key_pressed`

Write these **in this order**. Each one builds on the one before.

### `update_snake`: one step of the game

Follow the doc comment exactly. Things to notice:

- The safety check has to happen **before** the move. Why?
- When should `check_eating` be called: before or after moving?
- `waiting_time` resets **in both cases**.

### `key_pressed`: turning the keyboard into a direction

Concepts:

- `Key` is an enum from `piston_window` with a variant for every key on the
  keyboard (`Key::Up`, `Key::A`, `Key::Space`, ...). You can't list them all,
  so you will need a **catch-all** pattern `_`.
- The match produces an `Option<Direction>` (or a `Direction`, which you then
  wrap). Arrow keys map to their direction. Every other key means "keep
  going the way you're going".
- Comparing enums with `==` only works because `Direction` derives
  `PartialEq`.

Questions:

1. In what order should you check things: game over, the key mapping, the
   opposite-direction rule, then the step?
2. What happens in the real game if the opposite-direction check is missing
   and the player presses Left while moving Right?

### `update`: the clock

The game loop calls `update(delta_time)` many times per second with the
number of seconds since the last call. Your job is to add up that time and
act when enough has passed.

Questions:

1. Read the four bullet points in the doc comment. Which ones should stop the
   function early (`return`)?
2. The snake moves when `waiting_time` goes **over** `MOVING_PERIOD`. What
   resets `waiting_time` afterwards? (You already wrote it.)
3. In `update_waits_for_moving_period`, the test calls `update(0.05)` and
   then `update(0.06)`. Trace through your code by hand and check you get the
   expected head positions.
4. Why does the test compare `waiting_time` to `0.05` using `abs() < 1e-9`
   rather than `==`?

**Checkpoint:** `cargo test` shows **35 passed**. All the game logic is done.
What's left is drawing, which the tests can't check.

---

## Phase 9 — Drawing

**Functions:** `draw_block`, `draw_rectangle` (in `draw.rs`), `Snake::draw`,
`Game::draw`
**Tests:** none. You will check these by eye in Phase 10.

### Concepts

- `piston_window` re-exports a function called `rectangle`. Find it in the
  [docs](https://docs.rs/piston_window/0.117.0/piston_window/) (use the search
  bar) and read its parameters: a colour, a `[x, y, width, height]` array **in
  pixels**, a transform (use `con.transform`), and the graphics backend `g`.
- `Color` is a type alias for `[f32; 4]`: red, green, blue, alpha.

### Steps

1. `draw_block`: one cell. Convert the grid position to pixels with your
   Phase 1 function. How big is the square in pixels?
2. `draw_rectangle`: like `draw_block`, but `width` x `height` cells. What
   type conversion do you need for the size?
3. `Snake::draw`: draw every block of the body.
4. `Game::draw`, in this order:
   - the snake
   - the food, only if it exists
   - the four walls, each one a `draw_rectangle` call. Work out the
     top-left cell, width and height of each wall from section 5's diagram.
   - if the game is over, a `GAMEOVER_COLOR` rectangle covering the whole
     board

Question: does the drawing order matter? What would you see if the game-over
overlay were drawn first?

---

## Phase 10 — The game loop (`main.rs`)

### Goal

Open a window and run the game. The comments in `main.rs` list the steps.

### API pointers (from the `piston_window` docs)

- `WindowSettings::new(title, [width_px, height_px])` uses the **builder
  pattern**: chain settings like `.exit_on_esc(true)`, then `.build()`.
  `.build()` returns a `Result`. Look up what that means and how to handle
  it in a small program.
- The window type is `PistonWindow`. You'll need a type annotation on the
  `let`.
- `window.next()` returns `Option<Event>`: `Some` while the window is open,
  `None` once it closes. Which loop form keeps going as long as a pattern
  matches? (Rust Book §6.3 and §19.1.)
- On an event:
  - `event.press_args()` returns `Option<Button>`. A keyboard key is the
    variant `Button::Keyboard(key)`. You can match a pattern nested inside
    another pattern.
  - `window.draw_2d(&event, |c, g, _device| { ... })` takes a **closure**.
    Inside it, `clear(BACK_COLOR, g)` and then draw the game.
  - `event.update(|args| { ... })` runs its closure on update events.
    `args.dt` is the delta time in seconds.

Questions:

1. `game` is borrowed mutably in the key handler and the update closure, and
   immutably in the draw closure. Why does the borrow checker accept that?
   (Hint: when does each borrow end?)
2. Why is the window size passed through `to_coord_u32`?

**Checkpoint:** `cargo run` opens a window and you can play.

### Manual test plan

Since drawing isn't unit tested, check by hand:

- [ ] Grey background, black border on all four sides, border one cell thick
- [ ] Green snake, three blocks long, in the top-left area, moving right
- [ ] Red food block visible
- [ ] Arrow keys turn the snake; pressing the reverse direction does nothing
- [ ] Eating the food makes the snake one block longer and new food appears
- [ ] Hitting a wall or yourself shows a red overlay, then the game restarts
      about one second later
- [ ] Esc closes the window

---

## Phase 11 — Clean up

1. Delete the `#![allow(dead_code, unused_variables, unused_imports)]` line
   from `main.rs`. Run `cargo build` and fix every warning.
2. Run `cargo fmt` to format the code the standard way.
3. Run `cargo clippy` and read its suggestions. Clippy is Rust's linter. It
   often suggests a more idiomatic way to write something. Look up anything
   you don't understand. Don't just accept the change.
4. Run `cargo test` one last time: 35 passed.
5. Commit your work.

---

## Stretch goals

Done and want more practice? Try these (some will need new tests from you).

| Idea | What you'll practise |
|---|---|
| Show a score, or print it to the console when you die | new struct fields, `println!` formatting |
| Speed up as the snake grows | turning a constant into a field |
| Swap `LinkedList` for `VecDeque` | reading std docs; why `VecDeque` is usually preferred in Rust |
| Pause with the P key | more state, more `match` arms |
| Walls that wrap around to the other side | the `%` operator and its behaviour with negative numbers |
| Write your own tests for a stretch goal | `#[test]`, `assert!`, `assert_eq!` |
| Make `overlap_tail` a one-liner with iterator methods | `iter`, `take`, `any`, closures |

---

## Troubleshooting common compiler errors

| Error | Usually means |
|---|---|
| `E0308 mismatched types` | Mixing `i32`/`f64`/`u32` without `as`, or a stray `;` at the end of a function so it returns `()` |
| `E0004 non-exhaustive patterns` | A `match` is missing an enum variant, or a `_` arm for something like `Key` |
| `E0502 cannot borrow ... as mutable because it is also borrowed as immutable` | Holding a reference (from `front()`, `iter()` ...) while changing the same collection |
| `E0507 cannot move out of ... which is behind a mutable reference` | Trying to take an owned value out of a field. See Phase 5 (`clone` / `take`) |
| `E0596 cannot borrow ... as mutable` | The method takes `&self` but you're trying to change something, or a local variable needs `let mut` |
| `E0599 no method named gen_range found` | The `Rng` trait isn't in scope |
| A test panics with `called Option::unwrap() on a None value` | Something you assumed existed doesn't, e.g. calling `restore_tail` before the snake has ever moved |
| A test panics with `not yet implemented` | That test (or a function it uses) still has a `todo!()` |

For any error code, `rustc --explain E0502` gives a full explanation with
examples.

---

## Progress checklist

| Phase | Functions | Tests | Done |
|---|---|---|---|
| 1 | `to_coord`, `to_coord_u32` | 3 | [ ] |
| 2 | `Direction::opposite` | 2 | [ ] |
| 3 | `Snake::new`, `head_position`, `head_direction` | 3 | [ ] |
| 4 | `move_forward`, `next_head` | 6 | [ ] |
| 5 | `restore_tail`, `overlap_tail` | 4 | [ ] |
| 6 | `Game::new`, `restart` | 2 | [ ] |
| 7 | `check_if_snake_alive`, `check_eating`, `add_food` | 6 | [ ] |
| 8 | `update_snake`, `key_pressed`, `update` | 9 | [ ] |
| 9 | `draw_block`, `draw_rectangle`, `Snake::draw`, `Game::draw` | manual | [ ] |
| 10 | `main` | manual | [ ] |
| 11 | clean up, `fmt`, `clippy` | 35 | [ ] |
| | **Total** | **35** | |

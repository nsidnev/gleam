use crate::assert_js;

#[test]
fn guards_cause_badmatch_to_render() {
    assert_js!(
        r#"pub fn main(x, y) {
  case x {
    1 -> 1
    _ if y -> 0
  }
}
"#,
    );
}

#[test]
fn referencing_pattern_var() {
    assert_js!(
        r#"pub fn main(xs) {
  case xs {
    #(x) if x -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn rebound_var() {
    assert_js!(
        r#"pub fn main() {
  let x = False
  let x = True
  case x {
    _ if x -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn operator_wrapping_right() {
    assert_js!(
        r#"pub fn main(xs, y: Bool, z: Bool) {
  case xs {
    #(x) if x == { y == z } -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn operator_wrapping_left() {
    assert_js!(
        r#"pub fn main(xs, y: Bool, z: Bool) {
  case xs {
    #(x) if { x == y } == z -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn eq_scalar() {
    assert_js!(
        r#"pub fn main(xs, y: Int) {
  case xs {
    #(x) if x == y -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn not_eq_scalar() {
    assert_js!(
        r#"pub fn main(xs, y: Int) {
  case xs {
    #(x) if x != y -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn tuple_index() {
    assert_js!(
        r#"pub fn main(x, xs: #(Bool, Bool, Bool)) {
  case x {
    _ if xs.2 -> 1
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn not_eq_complex() {
    assert_js!(
        r#"pub fn main(xs, y) {
  case xs {
    #(x) if xs != y -> x
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn eq_complex() {
    assert_js!(
        r#"pub fn main(xs, y) {
  case xs {
    #(x) if xs == y -> x
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn constant() {
    assert_js!(
        r#"pub fn main(xs) {
  case xs {
    #(x) if x == 1 -> x
    _ -> 0
  }
}
"#,
    );
}

#[test]
fn alternative_patterns() {
    assert_js!(
        r#"pub fn main(xs) {
  case xs {
    1 | 2 -> 0
    _ -> 1
  }
}
"#,
    );
}

#[test]
fn alternative_patterns_list() {
    assert_js!(
        r#"pub fn main(xs) -> Int {
  case xs {
    [1] | [1, 2] -> 0
    _ -> 1
  }
}
"#,
    );
}

#[test]
fn alternative_patterns_assignment() {
    assert_js!(
        r#"pub fn main(xs) -> Int {
  case xs {
    [x] | [_, x] -> x
    _ -> 1
  }
}  
"#,
    );
}

#[test]
fn alternative_patterns_guard() {
    assert_js!(
        r#"pub fn main(xs) -> Int {
  case xs {
    [x] | [_, x] if x == 1 -> x
    _ -> 0
  }
}   
"#,
    );
}

#[test]
fn field_access() {
    assert_js!(
        r#"
        pub type Person {
          Person(username: String, name: String, age: Int)
        }
        
        pub fn main() {
          let given_name = "jack"
          let raiden = Person("raiden", "jack", 31)
          
          case given_name {
            name if name == raiden.name -> "It's jack"
            _ -> "It's not jack"
          }
        }
        "#
    )
}

#[test]
fn nested_record_access() {
    assert_js!(
        r#"
pub type A {
  A(b: B)
}

pub type B {
  B(c: C)
}

pub type C {
  C(d: Bool)
}

pub fn a(a: A) {
  case a {
    _ if a.b.c.d -> 1
    _ -> 0
  }
}
"#
    );
}

fn main() {
    copy_vec();
    using_lifetime()
}

fn copy_vec() {
    let vec1 = vec![
        String::from("First"),
        String::from("Second"),
        String::from("Third"),
        String::from("Fourth"),
    ];

    let _vec2 = vec![vec1.get(1).unwrap().clone()];
}

fn using_lifetime() {
    let vec1: Vec<String> = vec![String::from("First"), String::from("Second")];
    let mut vec2: Vec<String> = Vec::new();
    let vec3 = copy_to_new_vec(&vec1, &mut vec2);
    vec3.get(0);
}

// Functions that return a reference to an object must use a lifetime paramter in order
// to tell the the compiler exactly which object is referenced. If the object that is
// being referenced goes out of scope while the reference is still in scope, the
// compiler will complain. Lifetime parameters are normally inferred by the compiler,
// but in this example, because there are two parameters that the return value could
// potentially be referencing, the lifetime parameters must be explicitly provided.
fn copy_to_new_vec<'a>(vec1: &Vec<String>, vec2: &'a mut Vec<String>) -> &'a mut Vec<String> {
    vec2.push(vec1.get(1).unwrap().to_string());
    vec2
}

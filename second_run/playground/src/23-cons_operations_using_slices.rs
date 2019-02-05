fn p(list:&[i32]) -> &[i32] {

    if list.len() > 2 {

        println!("{}", list[0]);

        p(&list[1..])

    } else {

        &list[1..]

    }

}

/// Note, that this is a procedure, as it DOES HAVE SIDE EFFECTS...
fn add_one(list:&mut[i32]) {

    if list.len() > 0 {

        list[0] += 1;

        p(&mut list[1..]);

    }

}

fn main() {

    // EYO, you can create slices to use the way you'd do a cons operator recursion...

    // This file contains an example of how you can use slices for cons-esque recursive operations.
    // You wouldn't wanna do this, I'd say, cause no tail call recursion and whatnot, but it's
    // there.
    // Now, if one's doing something weird, one might eventually encounter the lifetimes,
    // but -- suprisingly, and unsurprisingly as well =) -- this example doesn't even need 'em.

    let i = [33, 92, 44, 88, 11];

    println!("The whole thing:\n\t{:?}", i);

    let si = &i[1..];

    println!("The rest of the thing:\n\t{:?}", si);

    let ssi = &si[1..];

    println!("The rest of the rest of the thing:\n\t{:?}", ssi);

    let sssi = &ssi[1..];

    println!("The rest of the rest of the rest of the thing:\n\t{:?}", sssi);

    let ssssi = &sssi[1..];

    println!("The rest of the rest of the rest of the rest of the thing:\n\t{:?}", ssssi);

    let sssssi = &ssssi[1..];

    println!("The rest of the rest of the rest of the rest of the rest of the thing:\n\t{:?}", sssssi);

    /* The next bit is out of bounds! */
    /*
    let ssssssi = &sssssi[1..];

    println!("The rest of the rest of the rest of the rest of the rest of the rest of the thing:\n\t{:?}", ssssssi);
    */

    let mut ll = vec![8374, 9387, 2837, 2938];

    println!("{:?}", p(&ll));

    add_one(&mut ll);

    println!("{:?}", ll);

}

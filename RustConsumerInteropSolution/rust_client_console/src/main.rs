use bindings::{
    component1::Class1,
    component2::Class2,
};


/*
Another useful example:
https://github.com/kennykerr/multiple-components
*/

fn main() -> windows::Result<()> {
    let c1 = Class1::new()?;
    let c2 = Class2::new()?;

    println!("C1 is {} | C2 is {}", c1.my_property1()?.to_string(), c2.my_property2()?.to_string());

    Ok(())
}

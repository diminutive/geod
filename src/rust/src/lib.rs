#![allow(non_snake_case)]

use extendr_api::prelude::*;
use geodesy::prelude::*;


/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}


/// @export
#[extendr]
fn add(x: i32, y: i32) -> i32 {
    x + y
}




fn internal() -> anyhow::Result<()> {
    let mut context = Minimal::new();
    let utm33 = context.op("utm zone=33")?;

    let cph = Coor4D::geo(55., 12., 0., 0.); // Copenhagen
    let sth = Coor4D::geo(59., 18., 0., 0.); // Stockholm
    let mut data = [cph, sth];

    context.apply(utm33, Fwd, &mut data)?;
    println!("{:?}", data);
    Ok(())
}

/// @export
#[extendr]
fn utm() -> i32 {
    //let mut context = Plain::new();
    //let utm33 = context.op("utm zone=33");

    let cph = Coor4D::geo(55., 12., 0., 0.); // Copenhagen
    //let sth = Coor4D::geo(59., 18., 0., 0.); // Stockholm

    let sth = internal();

    //let cph = Coor2D::geo(55., 12.); // Copenhagen
    //let sth = Coor2D::geo(59., 18.); // Stockholm
    //let mut data = [cph, sth];

    //context.apply(utm33, Fwd, &mut data);
    //println!("{:?}", data);
    1
}

/// @export
#[extendr]
fn rumination_000() -> i32 {

    // [1] Build some context
    let mut ctx = Minimal::default();

    // [2] Obtain a handle to the utm-operator
    let utm32 = ctx.op("utm zone=32");

    // [3] Coordinates of some Scandinavian capitals
    let copenhagen = Coor2D::geo(55., 12.);
    let stockholm = Coor2D::geo(59., 18.);

    // [4] Put the coordinates into an array
    let mut data = [copenhagen, stockholm];


    // [5] Then do the forward conversion, i.e. geo -> utm
    //ctx.apply(utm32, Fwd, &mut data);
    //for coord in data {
    //    println!("{:?}", coord);
    //}

    // [6] And go back, i.e. utm -> geo
    //ctx.apply(utm32, Inv, &mut data);
    //data.to_geo();
    //for coord in data {
    //    println!("{:?}", coord);
    //}

    3
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geod;
    fn hello_world;
    fn add;
    fn utm;
    fn rumination_000;
}



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


fn trans(xx: &[f64], yy: &[f64]) -> anyhow::Result<f64> {
  let mut context = Minimal::new();
  let utm33 = context.op("utm zone=33")?;

//  let x_vec: Option<Vec<f64>> = xx.as_real_vector();
//  let y_vec: Option<Vec<f64>> = yy.as_real_vector();

  let pt = Coor2D::geo(xx[0], yy[0]);
  let mut data = [pt];

  context.apply(utm33, Fwd, &mut data);
   let out = data[0][0];
  Ok(out)
}

fn internal() -> anyhow::Result<f64> {
    let mut context = Minimal::new();
    let utm33 = context.op("utm zone=33")?;

    let cph = Coor4D::geo(55., 12., 0., 0.); // Copenhagen
    let sth = Coor4D::geo(59., 18., 0., 0.); // Stockholm
    let mut data = [cph, sth];


    context.apply(utm33, Fwd, &mut data);
    let x = data[0];
    let y = x[0];
    println!("{:?}", y);
    Ok(y)
}
/// @export
#[extendr]
fn utm(xx: &[f64], yy: &[f64]) -> Robj {

    let mut context = Minimal::new();
    use geodesy::operator_authoring::Op;
    let utm33 = Op::new("utm zone=33", &context).expect("bad!"); // context.op("utm zone=33");

  //  let cph = Coor4D::geo(55., 12., 0., 0.); // Copenhagen
  //  let sth = Coor4D::geo(59., 18., 0., 0.); // Stockholm
  //  let mut data = [cph, sth];

// utm33.apply(&context, &mut data, Fwd);


  let mut outx = Doubles::from_values(xx);
  let mut outy = Doubles::from_values(yy);

  let len =  outx.len();
  for p0 in 0..len {

    let pt = Coor2D::geo(xx[p0], yy[p0]);
    let mut data = [pt];

    utm33.apply(&context, &mut data, Fwd);

    outx[p0] = Rfloat(data[0][0]);
    outy[p0] = Rfloat(data[0][1]);

  }
  r!(list!(x = outx, outy))
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



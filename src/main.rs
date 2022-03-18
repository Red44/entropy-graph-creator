use std::fs;
use std::ops::Mul;
use poloto::prelude::*;

fn render_frame(total_entropy : f64, propabilities : Vec<f64> ) -> String{

    let it = (0..).zip(propabilities);

    let data = poloto::build::histogram("", it)
        .build_with([255], [])
        .stage();

    let (_, by) = data.bounds();
    let (xtick, xtick_fmt) = poloto::ticks::from_iter((0..).step_by(6));
    let (ytick, ytick_fmt) = poloto::ticks::from_default(by);
    let mut pp = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "probability distribution",
            format!("total entropie {}",total_entropy),
            "probability",
            xtick_fmt.with_tick_fmt(|w, v| {
                return  if v % 16 == 0{
                     write!(w, "Byte {}", v)
                }else {
                    write!(w,"")
                };
            }),
            ytick_fmt,
        ),
    );

     format!("{}",poloto::disp(|w| pp.simple_theme(w)))
}


fn main() {
    let mut entropy = 0.0;
    let mut byte_counter = std::collections::HashMap::new();
    let mut propabilities = std::collections::HashMap::<u8,f64>::new();
    for x in 0_u8..255{
        propabilities.insert(x,0.0);
    }
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();
    let total_bytes  = std::fs::read(&file_name).unwrap();
    for ref byte in total_bytes {
        let count = byte_counter.get_mut(byte);
        match count {
            Some(v) =>{
                *v+=1;
            }
            None =>{
                byte_counter.insert(*byte,1_u32);
            }
        }
    }
    for (k,v) in byte_counter.iter() {
       propabilities.insert(*k, (*v as f64)/(byte_counter.len() as f64));
    }
    for (k,v) in propabilities.iter() {
        let propability = *v;
        entropy+= propability * (1.0/propability).log2();
    }
    entropy = entropy.abs();
    let mut props = Vec::with_capacity(propabilities.len());
     for (k,v) in propabilities.drain() {
        props.push(v);
    }
   std::fs::write(format!("{}_entropy.html",&file_name),render_frame(entropy,props));
    println!("finished file written to {}" ,format!("{}_entropy.html",&file_name));
}


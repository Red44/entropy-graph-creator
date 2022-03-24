use poloto::prelude::*;

fn render_frame(total_entropy: f64, propabilities: Vec<f64>) -> String {
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
            format!("total entropie {}", total_entropy),
            "probability",
            xtick_fmt.with_tick_fmt(|w, v| {
                return if v % 16 == 0 {
                    write!(w, "Byte {}", v)
                } else {
                    write!(w, "")
                };
            }),
            ytick_fmt,
        ),
    );

    format!("{}", poloto::disp(|w| pp.simple_theme(w)))
}

fn main() {
    let mut entropy = 0.0;
    let mut byte_counter = vec![0_u64; 256];
    let mut probabilities = vec![0.0; 256];
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();
    let total_bytes = std::fs::read(&file_name).unwrap();
    let total_len = total_bytes.len() as f64;
    for ref byte in total_bytes {
        *byte_counter.get_mut(*byte as usize).unwrap() += 1;
    }
    for (k, v) in byte_counter.iter().enumerate() {
        probabilities[k] = (*v as f64) / total_len;
    }
    for v in probabilities.iter() {
        let propability = *v;
        entropy += propability * (1.0 / propability).log2();
    }
    entropy = entropy.abs();
    let mut props = Vec::with_capacity(probabilities.len());

    for v in probabilities {
        props.push(v);
    }

    std::fs::write(
        format!("{}_entropy.html", &file_name),
        render_frame(entropy, props),
    )
    .unwrap();
    println!("finished file written to {}_entropy.html", &file_name);
}

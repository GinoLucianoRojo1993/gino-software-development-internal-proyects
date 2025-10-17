// Author: Gino Luciano Rojo
use std::fs::File; use std::io::{self,BufRead,BufReader,Write};
fn main(){
    println!("AR(1) CLI by Gino Luciano Rojo");
    print!("CSV path: "); std::io::stdout().flush().unwrap();
    let mut p=String::new(); std::io::stdin().read_line(&mut p).unwrap(); let p=p.trim();
    let f=File::open(p).expect("open"); let mut xs:Vec<f64>=Vec::new();
    for line in BufReader::new(f).lines(){
        if let Ok(l)=line{ if l.trim().is_empty(){continue;}
            if let Ok(v)=l.trim().parse::<f64>(){ xs.push(v);} else {
                if let Some(tok)=l.split(',').next(){ if let Ok(v2)=tok.parse::<f64>(){ xs.push(v2); } }
            }
        }
    }
    if xs.len()<3 { eprintln!("Need >=3 values."); return; }
    let mut num=0.0; let mut den=0.0;
    for t in 1..xs.len(){ num+=xs[t]*xs[t-1]; den+=xs[t-1]*xs[t-1]; }
    let phi= if den!=0.0 { num/den } else { 0.0 };
    let mut sse=0.0; for t in 1..xs.len(){ let e = xs[t]-phi*xs[t-1]; sse+=e*e; }
    let sigma2 = sse/((xs.len()-1) as f64);
    println!("phi={:.4} sigma2={:.4}", phi, sigma2);
    print!("horizon h: "); std::io::stdout().flush().unwrap(); let mut sh=String::new(); std::io::stdin().read_line(&mut sh).unwrap();
    let h:usize = sh.trim().parse().unwrap_or(5); let mut last=*xs.last().unwrap(); let mut fc=Vec::new();
    for _ in 0..h { last = phi*last; fc.push(last); }
    println!("forecasts={:?}", fc);
}

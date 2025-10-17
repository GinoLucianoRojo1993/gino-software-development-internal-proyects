
// Author: Gino Luciano Rojo
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
#[derive(Serialize)] struct Spiro{ time:Vec<f64>, flow:Vec<f64>, fev1:f64, fvc:f64, ratio:f64 }
#[get("/api/spiro")] async fn spiro()->impl Responder{
    let fs=100.0; let n=(5.0*fs) as usize;
    let mut time=Vec::with_capacity(n); let mut flow=Vec::with_capacity(n);
    for i in 0..n { let t=i as f64/fs; let f=5.0*(-t/1.5).exp() + 0.1*(t.sin()); time.push(t); flow.push(f.max(0.0)); }
    let mut vol=vec![0.0; n]; for i in 1..n { vol[i]=vol[i-1]+(flow[i]+flow[i-1])*(0.5/fs); }
    let fvc=*vol.last().unwrap_or(&0.0); let idx1=(1.0*fs) as usize; let fev1= if idx1<vol.len(){vol[idx1]} else {fvc}; let ratio= if fvc>0.0{fev1/fvc}else{0.0};
    web::Json(Spiro{time,flow,fev1,fvc,ratio})
}
#[actix_web::main] async fn main()->std::io::Result<()> { HttpServer::new(||App::new().service(spiro)).bind(("127.0.0.1",8098))?.run().await }

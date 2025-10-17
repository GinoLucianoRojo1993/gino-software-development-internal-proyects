
# Author: Gino Luciano Rojo
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import math
from typing import List

app = FastAPI(title="Bio Device Gateway")
app.add_middleware(CORSMiddleware, allow_origins=["*"], allow_methods=["*"], allow_headers=["*"])

STATE = {"fs": 200.0, "bpm": 72.0}

@app.get("/api/state")
def get_state(): return STATE

@app.post("/api/state")
def set_state(fs: float = 200.0, bpm: float = 72.0):
    STATE["fs"] = fs; STATE["bpm"] = bpm
    return STATE

@app.get("/api/ecg", response_model=List[float])
def ecg(n: int = 500):
    fs = STATE["fs"]; bpm = STATE["bpm"]
    out = []
    for i in range(n):
        baseline = 0.05*math.sin(2*math.pi*1.2*(i/fs))
        spike = 1.0 if (i % int(fs*60.0/bpm) == 0) else 0.0
        out.append(baseline + spike)
    return out

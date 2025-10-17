
# Author: Gino Luciano Rojo
import random, math
def mean(xs): return sum(xs)/len(xs)
def sd(xs): m=mean(xs); return (sum((x-m)**2 for x in xs)/(len(xs)-1))**0.5 if len(xs)>1 else 0.0
def bootstrap_mean_ci(xs,R=2000,alpha=0.05):
    means=[]; import random
    for _ in range(R):
        s=[xs[random.randrange(len(xs))] for _ in xs]
        means.append(mean(s))
    means.sort(); lo=means[int((alpha/2)*R)]; hi=means[int((1-alpha/2)*R)]
    return lo,hi
def linreg(x,y):
    n=len(x); mx=mean(x); my=mean(y)
    sxy=sum((xi-mx)*(yi-my) for xi,yi in zip(x,y)); sxx=sum((xi-mx)**2 for xi in x)
    b=sxy/sxx if sxx!=0 else 0.0; a=my-b*mx
    ss_tot=sum((yi-my)**2 for yi in y); ss_res=sum((yi-(a+b*xi))**2 for xi,yi in zip(x,y))
    r2=1-ss_res/ss_tot if ss_tot>0 else 0.0; return a,b,r2
def get_list(prompt):
    while True:
        s=input(prompt).strip()
        try:
            xs=[float(t) for t in s.split()]
            if xs: return xs
        except: pass
        print("Please enter space-separated numbers.")
def main():
    print("Statistics Tutor by Gino Luciano Rojo")
    xs=get_list("Enter numbers: ")
    m=mean(xs); s=sd(xs); lo,hi=bootstrap_mean_ci(xs)
    print(f"n={len(xs)} mean={m:.4f} sd={s:.4f} 95%CI=[{lo:.4f},{hi:.4f}]")
    if input("Linear regression? (y/n) ").strip().lower()=='y':
        x=get_list("x values: "); y=get_list("y values: ")
        if len(x)!=len(y): print("Length mismatch."); return
        a,b,r2=linreg(x,y); print(f"y = {a:.4f} + {b:.4f} x (R^2={r2:.4f})")
if __name__=='__main__': main()

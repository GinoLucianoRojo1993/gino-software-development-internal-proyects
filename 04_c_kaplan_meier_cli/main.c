/* Author: Gino Luciano Rojo */
#include <stdio.h>
#include <stdlib.h>
typedef struct{double t;int e;} Row;
static int cmp(const void*a,const void*b){const Row*x=a;const Row*y=b;return (x->t<y->t)?-1:(x->t>y->t);}
int main(){char path[256];printf("Kaplan-Meier by Gino Luciano Rojo\nFile path: "); if(!fgets(path,256,stdin))return 1; for(char*p=path;*p;p++){if(*p=='\n'){*p=0;break;}} FILE*f=fopen(path,"r"); if(!f){perror("open");return 1;} Row* A=0; size_t n=0; char line[256]; while(fgets(line,256,f)){double t;int e; if(sscanf(line,"%lf,%d",&t,&e)==2){A=realloc(A,(n+1)*sizeof(Row));A[n].t=t;A[n].e=e;n++;}} fclose(f); if(!n){printf("No data\n");return 1;} qsort(A,n,sizeof(Row),cmp); double S=1.0; printf("time,S(t)\n"); for(size_t i=0;i<n;){double t=A[i].t;int d=0;size_t j=i;for(;j<n&&A[j].t==t;j++) if(A[j].e==1) d++; int risk=0; for(size_t k=0;k<n;k++) if(A[k].t>=t) risk++; if(risk>0){ S*= (1.0-(double)d/risk); if(d>0) printf("%g,%.6f\n",t,S);} i=j;} free(A);}

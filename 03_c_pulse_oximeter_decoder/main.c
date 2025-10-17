/* Author: Gino Luciano Rojo */
#include <stdio.h>
#include <stdint.h>
#include <ctype.h>
#include <string.h>
static int hx(const char*s,uint8_t*out){int v=0;for(int i=0;i<2;i++){char c=s[i];if(!isxdigit((unsigned char)c))return 0;v=v*16+(isdigit((unsigned char)c)?c-'0':(tolower(c)-'a'+10));}*out=(uint8_t)v;return 1;}
int main(){printf("Pulse Oximeter Decoder by Gino Luciano Rojo\n");printf("Enter hex bytes (e.g., AA 4B 63 08)\n");char tok[8];uint8_t b[4];int i=0;while(scanf("%2s",tok)==1){uint8_t v;if(!hx(tok,&v)){printf("bad hex\n");continue;}b[i++]=v;if(i==4){if(b[0]!=0xAA)printf("bad start %02X\n",b[0]);else{uint8_t pr=b[1],sp=b[2],chk=b[3],ex=(uint8_t)(0xAA^pr^sp); if(chk!=ex) printf("bad chk\n"); else printf("PR=%u bpm, SpO2=%u%%\n",pr,sp);}i=0;}}}

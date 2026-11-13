import {iterate as base} from './base/bench.ts';
import {iterate as special} from './special/translated/bench.ts';
import {putUsize} from './base/harness.ts';
import {ByteSlice,ByteVec} from './base/rs.ts';
const bytes=Uint8Array.from({length:1024},(_,i)=>i&255);
const data=new ByteSlice(bytes,0,bytes.length);
(globalThis as any).measure=()=>{
 const results:any={base:[],special:[],hand:[]};let sink=0;
 const fns={base,special,hand:(_:number,d:any,out:any)=>{let n=0;for(const b of bytes)if((b>>1)<(b^3))n++;putUsize(out,n);}};
 for(let r=0;r<15;r++)for(const name of r%2?['special','base','hand']:['base','special','hand']){
  const f=fns[name];const start=performance.now();for(let i=0;i<10000;i++){const out=new ByteVec();f(8,data,out);sink+=out.length;}results[name].push((performance.now()-start)*1e6/10000/1024);
 }
 return {results,sink};
};
(globalThis as any).validate=()=>{
 for(let n=0;n<1025;n++){
  const bytes=Uint8Array.from({length:n},(_,i)=>(i*73+n)&255);
  const d=new ByteSlice(bytes,0,n), a=new ByteVec(), b=new ByteVec();
  base(8,d,a);special(8,d,b);
  if(JSON.stringify(a)!==JSON.stringify(b))throw new Error(`mismatch at length ${n}`);
 }
 return '1025 lengths match translated baseline';
};

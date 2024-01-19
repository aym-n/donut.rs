
                               use std::{thread,               
                            time }; fn main(){ let (          
                          mut a,mut b)=(1.0_f64,1.0_f64        
                        );loop{a+=0.07;b+=0.03;let ((c,d),     
                     (e,f),mut g,mut h,mut i)=(a.sin_cos(),     
                    b.sin_cos(),vec![' ';1920],vec![0.0;1920]   
                   ,0.0_f64);while i<6.28{let ((j, k),mut l)=(i.   
                   sin_cos(),0.0_f64);while l<6.28{let (m,n)= l.  
                  sin_cos();let o=2.0     +k;let (x,y,z)=(o*(f*n
                  +c*e*m)-j*d*e,o*(          e*n-c*f*m)+j*d*f,5.0 
                  +d*o*m+j*c);let              (q,t)=(1.0/z,k*n*e- 
                   d*k*m-c*j+f*(d*j           -k*c*m));let r=(40.0
                   +30.0*q*x)as usize       ;let s=(12.0+15.0*q*y)
                    as usize;if t>0.0{if    q>h[s*80+r]{h[s*80+r]
                     =q;g[s*80+r]=  ".`^:;i<+]1(tjxcXCQOma#W8@$"
                     .chars().nth((t * 17.5)as usize).unwrap();
                      }};l+=0.02;}i+=0.07;}print!(/*donut.rs*/
                        "\x1b[H{}{}",g.iter().collect::/*by*/
                           <String>(), "\x1b[2J");thread:://
                             sleep(time::Duration::/*#@!@*/
                                from_millis(50));}}/*$%&*/
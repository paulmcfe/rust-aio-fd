fn messy_function(x:i32,y:i32)->i32{
let result=if x>y{
x*2
}else{
y*2
};
return result;
}

fn main(){
let a=5;let b=3;
let answer=messy_function(a,b);
println!("Answer: {}",answer);
}
#![feature(type_alias_impl_trait)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(member_constraints)]

use std::ops::Shr;

pub struct FnCompose<A, B> {
    func: Box<dyn Fn(A) -> B>
}

impl<A, B> Fn<A> for FnCompose<A, B> {
    extern "rust-call" fn call(&self, args: A) -> Self::Output {
        self.func.call((args,))
    }
}

impl<A, B> FnMut<A> for FnCompose<A, B> {
    extern "rust-call" fn call_mut(&mut self, args: A) -> Self::Output {
        self.call(args)
    }
}

impl<A, B> FnOnce<A> for FnCompose<A, B> {
    type Output = B;

    extern "rust-call" fn call_once(self, args: A) -> Self::Output {
        self.call(args)
    }
}


impl<A: 'static, B: 'static, C: 'static> Shr<FnCompose<B, C>> for FnCompose<A, B> {
    type Output = FnCompose<A, C>;
    
    fn shr(self, other: FnCompose<B, C>) -> Self::Output {
        Self::Output { func: compose(other.func, self.func) }
    }
    
}

fn compose<A, B, C, F, G>(f: F, g: G) -> Box<dyn Fn(A) -> C>
where
    F: Fn(B) -> C + 'static,
    G: Fn(A) -> B + 'static,
{
    Box::new(move |x| f(g(x)))
}

fn main() {
    let f = |x: i32| x + 1;
    let g = |x| x * 2;
    let comp = FnCompose { func: Box::new(f) } >> FnCompose { func: Box::new(g) };

    println!("{:?}", (*comp.func).call((2,)));
}

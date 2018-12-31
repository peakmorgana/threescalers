#![feature(test)]
extern crate test;

pub mod errors;
pub mod credentials;
pub mod service;
pub mod application;
pub mod user;
pub mod usage;
pub mod apicall;
pub mod http;


pub trait ToParams<'k, 'v, E> where E: Extend<(&'k str, &'v str)> {
    fn to_params<'s: 'k + 'v>(&'s self, extendable: &mut E);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

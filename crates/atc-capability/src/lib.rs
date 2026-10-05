//! Capability policy boundary; authority remains outside this repository.
#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]pub enum Capability{Network,Filesystem,Clock,Randomness,Threads,Storage,Crypto}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum Profile{Contract,Application,System,Unrestricted}
#[derive(Debug,Clone,PartialEq,Eq)]pub struct CapabilityPolicy{pub profile:Profile,pub allowed:Vec<Capability>}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]pub enum CapabilityError{Denied(Capability)}
impl CapabilityPolicy{pub fn allows(&self,c:Capability)->bool{self.profile==Profile::Unrestricted||self.allowed.contains(&c)}pub fn require(&self,c:Capability)->Result<(),CapabilityError>{if self.allows(c){Ok(())}else{Err(CapabilityError::Denied(c))}}}
#[cfg(test)]mod tests{use super::*;#[test]fn explicit(){let p=CapabilityPolicy{profile:Profile::Contract,allowed:vec![Capability::Crypto]};assert!(p.require(Capability::Crypto).is_ok());assert!(p.require(Capability::Filesystem).is_err())}}

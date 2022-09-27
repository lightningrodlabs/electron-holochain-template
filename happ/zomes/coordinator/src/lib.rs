use hdk::prelude::*;
// use integrity::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}


#[hdk_extern]
pub fn first_zome_fn(_: ()) -> ExternResult<i32> {
    Ok(100)
}

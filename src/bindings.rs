pub mod host {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  pub trait Host: Sized {
    fn print(&mut self,message: & str,);
    
    fn double(&mut self,num: f64,) -> f64;
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Host
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    linker.func_wrap("host", "print", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32| {
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let param0 = _bc.slice_str(ptr0, len0)?;
      host.print(param0, );
      Ok(())
    })?;
    linker.func_wrap("host", "double", move |mut caller: wasmtime::Caller<'_, T>,arg0:f64| {
      let host = get(caller.data_mut());
      let param0 = arg0;
      let result0 = host.double(param0, );
      Ok(result0)
    })?;
    Ok(())
  }
}

pub mod host {
  #[allow(unused_imports)]
  use wit_bindgen_wasmtime::{wasmtime, anyhow};
  pub type Dbrecord = (String,Vec<(String,String,)>,);
  pub trait Host: Sized {
    fn print(&mut self,message: & str,);
    
    fn get_args(&mut self,) -> Vec<String>;
    
    fn get_all(&mut self,) -> Vec<Dbrecord>;
    
    fn get(&mut self,key: & str,) -> Dbrecord;
    
    fn upsert(&mut self,key: & str,values: Vec<(& str,& str,)>,);
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Host
  {
    use wit_bindgen_wasmtime::rt::get_memory;
    use wit_bindgen_wasmtime::rt::get_func;
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
    linker.func_wrap("host", "get-args", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32| {
      
      let func = get_func(&mut caller, "canonical_abi_realloc")?;
      let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let host = get(caller.data_mut());
      let result0 = host.get_args();
      let vec2 = result0;
      let len2 = vec2.len() as i32;
      let result2 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len2 * 8))?;
      for (i, e) in vec2.into_iter().enumerate() {
        let base = result2 + (i as i32) * 8;
        {
          let vec1 = e;
          let ptr1 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec1.len() as i32) * 1))?;
          let caller_memory = memory.data_mut(&mut caller);
          caller_memory.store_many(ptr1, vec1.as_ref())?;
          caller_memory.store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec1.len() as i32))?;
          caller_memory.store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr1))?;
        }}let caller_memory = memory.data_mut(&mut caller);
        caller_memory.store(arg0 + 8, wit_bindgen_wasmtime::rt::as_i32(len2))?;
        caller_memory.store(arg0 + 0, wit_bindgen_wasmtime::rt::as_i32(result2))?;
        Ok(())
      })?;
      linker.func_wrap("host", "get-all", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32| {
        
        let func = get_func(&mut caller, "canonical_abi_realloc")?;
        let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
        let memory = &get_memory(&mut caller, "memory")?;
        let host = get(caller.data_mut());
        let result0 = host.get_all();
        let vec7 = result0;
        let len7 = vec7.len() as i32;
        let result7 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len7 * 16))?;
        for (i, e) in vec7.into_iter().enumerate() {
          let base = result7 + (i as i32) * 16;
          {
            let (t1_0, t1_1, ) = e;
            let vec2 = t1_0;
            let ptr2 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec2.len() as i32) * 1))?;
            let caller_memory = memory.data_mut(&mut caller);
            caller_memory.store_many(ptr2, vec2.as_ref())?;
            caller_memory.store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec2.len() as i32))?;
            caller_memory.store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr2))?;
            let vec6 = t1_1;
            let len6 = vec6.len() as i32;
            let result6 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len6 * 16))?;
            for (i, e) in vec6.into_iter().enumerate() {
              let base = result6 + (i as i32) * 16;
              {
                let (t3_0, t3_1, ) = e;
                let vec4 = t3_0;
                let ptr4 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec4.len() as i32) * 1))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr4, vec4.as_ref())?;
                caller_memory.store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec4.len() as i32))?;
                caller_memory.store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr4))?;
                let vec5 = t3_1;
                let ptr5 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec5.len() as i32) * 1))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr5, vec5.as_ref())?;
                caller_memory.store(base + 12, wit_bindgen_wasmtime::rt::as_i32(vec5.len() as i32))?;
                caller_memory.store(base + 8, wit_bindgen_wasmtime::rt::as_i32(ptr5))?;
              }}let caller_memory = memory.data_mut(&mut caller);
              caller_memory.store(base + 12, wit_bindgen_wasmtime::rt::as_i32(len6))?;
              caller_memory.store(base + 8, wit_bindgen_wasmtime::rt::as_i32(result6))?;
            }}let caller_memory = memory.data_mut(&mut caller);
            caller_memory.store(arg0 + 8, wit_bindgen_wasmtime::rt::as_i32(len7))?;
            caller_memory.store(arg0 + 0, wit_bindgen_wasmtime::rt::as_i32(result7))?;
            Ok(())
          })?;
          linker.func_wrap("host", "get", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32| {
            
            let func = get_func(&mut caller, "canonical_abi_realloc")?;
            let func_canonical_abi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
            let memory = &get_memory(&mut caller, "memory")?;
            let (mem, data) = memory.data_and_store_mut(&mut caller);
            let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
            let host = get(data);
            let ptr0 = arg0;
            let len0 = arg1;
            let param0 = _bc.slice_str(ptr0, len0)?;
            let result1 = host.get(param0, );
            let (t2_0, t2_1, ) = result1;
            let vec3 = t2_0;
            let ptr3 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec3.len() as i32) * 1))?;
            let caller_memory = memory.data_mut(&mut caller);
            caller_memory.store_many(ptr3, vec3.as_ref())?;
            let vec7 = t2_1;
            let len7 = vec7.len() as i32;
            let result7 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 4, len7 * 16))?;
            for (i, e) in vec7.into_iter().enumerate() {
              let base = result7 + (i as i32) * 16;
              {
                let (t4_0, t4_1, ) = e;
                let vec5 = t4_0;
                let ptr5 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec5.len() as i32) * 1))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr5, vec5.as_ref())?;
                caller_memory.store(base + 4, wit_bindgen_wasmtime::rt::as_i32(vec5.len() as i32))?;
                caller_memory.store(base + 0, wit_bindgen_wasmtime::rt::as_i32(ptr5))?;
                let vec6 = t4_1;
                let ptr6 = func_canonical_abi_realloc.call(&mut caller, (0, 0, 1, (vec6.len() as i32) * 1))?;
                let caller_memory = memory.data_mut(&mut caller);
                caller_memory.store_many(ptr6, vec6.as_ref())?;
                caller_memory.store(base + 12, wit_bindgen_wasmtime::rt::as_i32(vec6.len() as i32))?;
                caller_memory.store(base + 8, wit_bindgen_wasmtime::rt::as_i32(ptr6))?;
              }}let caller_memory = memory.data_mut(&mut caller);
              caller_memory.store(arg2 + 24, wit_bindgen_wasmtime::rt::as_i32(len7))?;
              caller_memory.store(arg2 + 16, wit_bindgen_wasmtime::rt::as_i32(result7))?;
              caller_memory.store(arg2 + 8, wit_bindgen_wasmtime::rt::as_i32(vec3.len() as i32))?;
              caller_memory.store(arg2 + 0, wit_bindgen_wasmtime::rt::as_i32(ptr3))?;
              Ok(())
            })?;
            linker.func_wrap("host", "upsert", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32| {
              let memory = &get_memory(&mut caller, "memory")?;
              let (mem, data) = memory.data_and_store_mut(&mut caller);
              let mut _bc = wit_bindgen_wasmtime::BorrowChecker::new(mem);
              let host = get(data);
              let ptr0 = arg0;
              let len0 = arg1;
              let len7 = arg3;
              let base7 = arg2;
              let mut result7 = Vec::with_capacity(len7 as usize);
              for i in 0..len7 {
                let base = base7 + i *16;
                result7.push({
                  let load1 = _bc.load::<i32>(base + 0)?;
                  let load2 = _bc.load::<i32>(base + 4)?;
                  let ptr3 = load1;
                  let len3 = load2;
                  let load4 = _bc.load::<i32>(base + 8)?;
                  let load5 = _bc.load::<i32>(base + 12)?;
                  let ptr6 = load4;
                  let len6 = load5;
                  (_bc.slice_str(ptr3, len3)?, _bc.slice_str(ptr6, len6)?)
                });
              }
              let param0 = _bc.slice_str(ptr0, len0)?;
              let param1 = result7;
              host.upsert(param0, param1, );
              Ok(())
            })?;
            Ok(())
          }
          use wit_bindgen_wasmtime::rt::RawMem;
        }
        
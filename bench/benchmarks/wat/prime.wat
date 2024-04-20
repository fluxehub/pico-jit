(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 f32)
    local.get 1
    i32.const 2
    i32.lt_s
    if  ;; label = @1
      i32.const 0
      return
    end
    local.get 1
    f32.convert_i32_s
    f32.sqrt
    local.tee 8
    f32.const 0x1p+1 (;=2;)
    f32.ge
    if  ;; label = @1
      i32.const 4
      local.set 4
      i32.const 2
      local.set 3
      i32.const 2
      local.set 2
      loop  ;; label = @2
        block  ;; label = @3
          local.get 0
          local.get 2
          i32.add
          i32.load8_u
          br_if 0 (;@3;)
          local.get 2
          local.get 2
          i32.mul
          local.tee 6
          local.get 1
          i32.ge_s
          br_if 0 (;@3;)
          i32.const 0
          local.set 5
          local.get 3
          local.set 7
          loop  ;; label = @4
            local.get 0
            local.get 6
            i32.add
            i32.const 1
            i32.store8
            local.get 2
            local.get 5
            i32.add
            local.get 2
            i32.mul
            local.set 6
            local.get 5
            i32.const 1
            i32.add
            local.set 5
            local.get 2
            local.get 7
            i32.add
            local.tee 7
            local.get 1
            i32.lt_s
            br_if 0 (;@4;)
          end
        end
        local.get 3
        local.get 4
        i32.add
        local.set 3
        local.get 4
        i32.const 2
        i32.add
        local.set 4
        local.get 8
        local.get 2
        i32.const 1
        i32.add
        local.tee 2
        f32.convert_i32_s
        f32.ge
        br_if 0 (;@2;)
      end
    end
    local.get 0
    i32.const 1
    i32.sub
    local.set 0
    loop  ;; label = @1
      local.get 1
      i32.const 3
      i32.lt_s
      if  ;; label = @2
        i32.const 0
        return
      end
      local.get 0
      local.get 1
      i32.add
      local.get 1
      i32.const 1
      i32.sub
      local.set 1
      i32.load8_u
      br_if 0 (;@1;)
    end
    local.get 1)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "prime" (func 0)))

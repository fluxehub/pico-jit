(module
  (type (;0;) (func (param i32) (result i32)))
  (func (;0;) (type 0) (param i32) (result i32)
    (local i32 i32 i32 i32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32 f32)
    local.get 0
    i32.const 0
    i32.le_s
    if  ;; label = @1
      i32.const 0
      return
    end
    local.get 0
    f32.convert_i32_s
    local.set 6
    i32.const 8
    local.get 0
    i32.const 7
    i32.and
    i32.sub
    local.set 4
    local.get 0
    i32.const 1
    i32.sub
    f32.convert_i32_s
    local.set 14
    loop  ;; label = @1
      local.get 7
      local.get 7
      f32.add
      local.get 6
      f32.div
      f32.const -0x1p+0 (;=-1;)
      f32.add
      local.set 15
      f32.const 0x0p+0 (;=0;)
      local.set 5
      loop  ;; label = @2
        local.get 5
        local.get 5
        f32.add
        local.get 6
        f32.div
        f32.const -0x1.8p+0 (;=-1.5;)
        f32.add
        local.set 16
        i32.const 0
        local.set 0
        f32.const 0x0p+0 (;=0;)
        local.set 10
        f32.const 0x0p+0 (;=0;)
        local.set 11
        f32.const 0x0p+0 (;=0;)
        local.set 8
        f32.const 0x0p+0 (;=0;)
        local.set 9
        loop  ;; label = @3
          block  ;; label = @4
            local.get 16
            local.get 11
            local.get 10
            f32.sub
            f32.add
            local.tee 12
            local.get 12
            f32.mul
            local.tee 11
            local.get 9
            local.get 9
            f32.add
            local.get 8
            f32.mul
            local.get 15
            f32.add
            local.tee 8
            local.get 8
            f32.mul
            local.tee 10
            f32.add
            local.set 13
            local.get 0
            i32.const 48
            i32.gt_u
            br_if 0 (;@4;)
            local.get 0
            i32.const 1
            i32.add
            local.set 0
            local.get 12
            local.set 9
            local.get 13
            f32.const 0x1p+2 (;=4;)
            f32.le
            br_if 1 (;@3;)
          end
        end
        local.get 1
        i32.const 1
        i32.shl
        local.get 13
        f32.const 0x1p+2 (;=4;)
        f32.le
        i32.or
        local.set 1
        block  ;; label = @3
          block (result i32)  ;; label = @4
            local.get 3
            i32.const 1
            i32.add
            local.tee 3
            i32.const 8
            i32.ne
            if  ;; label = @5
              local.get 5
              local.get 14
              f32.ne
              br_if 2 (;@3;)
              local.get 2
              local.get 1
              local.get 4
              i32.shl
              i32.extend8_s
              i32.sub
              br 1 (;@4;)
            end
            local.get 2
            local.get 1
            i32.extend8_s
            i32.add
          end
          local.set 2
          i32.const 0
          local.set 1
          i32.const 0
          local.set 3
        end
        local.get 5
        f32.const 0x1p+0 (;=1;)
        f32.add
        local.tee 5
        local.get 6
        f32.lt
        br_if 0 (;@2;)
      end
      local.get 7
      f32.const 0x1p+0 (;=1;)
      f32.add
      local.tee 7
      local.get 6
      f32.lt
      br_if 0 (;@1;)
    end
    local.get 2)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "mandelbrot" (func 0)))

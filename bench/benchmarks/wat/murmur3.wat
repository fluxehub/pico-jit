(module
  (type (;0;) (func (param i32 i32) (result i32)))
  (func (;0;) (type 0) (param i32 i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 1
    i32.const 16
    i32.div_s
    local.tee 9
    i32.const 4
    i32.shl
    local.set 10
    local.get 1
    i32.const 15
    i32.add
    i32.const 31
    i32.ge_u
    if  ;; label = @1
      local.get 0
      local.set 2
      loop  ;; label = @2
        local.get 2
        i32.const 12
        i32.add
        local.set 11
        local.get 4
        local.get 2
        i32.load
        local.tee 3
        i32.const 597399067
        i32.mul
        i32.const 17
        i32.shr_u
        local.get 3
        i32.const -888307712
        i32.mul
        i32.or
        i32.const -1425107063
        i32.mul
        i32.xor
        i32.const 19
        i32.rotl
        local.get 7
        i32.add
        local.get 2
        i32.const 8
        i32.add
        local.set 8
        local.get 7
        local.get 2
        i32.const 4
        i32.add
        i32.load
        local.tee 3
        i32.const -1425107063
        i32.mul
        i32.const 16
        i32.shr_u
        local.get 3
        i32.const -1752629248
        i32.mul
        i32.or
        i32.const 951274213
        i32.mul
        i32.xor
        i32.const 17
        i32.rotl
        local.get 6
        i32.add
        i32.const 5
        i32.mul
        i32.const 197830471
        i32.add
        local.set 7
        local.get 2
        i32.const 16
        i32.add
        local.set 2
        local.get 6
        local.get 8
        i32.load
        local.tee 3
        i32.const 951274213
        i32.mul
        i32.const 15
        i32.shr_u
        local.get 3
        i32.const -1781923840
        i32.mul
        i32.or
        i32.const -1578923117
        i32.mul
        i32.xor
        i32.const 15
        i32.rotl
        local.get 5
        i32.add
        i32.const 5
        i32.mul
        i32.const 1764942795
        i32.sub
        local.set 6
        i32.const 5
        i32.mul
        i32.const 1444728091
        i32.add
        local.tee 4
        local.get 5
        local.get 11
        i32.load
        local.tee 3
        i32.const -1578923117
        i32.mul
        i32.const 14
        i32.shr_u
        local.get 3
        i32.const 776732672
        i32.mul
        i32.or
        i32.const 597399067
        i32.mul
        i32.xor
        i32.const 13
        i32.rotl
        i32.add
        i32.const 5
        i32.mul
        i32.const 850148119
        i32.add
        local.set 5
        local.get 9
        i32.const 1
        i32.sub
        local.tee 9
        br_if 0 (;@2;)
      end
    end
    local.get 0
    local.get 10
    i32.add
    local.set 3
    i32.const 0
    local.set 2
    block  ;; label = @1
      block  ;; label = @2
        block  ;; label = @3
          block  ;; label = @4
            block  ;; label = @5
              block  ;; label = @6
                block  ;; label = @7
                  block  ;; label = @8
                    block  ;; label = @9
                      block  ;; label = @10
                        block  ;; label = @11
                          block  ;; label = @12
                            block  ;; label = @13
                              block  ;; label = @14
                                block  ;; label = @15
                                  block  ;; label = @16
                                    local.get 1
                                    i32.const 15
                                    i32.and
                                    i32.const 1
                                    i32.sub
                                    br_table 14 (;@2;) 13 (;@3;) 12 (;@4;) 11 (;@5;) 10 (;@6;) 9 (;@7;) 8 (;@8;) 7 (;@9;) 6 (;@10;) 5 (;@11;) 4 (;@12;) 3 (;@13;) 2 (;@14;) 1 (;@15;) 0 (;@16;) 15 (;@1;)
                                  end
                                  local.get 3
                                  i32.load8_u offset=14
                                  i32.const 16
                                  i32.shl
                                  local.set 2
                                end
                                local.get 3
                                i32.load8_u offset=13
                                i32.const 8
                                i32.shl
                                local.get 2
                                i32.or
                                local.set 2
                              end
                              local.get 5
                              local.get 2
                              local.get 3
                              i32.load8_u offset=12
                              i32.xor
                              local.tee 0
                              i32.const -1578923117
                              i32.mul
                              i32.const 14
                              i32.shr_u
                              local.get 0
                              i32.const 776732672
                              i32.mul
                              i32.or
                              i32.const 597399067
                              i32.mul
                              i32.xor
                              local.set 5
                            end
                            local.get 3
                            i32.load8_u offset=11
                            i32.const 24
                            i32.shl
                            local.set 2
                          end
                          local.get 3
                          i32.load8_u offset=10
                          i32.const 16
                          i32.shl
                          local.get 2
                          i32.or
                          local.set 2
                        end
                        local.get 3
                        i32.load8_u offset=9
                        i32.const 8
                        i32.shl
                        local.get 2
                        i32.xor
                        local.set 2
                      end
                      local.get 6
                      local.get 2
                      local.get 3
                      i32.load8_u offset=8
                      i32.xor
                      local.tee 0
                      i32.const 951274213
                      i32.mul
                      i32.const 15
                      i32.shr_u
                      local.get 0
                      i32.const -1781923840
                      i32.mul
                      i32.or
                      i32.const -1578923117
                      i32.mul
                      i32.xor
                      local.set 6
                    end
                    local.get 3
                    i32.load8_u offset=7
                    i32.const 24
                    i32.shl
                    local.set 2
                  end
                  local.get 3
                  i32.load8_u offset=6
                  i32.const 16
                  i32.shl
                  local.get 2
                  i32.or
                  local.set 2
                end
                local.get 3
                i32.load8_u offset=5
                i32.const 8
                i32.shl
                local.get 2
                i32.xor
                local.set 2
              end
              local.get 7
              local.get 2
              local.get 3
              i32.load8_u offset=4
              i32.xor
              local.tee 0
              i32.const -1425107063
              i32.mul
              i32.const 16
              i32.shr_u
              local.get 0
              i32.const -1752629248
              i32.mul
              i32.or
              i32.const 951274213
              i32.mul
              i32.xor
              local.set 7
            end
            local.get 3
            i32.load8_u offset=3
            i32.const 24
            i32.shl
            local.set 2
          end
          local.get 3
          i32.load8_u offset=2
          i32.const 16
          i32.shl
          local.get 2
          i32.or
          local.set 2
        end
        local.get 3
        i32.load8_u offset=1
        i32.const 8
        i32.shl
        local.get 2
        i32.xor
        local.set 2
      end
      local.get 2
      local.get 3
      i32.load8_u
      i32.xor
      local.tee 0
      i32.const 597399067
      i32.mul
      i32.const 17
      i32.shr_u
      local.get 0
      i32.const -888307712
      i32.mul
      i32.or
      i32.const -1425107063
      i32.mul
      local.get 4
      i32.xor
      local.set 4
    end
    i32.const 66560
    local.get 1
    local.get 7
    i32.xor
    local.tee 0
    local.get 1
    local.get 6
    i32.xor
    local.tee 8
    local.get 1
    local.get 5
    i32.xor
    local.tee 3
    i32.add
    i32.add
    local.get 1
    local.get 4
    i32.xor
    i32.add
    local.tee 4
    local.get 0
    i32.add
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -2048144789
    i32.mul
    local.tee 0
    i32.const 13
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -1028477387
    i32.mul
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    local.tee 2
    local.get 4
    i32.const 16
    i32.shr_u
    local.get 4
    i32.xor
    i32.const -2048144789
    i32.mul
    local.tee 0
    i32.const 13
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -1028477387
    i32.mul
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    i32.add
    local.get 4
    local.get 8
    i32.add
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -2048144789
    i32.mul
    local.tee 0
    i32.const 13
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -1028477387
    i32.mul
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    local.tee 1
    i32.add
    local.get 3
    local.get 4
    i32.add
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -2048144789
    i32.mul
    local.tee 0
    i32.const 13
    i32.shr_u
    local.get 0
    i32.xor
    i32.const -1028477387
    i32.mul
    local.tee 0
    i32.const 16
    i32.shr_u
    local.get 0
    i32.xor
    local.tee 0
    i32.add
    local.tee 3
    i32.store
    i32.const 66572
    local.get 0
    local.get 3
    i32.add
    i32.store
    i32.const 66568
    local.get 1
    local.get 3
    i32.add
    i32.store
    i32.const 66564
    local.get 2
    local.get 3
    i32.add
    i32.store
    i32.const 66560)
  (memory (;0;) 2)
  (export "memory" (memory 0))
  (export "hash" (func 0)))

(module
  (type (;0;) (func (param i32 i32 i32)))
  (type (;1;) (func (param i32 i32 i32 i32)))
  (type (;2;) (func (param i32 i32) (result i32)))
  (type (;3;) (func (param i32) (result i32)))
  (type (;4;) (func))
  (import "env" "abort" (func (;0;) (type 1)))
  (func (;1;) (type 0) (param i32 i32 i32)
    local.get 1
    local.get 0
    i32.const 20
    i32.sub
    i32.load offset=16
    i32.const 2
    i32.shr_u
    i32.ge_u
    if  ;; label = @1
      i32.const 1296
      i32.const 1104
      i32.const 93
      i32.const 41
      call 0
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 2
    i32.shl
    i32.add
    local.get 2
    i32.store)
  (func (;2;) (type 2) (param i32 i32) (result i32)
    local.get 1
    local.get 0
    i32.const 20
    i32.sub
    i32.load offset=16
    i32.const 2
    i32.shr_u
    i32.ge_u
    if  ;; label = @1
      i32.const 1296
      i32.const 1104
      i32.const 78
      i32.const 41
      call 0
      unreachable
    end
    local.get 0
    local.get 1
    i32.const 2
    i32.shl
    i32.add
    i32.load)
  (func (;3;) (type 0) (param i32 i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    local.get 2
    i32.lt_s
    local.get 1
    local.get 2
    i32.or
    i32.const 0
    i32.ge_s
    i32.and
    if  ;; label = @1
      local.get 0
      local.get 1
      call 2
      local.set 5
      local.get 1
      i32.const 1
      i32.sub
      local.set 4
      local.get 2
      i32.const 1
      i32.add
      local.set 3
      loop  ;; label = @2
        loop  ;; label = @3
          local.get 0
          local.get 4
          i32.const 1
          i32.add
          local.tee 4
          call 2
          local.get 5
          i32.lt_s
          br_if 0 (;@3;)
        end
        loop  ;; label = @3
          local.get 0
          local.get 3
          i32.const 1
          i32.sub
          local.tee 3
          call 2
          local.get 5
          i32.gt_s
          br_if 0 (;@3;)
        end
        local.get 3
        local.get 4
        i32.le_s
        if  ;; label = @3
          nop
        else
          local.get 0
          local.get 4
          call 2
          local.set 6
          local.get 0
          local.get 4
          local.get 0
          local.get 3
          call 2
          call 1
          local.get 0
          local.get 3
          local.get 6
          call 1
          br 1 (;@2;)
        end
      end
      local.get 0
      local.get 1
      local.get 3
      call 3
      local.get 0
      local.get 3
      i32.const 1
      i32.add
      local.get 2
      call 3
    end)
  (func (;4;) (type 3) (param i32) (result i32)
    (local i32 i32 i32 i32 i32 i32 i32 i32)
    local.get 0
    i32.const 268435455
    i32.gt_u
    if  ;; label = @1
      i32.const 1056
      i32.const 1104
      i32.const 51
      i32.const 60
      call 0
      unreachable
    end
    local.get 0
    i32.const 2
    i32.shl
    local.tee 4
    i32.const 1073741804
    i32.gt_u
    if  ;; label = @1
      i32.const 1168
      i32.const 1232
      i32.const 86
      i32.const 30
      call 0
      unreachable
    end
    local.get 4
    i32.const 16
    i32.add
    local.tee 3
    i32.const 1073741820
    i32.gt_u
    if  ;; label = @1
      i32.const 1168
      i32.const 1232
      i32.const 33
      i32.const 29
      call 0
      unreachable
    end
    global.get 0
    local.set 2
    global.get 0
    i32.const 4
    i32.add
    local.tee 6
    local.get 3
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    i32.const 4
    i32.sub
    local.tee 3
    i32.add
    local.tee 5
    memory.size
    local.tee 7
    i32.const 16
    i32.shl
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    local.tee 8
    i32.gt_u
    if  ;; label = @1
      local.get 7
      local.get 5
      local.get 8
      i32.sub
      i32.const 65535
      i32.add
      i32.const -65536
      i32.and
      i32.const 16
      i32.shr_u
      local.tee 8
      local.get 7
      local.get 8
      i32.gt_s
      select
      memory.grow
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        local.get 8
        memory.grow
        i32.const 0
        i32.lt_s
        if  ;; label = @3
          unreachable
        end
      end
    end
    local.get 5
    global.set 0
    local.get 2
    local.get 3
    i32.store
    local.get 6
    i32.const 4
    i32.sub
    local.tee 2
    i32.const 0
    i32.store offset=4
    local.get 2
    i32.const 0
    i32.store offset=8
    local.get 2
    i32.const 4
    i32.store offset=12
    local.get 2
    local.get 4
    i32.store offset=16
    local.get 6
    i32.const 16
    i32.add
    local.tee 2
    i32.const 0
    local.get 4
    memory.fill
    loop  ;; label = @1
      local.get 0
      local.get 1
      i32.gt_s
      if  ;; label = @2
        local.get 2
        local.get 1
        local.get 0
        local.get 1
        i32.sub
        call 1
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1 (;@1;)
      end
    end
    local.get 2
    i32.const 0
    local.get 0
    i32.const 1
    i32.sub
    call 3
    local.get 2)
  (func (;5;) (type 4)
    i32.const 1340
    global.set 0)
  (memory (;0;) 2 2)
  (global (;0;) (mut i32) (i32.const 0))
  (export "sort" (func 4))
  (export "memory" (memory 0))
  (start 5)
  (data (;0;) (i32.const 1036) ",")
  (data (;1;) (i32.const 1048) "\02\00\00\00\1c\00\00\00I\00n\00v\00a\00l\00i\00d\00 \00l\00e\00n\00g\00t\00h")
  (data (;2;) (i32.const 1084) "<")
  (data (;3;) (i32.const 1096) "\02\00\00\00&\00\00\00~\00l\00i\00b\00/\00s\00t\00a\00t\00i\00c\00a\00r\00r\00a\00y\00.\00t\00s")
  (data (;4;) (i32.const 1148) "<")
  (data (;5;) (i32.const 1160) "\02\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e")
  (data (;6;) (i32.const 1212) "<")
  (data (;7;) (i32.const 1224) "\02\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00s\00t\00u\00b\00.\00t\00s")
  (data (;8;) (i32.const 1276) "<")
  (data (;9;) (i32.const 1288) "\02\00\00\00$\00\00\00I\00n\00d\00e\00x\00 \00o\00u\00t\00 \00o\00f\00 \00r\00a\00n\00g\00e"))

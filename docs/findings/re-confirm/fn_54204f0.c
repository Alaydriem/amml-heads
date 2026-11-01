
undefined8 FUN_1454204f0(undefined8 param_1,undefined8 param_2,longlong param_3)

{
  int *piVar1;
  undefined8 uVar2;
  undefined1 auStack_90 [8];
  longlong lStack_88;
  undefined *puStack_38;
  undefined8 uStack_30;
  longlong lStack_28;
  undefined8 uStack_20;
  
  uStack_20 = 0xfffffffffffffffe;
  puStack_38 = &UNK_14f8469d4;
  uStack_30 = 0;
  lStack_28 = param_3;
  FUN_144df8eb0(auStack_90,param_3,1);
  uVar2 = FUN_145420ec0(param_1,param_2,auStack_90,&puStack_38);
  if (lStack_88 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_88 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_88);
      LOCK();
      piVar1 = (int *)(lStack_88 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_88);
      }
    }
  }
  if (*(longlong *)(lStack_28 + 8) != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  return uVar2;
}


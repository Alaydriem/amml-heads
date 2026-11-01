
void FUN_1454241b0(longlong param_1,undefined8 param_2,undefined8 *param_3)

{
  int *piVar1;
  longlong lVar2;
  longlong alStack_48 [2];
  undefined8 uStack_38;
  undefined8 uStack_30;
  undefined8 *puStack_20;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  puStack_20 = param_3;
  FUN_1454756c0(param_1 + 0x1a8,alStack_48,param_2);
  if (puStack_20[1] == 0) {
    uStack_30 = 0;
  }
  else {
    LOCK();
    piVar1 = (int *)(puStack_20[1] + 8);
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    uStack_30 = puStack_20[1];
  }
  uStack_38 = *puStack_20;
  FUN_145676b30(alStack_48[0] + 0x58,&uStack_38);
  lVar2 = puStack_20[1];
  if (lVar2 != 0) {
    LOCK();
    piVar1 = (int *)(lVar2 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar2);
      LOCK();
      piVar1 = (int *)(lVar2 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar2);
      }
    }
  }
  return;
}


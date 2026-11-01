
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

longlong FUN_14541b350(longlong param_1,undefined8 param_2,char param_3)

{
  int *piVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong *plVar4;
  longlong lVar5;
  undefined1 auStack_b0 [8];
  longlong lStack_a8;
  undefined1 auStack_80 [32];
  longlong lStack_60;
  char cStack_50;
  undefined8 *puStack_48;
  undefined8 *puStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lVar2 = FUN_140ab6b70(param_1 + 0x230);
  if (lVar2 != *(longlong *)(param_1 + 0x230)) {
    if (*(longlong *)(lVar2 + 0x60) == 0) {
      lVar5 = *(longlong *)(lVar2 + 0x58);
    }
    else {
      LOCK();
      piVar1 = (int *)(*(longlong *)(lVar2 + 0x60) + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      lVar5 = *(longlong *)(lVar2 + 0x58);
      lVar2 = *(longlong *)(lVar2 + 0x60);
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
    }
    if ((param_3 == '\0') && (lVar5 != 0)) {
      return lVar5;
    }
  }
  lVar2 = 0;
  FUN_140398fc0(auStack_80,param_2,0,0);
  if (cStack_50 == '\x01') {
    FUN_145a4c000(auStack_b0,auStack_80);
    puVar3 = (undefined8 *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,0x40);
    if (puVar3 == (undefined8 *)0x0) {
      FUN_1409bc820(0x40);
    }
    puVar3[1] = 0x100000001;
    *puVar3 = &UNK_14f09a340;
    FUN_145a4bd30(puVar3 + 2,auStack_b0);
    puStack_48 = puVar3 + 2;
    puStack_40 = puVar3;
    plVar4 = (longlong *)FUN_14095d880(param_1 + 0x230,param_2,&puStack_48);
    lVar2 = *plVar4;
    if (lStack_a8 != 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
    if ((cStack_50 == '\x01') && (lStack_60 != 0)) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
  }
  return lVar2;
}


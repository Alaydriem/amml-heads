
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

undefined8 FUN_14541b5a0(longlong param_1)

{
  int *piVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  
  if (*(char **)(param_1 + 8) == (char *)0x0) {
    _DAT_00000000 = 0xdeadc0de;
  }
  else if (**(char **)(param_1 + 8) == '\0') {
    _DAT_00000000 = 0xdeadc0de;
  }
  lVar2 = *(longlong *)(param_1 + 0x18);
  lVar3 = FUN_140ab6b70(lVar2 + 0x230);
  if (lVar3 == *(longlong *)(lVar2 + 0x230)) {
    uVar4 = 0;
  }
  else if (*(longlong *)(lVar3 + 0x60) == 0) {
    uVar4 = *(undefined8 *)(lVar3 + 0x58);
  }
  else {
    LOCK();
    piVar1 = (int *)(*(longlong *)(lVar3 + 0x60) + 8);
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    uVar4 = *(undefined8 *)(lVar3 + 0x58);
    lVar2 = *(longlong *)(lVar3 + 0x60);
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
  return uVar4;
}


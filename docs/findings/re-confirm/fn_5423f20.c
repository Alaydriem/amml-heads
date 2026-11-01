
void FUN_145423f20(longlong param_1,longlong *param_2,undefined8 param_3)

{
  int *piVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined1 auStack_60 [8];
  longlong lStack_58;
  undefined1 auStack_50 [8];
  longlong lStack_48;
  
  if (*param_2 != 0) {
    lVar3 = param_2[1];
    lVar4 = *param_2 * 0x38;
    do {
      lVar2 = FUN_140ab6b70((longlong *)(param_1 + 0x220),lVar3);
      if (lVar2 == *(longlong *)(param_1 + 0x220)) {
        FUN_145421920(param_1,auStack_50,lVar3,0,param_3,0);
        lVar2 = lStack_48;
      }
      else {
        FUN_145421920(param_1,auStack_60,lVar3,1,param_3,*(undefined8 *)(lVar2 + 0x58));
        lVar2 = lStack_58;
      }
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
      lVar3 = lVar3 + 0x38;
      lVar4 = lVar4 + -0x38;
    } while (lVar4 != 0);
  }
  return;
}


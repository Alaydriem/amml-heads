
void FUN_1457e2220(undefined8 *param_1)

{
  int *piVar1;
  longlong *plVar2;
  longlong lVar3;
  
  *param_1 = &UNK_14f261b60;
  plVar2 = (longlong *)param_1[0x15];
  if (plVar2 != (longlong *)0x0) {
    switch((char)plVar2[3]) {
    case '\x01':
      lVar3 = *plVar2;
      if (lVar3 != 0) {
        if (*(longlong *)(lVar3 + 8) != 0) {
          LOCK();
          piVar1 = (int *)(*(longlong *)(lVar3 + 8) + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          }
        }
        FUN_14e934b00(lVar3,0x20);
      }
      break;
    case '\x02':
    case '\x03':
      lVar3 = plVar2[2];
      if (lVar3 != 0) {
        LOCK();
        piVar1 = (int *)(lVar3 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
          LOCK();
          piVar1 = (int *)(lVar3 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
          }
        }
      }
    }
    FUN_14e934b00(plVar2,0x238);
  }
  if (param_1[0xe] != 0) {
    LOCK();
    piVar1 = (int *)(param_1[0xe] + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
  }
  lVar3 = param_1[8];
  if (lVar3 != 0) {
    LOCK();
    piVar1 = (int *)(lVar3 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3);
      LOCK();
      piVar1 = (int *)(lVar3 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar3,PTR__guard_dispatch_icall_14f99ef70);
        return;
      }
    }
  }
  return;
}

